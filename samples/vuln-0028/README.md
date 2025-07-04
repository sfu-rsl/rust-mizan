# Vulnerability: CVE-2019-16141

| **Information**       | **Details**                                                                                   |
|-----------------------|-----------------------------------------------------------------------------------------------|
| **CVE**               | [CVE-2019-16141](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16141)               |
| **Vulnerable Commit** | [729fef1](https://github.com/matklad/once_cell/tree/729fef18c5a014b70d4085d2b606d1700168ed4c) |
| **Fixed Commit**      | [afcca95](https://github.com/matklad/once_cell/tree/afcca95a05240ebd931ab20998c946f77ef1e284) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                    |
|                       | - [vuln-function](vuln-function)                                                              |
|                       | - [fixed-crate](fixed-crate)                                                                  |
|                       | - [fixed-function](fixed-function)                                                            |

### Vulnerable lines

`unreachable_unchecked` is used. This core Rust function tells the compiler that the site which is calling this function 
is not reachable, possibly enabling further optimizations. It also notes that reaching this function is undefined 
behavior.

```rust
pub fn force(this: &Lazy<T, F>) -> &T {
    // Safe because closure is guaranteed to be called at most once
    // so we only call `F` once, this also guarantees no race conditions
    this.cell.get_or_init(|| unsafe {
        match (*this.init.get()).take() {
            Some(f) => f(),
            None => unreachable_unchecked(),
        }
    })
}
```

A PoC was created that was able to reach that call site:

```rust
use once_cell::sync::Lazy;

static HI: Lazy<i32> = Lazy::new(|| {
    panic!();
});

fn main() {
    let _ = std::panic::catch_unwind(|| println!("{}", *HI));
    println!("{}", *HI);
}
```
