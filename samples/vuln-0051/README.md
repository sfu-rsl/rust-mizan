# Vulnerability: CVE-2021-28027

| **Information**       | **Details**                                                                                  |
| --------------------- | -------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2021-28027](https://www.cve.org/CVERecord?id=CVE-2021-28027)                            |
| **Vulnerable Commit** | [82de30b5](https://gitlab.com/tprodanov/bam/-/tree/856c9432946d8bfe7ebb5e7bd8adf66f64a8bd5c) |
| **Fixed Commit**      | [061eee38](https://gitlab.com/tprodanov/bam/-/tree/061eee38d47c96339b796c943d3163235490532d) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                   |
|                       | - [vuln-file](vuln-file)                                                                     |
|                       | - [vuln-function](vuln-file)                                                                 |
|                       | - [fixed-crate](fixed-crate)                                                                 |
|                       | - [fixed-file](fixed-file)                                                                   |
|                       | - [fixed-function](fixed-file)                                                               |

### Vulnerable Lines

`src/bgzip/mod.rs`

```rust
// There are two issues as follows:
// 1. This function passes uninitialized buffer to read_exact function of trait Read. (Possible user defined read function)
// 2. This function increases the length of the buffer without reserving extra memory. This side effect happens due to an integer underflow.

//Vulnerable function
pub fn load<R: Read>(&mut self, offset: Option<u64>, stream: &mut R) -> Result<(), BlockError> {
        assert!(
            self.compressed.is_empty() && self.uncompressed.is_empty(),
            "Cannot load into a non-empty block"
        );
        self.offset = offset;

        let extra_len = {
            self.buffer.resize(HEADER_SIZE + MIN_EXTRA_SIZE, 0);
            match stream.read_exact(&mut self.buffer) {
                Ok(()) => {}
                Err(e) => {
                    if e.kind() == ErrorKind::UnexpectedEof {
                        return Err(BlockError::EndOfStream);
                    } else {
                        return Err(BlockError::from(e));
                    }
                }
            }
            analyze_header(&self.buffer)? as usize
        };

        if extra_len > MIN_EXTRA_SIZE {
            self.buffer.resize(HEADER_SIZE + extra_len, 0);
            stream.read_exact(&mut self.buffer[HEADER_SIZE..])?;
        }
        let block_size = analyze_extra_fields(&self.buffer[HEADER_SIZE..])? as usize + 1;
        if block_size > MAX_BLOCK_SIZE {
            return Err(BlockError::Corrupted(format!(
                "Block size {} > {}",
                block_size, MAX_BLOCK_SIZE
            )));
        }

        unsafe {
            // Vulnerable line 2
            self.compressed
                .set_len(block_size - HEADER_SIZE - MIN_EXTRA_SIZE);
        }
        // Vulnerable line 1
        stream.read_exact(&mut self.compressed)?;
        Ok(())
    }

```
