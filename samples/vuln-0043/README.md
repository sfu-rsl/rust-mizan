# Vulnerability: CVE-2020-36513

| **Information**       | **Details**                                                                                  |
| --------------------- | -------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-36513](https://nvd.nist.gov/vuln/detail/CVE-2020-36513)                            |
| **Vulnerable Commit** | [95a54aa](https://github.com/netvl/acc_reader/tree/95a54aab52339a66707646d67e9a5fa11a65529b) |
| **Fixed Commit**      |                                                                                              |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                   |
|                       | - [vuln-function](vuln-function)                                                             |

### Vulnerable Lines

`src/lib.rs`

```rust
// The issue is the usage of user defined read() on the vec after increasing its length without initializing the increased vec space which leads to the underfinded behaviour.

fn read_up_to(&mut self, n: u64) -> io::Result<()> {
    let old_len = self.buf.len();
    self.buf.reserve(n as usize);

    // The length of the buf - vector is increased and n bytes of memory is unintialized.
    unsafe {
        self.buf.set_len(old_len + n as usize);
    }

    let mut error = None;
    let mut read = 0;
    {
        let mut target = &mut self.buf[old_len..];
        while !target.is_empty() {

            // The user defined read function is being called on uninitialized memory
            match self.source.read(target) {
                Ok(0) => break,
                Ok(n) => {
                    read += n;
                    let tmp = target;
                    target = &mut tmp[n..];
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => {
                    error = Some(e);
                    break;
                }
            }
        }
    }
    unsafe {
        self.buf.set_len(old_len + read as usize);
    }

    if let Some(e) = error {
        Err(e)
    } else {
        Ok(())
    }
}

```
