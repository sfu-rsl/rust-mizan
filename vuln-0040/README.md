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

`file/location`

```rust
fn vuln_function() {}
```
