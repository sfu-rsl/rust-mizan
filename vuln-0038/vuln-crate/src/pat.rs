//! Patterns.
//!
//! Patterns can be created statically from its components or parsed from a string.
//!
//! ```
//! use scanner::pat::{parse, Unit};
//! 
//! const MY_PATTERN: &'static [Unit] = &[Unit::Byte(0xE9), Unit::Store, Unit::Skip(4), Unit::Byte(0xC3)];
//! 
//! let pat = parse("E9*???? C3").unwrap();
//! assert_eq!(pat, MY_PATTERN);
//! ```

use super::{MAX_DEPTH, MAX_STORE};

/// Parsing errors when parsing a pattern.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatError {
	/// Unexpected end of string.
	///
	/// Expected `+` followed by `[jJrR]`, `[rR]` followed by `[14]`, hex digits must be paired.
	UnexpectedEos,
	/// Invalid recursion.
	///
	/// Expected `+` followed by `[jJrR]`.
	InvalidRecursion,
	/// Invalid size of relative jump argument.
	///
	/// Valid sizes for `[rR]` are `[14]`.
	InvalidSizeOf,
	/// Unpaired hex digit.
	///
	/// Hex digits must always come in pairs to form a single byte.
	UnpairedHexDigit,
	/// Illegal character.
	UnknownChar,
	/// Stack error.
	///
	/// More than `MAX_DEPTH` `+` or more `-` than `+`.
	StackError,
	/// Unknown escape sequence.
	///
	/// Escape sequences are not supported at this time.
	UnknownEscape,
	/// Store overflow.
	///
	/// More than `MAX_STORE` groups of `*`.
	StoreOverflow,
}

/// Pattern atom.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
	/// Match a single byte.
	Byte(u8),
	/// Skip a fixed number of bytes.
	Skip(u8),
	/// Store current ptr.
	Store,
	/// Push current ptr to the stack.
	Push,
	/// Pop from the stack to current ptr.
	Pop,
	/// Relative jump, signed one byte
	RelByte,
	/// Relative jump, signed four bytes
	RelDword,
	/// Absolute 32bit jump.
	Jump32,
	/// Absolute 64bit jump.
	Jump64,
}

/// Patterns are just sequences of `Unit`s.
pub type Pattern = Vec<Unit>;

/// Parse a string to `Pattern`.
///
/// # Arguments
///
/// * `pat`
///
///   Input string to parse, see remarks for formatting.
///
/// Uses the platform `target_pointer_width` for absolute jumps. If you need precise control over pointer width, see `parse32` and `parse64`.
///
/// # Return value
///
/// The `Pattern` or a `PatError`.
///
/// # Remarks
///
/// Valid characters for the pattern string:
///
/// * `hex` : Two hexadecimal digits, match this exact byte.
/// * `   ` : Spaces are ignored, any other character not otherwise mentioned is illegal.
/// * ` * ` : Store the current ptr. The ptr is dereferenced by the number of `*`. Match fails if the resulting ptr is not valid.
/// * ` ? ` : Skip a byte.
/// * ` + ` : Save the current ptr so it can later be continued being matched with `-`. Must be followed by `j` or `r`.
/// * ` - ` : Return to previously saved position, the bytes used to follow the `j` or `r` are skipped.
/// * ` j ` : Dereference this as an absolute address and continue matching.
/// * `r14` : Dereference this as a byte (`1`) or dword (`4`), sign extend to pointer width, add to the current ptr and continue matching.
#[inline]
pub fn parse(pat: &str) -> Result<Pattern, PatError> {
	if cfg!(target_pointer_width = "32") {
		parse_impl(pat, false)
	}
	else if cfg!(target_pointer_width = "64") {
		parse_impl(pat, true)
	}
	else {
		panic!("Not implemented for target_pointer_width!");
	}
}
/// 32bit signatures. See `parse` for docs.
#[inline]
pub fn parse32(pat: &str) -> Result<Pattern, PatError> {
	parse_impl(pat, false)
}
/// 64bit signatures. See `parse` for docs.
#[inline]
pub fn parse64(pat: &str) -> Result<Pattern, PatError> {
	parse_impl(pat, true)
}

fn parse_impl(pat: &str, ptr: bool) -> Result<Pattern, PatError> {
	let mut units = Vec::<Unit>::with_capacity(pat.as_bytes().len() / 2);
	let mut it = pat.chars();
	let mut rec = 0;
	let mut store = 0;
	let mut pops = [0u8; MAX_DEPTH];
	while let Some(chr) = it.next() {
		match chr {
			// Recursive operator
			'+' => {
				// Limited recursive depth
				rec += 1;
				if rec > (MAX_DEPTH as i32) {
					return Err(PatError::StackError);
				}
				// Must be followed by Jump or Rel
				if let Some(chr) = it.clone().next() {
					match chr {
						'r' | 'R' | 'j' | 'J' => {
							units.push(Unit::Push);
						},
						_ => {
							return Err(PatError::InvalidRecursion);
						},
					}
				}
				else {
					return Err(PatError::UnexpectedEos);
				}
			},
			// Return from recursion
			'-' => {
				rec -= 1;
				if rec < 0 {
					return Err(PatError::StackError);
				}
				units.push(Unit::Pop);
				// Skip the bytes interpreted for the Jump or Rel
				units.push(Unit::Skip(pops[rec as usize]));
			},
			// Relative jump
			'r' | 'R' => {
				// Followed by the operand size
				if let Some(chr) = it.next() {
					let (unit, skip) =
						if chr == '1' {
							(Unit::RelByte, 1u8)
						}
						else if chr == '4' {
							(Unit::RelDword, 4u8)
						}
						else {
							return Err(PatError::InvalidSizeOf);
						};
					units.push(unit);
					if rec > 0 {
						pops[rec as usize - 1] = skip;
					}
				}
				else {
					return Err(PatError::UnexpectedEos);
				}
			},
			// Absolute jump
			'j' | 'J' => {
				let (unit, skip) =
					if ptr {
						(Unit::Jump64, 8u8)
					}
					else {
						(Unit::Jump32, 4u8)
					};
				units.push(unit);
				if rec > 0 {
					pops[rec as usize - 1] = skip;
				}
			},
			// Match a byte
			'0' ..= '9' | 'A' ..= 'F' | 'a' ..= 'f' => {
				if let Some(chr2) = it.next() {
					// High nibble of the byte
					let hi = if chr >= 'a' { chr as u8 - 'a' as u8 + 0xA }
						else if chr >= 'A' { chr as u8 - 'A' as u8 + 0xA }
						else { chr as u8 - '0' as u8 };
					// Low nibble of the byte
					let lo = if chr2 >= 'a' && chr2 <= 'f' { chr2 as u8 - 'a' as u8 + 0xA }
						else if chr2 >= 'A' && chr2 <= 'F' { chr2 as u8 - 'A' as u8 + 0xA }
						else if chr2 >= '0' && chr2 <= '9' { chr2 as u8 - '0' as u8 }
						else { return Err(PatError::UnpairedHexDigit); };
					// Add byte to the pattern
					units.push(Unit::Byte((hi << 4) + lo));
				}
				else {
					return Err(PatError::UnexpectedEos);
				}
			},
			// Store current ptr
			'*' => {
				// Limited store space
				store += 1;
				if store > (MAX_STORE as i32) {
					return Err(PatError::StoreOverflow);
				}
				units.push(Unit::Store);
			},
			// Skip bytes
			'?' => {
				// Coalescence skips together
				if let Some(&mut Unit::Skip(ref mut s)) = units.last_mut() {
					if *s < 255u8 {
						*s += 1;
						continue;
					}
				}
				units.push(Unit::Skip(1));
			},
			// Allow spaces as padding
			' ' => {
			},
			// Everything else is illegal
			_ => {
				return Err(PatError::UnknownChar);
			},
		}
	}
	Ok(units)
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;
	use std::mem;

	#[test]
	fn patterns() {
		assert_eq!(mem::size_of::<Unit>(), 2);

		assert_eq!(parse("12 34 56"), Ok(vec![Unit::Byte(0x12), Unit::Byte(0x34), Unit::Byte(0x56)]));

		assert_eq!(parse("B9*?? 68??? E8+r4*- 8B"), Ok(vec![
			Unit::Byte(0xB9), Unit::Store, Unit::Skip(2), Unit::Byte(0x68),
			Unit::Skip(3), Unit::Byte(0xE8), Unit::Push, Unit::RelDword,
			Unit::Store, Unit::Pop, Unit::Skip(4), Unit::Byte(0x8B)]));

		assert_eq!(parse("+r4+r1+r4+r1"), Ok(vec![
			Unit::Push, Unit::RelDword,
			Unit::Push, Unit::RelByte,
			Unit::Push, Unit::RelDword,
			Unit::Push, Unit::RelByte]));

		assert_eq!(parse("24 5A 9e D0 AF Be a3 fC dd"), Ok(vec![
			Unit::Byte(0x24), Unit::Byte(0x5A), Unit::Byte(0x9E),
			Unit::Byte(0xD0), Unit::Byte(0xAF), Unit::Byte(0xBE),
			Unit::Byte(0xA3), Unit::Byte(0xFC), Unit::Byte(0xDD)]));
	}

	#[test]
	fn failures() {
		// Test all types of pattern errors
		assert_eq!(parse("+r4+r1+r4+r1+r4"),
			Err(PatError::StackError));
		assert_eq!(parse("?+"),
			Err(PatError::UnexpectedEos));
		assert_eq!(parse("+12"),
			Err(PatError::InvalidRecursion));
		assert_eq!(parse("-"),
			Err(PatError::StackError));
		assert_eq!(parse("+r4+r1+r4+r1+r4-----"),
			Err(PatError::StackError));
		assert_eq!(parse("r2"),
			Err(PatError::InvalidSizeOf));
		assert_eq!(parse("r4 r"),
			Err(PatError::UnexpectedEos));
		assert_eq!(parse("E"),
			Err(PatError::UnexpectedEos));
		assert_eq!(parse("a?"),
			Err(PatError::UnpairedHexDigit));
		assert_eq!(parse("EE BZ"),
			Err(PatError::UnpairedHexDigit));
		assert_eq!(parse("*?*?*?*?*?*"),
			Err(PatError::StoreOverflow));
		assert_eq!(parse("(&)"),
			Err(PatError::UnknownChar));
	}
}
