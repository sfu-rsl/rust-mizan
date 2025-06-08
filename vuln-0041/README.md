# Vulnerability: RUSTSEC-2025-0018

| **Information**       | **Details**                                                                                |
| --------------------- | ------------------------------------------------------------------------------------------ |
| **RUSTSEC**           | [RUSTSEC-2025-0018](https://rustsec.org/advisories/RUSTSEC-2025-0018.html)                 |
| **Vulnerable Commit** | [7011c19](https://github.com/nrc/xmas-elf/commit/7011c1925ac10618a92a5170afec7da86f13fa92) |
| **Fixed Commit**      | [57685c3](https://github.com/nrc/xmas-elf/commit/57685c35512a57269086314a42a70441af4ef451) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                               |
|                       | - [fixed-function](fixed-function)                                                         |
|                       | - [vuln-crate](vuln-crate)                                                                 |
|                       | - [vuln-function](vuln-function)                                                           |

### Vulnerable Lines

Not vulnerable but for reference

```rust
pub struct HashTable {
    bucket_count: u32,
    chain_count: u32,
    first_bucket: u32,
}
```

`src/hash.rs`

```rust
// HashTable::get_bucket() and HashTable::get_chain() only validates the index with the values from the
// ELF file but not the actual size from the ELF in memory. If the ELF file contains a bucket_count or chain_count
// larger than the actual space, it can lead to an out of bounds read.
impl HashTable {
    pub fn get_bucket(&self, index: u32) -> u32 {
        assert!(index < self.bucket_count);

        // VULNERABILITY: Lack of a bounds check for the bucket count can cause undefined behaviour
        unsafe { let ptr = (&self.first_bucket as *const u32).offset(index as isize);
            *ptr
        }
    }

    pub fn get_chain(&self, index: u32) -> u32 {
        assert!(index < self.chain_count);
        let index = self.bucket_count + index;

        // VULNERABILITY: Lack of a bounds check for the chain count can cause undefined behaviour
        unsafe { let ptr = (&self.first_bucket as *const u32).offset(index as isize);
            *ptr
        }
    }
}
```
