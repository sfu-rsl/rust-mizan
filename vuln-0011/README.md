# Vulnerability: RUSTSEC-2025-0019

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [RUSTSEC-2025-0019](https://rustsec.org/advisories/RUSTSEC-2025-0019.html)                         |
| **Vulnerable Commit** | [1cf18d1](https://github.com/planus-org/planus/commit/1cf18d16af7cf0b17c8f95f7c0fd362c69c78236) |
| **Fixed Commit**      | [be6f99a](https://github.com/planus-org/planus/commit/be6f99afde8760dcf87b5dcdade832400e826791) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [fixed-function](fixed-function)                                                              |
|                       | - [vuln-crate](vuln-crate)                                                                      |
|                       | - [vuln-function](vuln-function)                                                                |

### Vulnerable lines

`src/lib.rs`

```rust
fn write_impl(&mut self, value: [T; N]) {
    *self.slice = unsafe {
        let ptr = &value as *const [T; N] as *const [MaybeUninit<T>; N];
        let read_value = core::ptr::read(ptr);
        // VULNERABILITY: This calls `Drop` implementation for `value` before it naturally goes out of scope so `Drop` will be called twice
        core::mem::drop(value);
        read_value
    };
}
```
