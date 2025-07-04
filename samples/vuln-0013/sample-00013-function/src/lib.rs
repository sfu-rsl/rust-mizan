/*! `bitvec` – `[bool]` in overdrive.

This crate provides views into slices of bits that are truly `[u1]`. Each bit in
the data segment is used, unlike `[bool]` which ignores seven bits out of every
byte.

`bitvec`’s data structures provide strong guarantees about, and fine-grained
control of, the bit-level representation of a sequence of memory. The user is
empowered to choose the fundamental type underlying the store – `u8`, `u16`,
`u32`, or `u64` – and the order in which each primitive is traversed – `Msb0`,
from the most significant bit to the least, or `Lsb0`, from the least
significant bit to the most.

This level of control is not necessary for most use cases where users just want
to put bits in a sequence, but it is critically important for users making
packets that leave main memory and hit some external device like a peripheral
controller or a network socket. In order to provide convenience to users for
whom the storage details do not matter, `bitvec` types default to using the
local C bitfield ordering and CPU word size.

In addition to providing compact, efficient, and powerful storage and
manipulation of bits in memory, the `bitvec` structures are capable of acting as
a queue, set, or stream of bits. They implement the bit-wise operators for
Boolean arithmetic, arithmetic operators for 2’s-complement numeric arithmetic,
read indexing, bit shifts, and access to the underlying storage fundamental
elements as a slice.

(Write indexing is impossible in Rust semantics.)
!*/

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(debug_assertions, warn(missing_docs))]
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![deny(unconditional_recursion)]

#[cfg(feature = "alloc")]
extern crate alloc;

use alloc::boxed::Box;
use core::marker::PhantomData;
use core::{mem, slice};
use core::ptr::NonNull;
#[cfg(feature = "atomic")]
use core::sync::atomic;
use core::sync::atomic::Ordering;
use core::cell::Cell;
use core::fmt::Debug;
use core::ops::{Deref, DerefMut};
use funty::IsUnsigned;
use radium::marker::BitOps;
use radium::Radium;
use funty::IsInteger;

impl<O, T> BitBox<O, T>
where
	O: BitOrder,
	T: BitStore,
{
	/// Constructs a bit box from a raw bit pointer.
	///
	/// After calling this function, the raw pointer is owned by the resulting
	/// `BitBox`. Specifically, the `BitBox` destructor will free the allocated
	/// memory. For this to be safe, the memory must have been allocated by
	/// `BitBox` earlier in the program.
	///
	/// # Safety
	///
	/// This function is unsafe because improper use may lead to memory
	/// problems. For example, a double-free may occurr if the function is
	/// called twice on the same raw pointer.
	///
	/// # Notes
	///
	/// This function, and `into_raw`, exchange ordinary raw pointers
	/// `*mut BitSlice<O, T>`. Values of these types can be created from, and
	/// converted to, other region pointers such as `*mut [T]` through ordinary
	/// `as`-casting.
	///
	/// This is valid in the Rust type system, but is incorrect at runtime. You
	/// must not, ever, use `as` to cast in either direction to or from a
	/// `BitSlice` pointer.
	///
	/// # Examples
	///
	/// Recreate a `BitBox` which was previously converted to a raw pointer
	/// using [`BitBox::into_raw`]:
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let b = BitBox::new(0u8.bits::<Lsb0>());
	/// let ptr = BitBox::into_raw(b);
	/// let b = unsafe { BitBox::<Lsb0, _>::from_raw(ptr) };
	/// ```
	///
	/// [`BitBox::into_raw`]: #method.into_raw
	pub unsafe fn from_raw(raw: *mut BitSlice<O, T>) -> Self {
		Self {
			_order: PhantomData,
			pointer: BitPtr::from_mut_ptr(raw),
		}
	}

	/// Accesses the vector’s backing store as an element slice.
	///
	/// Unlike `BitSlice`’s method of the same name, this includes the partial
	/// edges, as `BitBox` forbids fragmentation that leads to contention.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// The slice of all live elements in the backing storage, including the
	/// partial edges if present.
	pub fn as_slice(&self) -> &[T] {
		self.bitptr().as_slice()
	}

	/// Accesses the vector’s backing store as an element slice.
	///
	/// Unlike `BitSlice`’s method of the same name, this includes the partial
	/// edges, as `BitBox` forbids fragmentation that leads to contention.
	///
	/// # Parameters
	///
	/// - `&mut self`
	///
	/// # Returns
	///
	/// The slice of all live elements in the backing storage, including the
	/// partial edges if present.
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		self.bitptr().as_mut_slice()
	}

	/// Gives read access to the `BitPtr<T>` structure powering the box.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// A copy of the interior `BitPtr<T>`.
	pub(crate) fn bitptr(&self) -> BitPtr<T> {
		self.pointer
	}
}

impl<O, T> From<BitVec<O, T>> for BitBox<O, T>
where
	O: BitOrder,
	T: BitStore {
	fn from(src: BitVec<O, T>) -> Self {
		src.into_boxed_bitslice()
	}
}

impl<O, T> Drop for BitBox<O, T>
where
	O: BitOrder,
	T: BitStore,
{
	fn drop(&mut self) {
		let ptr = self.as_mut_slice().as_mut_ptr();
		let len = self.as_slice().len();
		//  Run the `Box<[T]>` destructor.
		drop(unsafe { Vec::from_raw_parts(ptr, 0, len) }.into_boxed_slice());
	}
}

/** Access interface for shared/mutable memory access.

`&BitSlice` and `&mut BitSlice` contexts must route through their `Access`
associated type, which implements this trait, in order to perform *any* access
to underlying memory. This trait extends the `Radium` element-wise shared
mutable access with single-bit operations suited for use by `BitSlice`.
**/
pub trait BitAccess<M>: Debug + Radium<M> + Sized
where M: BitMemory
{
	/// Set a single bit in an element low.
	///
	/// `BitAccess::set` calls this when its `value` is `false`; it
	/// unconditionally writes a `0` bit into the electrical position that
	/// `place` controls according to the `BitOrder` parameter `O`.
	///
	/// # Type Parameters
	///
	/// - `O`: A `BitOrder` implementation which translates `place` into a
	///   usable bit-mask.
	///
	/// # Parameters
	///
	/// - `&self`: A shared reference to underlying memory.
	/// - `place`: A semantic bit index in the `self` element.
	fn clear_bit<O>(&self, place: BitIdx<M>)
	where O: BitOrder {
		self.fetch_and(!*O::select(place), Ordering::Relaxed);
	}


	/// Set a single bit in an element high.
	///
	/// `BitAccess::set` calls this when its `value` is `true`; it
	/// unconditionally writes a `1` bit into the electrical position that
	/// `place` controls according to the `BitOrder` parameter `O`.
	///
	/// # Type Parameters
	///
	/// - `O`: A `BitOrder` implementation which translates `place` into a
	///   usable bit-mask.
	///
	/// # Parameters
	///
	/// - `&self`: A shared reference to underlying memory.
	/// - `place`: A semantic bit index in the `self` element.
	fn set_bit<O>(&self, place: BitIdx<M>)
	where O: BitOrder {
		self.fetch_or(*O::select(place), Ordering::Relaxed);
	}

	/// Retrieve a single bit from an element.
	///
	/// # Type Parameters
	///
	/// - `O`: A `BitOrder` implementation which translates `place` into a
	///   usable bit-mask.
	///
	/// # Parameters
	///
	/// - `&self`: A shared reference to underlying memory.
	/// - `place`: A semantic bit index in the `self` element.
	#[inline]
	fn get<O>(&self, place: BitIdx<M>) -> bool
	where O: BitOrder {
		BitAccess::load(self) & *O::select(place) != M::ZERO
	}

	/// Set a single bit in an element to some value.
	///
	/// # Type Parameters
	///
	/// - `O`: A `BitOrder` implementation which translates `place` into a
	///   usable bit-mask.
	///
	/// # Parameters
	///
	/// - `&self`: A shared reference to underlying memory.
	/// - `place`: A semantic bit index in the `self` element.
	/// - `value`: The value to which the bit controlled by `place` shall be
	///   set.
	#[inline]
	fn set<O>(&self, place: BitIdx<M>, value: bool)
	where O: BitOrder {
		if value {
			self.set_bit::<O>(place);
		}
		else {
			self.clear_bit::<O>(place);
		}
	}

	/// Read a value out of a contended memory element and into a local scope.
	///
	/// # Parameters
	///
	/// - `&self`: A shared reference to underlying memory.
	///
	/// # Returns
	///
	/// The value of `*self`. This value is only useful when access is
	/// uncontended by multiple `BitSlice` regions.
	fn load(&self) -> M {
		Radium::load(self, Ordering::Relaxed)
	}

	/// Stores a value into a contended memory element.
	///
	/// # Parameters
	///
	/// - `&self`: A shared reference to underlying memory.
	/// - `value`: The new value to write into `*self`.
	fn store(&self, value: M) {
		Radium::store(self, value, Ordering::Relaxed)
	}
}

impl<M, R> BitAccess<M> for R
where
	M: BitMemory,
	R: Debug + Radium<M>,
{
}

/** A pointer type for owned bit sequences.

This type is essentially a `&BitSlice` that owns its own memory. It can change
the contents of its domain, but it cannot change its own domain like `BitVec`
can. It is useful for fixed-size collections without lifetime tracking.

# Type Parameters

- `O: BitOrder`: An implementor of the [`BitOrder`] trait. This type is used to
  convert semantic indices into concrete bit positions in elements, and store or
  retrieve bit values from the storage type.
- `T: BitStore`: An implementor of the [`BitStore`] trait: `u8`, `u16`, `u32`,
  or `u64` (64-bit systems only). This is the actual type in memory that the box
  will use to store data.

# Safety

The `BitBox` handle has the same *size* as standard Rust `Box<[T]>` handles, but
it is ***extremely binary incompatible*** with them. Attempting to treat
`BitBox<_, T>` as `Box<[T]>` in any manner except through the provided APIs is
***catastrophically*** unsafe and unsound.

# Trait Implementations

`BitBox<O, T>` implements all the traits that `BitSlice<O, T>` does, by
deferring to the `BitSlice` implementation. It also implements conversion traits
to and from `BitSlice`, and to/from `BitVec`.
**/
#[repr(C)]
pub struct BitBox<O = Local, T = usize>
where
	O: BitOrder,
	T: BitStore,
{
	_order: PhantomData<O>,
	pointer: BitPtr<T>,
}

/** Indicates a semantic index of a bit within a memory element.

This is a counter in the domain `0 .. M::BITS`, and marks a semantic position
in the ordering sequence described by a [`BitOrder`] implementation. It is used
for both position computation through `BitOrder` and range computation in
[`BitPtr`].

# Type Parameters

- `M`: The memory element type controlled by this index.

[`BitOrder`]: ../order/trait.BitOrder.html
[`BitPtr`]: ../pointer/struct.BitPtr.html
**/
//  If Rust had user-provided ranged integers, this would be communicable to the
//  compiler:
//  #[rustc_layout_scalar_valid_range_end(M::BITS)]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitIdx<M>
where M: BitMemory
{
	/// Semantic index within an element. Constrained to `0 .. M::BITS`.
	idx: u8,
	/// Marker for the indexed type.
	_ty: PhantomData<M>,
}

impl<M> BitIdx<M>
where M: BitMemory
{
	/// The zero index.
	pub const ZERO: Self = Self {
		idx: 0,
		_ty: PhantomData,
	};

	/// Wraps a counter value as a known-good index of the `M` element type.
	///
	/// # Parameters
	///
	/// - `idx`: A semantic index within a `M` memory element. It must be in the
	///   range `0 .. M::BITS`.
	///
	/// # Safety
	///
	/// If `idx` is outside the range, then the produced value will cause errors
	/// and memory unsafety when used.
	#[inline]
	pub unsafe fn new_unchecked(idx: u8) -> Self {
		debug_assert!(
			idx < M::BITS,
			"Bit index {} cannot exceed type width {}",
			idx,
			M::BITS,
		);
		Self {
			idx,
			_ty: PhantomData,
		}
	}

	/// Finds the destination bit a certain distance away from a starting bit.
	///
	/// This produces the number of elements to move from the starting point,
	/// and then the bit index of the destination bit in the destination
	/// element.
	///
	/// # Parameters
	///
	/// - `self`: A bit index in some memory element, used as the starting
	///   position for the offset calculation.
	/// - `by`: The number of bits by which to move. Negative values move
	///   downwards in memory: towards index zero, then counting from index
	///   `M::MASK` to index zero in the next element lower in memory, repeating
	///   until arrival. Positive values move upwards in memory: towards index
	///   `M::MASK`, then counting from index zero to index `M::MASK` in the
	///   next element higher in memory, repeating until arrival.
	///
	/// # Returns
	///
	/// - `.0`: The number of elements by which to offset the caller’s element
	///   cursor. This value can be passed directly into [`ptr::offset`].
	/// - `.1`: The bit index of the destination bit in the element selected by
	///   applying the `.0` pointer offset.
	///
	/// # Safety
	///
	/// `by` must not be far enough to cause the returned element offset value
	/// to, when applied to the original memory address via [`ptr::offset`],
	/// produce a reference out of bounds of the original allocation. This
	/// method has no way of checking this requirement.
	///
	/// [`ptr::offset`]: https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset
	pub(crate) fn offset(self, by: isize) -> (isize, Self) {
		let val = *self;

		/* Signed-add `*self` and the jump distance. Overflowing is the unlikely
		branch. The result is a bit index, and an overflow marker. `far` is
		permitted to be negative; this means that it is lower in memory than the
		origin bit. The number line has its origin at the front edge of the
		origin element, so `-1` is the *last* bit of the prior memory element.
		*/
		let (far, ovf) = by.overflowing_add(val as isize);
		//  If the `isize` addition does not overflow, then the sum can be used
		//  directly.
		if !ovf {
			//  If `far` is in the origin element, then the jump moves zero
			//  elements and produces `far` as an absolute index directly.
			if (0 .. M::BITS as isize).contains(&far) {
				(0, (far as u8).idx())
			}
			/* Otherwise, downshift the bit distance to compute the number of
			elements moved in either direction, and mask to compute the absolute
			bit index in the destination element.
			*/
			else {
				(far >> M::INDX, (far as u8 & M::MASK).idx())
			}
		}
		else {
			/* Overflowing `isize` addition happens to produce ordinary `usize`
			addition. In point of fact, `isize` addition and `usize` addition
			are the same machine instruction to perform the sum; it is merely
			the signed interpretation of the sum that differs. The sum can be
			recast back to `usize` without issue.
			*/
			let far = far as usize;
			//  This is really only needed in order to prevent sign-extension of
			//  the downshift; once shifted, the value can be safely re-signed.
			((far >> M::INDX) as isize, (far as u8 & M::MASK).idx())
		}
	}

	/// Computes the size of a span from `self` for `len` bits.
	///
	/// Spans always extend upwards in memory.
	///
	/// # Parameters
	///
	/// - `self`: The starting bit position of the span.
	/// - `len`: The number of bits to include in the span.
	///
	/// # Returns
	///
	/// - `.0`: The number of elements of `M` included in the span. If `len` is
	///   `0`, this will be `0`; otherwise, it will be at least one.
	/// - `.1`: The index of the first dead bit *after* the span. If `self` and
	///   `len` are both `0`, this will be `0`; otherwise, it will be in the
	///   domain `1 ..= M::BITS`.
	///
	/// # Notes
	///
	/// This defers to [`BitTail::span`], because `BitTail` is a strict superset
	/// of `BitIdx` (it is `{ BitIdx | M::BITS }`), and spans frequently begin
	/// from the tail of a slice in this crate. The `offset` function is *not*
	/// implemented on `BitTail`, and remains on `BitIdx` because offsets can
	/// only be computed from bit addresses that exist. It does not make sense
	/// to compute the offset from a `M::BITS` tail.
	///
	/// [`BitTail::span`]: struct.BitTail.html#method.span
	#[inline]
	pub(crate) fn span(self, len: usize) -> (usize, BitTail<M>) {
		unsafe { BitTail::new_unchecked(*self) }.span(len)
	}
}
impl<M> Deref for BitIdx<M>
where M: BitMemory
{
	type Target = u8;

	fn deref(&self) -> &Self::Target {
		&self.idx
	}
}

/** Indicates a semantic index of a dead bit *beyond* a memory element.

This type is equivalent to `BitIdx<M>`, except that it includes `M::BITS` in its
domain. Instances of this type will only ever contain `0` when the span they
describe is *empty*. Non-empty spans always cycle through the domain
`1 ..= M::BITS`.

This type cannot be used for indexing, and does not translate to `BitPos<M>`.
This type has no behavior other than viewing its internal `u8` for arithmetic.

# Type Parameters

- `M`: The memory element type controlled by this tail.
**/
//  #[rustc_layout_scalar_valid_range_end(M::BITS + 1)]#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitTail<M>
where M: BitMemory
{
	/// Semantic index *after* an element. Constrained to `0 ..= M::BITS`.
	end: u8,
	/// Marker for the tailed type.
	_ty: PhantomData<M>,
}

impl<M> BitTail<M>
where M: BitMemory
{
	/// The termination index.
	pub const END: Self = Self {
		end: M::BITS,
		_ty: PhantomData,
	};

	/// Mark that `end` is a tail index for a type.
	///
	/// # Parameters
	///
	/// - `end` must be in the range `0 ..= M::BITS`.
	pub(crate) unsafe fn new_unchecked(end: u8) -> Self {
		debug_assert!(
			end <= M::BITS,
			"Bit tail {} cannot surpass type width {}",
			end,
			M::BITS,
		);
		Self {
			end,
			_ty: PhantomData,
		}
	}

	pub(crate) fn span(self, len: usize) -> (usize, Self) {
		let val = *self;
		debug_assert!(
			val <= M::BITS,
			"Tail out of range: {} overflows type width {}",
			val,
			M::BITS,
		);

		if len == 0 {
			return (0, self);
		}

		let head = val & M::MASK;

		let bits_in_head = (M::BITS - head) as usize;

		if len <= bits_in_head {
			return (1, (head + len as u8).tail());
		}

		let bits_after_head = len - bits_in_head;

		let elts = bits_after_head >> M::INDX;
		let tail = bits_after_head as u8 & M::MASK;

		let is_zero = (tail == 0) as u8;
		let edges = 2 - is_zero as usize;
		(elts + edges, ((is_zero << M::INDX) | tail).tail())

		/* The above expression is the branchless equivalent of this structure:

		if tail == 0 {
			(elts + 1, M::BITS.tail())
		}
		else {
			(elts + 2, tail.tail())
		}
		*/
	}
}

impl<M> Deref for BitTail<M>
where M: BitMemory
{
	type Target = u8;

	fn deref(&self) -> &Self::Target {
		&self.end
	}
}

/** Indicates a real electrical index within an element.

This type is produced by [`BitOrder`] implementors, and marks a specific
electrical bit within a memory element, rather than [`BitIdx`]’s semantic bit.

# Type Parameters

- `M`: A `BitMemory` element which provides bounds-checking information. The
  [`new`] constructor uses [`M::BITS`] to ensure that constructed `BitPos`
  instances are always valid to use within `M` elements.

[`BitIdx`]: struct.BitIdx.html
[`BitOrder`]: ../order/trait.BitOrder.html
[`M::BITS`]: ../mem/trait.BitMemory.html#associatedconstant.BITS
[`new`]: #method.new
**/
//  #[rustc_layout_scalar_valid_range_end(M::BITS)]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitPos<M>
where M: BitMemory
{
	/// Electrical position within an element. Constrained to `0 .. M::BITS`.
	pos: u8,
	/// Marker for the positioned type.
	_ty: PhantomData<M>,
}

#[doc= " Wrapper type indicating a one-hot encoding of a bit mask for an element.\n\nThis type is produced by [`BitOrder`] implementations to speed up access to the\nunderlying memory. It ensures that masks have exactly one set bit, and can\nsafely be used as a mask for read/write access to memory.\n\n# Type Parameters\n\n- `M`: The storage type being masked.\n\n[`BitOrder`]: ../order/trait.BitOrder.html\n*"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitSel<M>
where M: BitMemory
{
	/// Mask value.
	sel: M,
}

impl<M> BitSel<M>
where M: BitMemory
{
	/// Produce a new bit-mask wrapper around any value.
	///
	/// # Safety
	///
	/// The caller *must* ensure that `mask` has exactly one bit set. `BitOrder`
	/// implementations should prefer [`::new`], which always panics on failure.
	///
	/// # Parameters
	///
	/// - `mask`: The mask value to encode. This must have exactly one bit set.
	///   Failure to uphold this requirement will introduce uncontrolled state
	///   contamination.
	///
	/// # Returns
	///
	/// `mask` wrapped in the `BitMask` marker type.
	///
	/// # Panics
	///
	/// This function panics if `mask` has zero or multiple bits set, only in
	/// debug builds. It does not inspect `mask` in release builds.
	///
	/// [`::new`]: #method.new
	#[inline]
	pub unsafe fn new_unchecked(sel: M) -> Self {
		debug_assert!(
			sel.count_ones() == 1,
			"Masks are required to have exactly one set bit: {:0>1$b}",
			sel,
			M::BITS as usize,
		);
		Self { sel }
	}
}

impl<M> Deref for BitSel<M>
where M: BitMemory
{
	type Target = M;

	fn deref(&self) -> &Self::Target {
		&self.sel
	}
}

/** Internal convenience trait for wrapping numbers with appropriate markers.

This trait must only be used on values that are known to be valid for their
context. It provides an internal-only shorthand for wrapping integer literals
and known-good values in marker types.

It is only implemented on `u8`.
**/
pub(crate) trait Indexable {
	/// Wraps a value as a `BitIdx<M>`.
	fn idx<M>(self) -> BitIdx<M>
	where M: BitMemory;

	/// Wraps a value as a `BitTail<M>`.
	fn tail<M>(self) -> BitTail<M>
	where M: BitMemory;
}

impl Indexable for u8 {
	fn idx<M>(self) -> BitIdx<M>
	where M: BitMemory {
		unsafe { BitIdx::<M>::new_unchecked(self) }
	}

	fn tail<M>(self) -> BitTail<M>
	where M: BitMemory {
		unsafe { BitTail::<M>::new_unchecked(self) }
	}
}

/** Describes properties of register types.

This trait describes raw memory, without any access modifiers. It provides
information about the width of a memory element and useful constants.
**/
pub trait BitMemory: IsUnsigned + BitOps {
	/// The width, in bits, of the memory element.
	const BITS: u8 = mem::size_of::<Self>() as u8 * 8;
	/// The number of bits required to hold a bit index into the element.
	const INDX: u8 = Self::BITS.trailing_zeros() as u8;
	/// The maximum value of a bit index for the element.
	const MASK: u8 = Self::BITS - 1;

	/// The element value with only the least significant bit high.
	const ONE: Self;
	/// The element value with all bits high.
	const ALL: Self;

	/// The element’s name.
	const TYPENAME: &'static str;

	/// Gets a specific bit in an element.
	///
	/// # Safety
	///
	/// This method cannot be called from within an `&BitSlice` context; it may
	/// only be called by construction of an `&Self` reference from a `Self`
	/// element directly.
	///
	/// # Parameters
	///
	/// - `&self`
	/// - `place`: A bit index in the element. The bit under this index, as
	///   governed by the `O` `BitOrder`, will be retrieved as a `bool`.
	///
	/// # Returns
	///
	/// The value of the bit under `place`.
	///
	/// # Type Parameters
	///
	/// - `O`: A `BitOrder` implementation to translate the index into a
	///   position.
	fn get<O>(&self, place: BitIdx<Self>) -> bool
	where O: BitOrder {
		*self & *O::select(place) != Self::ZERO
	}

	/// Sets a specific bit in an element to a given value.
	///
	/// # Safety
	///
	/// This method cannot be called from within an `&mut BitSlice` context; it
	/// may only be called by construction of an `&mut Self` reference from a
	/// `Self` element directly.
	///
	/// # Parameters
	///
	/// - `place`: A bit index in the element. The bit under this index, as
	///   governed by the `O` `BitOrder`, will be set according to `value`.
	///
	/// # Type Parameters
	///
	/// - `O`: A `BitOrder` implementation to translate the index into a
	///   position.
	fn set<O>(&mut self, place: BitIdx<Self>, value: bool)
	where O: BitOrder {
		let sel = *O::select(place);
		if value {
			*self |= sel;
		}
		else {
			*self &= !sel;
		}
	}

	/// Computes the number of elements of `Self` required to hold some bits.
	///
	/// # Parameters
	///
	/// - `bits`: The number of bits to store in an array of `[Self]`.
	///
	/// # Returns
	///
	/// The number of elements of `Self` required to hold the requested bits.
	fn elts(bits: usize) -> usize {
		elts::<Self>(bits)
	}

	#[doc(hidden)]
	fn retype<T>(self) -> T::Mem
	where T: BitStore {
		unsafe { *(&self as *const _ as *const _) }
	}
}

/** Computes the number of elements required to store a number of bits.

# Parameters

- `bits`: The number of bits to store in an element `T` array.

# Returns

The number of elements `T` required to store `bits`.

Because this is a const function, when `bits` is a const-expr, this function can
be used in array types `[T; elts(len)]`.
**/
#[doc(hidden)]
pub const fn elts<T>(bits: usize) -> usize {
	let width = mem::size_of::<T>() * 8;
	bits / width + (bits % width != 0) as usize
}

macro_rules! memory {
	($($t:ty),* $(,)?) => { $(
		impl BitMemory for $t {
			const ONE: Self = 1;
			const ALL: Self = !0;

			const TYPENAME: &'static str = stringify!($t);
		}
	)* };
}

memory!(u8, u16, u32, usize);

#[cfg(target_pointer_width = "64")]
memory!(u64);

/// Traverses an element from `MSbit` to `LSbit`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Msb0;

/// Traverses an element from `LSbit` to `MSbit`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Lsb0;

/** An ordering over an element.

# Usage

`bitvec` structures store and operate on semantic counts, not bit positions. The
`BitOrder::at` function takes a semantic ordering, `BitIdx`, and produces an
electrical position, `BitPos`.
**/
pub trait BitOrder {
	/// Name of the ordering type, for use in text display.
	const TYPENAME: &'static str;

	/// Translate a semantic bit index into an electrical bit mask.
	///
	/// This is an optional function; a default implementation is provided for
	/// you.
	///
	/// The default implementation of this function calls `Self::at` to produce
	/// an electrical position, then turns that into a bitmask by setting the
	/// `n`th bit more significant than the least significant bit of the
	/// element. `BitOrder` implementations may choose to provide a faster mask
	/// production here, but they must satisfy the invariants listed below.
	///
	/// # Parameters
	///
	/// - `place`: A semantic bit index into a memory element.
	///
	/// # Returns
	///
	/// A one-hot encoding of the provided `BitOrder`’s electrical position in
	/// the `M` element.
	///
	/// # Type Parameters
	///
	/// - `M`: The storage type for which the mask will be calculated. The mask
	///   must also be this type, as it will be applied to an element of `M` in
	///   order to set, clear, or test a single bit.
	///
	/// # Invariants
	///
	/// A one-hot encoding means that there is exactly one bit set in the
	/// produced value. It must be equivalent to `1 << *Self::at(place)`.
	///
	/// As with `at`, this function must produce a unique mapping from each
	/// legal index in the `M` domain to a one-hot value of `M`.
	///
	/// # Safety
	///
	/// This function requires that the output is always a one-hot value. It is
	/// illegal to produce a value with more than one bit set, and doing so will
	/// cause uncontrolled side effects.
	fn select<M>(place: BitIdx<M>) -> BitSel<M>
	where M: BitMemory;
}

impl BitOrder for Msb0 {
	const TYPENAME: &'static str = "Msb0";

	fn select<M>(place: BitIdx<M>) -> BitSel<M>
	where
		M: BitMemory {
		unsafe {
			BitSel::new_unchecked((M::ONE << M::MASK) >> *place)
		}
	}
}

impl BitOrder for Lsb0 {
	const TYPENAME: &'static str = "Lsb0";


	fn select<M>(place: BitIdx<M>) -> BitSel<M>
	where
		M: BitMemory {
		unsafe {
			BitSel::new_unchecked(M::ONE << *place)
		}
	}
}

/** A default bit ordering.

The target has big-endian byte ordering, so the default bit ordering is set to
big-endian as well, as a convenience. These two orderings are not related.
**/
#[cfg(target_endian = "big")]
pub type Local = Msb0;

/** A default bit ordering.

The target has little-endian byte ordering, so the default bit ordering is set
to little-endian as well, as a convenience. These two orderings are not related.
**/
#[cfg(target_endian = "little")]
pub type Local = Lsb0;

#[cfg(not(any(target_endian = "big", target_endian = "little")))]
compile_fail!(concat!("This architecture is currently not supported. File an issue at ", env!(CARGO_PKG_REPOSITORY)));

/** Union to permit reinterpreting a pointer-shaped value as a read pointer,
write pointer, or bare numeric address.

# Safety

Absolutely none whatsoever. This is probably flirting with undefined
behavior, and should be presumed to be the origin site of failure if the
crate ever breaks in the future.

# Type Parameters

- `T`: The referent data type.
**/
#[doc(hidden)]
pub(crate) union Address<T>
where T: BitStore
{
	/// A shareable pointer to some contended mutable data.
	a: *const <T as BitStore>::Access,
	/// A read pointer to some data.
	r: *const T,
	/// A write pointer to some data.
	w: *mut T,
	/// The pointer address as a bare integer.
	u: usize,
}

impl<T> Address<T>
where T: BitStore
{
	/// Accesses the address as a shared mutable pointer.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// The stored address, interpreted as a shared pointer to a mutable memory
	/// location.
	#[inline]
	pub(crate) fn a(self) -> *const <T as BitStore>::Access {
		unsafe { self.a }
	}

	/// Accesses the address as a read pointer.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// The stored address, as a read pointer.
	#[inline]
	pub(crate) fn r(self) -> *const T {
		unsafe { self.r }
	}

	/// Accesses the address as a write pointer.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// The stored address, as a write pointer.
	#[inline]
	pub(crate) fn w(self) -> *mut T {
		unsafe { self.w }
	}

	/// Accesses the address as a bare integral value.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// The stored address, as a bare integer.
	#[inline]
	pub(crate) fn u(self) -> usize {
		unsafe { self.u }
	}
}

impl<T> Clone for Address<T>
where T: BitStore
{
	fn clone(&self) -> Self {
		Self { u: self.u() }
	}
}

impl<T> From<&T> for Address<T>
where T: BitStore
{
	fn from(r: &T) -> Self {
		Self { r }
	}
}

impl<T> From<*const T> for Address<T>
where
	T: BitStore {
	fn from(r: *const T) -> Self {
		Self { r }
	}
}

impl<T> From<*mut T> for Address<T>
where
	T: BitStore {
	fn from(w: *mut T) -> Self {
		Self { w }
	}
}

impl<T> From<usize> for Address<T>
where
	T: BitStore {
	fn from(u: usize) -> Self {
		Self { u }
	}
}

impl<T> Copy for Address<T>
where
	T: BitStore { }

/** In-memory representation of `&BitSlice` handles.

# Layout

This structure is a more complex version of the `*const T`/`usize` tuple that
Rust uses to represent slices throughout the language. It breaks the pointer and
counter fundamentals into sub-field components. Rust does not have bitfield
syntax, so the below description of the element layout is in C++.

```cpp
template <typename T>
struct BitPtr {
  size_t ptr_head : __builtin_ctzll(alignof(T));
  size_t ptr_data : sizeof(uintptr_t) * 8 - __builtin_ctzll(alignof(T));

  size_t len_head : 3;
  size_t len_bits : sizeof(size_t) * 8 - 3;
};
```

This means that the `BitPtr<T>` structure has three *logical* fields, stored in
four segments across the two *structural* fields of the type. The widths and
placements of each segment are functions of the size of `*const T` and `usize`,
and the alignment of `T`.

# Fields

This section describes the purpose, meaning, and layout of the four logical
fields.

## Data Pointer

Aligned pointers to `T` always have low bits available for use to refine the
address of a `T` to the address of a `u8`. It is stored in the high bits of the
`ptr` field, running from MSb down to (inclusive)
`core::mem::align_of::<T>().trailing_zeros()`.

## Bit Counter

The memory representation stores a counter of the live bits contained in the
slice, starting at the head index. This counter occupies all but the lowest
three bits of the `len` structural field.

## Head Bit Index

For any fundamental type `T`, `core::mem::align_of::<T>().trailing_zeros() + 3`
bits are required to count the bit positions inside it.

|Type |Alignment|Trailing Zeros|Count Bits|
|:----|--------:|-------------:|---------:|
|`u8` |        1|             0|         3|
|`u16`|        2|             1|         4|
|`u32`|        4|             2|         5|
|`u64`|        8|             3|         6|

The head bit counter is split such that its bottom three bits are stored in the
low bits of the `len` field and the remaining high bits are stored in the low
bits of `ptr`.

The counter is a value in the range `0 .. (1 << Count)` that serves as a cursor
into the zeroth storage element to find the first live bit.

# Edge Cases

The following value sets are edge cases of valid `BitPtr` structures.

## Null Slice

The fully zeroed slot is not a valid member of the `BitPtr<T>` type; it is the
sentinel for `Option::<BitPtr<T>>::None`.

## Empty Slice

All empty slices have `0` in their `bits` logical field, and do not constrain
their `data` or `head` logical fields. The canonical empty slice structure uses
`NonNull::<T>::dangling()` as its `data` pointer, and `0` as its `head` index,
but any slice structure with `0` as `bits` is considered to be empty, and all
empty slices are equivalent to each other.

## Uninhabited Slice

The subset of empty slices with non-dangling pointers are considered
uninhabited. All `BitPtr` structures preserve their pointer information, even
when empty, because they may be the owners of the memory region at the pointer.
Uninhabited slices are also unconstrained in their `head` index value.

# Type Parameters

- `T: BitStore` is the storage type over which the pointer governs.

# Safety

A `BitPtr` must never be constructed such that the element addressed by
`self.pointer().offset(self.elements())` causes an addition overflow. This will
be checked in `new()`.

# Undefined Behavior

Using values of this type directly as pointers or counters will result in
undefined behavior. The pointer value will be invalid for the type, and both the
pointer and length values will be invalid for the memory model and allocation
regime.
**/
#[repr(C)]
#[derive(Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct BitPtr<T = u8>
where T: BitStore
{
	_ty: PhantomData<T>,
	/// Two-element bitfield structure, holding pointer and head information.
	///
	/// This stores a pointer to the zeroth element of the slice, and the high
	/// bits of the head bit cursor. It is typed as a `NonNull<u8>` in order to
	/// provide null-value optimizations to `Option<BitPtr<T>>`, and because the
	/// presence of head-bit cursor information in the lowest bits means the
	/// bit pattern will not uphold alignment properties assumed by
	/// `NonNull<T>`.
	///
	/// This field cannot be treated as an address of the zeroth byte of the
	/// slice domain, because the owning handle’s [`BitOrder`] implementation
	/// governs the bit pattern of the head cursor.
	///
	/// [`BitOrder`]: ../order/trait.BitOrder.html
	ptr: NonNull<u8>,
	/// Two-element bitfield structure, holding bit-count and head-index
	/// information.
	///
	/// This stores the bit count in its highest bits and the low three bits of
	/// the head `BitIdx` in the lowest three bits.
	///
	/// [`BitIdx`]: ../struct.BitIdx.html
	len: usize,
}

impl<T> BitPtr<T>
where T: BitStore
{
	/// The number of low bits in `self.len` that are the low bits of the head
	/// `BitIdx` cursor.
	///
	/// This is always `3`, until Rust tries to target a machine whose bytes are
	/// not eight bits wide.
	pub const LEN_HEAD_BITS: usize = 3;
	/// Marks the bits of `self.len` that are the `head` section.
	pub const LEN_HEAD_MASK: usize = 0b0111;
	/// The inclusive maximum bit index.
	pub const MAX_BITS: usize = !0 >> Self::LEN_HEAD_BITS;
	/// The inclusive maximum number of elements that can be stored in a
	/// `BitPtr` domain.
	pub const MAX_ELTS: usize = (Self::MAX_BITS >> 3) + 1;
	/// Marks the bits of `self.ptr` that are the `data` section.
	pub const PTR_DATA_MASK: usize = !Self::PTR_HEAD_MASK;
	/// The number of low bits in `self.ptr` that are the high bits of the head
	/// `BitIdx` cursor.
	pub const PTR_HEAD_BITS: usize = T::Mem::INDX as usize - Self::LEN_HEAD_BITS;
	/// Marks the bits of `self.ptr` that are the `head` section.
	pub const PTR_HEAD_MASK: usize =
		T::Mem::MASK as usize >> Self::LEN_HEAD_BITS;

	/// Produces an empty-slice representation.
	///
	/// This has no live bits, and has a dangling pointer. It is useful as a
	/// default value (and is the function used by `Default`) to indicate
	/// arbitrary empty slices.
	///
	/// # Returns
	///
	/// An uninhabited, uninhabitable, empty slice.
	///
	/// # Safety
	///
	/// The `BitPtr` returned by this function must never be dereferenced.
	pub fn empty() -> Self {
		Self {
			_ty: PhantomData,
			ptr: NonNull::dangling(),
			len: 0,
		}
	}

	/// Produces an uninhabited slice from a bare pointer.
	///
	/// # Parameters
	///
	/// - `ptr`: Some kind of pointer to `T`.
	///
	/// # Returns
	///
	/// If `ptr` is null, then this returns the empty slice; otherwise, the
	/// returned slice is uninhabited and points to the given address.
	///
	/// # Panics
	///
	/// This function panics if the given pointer is not well aligned to its
	/// type.
	///
	/// # Safety
	///
	/// The provided pointer must be either null, or valid in the caller’s
	/// memory model and allocation regime.
	#[cfg(feature = "alloc")]
	pub(crate) fn uninhabited(ptr: impl Into<Address<T>>) -> Self {
		let ptr = ptr.into();
		//  Check that the pointer is properly aligned for the storage type.
		//  Null pointers are always well aligned.
		assert!(
			(ptr.u()).trailing_zeros() as usize >= Self::PTR_HEAD_BITS,
			"Pointer {:p} does not satisfy minimum alignment requirements {}",
			ptr.r(),
			Self::PTR_HEAD_BITS,
		);
		Self {
			_ty: PhantomData,
			ptr: NonNull::new(ptr.w() as *mut u8)
				.unwrap_or_else(NonNull::dangling),
			len: 0,
		}
	}

	/// Creates a new `BitPtr<T>` from its components, without any validity
	/// checks.
	///
	/// # Safety
	///
	/// ***ABSOLUTELY NONE.*** This function *only* packs its arguments into the
	/// bit pattern of the `BitPtr<T>` type. It should only be used in contexts
	/// where a previously extant `BitPTR<T>` was constructed with ancestry
	/// known to have survived [`::new`], and any manipulations of its raw
	/// components are known to be valid for reconstruction.
	///
	/// # Parameters
	///
	/// See [`::new`].
	///
	/// # Returns
	///
	/// See [`::new`].
	///
	/// [`::new`]: #method.new
	pub(crate) unsafe fn new_unchecked(
		data: impl Into<Address<T>>,
		head: BitIdx<T::Mem>,
		bits: usize,
	) -> Self
	{
		let (data, head) = (data.into(), *head as usize);

		let ptr_data = data.u() & Self::PTR_DATA_MASK;
		let ptr_head = head >> Self::LEN_HEAD_BITS;

		let len_head = head & Self::LEN_HEAD_MASK;
		let len_bits = bits << Self::LEN_HEAD_BITS;

		let ptr: Address<u8> = (ptr_data | ptr_head).into();

		Self {
			_ty: PhantomData,
			ptr: NonNull::new_unchecked(ptr.w()),
			len: len_bits | len_head,
		}
	}

	/// Extracts the pointer to the first storage element.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// A pointer to the first storage element in the slice domain. The concrete
	/// type returned is opaque, and unusable outside this library.
	///
	/// # Safety
	///
	/// This pointer must be valid in the user’s memory model and allocation
	/// regime in order for the caller to dereference it.
	#[inline]
	pub(crate) fn pointer(&self) -> Address<T> {
		(self.ptr.as_ptr() as usize & Self::PTR_DATA_MASK).into()
	}

	/// Overwrites the data pointer with a new address. This method does not
	/// perform safety checks on the new pointer.
	///
	/// # Parameters
	///
	/// - `&mut self`
	/// - `ptr`: The new address of the `BitPtr<T>`’s domain.
	///
	/// # Safety
	///
	/// None. The invariants of `::new` must be checked at the caller.
	#[inline]
	#[cfg(feature = "alloc")]
	pub(crate) unsafe fn set_pointer(&mut self, ptr: impl Into<Address<T>>) {
		let mut data = ptr.into();
		if data.r().is_null() {
			*self = Self::empty();
			return;
		}
		data.u &= Self::PTR_DATA_MASK;
		data.u |= self.ptr.as_ptr() as usize & Self::PTR_HEAD_MASK;
		self.ptr = NonNull::new_unchecked(data.w() as *mut u8);
	}

	/// Extracts the element cursor of the head bit.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// A `BitIdx` that is the index of the first live bit in the first element.
	/// This will be in the domain `0 .. T::Mem::BITS`.
	#[inline]
	pub fn head(&self) -> BitIdx<T::Mem> {
		let ptr = self.ptr.as_ptr() as usize;
		let ptr_head = (ptr & Self::PTR_HEAD_MASK) << Self::LEN_HEAD_BITS;
		let len_head = self.len & Self::LEN_HEAD_MASK;
		((ptr_head | len_head) as u8).idx()
	}

	/// Counts how many bits are in the domain of a `BitPtr` slice.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// A count of the live bits in the slice.
	#[inline]
	pub fn len(&self) -> usize {
		self.len >> Self::LEN_HEAD_BITS
	}

	/// Overwrites the bit count with a new counter. This does not perform any
	/// safety checks.
	///
	/// # Parameters
	///
	/// - `&mut self`
	/// - `len: usize`: A new bit length for the `BitPtr<T>`’s domain.
	///
	/// # Safety
	///
	/// None. The caller must ensure that the invariants of `::new` are upheld.
	#[inline]
	pub unsafe fn set_len(&mut self, len: usize) {
		let n = (len << Self::LEN_HEAD_BITS) | (self.len & Self::LEN_HEAD_MASK);
		self.len = n;
	}

	/// Produces the raw components of the pointer structure.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// - `.0`: An opaque pointer to the `BitPtr<T>`’s memory region.
	/// - `.1`: The index of the first live bit in the bit region.
	/// - `.2`: The number of live bits in the bit region.
	#[inline]
	pub(crate) fn raw_parts(&self) -> (Address<T>, BitIdx<T::Mem>, usize) {
		(self.pointer(), self.head(), self.len())
	}

	/// Produces the count of all elements in the slice domain.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// The number of `T` elements in the slice domain.
	///
	/// # Safety
	///
	/// This size must be valid in the user’s memory model and allocation
	/// regime.
	pub fn elements(&self) -> usize {
		self.head().span(self.len()).0
	}

	/// Extracts the element cursor of the first dead bit *after* the tail bit.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// A `BitTail` that is the index of the first dead bit after the last live
	/// bit in the last element. This will almost always be in the domain
	/// `1 ..= T::BITS`.
	#[cfg(any(test, feature = "alloc"))]
	#[inline]
	pub(crate) fn tail(&self) -> BitTail<T::Mem> {
		let (head, len) = (self.head(), self.len());

		if *head == 0 && len == 0 {
			return 0u8.tail();
		}

		//  Compute the in-element tail index as the head plus the length,
		//  modulated to the element width.
		let tail = (*self.head() as usize + len) & T::Mem::MASK as usize;
		//  If the tail is zero, wrap it to `T::BITS` as the maximal. This
		//  upshifts `1` (tail is zero) or `0` (tail is not), then sets the
		//  upshift on the rest of the tail, producing something in the range
		//  `1 ..= T::BITS`.
		((((tail == 0) as u8) << T::Mem::INDX) | tail as u8).tail()
	}

	/// Accesses the element slice behind the pointer as a Rust slice.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// Standard Rust slice handle over the data governed by this pointer.
	///
	/// # Lifetimes
	///
	/// - `'a`: Lifetime for which the data behind the pointer is live.
	#[inline]
	pub fn as_slice<'a>(&self) -> &'a [T] {
		unsafe { slice::from_raw_parts(self.pointer().r, self.elements()) }
	}

	/// Accesses the element slice behind the pointer as a Rust mutable slice.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// Standard Rust slice handle over the data governed by this pointer.
	///
	/// # Lifetimes
	///
	/// - `'a`: Lifetime for which the data behind the pointer is live.
	#[inline]
	#[cfg(feature = "alloc")]
	pub fn as_mut_slice<'a>(&self) -> &'a mut [T] {
		unsafe { slice::from_raw_parts_mut(self.pointer().w, self.elements()) }
	}

	/// Converts a `BitSlice` handle into its `BitPtr` representation.
	///
	/// # Parameters
	///
	/// - `bs`: a `BitSlice` handle
	///
	/// # Returns
	///
	/// The `BitPtr<T>` structure composing the handle.
	pub(crate) fn from_bitslice<O>(bs: &BitSlice<O, T>) -> Self
	where O: BitOrder {
		let src = unsafe { &*(bs as *const BitSlice<O, T> as *const [()]) };
		let ptr = Address::from(src.as_ptr() as *const u8);
		let (ptr, len) = match (ptr.w(), src.len()) {
			(_, 0) => (NonNull::dangling(), 0),
			(p, _) if p.is_null() => unreachable!("Rust forbids null refs"),
			(p, l) => (unsafe { NonNull::new_unchecked(p) }, l),
		};
		Self {
			ptr,
			len,
			_ty: PhantomData,
		}
	}

	/// Converts a `BitPtr` structure into an immutable `BitSlice` handle.
	///
	/// # Parameters
	///
	/// - `self`
	///
	/// # Returns
	///
	/// A `BitSlice` handle composed of the `BitPtr` structure.
	pub(crate) fn into_bitslice<'a, O>(self) -> &'a BitSlice<O, T>
	where O: BitOrder {
		unsafe {
			&*(slice::from_raw_parts(
				Address::from(self.ptr.as_ptr()).r() as *const (),
				self.len,
			) as *const [()] as *const BitSlice<O, T>)
		}
	}

	/// Converts a `BitPtr` structure into a mutable `BitSlice` handle.
	///
	/// # Parameters
	///
	/// - `self`
	///
	/// # Returns
	///
	/// A `BitSlice` handle composed of the `BitPtr` structure.
	pub(crate) fn into_bitslice_mut<'a, O>(self) -> &'a mut BitSlice<O, T>
	where O: BitOrder {
		unsafe {
			&mut *(slice::from_raw_parts_mut(
				Address::from(self.ptr.as_ptr()).w() as *mut (),
				self.len,
			) as *mut [()] as *mut BitSlice<O, T>)
		}
	}

	/// Cast a `BitPtr<T>` into an equivalent `*mut BitSlice<O, T>`.
	#[cfg(feature = "alloc")]
	pub(crate) fn as_mut_ptr<O>(self) -> *mut BitSlice<O, T>
	where O: BitOrder {
		self.into_bitslice_mut() as *mut BitSlice<O, T>
	}

	/// Cast a `*mut BitSlice<O, T>` raw pointer into an equivalent `BitPtr<T>`.
	#[cfg(feature = "alloc")]
	pub(crate) fn from_mut_ptr<O>(ptr: *mut BitSlice<O, T>) -> Self
	where O: BitOrder {
		unsafe { &*ptr }.bitptr()
	}
}

impl<T> Clone for BitPtr<T>
where T: BitStore
{
	fn clone(&self) -> Self {
		Self { ..*self }
	}
}

impl<T> Copy for BitPtr<T> where T: BitStore
{
}

/** Proxy referential type, equivalent to `&mut bool`.

This structure is three words wide, and cannot ever fit into the existing Rust
language and library infrastructure in the way `&BitSlice` does. While `&mut`
write references are themselves an affine type, with a guaranteed single point
of destruction and no duplication, the language forbids writing finalization
logic for them.

This means that a custom reference type which implements `Deref` and `DerefMut`
to a location within the canonical handle, and on `Drop` writes the `Deref`
location into referent memory, is impossible. Short of that, a C++-style thick
reference-like type is as close as Rust will allow.
**/
pub struct BitMut<'a, O, T>
where
	O: BitOrder,
	T: 'a + BitStore,
{
	/// Inform the compiler that this has an exclusive borrow of a `BitSlice`
	pub(crate) _parent: PhantomData<&'a mut BitSlice<O, T>>,
	/// Typed pointer to the memory element containing the proxied bit.
	pub(crate) data: NonNull<T::Access>,
	/// Index of the proxied bit inside the targeted memory element.
	pub(crate) head: BitIdx<T::Mem>,
	/// A local cache for `Deref` usage.
	pub(crate) bit: bool,
}


/// Reimplementation of the `[T]` inherent-method API.
impl<O, T> BitSlice<O, T>
where
	O: BitOrder,
	T: BitStore,
{
	/// Returns the number of bits in the slice.
	///
	/// # Original
	///
	/// [`slice::len`](https://doc.rust-lang.org/std/primitive.slice.html#method.len)
	///
	/// # Examples
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let bits = 0u8.bits::<Local>();
	/// assert_eq!(bits.len(), 8);
	/// ```
	pub fn len(&self) -> usize {
		self.bitptr().len()
	}

	/// Returns `true` if the slice has a length of 0.
	///
	/// # Original
	///
	/// [`slice::is_empty`](https://doc.rust-lang.org/std/primitive.slice.html#method.is_empty)
	///
	/// # Examples
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let bits = 0u8.bits::<Local>();
	/// assert!(!bits.is_empty());
	///
	/// assert!(BitSlice::<Local, usize>::empty().is_empty())
	/// ```
	pub fn is_empty(&self) -> bool {
		self.bitptr().len() == 0
	}
}

impl<O, T> BitVec<O, T>
where
	O: BitOrder,
	T: BitStore,
{
	/// Constructs a new, empty `BitVec<O, T>` with the specified capacity.
	///
	/// The vector will be able to hold at least `capacity` bits without
	/// reallocating. If `capacity` is 0, the vector will not allocate.
	///
	/// It is important to note that although the returned vector has the
	/// *capacity* specified, the vector will have a zero *length*. For an
	/// explanation of the difference between length and capacity, see
	/// [*Capacity and reallocation*].
	///
	/// [*Capacity and reallocation*]: #capacity-and-reallocation
	pub fn with_capacity(capacity: usize) -> Self {
		//  Get the number of `T` elements needed to store the requested bit
		//  capacity.
		let elts = T::Mem::elts(capacity);
		//  Allocate a buffer that can hold that many elements.
		let v = Vec::with_capacity(elts);
		let (ptr, cap) = (v.as_ptr(), v.capacity());
		//  Disarm the `Vec` destructor.
		mem::forget(v);
		Self {
			_order: PhantomData,
			pointer: BitPtr::uninhabited(ptr),
			capacity: cap,
		}
	}

	/// Converts the bit-vector into [`Box<[T]>`].
	///
	/// Note that this will drop any excess capacity.
	///
	/// For the vec-to-box equivalent that produces a [`BitBox<O, T>`], see
	/// [`into_boxed_bitslice`].
	///
	/// # Examples
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let bv = bitvec![1, 0, 1];
	///
	/// let slice = bv.into_boxed_slice();
	/// ```
	///
	/// Any excess capacity is removed:
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let mut bv = BitVec::<Local, usize>::with_capacity(100);
	/// bv.extend([true, false, true].iter().copied());
	///
	/// assert!(bv.capacity() >= 100);
	/// let slice = bv.into_boxed_slice();
	/// let boxed_bitslice = BitBox::<Local, usize>::from_boxed_slice(slice);
	/// let bv = BitVec::from_boxed_bitslice(boxed_bitslice);
	/// assert!(bv.capacity() >= 3);
	/// ```
	///
	/// [`BitBox<O, T>`]: ../boxed/struct.BitBox.html
	/// [`Box<[T]>`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
	/// [`into_boxed_bitslice`]: #method.into_boxed_bitslice
	#[inline]
	pub fn into_boxed_slice(self) -> Box<[T]> {
		self.into_vec().into_boxed_slice()
	}

	/// Appends a bit to the back of the vector.
	///
	/// If the vector is at capacity, this may cause a reallocation.
	///
	/// # Panics
	///
	/// This will panic if the push will cause the vector to allocate above
	/// `BitPtr<T>::MAX_ELTS` or machine capacity.
	///
	/// # Examples
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let mut bv: BitVec = BitVec::new();
	/// assert!(bv.is_empty());
	/// bv.push(true);
	/// assert_eq!(bv.len(), 1);
	/// assert!(bv[0]);
	/// ```
	pub fn push(&mut self, value: bool) {
		let len = self.len();
		assert!(
			len <= BitPtr::<T>::MAX_BITS,
			"Capacity overflow: {} >= {}",
			len,
			BitPtr::<T>::MAX_BITS,
		);
		//  If self is empty *or* tail is at the back edge of an element, push
		//  an element onto the vector.
		if self.is_empty() || *self.pointer.tail() == T::Mem::BITS {
			self.with_vec(|v| v.push(T::Mem::ZERO.into()));
		}
		//  At this point, it is always safe to increment the tail, and then
		//  write to the newly live bit.
		unsafe {
			self.pointer.set_len(len + 1);
			self.set_unchecked(len, value);
		}
	}
}

/** Reborrows the `BitVec` as a `BitSlice`.

This mimics the separation between `Vec<T>` and `[T]`.
**/
impl<O, T> Deref for BitVec<O, T>
where
	O: BitOrder,
	T: BitStore,
{
	type Target = BitSlice<O, T>;

	/// Dereferences `&BitVec` down to `&BitSlice`.
	///
	/// # Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let bv: BitVec = bitvec![1; 4];
	/// let bref: &BitSlice = &bv;
	/// assert!(bref[2]);
	/// ```
	fn deref(&self) -> &Self::Target {
		self.as_bitslice()
	}
}

/** Mutably reborrows the `BitVec` as a `BitSlice`.

This mimics the separation between `Vec<T>` and `[T]`.
**/
impl<O, T> DerefMut for BitVec<O, T>
where
	O: BitOrder,
	T: BitStore,
{
	/// Dereferences `&mut BitVec` down to `&mut BitSlice`.
	///
	/// # Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let mut bv: BitVec = bitvec![0; 6];
	/// let bref: &mut BitSlice = &mut bv;
	/// assert!(!bref[5]);
	/// bref.set(5, true);
	/// assert!(bref[5]);
	/// ```
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_bitslice()
	}
}

/** A compact slice of bits, whose order and storage types can be customized.

`BitSlice` is a specialized slice type, which can only ever be held by
reference or specialized owning pointers provided by this crate. The value
patterns of its handles are opaque binary structures, which cannot be
meaningfully inspected by user code.

`BitSlice` can only be dynamically allocated by this library. Creation of any
other `BitSlice` collections will result in severely incorrect behavior.

A `BitSlice` reference can be created through the [`bitvec!`] macro, from a
[`BitVec`] collection, or from most common Rust types (fundamentals, slices of
them, and small arrays) using the [`Bits`] and [`BitsMut`] traits.

`BitSlice`s are a view into a block of memory at bit-level resolution. They are
represented by a crate-internal pointer structure that ***cannot*** be used with
other Rust code except through the provided conversion APIs.

```rust
use bitvec::prelude::*;

# #[cfg(feature = "alloc")] {
let bv = bitvec![0, 1, 0, 1];
//  slicing a bitvec
let bslice: &BitSlice = &bv[..];
# }

//  coercing an array to a bitslice
let bslice: &BitSlice<_, _> = [1u8, 254u8].bits::<Msb0>();
```

Bit slices are either mutable or shared. The shared slice type is
`&BitSlice<O, T>`, while the mutable slice type is `&mut BitSlice<O, T>`. For
example, you can mutate bits in the memory to which a mutable `BitSlice` points:

```rust
use bitvec::prelude::*;

let mut base = [0u8, 0, 0, 0];
let bs: &mut BitSlice<_, _> = base.bits_mut::<Msb0>();
bs.set(13, true);
eprintln!("{:?}", bs.as_slice());
assert!(bs[13]);
assert_eq!(base[1], 4);
```

# Type Parameters

- `O`: An implementor of the `BitOrder` trait. This type is used to convert
  semantic indices into concrete bit positions in elements, and store or
  retrieve bit values from the storage type.
- `T`: An implementor of the `BitStore` trait: `u8`, `u16`, `u32`, or `u64`
  (64-bit systems only). This is the actual type in memory that the slice will
  use to store data.

# Safety

The `&BitSlice` reference handle has the same *size* as standard Rust slice
handles, but it is ***extremely value-incompatible*** with them. Attempting to
treat `&BitSlice<_, T>` as `&[T]` in any manner except through the provided APIs
is ***catastrophically*** unsafe and unsound.

[`BitVec`]: ../vec/struct.BitVec.html
[`Bits`]: ../bits/trait.Bits.html
[`BitsMut`]: ../bits/trait.BitsMut.html
[`From`]: https://doc.rust-lang.org/stable/std/convert/trait.From.html
[`bitvec!`]: ../macro.bitvec.html
**/
#[repr(transparent)]
pub struct BitSlice<O = Local, T = usize>
where
	O: BitOrder,
	T: BitStore,
{
	/// BitOrder type for selecting bits inside an element.
	_kind: PhantomData<O>,
	/// Element type of the slice.
	///
	/// eddyb recommends using `PhantomData<T>` and `[()]` instead of `[T]`
	/// alone.
	_type: PhantomData<T>,
	/// Slice of elements `T` over which the `BitSlice` has usage.
	_elts: [()],
}

impl<O, T> BitSlice<O, T>
where
	O: BitOrder,
	T: BitStore,
{
	/// Sets a bit at an index, without doing bounds checking.
	///
	/// This is generally not recommended; use with caution! For a safe
	/// alternative, see [`set`].
	///
	/// # Parameters
	///
	/// - `&mut self`
	/// - `index`: The bit index to retrieve. This index is *not* checked
	///   against the length of `self`.
	///
	/// # Effects
	///
	/// The bit at `index` is set to `value`.
	///
	/// # Safety
	///
	/// This method is **not** safe. It performs raw pointer arithmetic to seek
	/// from the start of the slice to the requested index, and set the bit
	/// there. It does not inspect the length of `self`, and it is free to
	/// perform out-of-bounds memory *write* access.
	///
	/// Use this method **only** when you have already performed the bounds
	/// check, and can guarantee that the call occurs with a safely in-bounds
	/// index.
	///
	/// # Examples
	///
	/// This example uses a bit slice of length 2, and demonstrates
	/// out-of-bounds access to the last bit in the element.
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let mut src = 0u8;
	/// let bits = &mut src.bits_mut::<Msb0>()[2 .. 4];
	/// assert_eq!(bits.len(), 2);
	/// unsafe {
	///     bits.set_unchecked(5, true);
	/// }
	/// assert_eq!(src, 1);
	/// ```
	///
	/// [`set`]: #method.set
	pub unsafe fn set_unchecked(&mut self, index: usize, value: bool) {
		let bitptr = self.bitptr();
		let (elt, bit) = bitptr.head().offset(index as isize);
		let data_ptr = bitptr.pointer().a();
		(*data_ptr.offset(elt)).set::<O>(bit, value);
	}

	/// Accesses the underlying pointer structure.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// The [`BitPtr`] structure of the slice handle.
	///
	/// [`BitPtr`]: ../pointer/struct.BitPtr.html
	#[inline]
	pub(crate) fn bitptr(&self) -> BitPtr<T> {
		BitPtr::from_bitslice(self)
	}
}

/** Generalize over types which may be used to access memory holding bits.

This trait is implemented on the fundamental integers, their `Cell<>` wrappers,
and (if present) their `Atomic` variants. Users provide this type as a parameter
to their data structures in order to inform the structure of how it may access
memory.

Specifically, this has the advantage that a `BitSlice<_, Cell<_>>` knows that it
has a view of memory that will not undergo concurrent modification. As such, it
can skip using atomic accesses, and just use ordinary load/store instructions,
without fear of causing observable race conditions.

The associated types `Mem` and `Alias` allow implementors to know the register
width of the memory they describe (`Mem`) and to change the aliasing status of
a slice.

A universal property of `BitSlice` regions is that for any handle, it may be
described as a triad of:

- zero or one partially-used, aliased, elements at the head
- zero or more wholly-used, unaliased, elements in the body
- zero or one partially-used, aliased, elements at the tail

As such, a `&BitSlice` reference with any aliasing type can be split into its
`Self::Alias` variant for the edges, and `Cell<Self::Mem>` for the interior,
without violating memory safety.
**/
pub trait BitStore: seal::Sealed + Sized {
	/// The fundamental integer type of the governed memory.
	type Mem: BitMemory + Into<Self>;
	/// The type used for performing memory accesses.
	type Access: BitAccess<Self::Mem> + BitStore;
	/// The destination type when marking a region as known-aliased.
	type Alias: BitStore + BitAccess<Self::Mem>;
	/// The destination type when marking a region as known-unaliased.
	type NoAlias: BitStore;

	/// Mark whether a type is threadsafe when viewed as bits.
	///
	/// This is necessary because `Cell<T: Send>` is `Send`, but `Cell` is *not*
	/// synchronized and thus cannot be used for aliasing, parallel, bit
	/// manipulation.
	#[doc(hidden)]
	type Threadsafe;

	/* Note: The `NoAlias` type had its `BitAccess` bound removed so that the
	integers and atoms could form a cycle, rather than trending into `Cell`.
	This had the unpleasant side effect of making `T::NoAlias` use sites much
	less pleasant to use in generic contexts, due to the inability of the Rust
	type system to unwind associated types. This removal necessitated the
	addition of the `.retype()` method in `BitMemory`, and the `.get_elem()` and
	`.set_elem()` methods below.

	Attempting to do this same `BitAccess` bound removal on `Alias` proved to
	be *extremely* awful, as the change required adding these new methods
	throughout the crate. This is needless spaghetti code, required only by the
	inadequacy of the type system to smoothly handle the graph of associated
	types used in this crate. It is, to be fair, my fault for attempting to
	cause cycles, when the type system expects a DAG.

	Long story short: don’t try to remove the bound in the future. It needs to
	stay off of `NoAlias`, because making a `BitAccess` wrapper type for the
	integers is profoundly unpleasant. Don’t do that either.

	Hopefully, the aliasing detection work is the last major overhaul of the
	memory access system, and these modules will be left alone unless
	demonstrated to be unsound.
	*/
}

/// Batch implementation of `BitStore` for appropriate types.
macro_rules! bitstore {
	($($t:ty => $a:ty),* $(,)?) => { $(
		impl seal::Sealed for $t {}

		impl BitStore for $t {
			/// The unsigned integers are only the `BitStore` parameter for
			/// unaliased slices.
			type Access = Cell<Self>;

			/// Aliases are required to use atomic access, as `BitSlice`s of
			/// this type are safe to move across threads.
			#[cfg(feature = "atomic")]
			type Alias = $a;

			/// Aliases are permitted to use `Cell` wrappers and ordinary
			/// access, as `BitSlice`s of this type are forbidden from crossing
			/// threads.
			#[cfg(not(feature = "atomic"))]
			type Alias = Cell<Self>;

			type Mem = Self;

			type NoAlias = Self;

			#[doc(hidden)]
			type Threadsafe = Self;
		}

		#[cfg(feature = "atomic")]
		impl seal::Sealed for $a {}

		#[cfg(feature = "atomic")]
		impl BitStore for $a {
			/// Atomic stores always use atomic accesses.
			type Access = Self;

			type Alias = Self;

			type Mem = $t;

			type NoAlias = $t;

			#[doc(hidden)]
			type Threadsafe = Self;
		}

		impl seal::Sealed for Cell<$t> {}

		impl BitStore for Cell<$t> {
			/// `Cell`s always use ordinary, unsynchronized, accesses, as the
			/// type system forbids them from creating memory collisions.
			type Access = Self;

			type Alias = Self;

			type Mem = $t;

			type NoAlias = Self;

			/// Raw pointers are never threadsafe, so this prevents
			/// `BitSlice<_, Cell<_>>` from crossing threads.
			#[doc(hidden)]
			type Threadsafe = *const Self;
		}
	)* };
}

bitstore!(
	u8 => atomic::AtomicU8,
	u16 => atomic::AtomicU16,
	u32 => atomic::AtomicU32,
	usize => atomic::AtomicUsize,
);

#[cfg(target_pointer_width = "64")]
bitstore!(u64 => atomic::AtomicU64);

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_fail!(concat!(
	"This architecture is currently not supported. File an issue at ",
	env!("CARGO_PKG_REPOSITORY")
));

/// Enclose the `Sealed` trait against client use.
mod seal {
	/// Marker trait to seal `BitStore` against downstream implementation.
	///
	/// This trait is public in the module, so that other modules in the crate
	/// can use it, but so long as it is not exported by the crate root and this
	/// module is private, this trait effectively forbids downstream
	/// implementation of the `BitStore` trait.
	#[doc(hidden)]
	pub trait Sealed {}
}

/** A compact [`Vec`] of bits, whose order and storage type can be customized.

`BitVec` is a newtype wrapper over `Vec`, and as such is exactly three words in
size on the stack.

# Examples

```rust
use bitvec::prelude::*;

let mut bv: BitVec = BitVec::new();
bv.push(false);
bv.push(true);

assert_eq!(bv.len(), 2);
assert_eq!(bv[0], false);

assert_eq!(bv.pop(), Some(true));
assert_eq!(bv.len(), 1);

bv.set(0, true);
assert_eq!(bv[0], true);

bv.extend([0u8, 1, 0].iter().map(|n| *n != 0u8));
for bit in &*bv {
  println!("{}", bit);
}
assert_eq!(bv, bitvec![1, 0, 1, 0]);
```

The [`bitvec!`] macro is provided to make initialization more convenient.

```rust
use bitvec::prelude::*;

let mut bv = bitvec![0, 1, 2, 3];
bv.push(false);
assert_eq!(bv, bitvec![0, 1, 1, 1, 0]);
```

It can also initialize each element of a `BitVec<_, T>` with a given value. This
may be more efficient than performing allocation and initialization in separate
steps, especially when initializing a vector of zeros:

```rust
use bitvec::prelude::*;

let bv = bitvec![0; 15];
assert_eq!(bv, bitvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

// The following is equivalent, but potentially slower:
let mut bv1: BitVec = BitVec::with_capacity(15);
bv1.resize(15, false);
```

Use a `BitVec<T>` as an efficient stack:

```rust
use bitvec::prelude::*;
let mut stack: BitVec = BitVec::new();

stack.push(false);
stack.push(true);
stack.push(true);

while let Some(top) = stack.pop() {
  //  Prints true, true, false
  println!("{}", top);
}
```

# Indexing

The `BitVec` type allows you to access values by index, because it implements
the [`Index`] trait. An example will be more explicit:

```rust
use bitvec::prelude::*;

let bv = bitvec![0, 0, 1, 1];
println!("{}", bv[1]); // it will display 'false'
```

However, be careful: if you try to access an index which isn’t in the `BitVec`,
your software will panic! You cannot do this:

```rust,should_panic
use bitvec::prelude::*;

let bv = bitvec![0, 1, 0, 1];
println!("{}", bv[6]); // it will panic!
```

In conclusion: always check if the index you want to get really exists before
doing it.

# Slicing

A `BitVec` is growable. A [`BitSlice`], on the other hand, is fixed size. To get
a bit slice, use `&`. Example:

```rust
use bitvec::prelude::*;
fn read_bitslice(slice: &BitSlice) {
  // use slice
}

let bv = bitvec![0, 1];
read_bitslice(&bv);

// … and that’s all!
// you can also do it like this:
let bs: &BitSlice = &bv;
```

In Rust, it’s more common to pass slices as arguments rather than vectors when
you do not want to grow or shrink it. The same goes for [`Vec`] and [`&[]`], and
[`String`] and [`&str`].

# Capacity and reallocation

The capacity of a bit vector is the amount of space allocated for any future
bits that will be added onto the vector. This is not to be confused with the
*length* of a vector, which specifies the number of live, useful bits within the
vector. If a vector’s length exceeds its capacity, its capacity will
automatically be increased, but its storage elements will have to be
reallocated.

For example, a bit vector with capacity 10 and length 0 would be an allocated,
but uninhabited, vector, with space for ten more bits. Pushing ten or fewer bits
onto the vector will not change its capacity or cause reallocation to occur.
However, if the vector’s length is increased to eleven, it will have to
reallocate, which can be slow. For this reason, it is recommended to use
[`BitVec::with_capacity`] whenever possible to specify how big the bit vector is
expected to get.

# Guarantees

Due to its incredibly fundamental nature, `BitVec` makes a lot of guarantees
about its design. This ensures that it is as low-overhead as possible in the
general case, and can be correctly manipulated in fundamental ways by `unsafe`
code.

Most fundamentally, `BitVec` is and always will be a `([`BitPtr`], capacity)`
doublet. No more, no less. The order of these fields is unspecified, and you
should **only** interact with the members through the provided APIs. Note that
`BitPtr` is ***not directly manipulable***, and must ***never*** be written or
interpreted as anything but opaque binary data by user code.

When a `BitVec` has allocated memory, then the memory to which it points is on
the heap (as defined by the allocator Rust is configured to use by default), and
its pointer points to [`len`] initialized bits in order of the [`BitOrder`] type
parameter, followed by `capacity - len` logically uninitialized bits.

`BitVec` will never perform a “small optimization” where elements are stored in
its handle representation, for two reasons:

- It would make it more difficult for user code to correctly manipulate a
  `BitVec`. The contents of the `BitVec` would not have a stable address if the
  handle were moved, and it would be more difficult to determine if a `BitVec`
  had allocated memory.

- It would penalize the general, heap-allocated, case by incurring a branch on
  every access.

`BitVec` will never automatically shrink itself, even if it is emptied. This
ensures that no unnecessary allocations or deallocations occur. Emptying a
`BitVec` and then refilling it to the same length will incur no calls to the
allocator. If you wish to free up unused memory, use [`shrink_to_fit`].

## Erasure

`BitVec` will not specifically overwrite any data that is removed from it, nor
will it specifically preserve it. Its uninitialized memory is scratch space that
may be used however the implementation desires, and must not be relied upon as
stable. Do not rely on removed data to be erased for security purposes. Even if
you drop a `BitVec`, its buffer may simply be reused for other data structures
in your program. Even if you zero a `BitVec`’s memory first, that may not
actually occur if the optimizer does not consider this an observable side
effect. There is one case that will never break, however: using `unsafe` to
construct a `[T]` slice over the `BitVec`’s capacity, and writing to the excess
space, then increasing the length to match, is always valid.

# Type Parameters

- `O: BitOrder`: An implementor of the [`BitOrder`] trait. This type is used to
  convert semantic indices into concrete bit positions in elements, and store or
  retrieve bit values from the storage type.
- `T: BitStore`: An implementor of the [`BitStore`] trait: `u8`, `u16`, `u32`,
  or `u64` (64-bit systems only). This is the actual type in memory that the
  vector will use to store data.

# Safety

The `BitVec` handle has the same *size* as standard Rust `Vec` handles, but it
is ***extremely binary incompatible*** with them. Attempting to treat
`BitVec<_, T>` as `Vec<T>` in any manner except through the provided APIs is
***catastrophically*** unsafe and unsound.

[`BitSlice`]: ../slice/struct.BitSlice.html
[`BitVec::with_capacity`]: #method.with_capacity
[`BitStore`]: ../store/trait.BitStore.html
[`BitOrder`]: ../order/trait.BitOrder.html
[`Index`]: https://doc.rust-lang.org/stable/std/ops/trait.Index.html
[`String`]: https://doc.rust-lang.org/stable/std/string/struct.String.html
[`Vec`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
[`bitvec!`]: ../macro.bitvec.html
[`clear_on_drop`]: https://docs.rs/clear_on_drop
[`len`]: #method.len
[`shrink_to_fit`]: #method.shrink_to_fit
[`&str`]: https://doc.rust-lang.org/stable/std/primitive.str.html
[`&[]`]: https://doc.rust-lang.org/stable/std/primitive.slice.html
**/
#[repr(C)]
pub struct BitVec<O = Local, T = usize>
where
	O: BitOrder,
	T: BitStore,
{
	/// Phantom `BitOrder` member to satisfy the constraint checker.
	_order: PhantomData<O>,
	/// Slice pointer over the owned memory.
	pointer: BitPtr<T>,
	/// The number of *elements* this vector has allocated.
	capacity: usize,
}

impl<O, T> BitVec<O, T>
where
	O: BitOrder,
	T: BitStore,
{
	/// Produces a `BitSlice` containing the entire vector.
	///
	/// Equivalent to `&s[..]`.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// A `BitSlice` over the vector.
	///
	/// # Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let bv = bitvec![0, 1, 1, 0];
	/// let bs = bv.as_bitslice();
	/// ```
	#[inline]
	pub fn as_bitslice(&self) -> &BitSlice<O, T> {
		self.pointer.into_bitslice()
	}

	/// Produces a mutable `BitSlice` containing the entire vector.
	///
	/// Equivalent to `&mut s[..]`.
	///
	/// # Parameters
	///
	/// - `&mut self`
	///
	/// # Returns
	///
	/// A mutable `BitSlice` over the vector.
	///
	/// # Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let mut bv = bitvec![0, 1, 1, 0];
	/// let bs = bv.as_mut_bitslice();
	/// ```
	#[inline]
	pub fn as_mut_bitslice(&mut self) -> &mut BitSlice<O, T> {
		self.pointer.into_bitslice_mut()
	}

	/// Degrades a `BitVec` to a `BitBox`, freezing its size.
	///
	/// # Parameters
	///
	/// - `self`
	///
	/// # Returns
	///
	/// Itself, with its size frozen and ungrowable.
	pub fn into_boxed_bitslice(self) -> BitBox<O, T> {
		let pointer = self.pointer;
		//  Convert the Vec allocation into a Box<[T]> allocation
		mem::forget(self.into_boxed_slice());
		unsafe { BitBox::from_raw(pointer.as_mut_ptr()) }
	}

	/// Degrades a `BitVec` to a standard `Vec`.
	///
	/// # Parameters
	///
	/// - `self`
	///
	/// # Returns
	///
	/// The plain vector underlying the `BitVec`.
	pub fn into_vec(self) -> Vec<T> {
		let slice = self.pointer.as_mut_slice();
		let out = unsafe {
			Vec::from_raw_parts(slice.as_mut_ptr(), slice.len(), self.capacity)
		};
		mem::forget(self);
		out
	}

	/// Permits a function to modify the `Vec<T>` underneath a `BitVec<_, T>`.
	///
	/// This produces a `Vec<T>` structure referring to the same data region as
	/// the `BitVec<_, T>`, allows a function to mutably view it, and then
	/// forgets the `Vec<T>` after the function concludes.
	///
	/// # Parameters
	///
	/// - `&mut self`
	/// - `func`: A function which receives a mutable borrow to the `Vec<T>`
	///   underlying the `BitVec<_, T>`.
	///
	/// # Type Parameters
	///
	/// - `F: FnOnce(&mut Vec<T>) -> R`: Any callable object (function or
	///   closure) which receives a mutable borrow of a `Vec<T>`.
	///
	/// - `R`: The return value from the called function or closure.
	fn with_vec<F, R>(&mut self, func: F) -> R
	where F: FnOnce(&mut Vec<T>) -> R {
		let slice = self.pointer.as_mut_slice();
		let mut v = unsafe {
			Vec::from_raw_parts(slice.as_mut_ptr(), slice.len(), self.capacity)
		};
		let out = func(&mut v);
		//  The only change is that the pointer might relocate. The region data
		//  will remain untouched. Vec guarantees it will never produce an
		//  invalid pointer.
		unsafe {
			self.pointer.set_pointer(v.as_ptr());
		}
		// self.pointer = unsafe { BitPtr::new_unchecked(v.as_ptr(), e, h, t) };
		self.capacity = v.capacity();
		mem::forget(v);
		out
	}
}
