# Vulnerability: CVE-2021-26305

| **Information**       | **Details**                                                                                |
| --------------------- | ------------------------------------------------------------------------------------------ |
| **CVE**               | [CVE-2021-26305](https://www.cve.org/CVERecord?id=CVE-2021-26305)                          |
| **Vulnerable Commit** | [880a281](https://github.com/hrektts/cdr-rs/tree/880a281afe6f49109a90e2a8bb943c02a360bf49) |
| **Fixed Commit**      | [0e6006d](https://github.com/hrektts/cdr-rs/tree/0e6006de464caa331643f86cd2d9ba3b32b09833) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                 |
|                       | - [vuln-file](vuln-file)                                                                   |
|                       | - [fixed-crate](fixed-crate)                                                               |
|                       | - [fixed-file](fixed-file)                                                                 |

### Vulnerable Lines

`src/de.rs`

```rust
// The issues is that this function passes uninitialized vector to read_exact function of trait Read. (Possible user defined read function)

// Vulnerable function
 fn read_vec(&mut self) -> Result<Vec<u8>> {
        let len: u32 = de::Deserialize::deserialize(&mut *self)?;
        let mut buf = Vec::with_capacity(len as usize);
        unsafe { buf.set_len(len as usize) }
        self.read_size(u64::from(len))?;
        //Vulnerable line
        self.reader.read_exact(&mut buf[..])?;
        Ok(buf)
    }

```
