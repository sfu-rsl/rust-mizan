/*! `BitVec` structure

This module holds the main working type of the library. Clients can use
`BitSlice` directly, but `BitVec` is much more useful for most work.

The `BitSlice` module discusses the design decisions for the separation between
slice and vector types.
!*/

#![cfg(feature = "alloc")]

use crate::{
	boxed::BitBox,
	order::{
		BitOrder,
		Local,
	},
	pointer::BitPtr,
	slice::BitSlice,
	store::BitStore,
};

use alloc::vec::Vec;

use core::{
	marker::PhantomData,
	mem,
	ptr,
};

use funty::IsInteger;

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
		let (_, head, bits) = self.bitptr().raw_parts();
		let boxed = self.into_vec().into_boxed_slice();
		let addr = boxed.as_ptr();
		mem::forget(boxed);
		unsafe {
			BitBox::from_raw(
				BitPtr::new_unchecked(addr, head, bits).as_mut_ptr(),
			)
		}
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

mod api;
mod ops;

pub use api::*;
