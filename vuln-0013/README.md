# Vulnerability: CVE-2020-35862

| **Information**       | **Details**                                                                                 |
| --------------------- | ------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35862](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35862)                     |
| **Vulnerable Commit** | [c5587f0](https://github.com/ferrilab/bitvec/commit/c5587f0cab99abd6b363afa84a3acad9e8f2cce6) |
| **Fixed Commit**      | [3ffe951](https://github.com/ferrilab/bitvec/commit/3ffe9510c3368555f96e5e96e0c1b90381d07cc9) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                |
|                       | - [fixed-file](fixed-file)                                                                  |
|                       | - [fixed-function](fixed-function)                                                          |
|                       | - [vuln-crate](vuln-crate)                                                                  |
|                       | - [vuln-file](vuln-file)                                                                    |
|                       | - [vuln-function](vuln-function)                                                            |

### Vulnerable lines

The test code from the original issue only failed on macOS, but the vulnerability is nevertheless general.

```rust
pub fn into_boxed_bitslice(self) -> BitBox<O, T> {
    let pointer = self.pointer;
    // Convert the Vec allocation into a Box<[T]> allocation.
    // into_boxed_slice will call into_vec (Rust std Vec) and then call into_boxed_slice, which
    // shrinks the allocation in order to get a frozen-length slice.
    mem::forget(self.into_boxed_slice());
    // However, the memory allocator is not required to preserve the address when shrinking an 
    // allocation! Notice how the same pointer of the Vec before is being used after the 
    // shrinking.
    //
    // The macOS allocator _probably_ has an inclusive limit at 8064 words for its bucket size.
    // The minimal test code / PoC used in the issue created a 8065-word vector. When shrinking, 
    // it was moved to a smaller allocation, and the large allocation was released back to the OS.
    //
    // The following line uses the original address, not the result of Vec::into_boxed_slice.
    // A use-after-free could be possible after this point.
    // (https://github.com/ferrilab/bitvec/issues/55#issuecomment-605318109)
    unsafe { BitBox::from_raw(pointer.as_mut_ptr()) }
}
```
