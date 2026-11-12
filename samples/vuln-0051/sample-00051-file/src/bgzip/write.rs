//! Bgzip files (BGZF) writer.

use std::sync::{Arc, Weak, Mutex};
use std::sync::atomic::{
    AtomicBool,
    Ordering::Relaxed,
};
use std::collections::VecDeque;
use std::io::{self, Write, ErrorKind};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::path::Path;
use std::cmp::min;

use super::{Block, ObjectPool};
use super::{SLEEP_TIME, TIMEOUT};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct WorkerId(u16);

enum Task {
    Ready(Block, Result<(), io::Error>),
    NotReady(WorkerId),
}

impl Task {
    fn is_not_ready(&self, worker_id: WorkerId) -> bool {
        match self {
            Task::NotReady(task_worker_id) if *task_worker_id == worker_id => true,
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
    compression: flate2::Compression,
}

impl Worker {
    fn run(self) -> Self {
        'outer: while !self.finish.load(Relaxed) {
            let queue = match self.working_queue.upgrade() {
                Some(value) => value,
                // Writer was dropped.
                None => break,
            };

            let block = if let Ok(mut guard) = queue.lock() {
                if let Some(block) = guard.blocks.pop_front() {
                    guard.tasks.push_back(Task::NotReady(self.worker_id));
                    Some(block)
                } else {
                    None
                }
            } else {
                // Panic in another thread.
                break
            };

            let mut block = if let Some(value) = block {
                value
            } else {
                thread::sleep(SLEEP_TIME);
                continue;
            };

            let res = match block.compress(self.compression) {
                Err(ref e) if e.kind() == ErrorKind::WriteZero => {
                    // Compressed size is too big.
                    block.reset_compression();
                    let mut second_half = block.split_into_two();

                    let res1 = block.compress(self.compression);
                    let res2 = second_half.compress(self.compression);

                    if let Ok(mut guard) = queue.lock() {
                        // Insert two Ready Tasks.
                        for i in 0..guard.tasks.len() {
                            if guard.tasks[i].is_not_ready(self.worker_id) {
                                guard.tasks[i] = Task::Ready(block, res1);
                                guard.tasks.insert(i + 1, Task::Ready(second_half, res2));
                                continue 'outer;
                            }
                        }
                        panic!("Task handler not found for worker {}", self.worker_id.0);
                    } else {
                        // Panic in another thread.
                        break
                    };
                },
                res => res,
            };

            if let Ok(mut guard) = queue.lock() {
                for task in guard.tasks.iter_mut().rev() {
                    if task.is_not_ready(self.worker_id) {
                        *task = Task::Ready(block, res);
                        continue 'outer;
                    }
                }
                panic!("Task handler not found for worker {}", self.worker_id.0);
            } else {
                // Panic in another thread.
                break
            };
        }
        self
    }
}

trait CompressionQueue<W: Write> {
    /// Adds the next uncompressed block to the queue, and tries to write to the output.
    /// The function returns an empty block and the result of the writing.
    ///
    /// It is highly not recommended to try to rewrite a block if the function returned an error.
    /// In that case it is possible that the resulting stream would have missing blocks because
    /// * If the block is too big, it may be split in two and only one of them could be written,
    /// * For multi-thread writer, any of the previous blocks could raise an error.
    fn add_block_and_write(&mut self, block: Block, stream: &mut W) -> (Block, io::Result<()>);

    /// Flush contents to strem. Multi-thread writer would wait until all blocks are compressed
    /// unless it encounters an error.
    fn flush(&mut self, stream: &mut W) -> io::Result<()>;

    fn pause(&mut self);
}

struct SingleThread {
    compression: flate2::Compression,
}

impl SingleThread {
    fn new(compression: flate2::Compression) -> Self {
        Self { compression }
    }
}

impl<W: Write> CompressionQueue<W> for SingleThread {
    fn add_block_and_write(&mut self, mut block: Block, stream: &mut W) -> (Block, io::Result<()>) {
        match block.compress(self.compression) {
            Ok(()) => {},
            Err(ref e) if e.kind() == ErrorKind::WriteZero => {
                // Compressed size is too big.
                block.reset_compression();
                let second_half = block.split_into_two();
                if let Err(e) = block.dump(stream) {
                    block.reset();
                    return (block, Err(e));
                }
                block = second_half;
            },
            Err(e) => {
                block.reset();
                return (block, Err(e));
            },
        }

        let res = block.dump(stream);
        block.reset();
        (block, res)
    }

    fn flush(&mut self, stream: &mut W) -> io::Result<()> {
        stream.flush()
    }

    fn pause(&mut self) {}
}

struct MultiThread {
    working_queue: Arc<Mutex<WorkingQueue>>,
    finish: Arc<AtomicBool>,
    blocks_pool: ObjectPool<Block>,
    worker_handles: Vec<thread::JoinHandle<Worker>>,
}

impl MultiThread {
    /// Creates a multi-thread writer from a stream.
    fn new(threads: u16, compression: flate2::Compression) -> Self {
        assert!(threads > 0);
        let working_queue = Arc::new(Mutex::new(WorkingQueue::default()));
        let finish = Arc::new(AtomicBool::new(false));
        let worker_handles = (0..threads).map(|i| {
            let worker = Worker {
                worker_id: WorkerId(i),
                working_queue: Arc::downgrade(&working_queue),
                finish: Arc::clone(&finish),
                compression,
            };
            thread::Builder::new()
                .name(format!("bgzip_write{}", i + 1))
                .spawn(|| worker.run())
                .expect("Cannot create a thread")
        }).collect();

        Self {
            working_queue,
            finish,
            blocks_pool: ObjectPool::new(|| Block::new()),
            worker_handles,
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

    fn write_compressed<W: Write>(&mut self, stream: &mut W, stop_if_not_ready: bool) -> io::Result<()> {
        if self.finish.load(Relaxed) {
            self.restart_workers();
        }

        let mut time_waited = Duration::new(0, 0);
        loop {
            let queue_top = if let Ok(mut guard) = self.working_queue.lock() {
                let need_pop = match guard.tasks.get(0) {
                    Some(Task::NotReady(_)) => {
                        if stop_if_not_ready {
                            return Ok(());
                        } else {
                            false
                        }
                    },
                    Some(Task::Ready(_, _)) => true,
                    None => {
                        if guard.blocks.is_empty() {
                            return Ok(());
                        } else {
                            false
                        }
                    },
                };

                if need_pop {
                    match guard.tasks.pop_front() {
                        Some(Task::Ready(block, res)) => Some((block, res)),
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            } else {
                return Err(io::Error::new(ErrorKind::Other, "Panic in one of the threads"));
            };

            if queue_top.is_none() {
                thread::sleep(SLEEP_TIME);
                time_waited += SLEEP_TIME;
                if time_waited > TIMEOUT {
                    return Err(io::Error::new(ErrorKind::TimedOut,
                        format!("Compression takes more than {:?}", TIMEOUT)));
                }
                continue;
            }

            time_waited = Duration::new(0, 0);
            let (block, res) = queue_top.unwrap();
            let res = res.and_then(|_| block.dump(stream));
            self.blocks_pool.bring(block);
            if res.is_err() {
                return res;
            }
        }
    }
}

impl<W: Write> CompressionQueue<W> for MultiThread {
    fn add_block_and_write(&mut self, block: Block, stream: &mut W) -> (Block, io::Result<()>) {
        if let Ok(mut guard) = self.working_queue.lock() {
            guard.blocks.push_back(block);
        } else {
            return (block, Err(io::Error::new(ErrorKind::Other, "Panic in one of the threads")));
        };

        let res = self.write_compressed(stream, true);
        let mut block = self.blocks_pool.take();
        block.reset();
        (block, res)
    }

    fn flush(&mut self, stream: &mut W) -> io::Result<()> {
        self.write_compressed(stream, false)?;
        stream.flush()
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

/// [Bgzip writer](struct.Writer.html) builder.
/// Allows to specify compression level and the number of additional threads.
pub struct WriterBuilder {
    additional_threads: u16,
    compression: flate2::Compression,
}

impl WriterBuilder {
    pub fn new() -> Self {
        Self {
            additional_threads: 0,
            compression: flate2::Compression::new(6),
        }
    }

    /// Specify the number of additional threads.
    /// Additional threads are used to compress blocks, while the
    /// main thread reads the writes to a file/stream.
    /// If `additional_threads` is 0 (default), the main thread
    /// will compress blocks itself.
    pub fn additional_threads(&mut self, additional_threads: u16) -> &mut Self {
        self.additional_threads = additional_threads;
        self
    }

    /// Specify compression level from 0 to 9, where 0 represents no compression,
    /// and 9 represents maximal compression. The builder uses 6 as default.
    pub fn compression_level(&mut self, level: u8) -> &mut Self {
        assert!(level <= 9, "Maximal compression level is 9");
        self.compression = flate2::Compression::new(level as u32);
        self
    }

    /// Creates a writer from file.
    pub fn from_path<P: AsRef<Path>>(&self, path: P) -> io::Result<Writer<File>> {
        let file = File::create(path)?;
        Ok(self.from_stream(file))
    }

    /// Creates a writer from stream.
    pub fn from_stream<W: Write>(&self, stream: W) -> Writer<W> {
        let compressor: Box<dyn CompressionQueue<W>> = if self.additional_threads == 0 {
            Box::new(SingleThread::new(self.compression))
        } else {
            Box::new(MultiThread::new(self.additional_threads, self.compression))
        };
        Writer::new(stream, compressor)
    }
}

// Struct that supports a temporary move of a value.
struct Moveout<T> {
    value: Option<T>,
}

impl<T> Moveout<T> {
    fn new(value: T) -> Self {
        Self {
            value: Some(value),
        }
    }

    fn take(&mut self) -> T {
        self.value.take().expect("Value should be defined")
    }

    fn set(&mut self, value: T) {
        self.value = Some(value);
    }

    fn is_defined(&self) -> bool {
        self.value.is_some()
    }
}

impl<T> std::ops::Deref for Moveout<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value.as_ref().expect("Value should be defined")
    }
}

impl<T> std::ops::DerefMut for Moveout<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().expect("Value should be defined")
    }
}

/// Writes bgzip file.
///
/// You can create a writer using [from_path](#method.from_path) or
/// using [WriterBuilder](struct.WriterBuilder.html).
/// Additional threads are used to compress blocks, while the
/// main thread reads the writes to a file/stream. If `additional_threads` is 0, the main thread
/// will compress blocks itself.
///
/// It is highly not recommended to continue writing after an error, as in that case the writer
/// may miss some blocks.
pub struct Writer<W: Write> {
    stream: W,
    compressor: Box<dyn CompressionQueue<W>>,
    block: Moveout<Block>,
    context_end: usize,
    buffer: Vec<u8>,
    was_error: bool,
}

impl Writer<File> {
    /// Creates a [Writer Builder](struct.WriterBuilder.html).
    pub fn build() -> WriterBuilder {
        WriterBuilder::new()
    }

    /// Opens a writer from a file with default parameters
    /// (see [Writer Builder](struct.WriterBuilder.html)).
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        WriterBuilder::new().from_path(path)
    }
}

impl<W: Write> Writer<W> {
    /// Pauses multi-thread writer this  and does nothing for single-thread writer.
    ///
    /// Practically, this function increases sleeping time for compressing threads, so the threads are still active,
    /// but wake up rarely.
    pub fn pause(&mut self) {
        self.compressor.pause();
    }
}

/// Maximal buffer size to avoid problems with too big compressed size.
const MAX_BUFFER_SIZE: usize = super::MAX_BLOCK_SIZE - 100;
const MAX_TAIL_SIZE: usize = 10000;
/// Minimal allowed block size (unless forced). If last context end is less than MIN_BUFFER_SIZE,
/// context will be ignored.
const MIN_BUFFER_SIZE: usize = MAX_BUFFER_SIZE - MAX_TAIL_SIZE;

impl<W: Write> Writer<W> {
    fn new(stream: W, compressor: Box<dyn CompressionQueue<W>>) -> Self {
        Self {
            stream, compressor,
            block: Moveout::new(Block::new()),
            context_end: 0,
            buffer: vec![0; MAX_TAIL_SIZE],
            was_error: false,
        }
    }

    /// Ends current context: marks this point as more preferable when
    /// splitting bgzip blocks.
    pub fn end_context(&mut self) {
        self.context_end = self.block.uncompressed_size() as usize;
    }

    /// Saves current contents into a block and adds to the queue.
    fn save_current_block(&mut self) -> io::Result<()> {
        let block = self.block.take();
        let (block, res) = self.compressor.add_block_and_write(block, &mut self.stream);
        self.block.set(block);
        self.context_end = 0;
        res
    }

    /// Saves current contents (if non-empty) into a block and adds to the compression queue.
    pub fn flush_contents(&mut self) -> io::Result<()> {
        if self.block.uncompressed_size() > 0 {
            self.save_current_block()
        } else {
            Ok(())
        }
    }

    /// Adds a block to the compression queue and returns an empty block and
    /// a result of compression/writing.
    ///
    /// If you use [write](#method.write) as well as [write_block](#method.write_block),
    /// call [flush_contents](#method.flush_contents) before using `write_block`.
    pub fn write_block(&mut self, block: Block) -> (Block, io::Result<()>) {
        self.compressor.add_block_and_write(block, &mut self.stream)
    }

    /// Saves current contents into a block, and adds an empty block to the queue.
    pub fn write_empty(&mut self) -> io::Result<()> {
        self.was_error = true;
        self.flush_contents()?;
        self.save_current_block()?;
        self.was_error = false;
        Ok(())
    }

    /// Finishes writing (writes an empty block and flushes contents).
    pub fn finish(&mut self) -> io::Result<()> {
        self.was_error = true;
        self.write_empty()?;
        self.flush()
    }

    /// Finishes writing, consumes the writer and returns inner stream.
    pub fn take_stream(mut self) -> W {
        if !self.was_error && self.block.is_defined() {
            let _ignore = self.finish();
        }
        self.was_error = true;
        unsafe {
            std::mem::replace(&mut self.stream, std::mem::MaybeUninit::uninit().assume_init())
        }
    }
}

impl<W: Write> Write for Writer<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.was_error = true;
        let block_size = self.block.uncompressed_size() as usize;
        if block_size < MAX_BUFFER_SIZE {
            let write_bytes = min(MAX_BUFFER_SIZE - block_size, buf.len());
            self.block.extend_contents(&buf[..write_bytes]);
            self.was_error = false;
            return Ok(write_bytes);
        }

        let buffer_size = if self.context_end >= MIN_BUFFER_SIZE {
            self.block.split_contents(self.context_end, &mut self.buffer)
        } else {
            0
        };

        let res = self.save_current_block();
        if buffer_size != 0 {
            self.block.extend_contents(&self.buffer[..buffer_size]);
        }
        res?;

        let write_bytes = min(MAX_BUFFER_SIZE - buffer_size, buf.len());
        self.block.extend_contents(&buf[..write_bytes]);
        self.was_error = false;
        Ok(write_bytes)
    }

    /// Saves current buffer to a block and writes all blocks in a queue.
    fn flush(&mut self) -> io::Result<()> {
        self.was_error = true;
        self.flush_contents()?;
        self.compressor.flush(&mut self.stream)?;
        self.was_error = false;
        Ok(())
    }
}

impl<W: Write> Drop for Writer<W> {
    fn drop(&mut self) {
        if !self.was_error && self.block.is_defined() {
            let _ignore = self.finish();
        }
    }
}
