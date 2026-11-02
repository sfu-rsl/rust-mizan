# Vulnerability: CVE-2020-36511

| **Information**       | **Details**                                                                              |
| --------------------- | ---------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-36511](https://www.cve.org/CVERecord?id=CVE-2020-36511)                        |
| **Vulnerable Commit** | [cc13bd5](https://github.com/hinaria/bite/tree/cc13bd52f2a97e50e8c2ed29a39dc484c4332423) |
| **Fixed Commit**      | -                                                                                        |
| **Variants**          | - [vuln-crate](vuln-crate)                                                               |
|                       | - [vuln-file](vuln-file)                                                                 |
|                       | - [vuln-function](vuln-function)                                                         |

### Vulnerable Lines

`src/bite/read.rs`

```rust
// The issues is that this function passes uninitialized vector to read_exact function of trait Read. (Possible user defined read function)

fn read_framed_max<T: Endianness>(
        &mut self,
        maximum: usize,
    ) -> Result<Vec<u8>, std::io::Error> {
        let length = match self.read_u32::<T>()? as usize {
            x if x <= maximum => x,
            _ => return Err(std::io::ErrorKind::InvalidData.into()),
        };

        unsafe {
            let mut data = Vec::with_capacity(length);
            let slice = std::slice::from_raw_parts_mut(data.as_mut_ptr(), length);

            //Vulnerable line
            self.read_exact(slice)?;
            data.set_len(length);

            Ok(data)
        }
    }

```
