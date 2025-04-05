# Vulnerability: CVE-2018-20992

| **Information**       | **Details**                                                                              |
|-----------------------|------------------------------------------------------------------------------------------|
| **CVE**               | [CVE-2018-20992](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-20992)          |
| **Vulnerable Commit** | [cd82be3](https://github.com/ruuda/claxon/tree/cd82be35f413940ba446d2a19f10d74b86466487) |
| **Fixed Commit**      | [e7dae6e](https://github.com/ruuda/claxon/tree/e7dae6ee16376f15a25b4126810b25902f82c8b4) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                               |
|                       | - [vuln-file](vuln-file)                                                                 |
|                       | - [vuln-function](vuln-function)                                                         |
|                       | - [fixed-crate](vuln-crate)                                                              |
|                       | - [fixed-file](vuln-file)                                                                |
|                       | - [fixed-function](vuln-function)                                                        |

### Vulnerable lines

[Certain malformed inputs would cause the contents of uninitialized memory to be written to the decoded audio.](https://github.com/ruuda/claxon/issues/10)

Before decoding a frame, the code ensures the buffer used for decoding has enough space.
However, for performance reasons, this code uses an unsafe function, `Vec::set_len`, instead of 
`Vec::resize` and filling the new area with `0`s. `set_len` will only change the len number in the
`Vec` struct; it does not resize / reallocate anything.

```rust
/// Decodes the next frame or returns an error if the data was invalid.
///
/// The buffer is moved into the returned block, so that the same buffer may
/// be reused to decode multiple blocks, avoiding a heap allocation every
/// time. It can be retrieved again with `block.into_buffer()`. If the
/// buffer is not large enough to hold all samples, a larger buffer is
/// allocated automatically.
pub fn read_next_or_eof(&mut self, mut buffer: Vec<i32>) -> FrameResult {
    // ...
    // We must allocate enough space for all channels in the block to be
    // decoded.
    let total_samples = header.channels() as usize * header.block_size as usize;
    buffer = ensure_buffer_len(buffer, total_samples);
    // ...
}

/// A macro to expand the length of a buffer, or replace the buffer altogether,
/// so it can hold at least `new_len` elements. The contents of the buffer can
/// be anything, it is assumed they will be overwritten anyway.
fn ensure_buffer_len(mut buffer: Vec<i32>, new_len: usize) -> Vec<i32> {
    if buffer.len() < new_len {
        // Previous data will be overwritten, so instead of resizing the
        // vector if it is too small, we might as well allocate a new one.
        if buffer.capacity() < new_len {
            buffer = Vec::with_capacity(new_len);
        }

        // We are going to fill the buffer anyway, so there is no point in
        // initializing it with default values. This does mean that there could
        // be garbage in the buffer, but that is not exposed, as the buffer is
        // only exposed if a frame has been decoded successfully, and hence the
        // entire buffer has been overwritten.
        unsafe { buffer.set_len(new_len); }
    } else {
        buffer.truncate(new_len);
    }
    buffer
}
```

The Safety documentation for `Vec::set_len` requires two things (the second point is relevant):

- `new_len` must be less than or equal to `capacity()`.
- The elements at `old_len..new_len` must be initialized.

The comment for `buffer.set_len(new_len);` acknowledges that there might be garbage in the buffer (i.e. uninitialized 
memory), but notes that the buffer isn't exposed outside decoding functions until a successful decoding. It asserts that
the entire buffer will be overwritten anyway, so there is no need to zero out the garbage in the buffer.

However, there is a decoding function, `decode_residual`, where a specially crafted input can result in a buffer to not 
be overwritten completely, leaving uninitialized memory in the buffer. This results in uninitialized memory being 
written to decoded audio files.

```rust
fn decode_residual<R: ReadBytes>(input: &mut Bitstream<R>,
                                 block_size: u16,
                                 buffer: &mut [i32])
                                 -> Result<()> {
    // Residual starts with two bits of coding method.
    let partition_type = match try!(input.read_leq_u8(2)) {
        0b00 => RicePartitionType::Rice,
        0b01 => RicePartitionType::Rice2,
        // 10 and 11 are reserved.
        _ => return fmt_err("invalid residual, encountered reserved value"),
    };

    // Next are 4 bits partition order.
    let order = try!(input.read_leq_u8(4));

    // There are 2^order partitions. Note: the specification states a 4-bit
    // partition order, so the order is at most 31, so there could be 2^31
    // partitions, but the block size is a 16-bit number, so there are at
    // most 2^16 - 1 samples in the block. No values have been marked as
    // invalid by the specification though.
    let n_partitions = 1u32 << order;
    // VULNERABILITY: A partition order could occur, such that the block size was not a
    // multiple of 2^order. Computation of the number of samples per partition
    // did not account for this case, rounding down due to the bit shift. This
    // meant that we would not fill the entire decode buffer.
    let n_samples = block_size >> order;
    // ...
}
```

From the fix commit message:

> A partition order could occur, such that the block size was not a
multiple of 2^order. Computation of the number of samples per partition
did not account for this case, rounding down due to the bit shift. This
meant that we would not fill the entire decode buffer.
>
> Claxon does not zero the decode buffer because it is (should be)
overwritten anyway, and in the case of a format error, where the buffer
might be only partially full, the buffer is not exposed again.
Furthermore, the way decoding works in most places, is that we fill the
entire buffer, just by looping to fill it. If the input bitstream does
not contain enough data to fill the buffer, then that's a format error.
In a few places though, we need to slice up the buffer before decoding
into it: for decoding individual channels, and also for decoding
residuals, which are split into partitions.
>
> This particular format error was especially nasty because it did not
cause a format error down the line. Instead, it caused the buffer to be
sliced in a way where the slices together did not cover the entire
buffer, and so parts of uninitialized memory could remain in the buffer.
