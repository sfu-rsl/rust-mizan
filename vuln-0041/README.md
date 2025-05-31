# Vulnerability: RUSTSEC-2025-0018

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **RUSTSEC**           | [RUSTSEC-2025-0018](https://rustsec.org/advisories/RUSTSEC-2025-0018.html)                      |
| **Vulnerable Commit** | [156fa9f](https://github.com/rusqlite/rusqlite/commit/156fa9fcf23cadd5db3bdeeca11e5a25cd813e5b) |
| **Fixed Commit**      | [54043c8](https://github.com/rusqlite/rusqlite/commit/54043c803c83517aa54b463b098a61dfa28c54f4) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [vuln-crate](vuln-crate)                                                                      |

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
