//! Bgzip files (BGZF) readers.
//!
//! The module contains two readers: [Consecutive Reader](struct.ConsecutiveReader.html)
//! and [Seek Reader](struct.SeekReader.html). Both readers have abilities to decompress blocks
//! using additional threads.
//!
//! Use [ReadBgzip](trait.ReadBgzip.html) trait if you wish to read blocks directly
//! (not via `io::Read`).

use std::sync::{Arc, Weak, Mutex};
use std::sync::atomic::{
    AtomicBool,
    Ordering::Relaxed,
};
use std::collections::VecDeque;
use std::io::{self, Read, ErrorKind, Seek, SeekFrom};
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::fs::File;
use std::cmp::{min, max};

use crate::index::{Chunk, VirtualOffset};
use super::{Block, BlockError, ObjectPool};
use super::{SLEEP_TIME, TIMEOUT};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct WorkerId(u16);

#[derive(Clone, Copy, PartialEq, Eq)]
enum TaskStatus {
    Waiting,
    Interrupted,
}

enum Task {
    Ready(Block, Result<(), BlockError>),
    // Second element = true if the block is no longer needed.
    NotReady(WorkerId, TaskStatus),
    Interrupted(Block),
}

impl Task {
    fn is_ready(&self) -> bool {
        match self {
            Task::Ready(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Default)]
struct WorkingQueue {
    blocks: VecDeque<Block>,
    tasks: VecDeque<Task>,
}

struct Worker {
    worker_id: WorkerId,
    working_queue: Weak<Mutex<WorkingQueue>>,
    finish: Arc<AtomicBool>,
}

impl Worker {
    fn run(self) -> Self {
        'outer: while !self.finish.load(Relaxed) {
            let queue = match self.working_queue.upgrade() {
                Some(value) => value,
                // Reader was dropped
                None => break,
            };

            let block = if let Ok(mut guard) = queue.lock() {
                if let Some(block) = guard.blocks.pop_front() {
                    guard.tasks.push_back(Task::NotReady(self.worker_id, TaskStatus::Waiting));
                    Some(block)
                } else {
                    None
                }
            } else {
                // Panic in another thread
                break
            };

            let mut block = if let Some(value) = block {
                value
            } else {
                thread::sleep(SLEEP_TIME);
                continue;
            };

            let res = block.decompress();
            if let Ok(mut guard) = queue.lock() {
                for task in guard.tasks.iter_mut().rev() {
                    match task {
                        Task::NotReady(worker_id, task_status)
                                if *worker_id == self.worker_id => {
                            let new_value = if *task_status == TaskStatus::Waiting {
                                Task::Ready(block, res)
                            } else {
                                Task::Interrupted(block)
                            };
                            *task = new_value;
                            continue 'outer;
                        },
                        _ => {},
                    }
                }
                panic!("Task handler not found for worker {}", self.worker_id.0);
            } else {
                // Panic in another thread
                break
            };
        }
        self
    }
}

trait ReadBlock {
    fn read_next(&mut self, block: &mut Block) -> Result<(), BlockError>;
}

struct ConsecutiveReadBlock<R: Read> {
    stream: R,
    offset: u64,
}

impl<R: Read> ConsecutiveReadBlock<R> {
    fn new(stream: R) -> Self {
        Self {
            stream,
            offset: 0,
        }
    }

    fn take_stream(self) -> R {
        self.stream
    }
}

impl<R: Read> ReadBlock for ConsecutiveReadBlock<R> {
    fn read_next(&mut self, block: &mut Block) -> Result<(), BlockError> {
        block.reset();
        block.load(Some(self.offset), &mut self.stream)?;
        self.offset += block.block_size().expect("Block size should be already defined") as u64;
        Ok(())
    }
}

struct JumpingReadBlock<R: Read + Seek> {
    stream: R,
    offset: u64,
    chunks: Vec<Chunk>,
    index: usize,
    started: bool,
}

impl<R: Read + Seek> JumpingReadBlock<R> {
    fn new(mut stream: R) -> io::Result<Self> {
        let offset = stream.seek(SeekFrom::Current(0))?;
        Ok(Self {
            stream, offset,
            chunks: Vec::new(),
            index: 0,
            started: false,
        })
    }

    fn set_chunks<I: IntoIterator<Item = Chunk>>(&mut self, chunks: I) {
        self.chunks.clear();
        self.chunks.extend(chunks);
        for i in 1..self.chunks.len() {
            if self.chunks[i - 1].intersect(&self.chunks[i]) {
                panic!("Cannot set chunks: chunk {:?} intersects chunk {:?}",
                    self.chunks[i - 1], self.chunks[i]);
            } else if self.chunks[i - 1] >= self.chunks[i] {
                panic!("Cannot set chunks: chunks are unordered: {:?} >= {:?}",
                    self.chunks[i - 1], self.chunks[i]);
            }
        }
        self.index = 0;
        self.started = false;
    }

    fn next_offset(&mut self) -> Option<u64> {
        if self.index >= self.chunks.len() {
            return None;
        }
        if !self.started {
            self.started = true;
            return Some(self.chunks[0].start().block_offset());
        }

        let curr_offset = VirtualOffset::new(self.offset, 0);
        while self.index < self.chunks.len() && curr_offset >= self.chunks[self.index].end() {
            self.index += 1;
        }
        if self.index >= self.chunks.len() {
            None
        } else {
            Some(max(self.offset, self.chunks[self.index].start().block_offset()))
        }
    }

    fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn take_stream(self) -> R {
        self.stream
    }
}

impl<R: Read + Seek> ReadBlock for JumpingReadBlock<R> {
    fn read_next(&mut self, block: &mut Block) -> Result<(), BlockError> {
        if let Some(new_offset) = self.next_offset() {
            if new_offset != self.offset {
                self.stream.seek(SeekFrom::Start(new_offset))?;
                self.offset = new_offset;
            }
            block.reset();
            block.load(Some(self.offset), &mut self.stream)?;
            self.offset += block.block_size().expect("Block size should be already defined") as u64;
            Ok(())
        } else {
            Err(BlockError::EndOfStream)
        }
    }
}

trait DecompressBlock<T: ReadBlock> {
    fn decompress_next(&mut self, reader: &mut T) -> Result<&Block, BlockError>;
    fn get_current(&self) -> Option<&Block>;
    fn reset_queue(&mut self);
    fn pause(&mut self);
}

struct SingleThread {
    block: Block,
    was_error: bool,
}

impl SingleThread {
    fn new() -> Self {
        Self {
            block: Block::new(),
            was_error: true,
        }
    }
}

impl<T: ReadBlock> DecompressBlock<T> for SingleThread {
    fn decompress_next(&mut self, reader: &mut T) -> Result<&Block, BlockError> {
        self.was_error = true;
        reader.read_next(&mut self.block)?;
        self.block.decompress()?;
        self.was_error = false;
        Ok(&self.block)
    }

    fn get_current(&self) -> Option<&Block> {
        if self.was_error {
            None
        } else {
            Some(&self.block)
        }
    }

    fn reset_queue(&mut self) {}

    fn pause(&mut self) {}
}

struct MultiThread {
    working_queue: Arc<Mutex<WorkingQueue>>,
    finish: Arc<AtomicBool>,
    blocks_pool: ObjectPool<Block>,
    worker_handles: Vec<thread::JoinHandle<Worker>>,
    reached_end: bool,
    current_block: Block,
    was_error: bool,
}

impl MultiThread {
    /// Creates a multi-thread reader from a stream.
    fn new(threads: u16) -> Self {
        assert!(threads > 0);
        let working_queue = Arc::new(Mutex::new(WorkingQueue::default()));
        let finish = Arc::new(AtomicBool::new(false));
        let worker_handles = (0..threads).map(|i| {
            let worker = Worker {
                worker_id: WorkerId(i),
                working_queue: Arc::downgrade(&working_queue),
                finish: Arc::clone(&finish),
            };
            thread::Builder::new()
                .name(format!("bgzip_read{}", i + 1))
                .spawn(|| worker.run())
                .expect("Cannot create a thread")
        }).collect();

        Self {
            working_queue,
            finish,
            blocks_pool: ObjectPool::new(|| Block::new()),
            worker_handles,
            reached_end: false,
            current_block: Block::new(),
            was_error: true,
        }
    }

    fn restart_workers(&mut self) {
        if !self.finish.load(Relaxed) {
            return;
        }
        let workers = self.worker_handles.drain(..).map(|thread| thread.join())
            .collect::<Result<Vec<Worker>, _>>()
            .unwrap_or_else(|e| panic!("Panic in one of the threads: {:?}", e));
        self.finish.store(false, Relaxed);
        for worker in workers {
            self.worker_handles.push(thread::Builder::new()
                .name(format!("bgzip_write{}", worker.worker_id.0 + 1))
                .spawn(|| worker.run())
                .expect("Cannot create a thread"));
        }
    }
}

impl<T: ReadBlock> DecompressBlock<T> for MultiThread {
    fn decompress_next(&mut self, reader: &mut T) -> Result<&Block, BlockError> {
        if self.finish.load(Relaxed) {
            self.restart_workers();
        }

        self.was_error = true;
        let blocks_to_read = if self.reached_end {
            0
        } else if let Ok(guard) = self.working_queue.lock() {
            let ready_tasks = guard.tasks.iter().filter(|task| task.is_ready()).count();
            self.worker_handles.len().saturating_sub(std::cmp::max(guard.blocks.len(), ready_tasks))
        } else {
            return Err(BlockError::IoError(io::Error::new(ErrorKind::Other,
                "Panic in one of the threads")));
        };

        for _ in 0..blocks_to_read {
            let mut block = self.blocks_pool.take();
            match reader.read_next(&mut block) {
                Err(BlockError::EndOfStream) => {
                    self.reached_end = true;
                    self.blocks_pool.bring(block);
                    break;
                },
                Err(e) => {
                    self.blocks_pool.bring(block);
                    return Err(e)
                },
                Ok(()) => {},
            }
            if let Ok(mut guard) = self.working_queue.lock() {
                guard.blocks.push_back(block);
            } else {
                self.blocks_pool.bring(block);
                return Err(BlockError::IoError(io::Error::new(ErrorKind::Other,
                    "Panic in one of the threads")));
            }
        }

        let mut time_waited = Duration::new(0, 0);
        let (block, result) = loop {
            if let Ok(mut guard) = self.working_queue.lock() {
                let need_pop = match guard.tasks.get(0) {
                    Some(Task::Ready(_, _)) => true,
                    Some(Task::NotReady(_, _)) => false,
                    Some(Task::Interrupted(_)) => true,
                    None => {
                        if guard.blocks.is_empty() {
                            assert!(self.reached_end);
                            return Err(BlockError::EndOfStream);
                        } else {
                            false
                        }
                    },
                };
                if need_pop {
                    match guard.tasks.pop_front() {
                        Some(Task::Ready(block, res)) => break (block, res),
                        Some(Task::Interrupted(block)) => self.blocks_pool.bring(block),
                        _ => unreachable!(),
                    }
                }
            } else {
                return Err(BlockError::IoError(io::Error::new(ErrorKind::Other,
                    "Panic in one of the threads")));
            }

            thread::sleep(SLEEP_TIME);
            time_waited += SLEEP_TIME;
            if time_waited > TIMEOUT {
                return Err(BlockError::IoError(io::Error::new(ErrorKind::TimedOut,
                    format!("Decompression takes more than {:?}", TIMEOUT))));
            }
        };

        match result {
            Ok(()) => {
                self.blocks_pool.bring(std::mem::replace(&mut self.current_block, block));
                self.was_error = false;
                Ok(&self.current_block)
            },
            Err(e) => {
                self.blocks_pool.bring(block);
                Err(e)
            },
        }
    }

    fn get_current(&self) -> Option<&Block> {
        if self.was_error {
            None
        } else {
            Some(&self.current_block)
        }
    }

    fn reset_queue(&mut self) {
        self.reached_end = false;
        self.was_error = true;
        match self.working_queue.lock() {
            Ok(mut guard) => {
                guard.blocks.clear();
                for _ in 0..guard.tasks.len() {
                    match guard.tasks.pop_front().unwrap() {
                        Task::Ready(block, _) => self.blocks_pool.bring(block),
                        Task::NotReady(worker_id, _) => guard.tasks.push_back(
                            Task::NotReady(worker_id, TaskStatus::Interrupted)),
                        Task::Interrupted(block) => self.blocks_pool.bring(block),
                    }
                }
            },
            Err(e) => panic!("Panic in one of the threads: {:?}", e),
        }
    }

    fn pause(&mut self) {
        self.finish.store(true, Relaxed);
    }
}

impl Drop for MultiThread {
    fn drop(&mut self) {
        self.finish.store(true, Relaxed);
    }
}

/// A trait that allows to read blocks directly.
pub trait ReadBgzip {
    /// Reads and returns [Block](../struct.Block.html). If no blocks present, the function returns
    /// [BlockError::EndOfStream](../enum.BlockError.html#variant.EndOfStream).
    ///
    /// The function returns a reference to the block, not the block itself, to reuse it later,
    /// however you can clone the block, if needed.
    fn next(&mut self) -> Result<&Block, BlockError>;

    /// Returns the current block, if possible. This function does not advance the stream.
    fn current(&self) -> Option<&Block>;

    /// Pauses multi-thread reader and does nothing for single-thread reader.
    ///
    /// This function increases sleeping time for decompressing threads, so the threads are still active,
    /// but wake up rarely.
    fn pause(&mut self);
}

/// A bgzip reader, that allows to jump between blocks.
///
/// You can open the reader using [from_path](#method.from_path) or
/// [from_stream](#method.from_stream).
/// Additional threads are used to decompress blocks, while the
/// main thread reads the blocks from a file/stream. If `additional_threads` is 0, the main thread
/// will decompress blocks itself.
///
/// When this reader is opened, it does read not anything, until you specify reading regions.
/// You can do that by using [Seek](#impl-Seek) trait, but it is recommended to use methods
/// [set_chunks](#method.set_chunks), [make_consecutive](#method.make_consecutive) and
/// [from_offset](#method.from_offset). Note, that in multi-threaded `SeekReader` it is important to
/// correctly set future reading queue (use `set_chunks` instead of running `from_offset` multiple times).
///
/// You can read the contents using `io::Read`,
/// or read blocks using [ReadBgzip](trait.ReadBgzip.html) trait.
pub struct SeekReader<R: Read + Seek> {
    decompressor: Box<dyn DecompressBlock<JumpingReadBlock<R>>>,
    reader: JumpingReadBlock<R>,
    chunks_index: usize,
    started: bool,
    contents_offset: usize,
}

impl SeekReader<File> {
    /// Opens a reader from a file.
    pub fn from_path<P: AsRef<Path>>(path: P, additional_threads: u16) -> io::Result<Self> {
        let file = File::open(path)?;
        Self::from_stream(file, additional_threads)
    }
}

impl<R: Read + Seek> SeekReader<R> {
    /// Opens a reader from a stream.
    pub fn from_stream(stream: R, additional_threads: u16) -> io::Result<Self> {
        let reader = JumpingReadBlock::new(stream)?;
        let decompressor: Box<dyn DecompressBlock<_>> = if additional_threads == 0 {
            Box::new(SingleThread::new())
        } else {
            Box::new(MultiThread::new(additional_threads))
        };
        Ok(Self {
            decompressor,
            reader,
            chunks_index: 0,
            started: false,
            contents_offset: 0,
        })
    }

    /// Sets the current chunks. Each [chunk](../..//index/struct.Chunk.html) specifies a region of
    /// the bgzip file. The multi-thread reader reads and decompresses blocks from the `chunks`
    /// in advance (but does not immediately read all blocks).
    ///
    /// This function resets the current reading queue (from the previous `set_chunks` or
    /// `make_consecutive` calls).
    ///
    /// `chunks` must be sorted and should not intersect each other.
    pub fn set_chunks<I: IntoIterator<Item = Chunk>>(&mut self, chunks: I) {
        self.reader.set_chunks(chunks);
        self.decompressor.reset_queue();
        self.chunks_index = 0;
        self.started = false;
        self.contents_offset = 0;
    }

    /// Sets the reader in a consecutive mode starting with offset 0, and continuing until the end of the stream.
    ///
    /// This function resets the current reading queue.
    pub fn make_consecutive(&mut self) {
        self.reader.set_chunks(vec![Chunk::new(VirtualOffset::MIN, VirtualOffset::MAX)])
    }

    /// Sets the reader in a consecutive mode starting with `offset`, and continuing until the end of the stream.
    ///
    /// This function resets the current reading queue.
    pub fn from_offset(&mut self, offset: VirtualOffset) {
        self.reader.set_chunks(vec![Chunk::new(offset, VirtualOffset::MAX)])
    }

    /// Consumes the reader and returns inner stream.
    pub fn take_stream(self) -> R {
        self.reader.take_stream()
    }

    /// Returns current [virtual offset](../../index/struct.VirtualOffset.html).
    ///
    /// If the reader has not started, returns the start of the first chunk.
    /// If the reader finished the current queue, returns the end of the last chunk.
    pub fn current_offset(&self) -> VirtualOffset {
        if !self.started {
            // If there is no reading queue, returns zero offset.
            return self.reader.chunks().first().map(|chunk| chunk.start()).unwrap_or(VirtualOffset::MIN);
        }

        let block = match self.current() {
            Some(value) => value,
            // Reader reached the end of the reading queue.
            None => return self.reader.chunks.last().map(|chunk| chunk.end()).unwrap_or(VirtualOffset::MAX),
        };
        VirtualOffset::new(block.offset().unwrap(), self.contents_offset as u16)
    }
}

impl<R: Read + Seek> ReadBgzip for SeekReader<R> {
    /// Reads the next block in a queue. Note, that if the `chunks` vector contain the same block
    /// twice, it will be read only once.
    fn next(&mut self) -> Result<&Block, BlockError> {
        self.started = true;
        let block = self.decompressor.decompress_next(&mut self.reader)?;
        let block_offset = VirtualOffset::new(block.offset().unwrap(), 0);
        let chunks = self.reader.chunks();
        if block_offset >= chunks[self.chunks_index].end() {
            self.chunks_index += 1;
        }
        self.contents_offset = max(block_offset, chunks[self.chunks_index].start()).contents_offset() as usize;
        Ok(block)
    }

    fn current(&self) -> Option<&Block> {
        self.decompressor.get_current()
    }

    fn pause(&mut self) {
        self.decompressor.pause()
    }
}

impl<R: Read + Seek> Read for SeekReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.started {
            match self.next() {
                Ok(_) => {},
                Err(BlockError::EndOfStream) => return Ok(0),
                Err(e) => return Err(e.into()),
            }
        }
        loop {
            let block = match self.current() {
                Some(value) => value,
                None => return Ok(0),
            };

            let block_offset = block.offset().expect("Block size should be already defined");
            let end_offset = self.reader.chunks()[self.chunks_index].end();
            let contents_end = if block_offset < end_offset.block_offset() {
                block.uncompressed_size() as usize
            } else {
                assert!(block_offset == end_offset.block_offset());
                end_offset.contents_offset() as usize
            };
            if self.contents_offset < contents_end {
                let read_bytes = min(contents_end - self.contents_offset, buf.len());
                buf[..read_bytes].copy_from_slice(
                    &block.uncompressed_data()[self.contents_offset..self.contents_offset + read_bytes]);
                std::mem::drop(block);
                self.contents_offset += read_bytes;
                return Ok(read_bytes)
            }
            std::mem::drop(block);

            let chunks = self.reader.chunks();
            let read_next = if block_offset == end_offset.block_offset() {
                self.chunks_index += 1;
                self.chunks_index >= chunks.len() || chunks[self.chunks_index].start().block_offset() != block_offset
            } else {
                true
            };

            if read_next {
                match self.next() {
                    Ok(_) => {},
                    Err(BlockError::EndOfStream) => return Ok(0),
                    Err(e) => return Err(e.into()),
                }
            }
        }
    }
}

impl<R: Read + Seek> Seek for SeekReader<R> {
    /// Seek to a block with a given block offset, and return new block offset.
    ///
    /// This function is not implemented for `SeekFrom::End` because it will increase running time and
    /// is rarely needed.
    ///
    /// If the offset is positive, but incorrect (out of bounds or does not point to a start of a block), this function
    /// will return `Ok`, but consecutive `read` run will return an error.
    ///
    /// **Note**: consider running functions [from_offset](#method.from_offset),
    /// [make_consecutive](#method.make_consecutive) or [set_chunks](#method.set_chunks), as they are more explicit.
    /// Additionally, `set_chunks` works faster in a multi-threaded reader.
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(offset) => {
                self.from_offset(VirtualOffset::new(offset, 0));
                Ok(offset)
            }
            SeekFrom::Current(offset) => {
                let curr_offset = self.current_offset();
                let new_offset = curr_offset.block_offset() as i64 + offset;
                if new_offset < 0 {
                    Err(io::Error::new(ErrorKind::InvalidInput, "invalid seek to a negative or overflowing position"))
                } else {
                    let new_offset = new_offset as u64;
                    self.from_offset(VirtualOffset::new(new_offset, 0));
                    Ok(new_offset)
                }
            },
            SeekFrom::End(_) => Err(io::Error::new(ErrorKind::InvalidInput,
                "SeekReader::seek not implemented for SeekFrom::End")),
        }
    }
}

/// Reads bgzip file in a consecutive mode. Therefore, the stream does not have to implement `io::Seek`.
///
/// You can open the reader using [from_path](#method.from_path) or
/// [from_stream](#method.from_stream).
/// Additional threads are used to decompress blocks, while the
/// main thread reads the blocks from a file/stream. If `additional_threads` is 0, the main thread
/// will decompress blocks itself.
///
/// You can read the contents using `io::Read`,
/// or read blocks using [ReadBgzip](trait.ReadBgzip.html) trait.
pub struct ConsecutiveReader<R: Read> {
    decompressor: Box<dyn DecompressBlock<ConsecutiveReadBlock<R>>>,
    reader: ConsecutiveReadBlock<R>,
    contents_offset: usize,
    started: bool,
}

impl ConsecutiveReader<File> {
    /// Opens a reader from a file.
    pub fn from_path<P: AsRef<Path>>(path: P, additional_threads: u16) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self::from_stream(file, additional_threads))
    }
}

impl<R: Read> ConsecutiveReader<R> {
    /// Opens a reader from a stream.
    pub fn from_stream(stream: R, additional_threads: u16) -> Self {
        let reader = ConsecutiveReadBlock::new(stream);
        let decompressor: Box<dyn DecompressBlock<_>> = if additional_threads == 0 {
            Box::new(SingleThread::new())
        } else {
            Box::new(MultiThread::new(additional_threads))
        };
        Self {
            decompressor, reader,
            contents_offset: 0,
            started: false,
        }
    }

    /// Consumes the reader and returns inner stream.
    pub fn take_stream(self) -> R {
        self.reader.take_stream()
    }
}

impl<R: Read> ReadBgzip for ConsecutiveReader<R> {
    fn next(&mut self) -> Result<&Block, BlockError> {
        self.started = true;
        self.contents_offset = 0;
        self.decompressor.decompress_next(&mut self.reader)
    }

    fn current(&self) -> Option<&Block> {
        self.decompressor.get_current()
    }

    fn pause(&mut self) {
        self.decompressor.pause()
    }
}

impl<R: Read> Read for ConsecutiveReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.started {
            match self.next() {
                Ok(_) => {},
                Err(BlockError::EndOfStream) => return Ok(0),
                Err(e) => return Err(e.into()),
            }
        } else {
            let block = match self.current() {
                Some(value) => value,
                None => return Ok(0),
            };

            let read_bytes = min(block.uncompressed_size() as usize - self.contents_offset,
                buf.len());
            if read_bytes > 0 {
                buf[..read_bytes].copy_from_slice(&block.uncompressed_data()
                    [self.contents_offset..self.contents_offset + read_bytes]);
                std::mem::drop(block);
                self.contents_offset += read_bytes;
                return Ok(read_bytes)
            }
            match self.next() {
                Ok(_) => {},
                Err(BlockError::EndOfStream) => return Ok(0),
                Err(e) => return Err(e.into()),
            }
        }

        let block = self.current().expect("Block cannot be None here");
        let read_bytes = min(block.uncompressed_size() as usize, buf.len());
        buf[..read_bytes].copy_from_slice(&block.uncompressed_data()[..read_bytes]);
        std::mem::drop(block);
        self.contents_offset = read_bytes;
        Ok(read_bytes)
    }
}
