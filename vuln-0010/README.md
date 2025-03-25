# Vulnerability: CVE-2020-36204

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-36204](https://rustsec.org/advisories/RUSTSEC-2020-0096.html)                         |
| **Vulnerable Commit** | [9c2726a](https://github.com/bodil/im-rs/commit/9c2726ad5d2917cc5e1b963ffc796b8d19a99ee7) |
| **Fixed Commit**      | [0b3a7b2](https://github.com/bodil/im-rs/commit/0b3a7b228b0fe70446393f55c8b893f349f3f6bd) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [fixed-file](fixed-file)                                                                      |
|                       | - [fixed-function](fixed-function)                                                              |
|                       | - [vuln-crate](vuln-crate)                                                                      |
|                       | - [vuln-file](vuln-file)                                                                        |
|                       | - [vuln-function](vuln-function)                                                                |

### Vulnerable lines

`TreeFocus` implements `Send` and `Sync` but doesn't enforce it for the type parameter, resulting in `A` being treated 
as thread-safe even if it's not ("smuggled").

```rust
#[allow(unsafe_code)]
#[cfg(threadsafe)]
unsafe impl<A> Send for TreeFocus<A> {}

#[allow(unsafe_code)]
#[cfg(threadsafe)]
unsafe impl<A> Sync for TreeFocus<A> {}
```

Remark: `TreeFocus` is not exposed to end-users.
