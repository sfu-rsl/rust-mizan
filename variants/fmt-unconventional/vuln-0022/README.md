# Vulnerability: CVE-2020-25795

| **Information**       | **Details**                                                                                    |
| --------------------- | ---------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-25795](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-25795)                |
| **Vulnerable Commit** | [40aa74b](https://github.com/bodil/sized-chunks/tree/40aa74b824688a4d4b1e1c65a50c679abb58b41e) |
| **Fixed Commit**      | [9f66983](https://github.com/bodil/sized-chunks/tree/9f66983f058944da5f402202ac5708089051a237) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                   |
|                       | - [fixed-file](fixed-file)                                                                     |
|                       | - [fixed-function](fixed-function)                                                             |
|                       | - [vuln-crate](vuln-crate)                                                                     |
|                       | - [vuln-file](vuln-file)                                                                       |
|                       | - [vuln-function](vuln-function)                                                               |

### Vulnerable Lines

`src/ring_buffer/mod.rs`

```rust
/// Insert multiple values at index `index`, shifting all the following values
/// to the right.
///
/// Panics if the index is out of bounds or the chunk doesn't have room for
/// all the values.
///
/// Time: O(m+n) where m is the number of elements inserted and n is the number
/// of elements following the insertion index. Calling `insert`
/// repeatedly would be O(m*n).
pub fn insert_from<Iterable, I>(&mut self, index: usize, iter: Iterable)
where
    Iterable: IntoIterator<Item = A, IntoIter = I>,
    I: ExactSizeIterator<Item = A>,
{
    let iter = iter.into_iter();
    let insert_size = iter.len();
    if self.len() + insert_size > Self::CAPACITY {
        panic!(
            "Chunk::insert_from: chunk cannot fit {} elements",
            insert_size
        );
    }
    if index > self.len() {
        panic!("Chunk::insert_from: index out of bounds");
    }
    if index == self.len() {
        self.extend(iter);
        return;
    }
    let right_count = self.len() - index;
    // Check which side has fewer elements to shift.
    if right_count < index {
        // Shift to the right.
        let mut i = self.raw(self.len() - 1);
        let target = self.raw(index);
        while i != target {
            unsafe { self.force_write(i + insert_size, self.force_read(i)) };
            i -= 1;
        }
        unsafe { self.force_write(target + insert_size, self.force_read(target)) };
        self.length += insert_size;
    } else {
        // Shift to the left.
        self.origin -= insert_size;
        self.length += insert_size;
        for i in self.range().take(index) {
            unsafe { self.force_write(i, self.force_read(i + insert_size)) };
        }
    }
    let mut index = self.raw(index);
    for value in iter {
        // If iter.next() panics, the function drops more than it initialized.
        unsafe { self.force_write(index, value) };
        index += 1;
    }
}
```
