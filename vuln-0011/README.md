# Vulnerability: CVE-2017-1000430

| **Information**       | **Details**                                                                                       |
| --------------------- | --------------------------------------------------------------------------------------------------|
| **CVE**               | [CVE-2017-1000430](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2017-1000430)                           |
| **Vulnerable Commit** | [21a9389](https://github.com/marshallpierce/rust-base64/tree/21a9389cf340f1e36e48856859d5713b97744383) |
| **Fixed Commit**      | [e0ffe70](https://github.com/marshallpierce/rust-base64/tree/e0ffe7096f8e1457c5e1ab5bcf34dce49ee1e43e) |
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

