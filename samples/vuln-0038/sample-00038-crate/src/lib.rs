pub mod pat;

use std::ops::Range;
use std::{mem, fmt, slice};

//----------------------------------------------------------------

pub const MAX_STORE: usize = 5;
pub const MAX_DEPTH: usize = 4;
const QS_BUF_LEN: usize = 16;

//----------------------------------------------------------------

/// Pattern scan context.
#[derive(Clone, Debug)]
pub struct Scanner<'a> {
	haystack: &'a [u8],
	scan: Range<u32>,
	vbase: u64,
}

impl<'a> Scanner<'a> {
	/// Create a new `Scanner` from raw components.
	///
	/// # Arguments
	///
	/// * `haystack`
	///
	///   Slice of valid memory to scan inside. Any time the scan goes outside this slice, the match has failed.
	///
	///   A reference to the haystack is kept, tying the Scanner's lifetime to the haystack.
	///
	/// * `scan`
	///
	///   Check only for matches within this range, but allow patterns to chase pointers in the whole haystack.
	///
	/// * `vbase`
	///
	///   Code might not be at the place where it's actually being executed, this would make it impossible to chase absolute pointers.
	///
	///   This is where `vbase` comes in, it translates absolute addresses to offsets within the haystack by simply subtracting it.
	///
	///   Set it to the virtual address of where `haystack` is expecting it is located in memory.
	///
	/// # Panics
	///
	/// Scan range must be contained within the haystack, enforced by panic.
	#[inline]
	pub fn new(haystack: &'a [u8], scan: Range<u32>, vbase: u64) -> Scanner<'a> {
		let _ = haystack[scan.start as usize..scan.end as usize];
		Scanner {
			haystack: haystack,
			scan: scan,
			vbase: vbase,
		}
	}
	/// Scan for a single unique match.
	///
	/// # Arguments
	///
	/// * `pat`
	///
	///   The pattern to match.
	///
	/// # Return value
	///
	/// Fails with `None` if there's no match, or more than one match was found.
	pub fn find(&self, pat: &[pat::Unit]) -> Option<Match<'a>> {
		let mut it = self.iter(pat);
		if let found @ Some(_) = it.next() {
			// Disallow more than one match as it means the signature isn't unique enough
			if it.next().is_some() {
				// More than one match!
				None
			}
			else {
				// Exactly one match
				found
			}
		}
		else {
			// No match
			None
		}
	}
	/// Iterate over all matches.
	///
	/// # Arguments
	///
	/// * `pat`
	///
	///   The pattern being matched.
	///
	/// # Return value
	///
	/// Iterator over the matches.
	#[inline]
	pub fn iter<'p, 's>(&'s self, pat: &'p [pat::Unit]) -> ScanIter<'a, 'p, 's> {
		ScanIter {
			container: self,
			pat: pat,
			ptr: self.scan.start,
			hits: 0,
		}
	}
	/// Match a pattern to a specific ptr.
	///
	/// # Arguments
	///
	/// * `ptr`
	///
	///   Offset in the haystack to match the pattern at.
	///
	/// * `pat`
	///
	///   The pattern to match.
	///
	/// # Return value
	///
	/// `None` if mismatch, `Some(Match)` if matched.
	///
	/// # Remarks
	///
	/// I'd like to name this function `match` but that's a keyword...
	pub fn mach(&self, mut ptr: u32, pat: &[pat::Unit]) -> Option<Match<'a>> {
		let mut stack = [0u32; MAX_DEPTH];
		let mut sci = 0usize; // Stack index pointer

		let mut mach = Match {
			haystack: self.haystack,
			store: [0u32; MAX_STORE],
			at: ptr,
		};
		let mut sti = 0usize; // Store index pointer

		for unit in pat {
			if ptr as usize >= self.haystack.len() {
				return None;
			}
			match *unit {
				pat::Unit::Byte(b) => {
					if self.haystack[ptr as usize] != b {
						return None;
					}
					ptr += 1;
				},
				pat::Unit::Skip(s) => {
					ptr += s as u32;
				},
				pat::Unit::Store => {
					mach.store[sti] = ptr;
					sti += 1;
				},
				pat::Unit::Push => {
					stack[sci] = ptr;
					sci += 1;
				},
				pat::Unit::Pop => {
					sci -= 1;
					ptr = stack[sci];
				},
				pat::Unit::RelByte => {
					if let Some(d) = self.read::<i8>(ptr) {
						ptr = ptr.wrapping_add(d as u32).wrapping_add(1);
					}
					else {
						return None;
					}
				},
				pat::Unit::RelDword => {
					if let Some(d) = self.read::<i32>(ptr) {
						ptr = ptr.wrapping_add(d as u32).wrapping_add(4);
					}
					else {
						return None;
					}
				},
				pat::Unit::Jump32 => {
					if let Some(d) = self.read::<u32>(ptr) {
						let newp = d.wrapping_sub(self.vbase as u32);
						if newp >= self.haystack.len() as u32 {
							return None;
						}
						ptr = newp;
					}
					else {
						return None;
					}
				},
				pat::Unit::Jump64 => {
					if let Some(d) = self.read::<u64>(ptr) {
						let newp = d.wrapping_sub(self.vbase);
						if newp >= self.haystack.len() as u64 {
							return None;
						}
						ptr = newp as u32;
					}
					else {
						return None;
					}
				},
			}
		}
		// Matched!
		Some(mach)
	}
	fn read<T: Copy>(&self, ptr: u32) -> Option<T> {
		let ptr = ptr as usize;
		let size_of = mem::size_of::<T>();
		if ptr > self.haystack.len() - size_of {
			None
		}
		else {
			// Inb4 UB how2transmute
			Some(unsafe {
				*(&self.haystack[ptr] as *const _ as *const T)
			})
		}
	}
}

//----------------------------------------------------------------

#[cfg(feature = "pelite")]
impl<'a, 'b> From<&'b pelite::pe32::peview::PeView<'a>> for Scanner<'a> {
	fn from(pe: &'b pelite::pe32::peview::PeView<'a>) -> Scanner<'a> {
		let opt = pe.optional_header();
		let vbase = pe.virtual_base() as u64;
		if opt.BaseOfCode != 0 && opt.SizeOfCode != 0 {
			Scanner::new(pe.image(), opt.BaseOfCode..opt.BaseOfCode + opt.SizeOfCode, vbase)
		}
		else {
			Scanner::new(pe.image(), 0..pe.image().len() as u32, vbase)
		}
	}
}
#[cfg(feature = "pelite")]
impl<'a, 'b> From<&'b pelite::pe64::peview::PeView<'a>> for Scanner<'a> {
	fn from(pe: &'b pelite::pe64::peview::PeView<'a>) -> Scanner<'a> {
		let opt = pe.optional_header();
		let vbase = pe.virtual_base() as u64;
		if opt.BaseOfCode != 0 && opt.SizeOfCode != 0 {
			Scanner::new(pe.image(), opt.BaseOfCode..opt.BaseOfCode + opt.SizeOfCode, vbase)
		}
		else {
			Scanner::new(pe.image(), 0..pe.image().len() as u32, vbase)
		}
	}
}

//----------------------------------------------------------------

/// Iterator over pattern matches.
#[derive(Clone, Debug)]
pub struct ScanIter<'a: 's, 'p, 's> {
	container: &'s Scanner<'a>,
	pat: &'p [pat::Unit],
	ptr: u32,
	hits: u32,
}

impl<'a, 'p, 's> ScanIter<'a, 'p, 's> {
	fn build_qsbuf<'b>(&self, qsbuf: &'b mut [u8; QS_BUF_LEN]) -> &'b [u8] {
		let mut qslen = 0usize;
		for unit in self.pat {
			match *unit {
				pat::Unit::Byte(b) => {
					if qslen >= QS_BUF_LEN {
						break;
					}
					qsbuf[qslen] = b;
					qslen += 1;
				},
				pat::Unit::Store => {
				},
				_ => {
					break;
				},
			}
		}
		&qsbuf[..qslen]
	}
}

impl<'a, 'p, 's> Iterator for ScanIter<'a, 'p, 's> {
	type Item = Match<'a>;
	fn next(&mut self) -> Option<Match<'a>> {
		// This should allow the compiler to elide some bounds checks later
		// FIXME! There's still some stubborn bounds checks
		let _ = self.container.haystack[self.ptr as usize..self.container.scan.end as usize];

		// This won't get called often, just rebuild the quicksearch buffer every time
		let mut qsbuf: [u8; QS_BUF_LEN] = unsafe { mem::uninitialized() };
		let qsbuf = self.build_qsbuf(&mut qsbuf);

		// Strategy:
		//  Cannot optimize the search, just brute-force it.
		#[inline(always)]
		fn strategy0<'a: 's, 'p, 's>(iter: &mut ScanIter<'a, 'p, 's>, _qsbuf: &[u8]) -> Option<Match<'a>> {
			let (mut ptr, end) = (iter.ptr, iter.container.scan.end);
			while ptr < end {
				iter.hits += 1;
				if let mach @ Some(_) = iter.container.mach(ptr, &iter.pat) {
					iter.ptr = ptr + 1;
					return mach;
				}
				ptr += 1;
			}
			iter.ptr = ptr;
			None
		}
		// Strategy:
		//  Raw pattern is too small for full blown quicksearch.
		//  Can still do a small optimization though.
		#[inline(always)]
		fn strategy1<'a: 's, 'p, 's>(iter: &mut ScanIter<'a, 'p, 's>, qsbuf: &[u8]) -> Option<Match<'a>> {
			let byte = qsbuf[0];
			let (mut ptr, end) = (iter.ptr, iter.container.scan.end);
			loop {
				// Fast inner loop to find one matching byte
				loop {
					if ptr >= end {
						iter.ptr = ptr;
						return None;
					}
					if iter.container.haystack[ptr as usize] == byte {
						break;
					}
					ptr += 1;
				}
				// First byte matched, now do the full slow match
				iter.hits += 1;
				if let mach @ Some(_) = iter.container.mach(ptr, &iter.pat) {
					iter.ptr = ptr + 1;
					return mach;
				}
				ptr += 1;
			}
		}
		// Strategy:
		//  Full blown quicksearch for raw pattern.
		//  Most likely completely unnecessary but oh well... was fun to write :)
		#[inline(always)]
		fn strategy2<'a: 's, 'p, 's>(iter: &mut ScanIter<'a, 'p, 's>, qsbuf: &[u8]) -> Option<Match<'a>> {
			let len = qsbuf.len();

			// Initialize jump table for quicksearch
			let mut jumps = [len as u8; 256];
			for i in 0..len - 1 {
				jumps[qsbuf[i as usize] as usize] = len as u8 - i as u8 - 1;
			}

			// Adjust end pointer
			let len = len - 1;
			let (mut ptr, end) = (iter.ptr, iter.container.scan.end - len as u32);

			// Quicksearch baby :)
			while ptr < end {
				// Create a slice to compare the quicksearch buffer to
				// SAFETY: len is subtracted from the end before entering this loop, this can never go out of bounds
				let tbuf = unsafe { slice::from_raw_parts(iter.container.haystack.as_ptr().offset(ptr as isize), len + 1) };
				let last = tbuf[len];
				let jump = jumps[last as usize] as u32;
				if qsbuf[len] == last {
					// If the qsbuf matches, perform full match
					if tbuf == qsbuf {
						iter.hits += 1;
						if let mach @ Some(_) = iter.container.mach(ptr as u32, &iter.pat) {
							iter.ptr = ptr + jump;
							return mach;
						}
					}
				}
				// Advance and skip a bunch
				ptr += jump;
			}
			iter.ptr = ptr;
			None
		}

		if qsbuf.len() == 0 {
			strategy0(self, qsbuf)
		}
		else if qsbuf.len() <= 2 {
			strategy1(self, qsbuf)
		}
		else {
			strategy2(self, qsbuf)
		}
	}
}

//----------------------------------------------------------------

/// Represents a pattern match.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Match<'a> {
	/// Haystack the match was found in.
	pub haystack: &'a [u8],
	/// Offset in the haystack the match was found.
	pub at: u32,
	/// Stored offsets as specified in the pattern.
	pub store: [u32; MAX_STORE],
}

impl<'a> Match<'a> {
	/// Get the pointer to the location that was matched.
	#[inline]
	pub fn ptr(&self) -> *const u8 {
		unsafe { self.haystack.as_ptr().offset(self.at as isize) }
	}
	/// Get a pointer from the store array.
	#[inline]
	pub fn get<T>(&self, idx: usize) -> *const T {
		unsafe { self.haystack.as_ptr().offset(self.store[idx] as isize) as *const T }
	}
}

impl<'a> fmt::Display for Match<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(write!(f, "{:08X}", self.at)?);
		if self.store[0] != 0 {
			(write!(f, " [{:08X}", self.store[0])?);
			for s in (&self.store[1..]).iter().take_while(|&s| *s != 0) {
				(write!(f, ", {:08X}", *s)?);
			}
			(write!(f, "]")?);
		}
		Ok(())
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;
	use super::pat::*;

	const HAYSTACK: &'static [u8] = b"\x12\x34\x56\x01\xFF\x22\x00\x10\x00\x00\x00\x00\x00\x00\xB8\xC3";
	const BYTES_1: &'static [u8] = b"\x22\x01\xD5\x44\x22\x01\x69\x55";
	const BYTES_2: &'static [u8] = b"\xDD\x7F\xDD\x15\xDD\xC1\xDD";

	#[test]
	fn mach() {
		let scan = Scanner::new(HAYSTACK, 0..16, 0x1000);

		assert_eq!(
			scan.mach(0, &[Unit::Byte(0x12), Unit::Skip(1), Unit::Byte(0x56)]),
			Some(Match { haystack: HAYSTACK, at: 0, store: [0, 0, 0, 0, 0] }));

		assert_eq!(
			scan.mach(14, &[Unit::Byte(0xB8), Unit::Byte(0xC3)]),
			Some(Match { haystack: HAYSTACK, at: 14, store: [0, 0, 0, 0, 0] }));

		assert_eq!(
			scan.mach(2, &[Unit::Byte(0x56), Unit::RelByte, Unit::Byte(0x22)]),
			Some(Match { haystack: HAYSTACK, at: 2, store: [0, 0, 0, 0, 0] }));

		assert_eq!(
			scan.mach(0, &[Unit::Byte(0x12), Unit::Jump64, Unit::Byte(0x56)]),
			None);

		assert_eq!(
			scan.mach(6, &[Unit::Jump64, Unit::Byte(0x12), Unit::Byte(0x34)]),
			Some(Match { haystack: HAYSTACK, at: 6, store: [0, 0, 0, 0, 0] }));

		assert_eq!(
			scan.mach(4, &[Unit::Byte(0xFF), Unit::Skip(1), Unit::Push, Unit::Jump64, Unit::Store, Unit::Pop, Unit::Skip(8), Unit::Byte(0xB8)]),
			Some(Match { haystack: HAYSTACK, at: 4, store: [0, 0, 0, 0, 0] }));
	}
	#[test]
	fn scan() {
		let scan = Scanner::new(HAYSTACK, 0..16, 0x1000);

		assert_eq!(
			scan.find(&[Unit::Byte(0xFF), Unit::Skip(1), Unit::Store, Unit::Push, Unit::Jump64, Unit::Store, Unit::Pop, Unit::Skip(8), Unit::Byte(0xB8)]),
			Some(Match { haystack: HAYSTACK, at: 4, store: [6, 0, 0, 0, 0] }));

		assert_eq!(
			Scanner::new(BYTES_1, 0..8, 0)
				.find(&[pat::Unit::Byte(0x22), pat::Unit::RelByte, pat::Unit::Store, pat::Unit::Byte(0x55)]),
			Some(Match { haystack: BYTES_1, at: 4, store: [7, 0, 0, 0, 0] }));

		assert_eq!(
			Scanner::new(BYTES_2, 0..7, 0)
				.iter(&[pat::Unit::Byte(0xDD), pat::Unit::Skip(1), pat::Unit::Byte(0xDD)])
				.collect::<Vec<_>>(),
			vec![
				Match { haystack: BYTES_2, at: 0, store: [0, 0, 0, 0, 0] },
				Match { haystack: BYTES_2, at: 2, store: [0, 0, 0, 0, 0] },
				Match { haystack: BYTES_2, at: 4, store: [0, 0, 0, 0, 0] },
			]);
	}
}
