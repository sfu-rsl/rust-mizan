# Vulnerability: CVE-2017-1000430

| **Information**       | **Details**                                                                                       |
| --------------------- | --------------------------------------------------------------------------------------------------|
| **CVE**               | [CVE-2017-1000430](https://rustsec.org/advisories/RUSTSEC-2017-0004.html)                           |
| **Vulnerable Commit** | [5cf6f26](https://github.com/marshallpierce/rust-base64/tree/5cf6f261e1147d7d1359e3759130280e22715dc2) |
| **Fixed Commit**      | [24ead98](https://github.com/marshallpierce/rust-base64/tree/24ead980daf11ba563e4fb2516187a56a71ad319) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                               |
|                       | - [fixed-function](fixed-function)                                                                            |
|                       | - [vuln-crate](vuln-crate)                                                                                |
|                       | - [vuln-function](vuln-function)                                                                             |

### Vulnerable Lines

```rust

// a buf relies on two unsafe arithmetic operations that could overflow
// 
// it's later assumed that this buf is correct and an incorrect amount of memory is reserved
//
// unchecked multiplicatoin
let complete_output_chars = complete_input_chunks * 4;
// possibly unchecked addition
let printing_output_chars = if rem == 0 {
    complete_output_chars
} else {
    complete_output_chars + 4
};

// unsafe accesses with incorrect assumption 
std::ptr::write(output_ptr, charset[((input_chunk >> 58) & 0x3F) as usize]);
std::ptr::write(output_ptr.offset(1), charset[((input_chunk >> 52) & 0x3F) as usize]);
std::ptr::write(output_ptr.offset(2), charset[((input_chunk >> 46) & 0x3F) as usize]);
std::ptr::write(output_ptr.offset(3), charset[((input_chunk >> 40) & 0x3F) as usize]);
std::ptr::write(output_ptr.offset(4), charset[((input_chunk >> 34) & 0x3F) as usize]);
std::ptr::write(output_ptr.offset(5), charset[((input_chunk >> 28) & 0x3F) as usize]);
std::ptr::write(output_ptr.offset(6), charset[((input_chunk >> 22) & 0x3F) as usize]);
std::ptr::write(output_ptr.offset(7), charset[((input_chunk >> 16) & 0x3F) as usize]);



```

