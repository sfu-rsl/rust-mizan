# Vulnerability: RUSTSEC-2025-0027

| **Information**       | **Details**                                                                                               |
|-----------------------|-----------------------------------------------------------------------------------------------------------|
| **RUSTSEC ID**        | [RUSTSEC-2025-0027](https://rustsec.org/advisories/RUSTSEC-2025-0027.<br/><br/>html)                      |
| **Vulnerable Commit** | [7699ac2](https://github.com/GuillaumeGomez/mp3-metadata/commit/7699ac280e10394bec52c85753bfbc148f298f17) |
| **Fixed Commit**      | [bb42abf](https://github.com/GuillaumeGomez/mp3-metadata/commit/bb42abf609605820010eb61996eb8c035f757c51) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                              |
|                       | - [fixed-file](fixed-file)                                                                                |
|                       | - [fixed-function](fixed-function)                                                                        |
|                       | - [vuln-crate](vuln-crate)                                                                                |
|                       | - [vuln-file](vuln-file)                                                                                  |
|                       | - [vuln-function](vuln-function)                                                                          |

### Vulnerable Lines

`src/metadata.rs`

```rust
fn get_id3(i: &mut u32, buf: &[u8], meta: &mut MP3Metadata) -> Result<(), Error> {
    /// ... more code before this
    // Recreate the tag if desynchronization is used inside; we need to replace
    // 0xFF 0x00 with 0xFF
    let mut v = Vec::new();
    let (buf, length) = if use_sync {
        let mut new_pos = 0;
        let mut skip = false;
        v.reserve(tag_size);

        for i in 0..tag_size {
            if skip {
                skip = false;
                continue;
            }
            if i + 1 >= buf.len() {
                return Ok(());
            }
            if i + 1 < tag_size && buf[i] == 0xFF && buf[i + 1] == 0 {
                /// VULNERABILITY: Inadequate bounds checking which results in an unexpected panic
                v[new_pos] = 0xFF;
                new_pos += 1;
                skip = true;
                continue;
            }
            if new_pos >= v.len() {
                return Ok(());
            }
            v[new_pos] = buf[i];
            new_pos += 1;
        }
        (v.as_slice(), new_pos)
    } else {
        (buf, tag_size)
    };
    /// more code after this ...
}
```
