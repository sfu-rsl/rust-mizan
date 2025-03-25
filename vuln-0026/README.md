# Vulnerability: CVE-2023-50711

| **Information**       | **Details**                                                                                       |
| --------------------- | ------------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2023-50711](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2023-50711)                   |
| **Vulnerable Commit** | [0bafbc1](https://github.com/rust-vmm/vmm-sys-util/tree/0bafbc13ec8f6483c5a10352ebdd6fa706edcc58) |
| **Fixed Commit**      | [30172fc](https://github.com/rust-vmm/vmm-sys-util/tree/30172fca2a8e0a38667d934ee56682247e13f167) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                      |
|                       | - [fixed-file](fixed-file)                                                                        |
|                       | - [fixed-function](fixed-function)                                                                |
|                       | - [vuln-crate](vuln-crate)                                                                        |
|                       | - [vuln-file](vuln-file)                                                                          |
|                       | - [vuln-function](vuln-function)                                                                  |

## Vulnerable Lines

`src/fam.rs`

```rust
/// Update the length of the FamStructWrapper.
///
/// The length of `self` will be updated to the specified value.
/// The length of the `T` structure and of `self.mem_allocator` will be updated accordingly.
/// If the len is increased additional capacity will be reserved.
/// If the len is decreased the unnecessary memory will be deallocated.
///
/// This method might trigger reallocations of the underlying buffer.
///
/// # Errors
///
/// When len is greater than the max possible len it returns Error::SizeLimitExceeded.
fn set_len(&mut self, len: usize) -> Result<(), Error> {
	let additional_elements = isize::try_from(len)
		.and_then(|len| isize::try_from(self.len()).map(|self_len| len - self_len))
		.map_err(|_| Error::SizeLimitExceeded)?;

	// If len == self.len there's nothing to do.
	if additional_elements == 0 {
		return Ok(());
	}

	// If the len needs to be increased:
	if additional_elements > 0 {
		// Check if the new len is valid.
		if len > T::max_len() {
			return Err(Error::SizeLimitExceeded);
		}
		// Reserve additional capacity.
		self.reserve(additional_elements as usize)?;
	}

	let current_mem_allocator_len = self.mem_allocator.len();
	let required_mem_allocator_len =
		FamStructWrapper::<T>::mem_allocator_len(len).ok_or(Error::SizeLimitExceeded)?;
	// Update the len of the `mem_allocator`.
	// SAFETY: This is safe since enough capacity has been reserved.
	unsafe {
		self.mem_allocator.set_len(required_mem_allocator_len);
	}
	// Zero-initialize the additional elements if any.
	for i in current_mem_allocator_len..required_mem_allocator_len {
		// SAFETY: Safe as long as the trait is only implemented for POD. This is a requirement
		// for the trait implementation.
		self.mem_allocator[i] = unsafe { mem::zeroed() }
	}
	// Update the len of the underlying `FamStruct`.
	// VULNERABILITY: This call updates the logical length of the flexible array without confirming that the underlying memory actually contains that many elements.
	self.as_mut_fam_struct().set_len(len);

	// If the len needs to be decreased, deallocate unnecessary memory
	if additional_elements < 0 {
		self.mem_allocator.shrink_to_fit();
	}

	Ok(())
}
```
