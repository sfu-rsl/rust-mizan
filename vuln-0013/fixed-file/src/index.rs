#![doc= " Indexing within memory elements.\n\nThis module provides types which guarantee certain properties about selecting\nbits within a memory element. These types enable their use sites to explicitly\ndeclare the indexing behavior they require, and move safety checks from runtime\nto compile time.\n\n# Bit Indexing\n\nThe [`BitIdx`] type represents the semantic index of a bit within a memory\nelement. It does not perform bit positioning, and cannot be used to create a\nshift instruction or mask value. It is transformed into a value which can do\nthese things – [`BitPos`] – through the [`BitOrder::at`] function.\n\n# Region End Marker\n\n`bitvec` uses “half-open” ranges, described by a starting point and a count of\nmembers that are live. This means that the “end” of a range is not the last\nmember that is *in*cluded in the range, but rather the first member that is\n*ex*cluded from it.\n\nThis requires the [`BitTail` end marker to include in its range the width of the\nelement type (`8` for `u8`, etc), in order to mark that a region includes the\nvery last bit in the element (index `7` for `u8`, etc`).\n\nThe starting number for a dead region cannot be used to perform bit selection,\nbut is used to provide range computation, so it is kept distinct from the\nindexing types.\n\n# Bit Positioning\n\nThe [`BitPos`] type corresponds directly to a bit position in a memory element.\nIts value can be used to create shift instructions which select part of memory.\nIt is only ever created by the `BitOrder::at` function.\n\n# Bit Selection\n\nThe [`BitSel`] type is a one-hot mask encoding for a memory element. Unlike the\nprevious types, which are range-limited integers, this type is a wrapper over a\nmemory element and guarantees that it can be used as a mask value in `&` and `|`\noperations to modify exactly one bit. It is equivalent to `1 << BitPos.value()`.\n\n# Bit Masking\n\nLastly, the [`BitMask`] type is a bitmask that permits any number of bits to be\nset or cleared. It is provided as a type rather than a bare value in order to\nclearly communicate that there is no restriction on what this mask may affect.\n\n[`BitIdx`]: struct.BitIdx.html\n[`BitMask`]: struct.BitMask.html\n[`BitOrder::at`]: ../order/trait.BitOrder.html#method.at\n[`BitPos`]: struct.BitPos.html\n[`BitSel`]: struct.BitSel.html\n[`BitTail`]: struct.BitTail.html\n!"]

use crate::mem::BitMemory;
use core::{
    fmt::{
        self,
        Binary,
        Formatter,
    },
    iter::{
        Product,
        Sum,
    },
    marker::PhantomData,
    ops::{
        BitAnd,
        BitOr,
        Deref,
        Not,
    },
};
#[cfg(feature = "serde")]
use core::convert::TryFrom;

#[doc= " Indicates a semantic index of a bit within a memory element.\n\nThis is a counter in the domain `0 .. M::BITS`, and marks a semantic position\nin the ordering sequence described by a [`BitOrder`] implementation. It is used\nfor both position computation through `BitOrder` and range computation in\n[`BitPtr`].\n\n# Type Parameters\n\n- `M`: The memory element type controlled by this index.\n\n[`BitOrder`]: ../order/trait.BitOrder.html\n[`BitPtr`]: ../pointer/struct.BitPtr.html\n*"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitIdx<M>
where
    M: BitMemory {
    #[doc= " Semantic index within an element. Constrained to `0 .. M::BITS`."]
    idx: u8,
    #[doc= " Marker for the indexed type."]
    _ty: PhantomData<M>,
}

impl<M> BitIdx<M>
where
    M: BitMemory {
    #[doc= " The zero index."]
    pub(crate) const ZERO: Self = Self {
        idx: 0,
        _ty: PhantomData,
    };

    #[doc= " Wraps a counter value as a known-good index of the `M` element type."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `idx`: A semantic index within a `M` memory element."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " If `idx` is within the range `0 .. M::BITS`, then this returns the index"]
    #[doc= " value wrapped in the index type; if `idx` exceeds this range, then this"]
    #[doc= " returns `None`."]
    pub(crate) fn new(idx: u8) -> Option<Self> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Wraps a counter value as a known-good index of the `M` element type."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `idx`: A semantic index within a `M` memory element. It must be in the"]
    #[doc= "   range `0 .. M::BITS`."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " If `idx` is outside the range, then the produced value will cause errors"]
    #[doc= " and memory unsafety when used."]
    #[inline]
    pub(crate) unsafe fn new_unchecked(idx: u8) -> Self {
        debug_assert!(idx < M::BITS, "Bit index {} cannot exceed type width {}", idx, M::BITS);
        Self {
            idx,
            _ty: PhantomData,
        }
    }

    #[doc= " Finds the destination bit a certain distance away from a starting bit."]
    #[doc= ""]
    #[doc= " This produces the number of elements to move from the starting point,"]
    #[doc= " and then the bit index of the destination bit in the destination"]
    #[doc= " element."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `self`: A bit index in some memory element, used as the starting"]
    #[doc= "   position for the offset calculation."]
    #[doc= " - `by`: The number of bits by which to move. Negative values move"]
    #[doc= "   downwards in memory: towards index zero, then counting from index"]
    #[doc= "   `M::MASK` to index zero in the next element lower in memory, repeating"]
    #[doc= "   until arrival. Positive values move upwards in memory: towards index"]
    #[doc= "   `M::MASK`, then counting from index zero to index `M::MASK` in the"]
    #[doc= "   next element higher in memory, repeating until arrival."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " - `.0`: The number of elements by which to offset the caller’s element"]
    #[doc= "   cursor. This value can be passed directly into [`ptr::offset`]."]
    #[doc= " - `.1`: The bit index of the destination bit in the element selected by"]
    #[doc= "   applying the `.0` pointer offset."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " `by` must not be far enough to cause the returned element offset value"]
    #[doc= " to, when applied to the original memory address via [`ptr::offset`],"]
    #[doc= " produce a reference out of bounds of the original allocation. This"]
    #[doc= " method has no way of checking this requirement."]
    #[doc= ""]
    #[doc= " [`ptr::offset`]: https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset"]
    pub(crate) fn offset(self, by: isize) -> (isize, Self) {
        let val = *self;
        let (far, ovf) = by.overflowing_add(val as isize);
        if !ovf {
            if (0 .. M::BITS as isize).contains(&far) {
                (0, (far as u8).idx())
            } else {
                (far >> M::INDX, (far as u8 & M::MASK).idx())
            }
        } else {
            let far = far as usize;
            ((far >> M::INDX) as isize, (far as u8 & M::MASK).idx())
        }
    }

    #[doc= " Computes the size of a span from `self` for `len` bits."]
    #[doc= ""]
    #[doc= " Spans always extend upwards in memory."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `self`: The starting bit position of the span."]
    #[doc= " - `len`: The number of bits to include in the span."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " - `.0`: The number of elements of `M` included in the span. If `len` is"]
    #[doc= "   `0`, this will be `0`; otherwise, it will be at least one."]
    #[doc= " - `.1`: The index of the first dead bit *after* the span. If `self` and"]
    #[doc= "   `len` are both `0`, this will be `0`; otherwise, it will be in the"]
    #[doc= "   domain `1 ..= M::BITS`."]
    #[doc= ""]
    #[doc= " # Notes"]
    #[doc= ""]
    #[doc= " This defers to [`BitTail::span`], because `BitTail` is a strict superset"]
    #[doc= " of `BitIdx` (it is `{ BitIdx | M::BITS }`), and spans frequently begin"]
    #[doc= " from the tail of a slice in this crate. The `offset` function is *not*"]
    #[doc= " implemented on `BitTail`, and remains on `BitIdx` because offsets can"]
    #[doc= " only be computed from bit addresses that exist. It does not make sense"]
    #[doc= " to compute the offset from a `M::BITS` tail."]
    #[doc= ""]
    #[doc= " [`BitTail::span`]: struct.BitTail.html#method.span"]
    #[inline]
    pub(crate) fn span(self, len: usize) -> (usize, BitTail<M>) {
        unsafe {
            BitTail::new_unchecked(*self)
        }.span(len)
    }
}

impl<M> Binary for BitIdx<M>
where
    M: BitMemory {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<M> Deref for BitIdx<M>
where
    M: BitMemory {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.idx
    }
}

#[cfg(feature = "serde")]
impl<M> TryFrom<u8> for BitIdx<M>
where
    M: BitMemory {
    type Error = &'static str;

    fn try_from(idx: u8) -> Result<Self, Self::Error> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Indicates a semantic index of a dead bit *beyond* a memory element.\n\nThis type is equivalent to `BitIdx<M>`, except that it includes `M::BITS` in its\ndomain. Instances of this type will only ever contain `0` when the span they\ndescribe is *empty*. Non-empty spans always cycle through the domain\n`1 ..= M::BITS`.\n\nThis type cannot be used for indexing, and does not translate to `BitPos<M>`.\nThis type has no behavior other than viewing its internal `u8` for arithmetic.\n\n# Type Parameters\n\n- `M`: The memory element type controlled by this tail.\n*"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitTail<M>
where
    M: BitMemory {
    #[doc= " Semantic index *after* an element. Constrained to `0 ..= M::BITS`."]
    end: u8,
    #[doc= " Marker for the tailed type."]
    _ty: PhantomData<M>,
}

impl<M> BitTail<M>
where
    M: BitMemory {
    #[doc= " The termination index."]
    pub(crate) const END: Self = Self {
        end: M::BITS,
        _ty: PhantomData,
    };

    #[doc= " Mark that `end` is a tail index for a type."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `end` must be in the range `0 ..= M::BITS`."]
    pub(crate) unsafe fn new_unchecked(end: u8) -> Self {
        debug_assert!(end <= M::BITS, "Bit tail {} cannot surpass type width {}", end, M::BITS);
        Self {
            end,
            _ty: PhantomData,
        }
    }

    pub(crate) fn span(self, len: usize) -> (usize, Self) {
        let val = *self;
        debug_assert!(val <= M::BITS, "Tail out of range: {} overflows type width {}", val, M::BITS);
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
    }
}

impl<M> Deref for BitTail<M>
where
    M: BitMemory {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.end
    }
}

#[doc= " Indicates a real electrical index within an element.\n\nThis type is produced by [`BitOrder`] implementors, and marks a specific\nelectrical bit within a memory element, rather than [`BitIdx`]’s semantic bit.\n\n# Type Parameters\n\n- `M`: A `BitMemory` element which provides bounds-checking information. The\n  [`new`] constructor uses [`M::BITS`] to ensure that constructed `BitPos`\n  instances are always valid to use within `M` elements.\n\n[`BitIdx`]: struct.BitIdx.html\n[`BitOrder`]: ../order/trait.BitOrder.html\n[`M::BITS`]: ../mem/trait.BitMemory.html#associatedconstant.BITS\n[`new`]: #method.new\n*"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitPos<M>
where
    M: BitMemory {
    #[doc= " Marker for the positioned type."]
    _ty: PhantomData<M>,
}

impl<M> BitPos<M>
where
    M: BitMemory {
    #[doc= " Produce a new bit position marker at a valid position value."]
    #[doc= ""]
    #[doc= " `BitOrder` implementations should prefer this method, but *may* use"]
    #[doc= " [`::new_unchecked`] if they can guarantee that the range invariant is"]
    #[doc= " upheld."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `pos`: The bit position value to encode. It must be in the range `0 .."]
    #[doc= "   M::BITS`."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " This function panics if `pos` is greater than or equal to `M::BITS`."]
    #[doc= ""]
    #[doc= " [`::new_unchecked`]: #method.new_unchecked"]
    #[inline]
    pub(crate) fn new(pos: u8) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Produce a new bit position marker at any position value."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " The caller *must* ensure that `pos` is less than `M::BITS`. `BitOrder`"]
    #[doc= " implementations should prefer [`::new`], which panics on range failure."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `pos`: The bit position value to encode. This must be in the range `0"]
    #[doc= "   .. M::BITS`."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " `pos` wrapped in the `BitPos` marker type."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " This function panics if `pos` is greater than or equal to `M::BITS`, but"]
    #[doc= " only in debug builds. It does not inspect `pos` in release builds."]
    #[doc= ""]
    #[doc= " [`::new`]: #method.new"]
    #[inline]
    pub(crate) unsafe fn new_unchecked(pos: u8) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Produces a one-hot selector mask from a position value."]
    #[doc= ""]
    #[doc= " This is equivalent to `1 << *self`."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A one-hot selector mask with the bit at `*self` set."]
    #[inline]
    pub(crate) fn select(self) -> BitSel<M> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<M> Deref for BitPos<M>
where
    M: BitMemory {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Wrapper type indicating a one-hot encoding of a bit mask for an element.\n\nThis type is produced by [`BitOrder`] implementations to speed up access to the\nunderlying memory. It ensures that masks have exactly one set bit, and can\nsafely be used as a mask for read/write access to memory.\n\n# Type Parameters\n\n- `M`: The storage type being masked.\n\n[`BitOrder`]: ../order/trait.BitOrder.html\n*"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitSel<M>
where
    M: BitMemory {
    #[doc= " Mask value."]
    sel: M,
}

impl<M> BitSel<M>
where
    M: BitMemory {
    #[doc= " Produce a new bit-mask wrapper around a one-hot mask value."]
    #[doc= ""]
    #[doc= " `BitOrder` implementations should prefer this method, but *may* use"]
    #[doc= " [`::new_unchecked`] if they can guarantee that the one-hot invariant is"]
    #[doc= " upheld."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `mask`: The mask value to encode. This **must** have exactly one bit"]
    #[doc= "   set high, and all others set low."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " `mask` wrapped in the `BitMask` marker type."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " This function unconditionally panics if `mask` has zero or multiple bits"]
    #[doc= " set high."]
    #[doc= ""]
    #[doc= " [`::new_unchecked`]: #method.new_unchecked"]
    #[inline]
    pub(crate) fn new(sel: M) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Produce a new bit-mask wrapper around any value."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " The caller *must* ensure that `mask` has exactly one bit set. `BitOrder`"]
    #[doc= " implementations should prefer [`::new`], which always panics on failure."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `mask`: The mask value to encode. This must have exactly one bit set."]
    #[doc= "   Failure to uphold this requirement will introduce uncontrolled state"]
    #[doc= "   contamination."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " `mask` wrapped in the `BitMask` marker type."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " This function panics if `mask` has zero or multiple bits set, only in"]
    #[doc= " debug builds. It does not inspect `mask` in release builds."]
    #[doc= ""]
    #[doc= " [`::new`]: #method.new"]
    #[inline]
    pub(crate) unsafe fn new_unchecked(sel: M) -> Self {
        debug_assert!(
            sel.count_ones() == 1,
            "Masks are required to have exactly one set bit: {:0>1$b}",
            sel,
            M::BITS as usize
        );
        Self { sel }
    }
}

impl<M> Deref for BitSel<M>
where
    M: BitMemory {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.sel
    }
}

#[doc= " A multi-bit selector mask.\n\nUnlike [`BitSel`], which enforces a strict one-hot mask encoding, this mask type\npermits any number of bits to be set or unset. This is used to combine batch\noperations in an element.\n\nIt is only constructed by accumulating [`BitPos`] or [`BitSel`] values. As\n`BitSel` is only constructed from `BitPos`, and `BitPos` is only constructed\nfrom [`BitIdx`] and [`BitOrder`], this enforces a chain of responsibility to\nprove that a given multimask is safe.\n\n[`BitIdx`]: struct.BitIdx.html\n[`BitOrder`]: ../order/trait.BitOrder.html\n[`BitPos`]: struct.BitPos.html\n[`BitSel`]: struct.BitSel.html\n*"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitMask<M>
where
    M: BitMemory {
    #[doc= " A mask of any number of bits to modify."]
    mask: M,
}

impl<M> BitMask<M>
where
    M: BitMemory {
    #[doc= " A full mask."]
    pub(crate) const ALL: Self = Self { mask: M::ALL };
    #[doc= " An empty mask."]
    pub(crate) const ZERO: Self = Self { mask: M::ZERO };

    #[doc= " Wraps a value as a bitmask."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " The caller must ensure that the mask value is correct in the caller’s"]
    #[doc= " provenance."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `mask`: Any integer, to be reïnterpreted as a bitmask."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The `mask` value as a bitmask."]
    pub(crate) fn new(mask: M) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<M> Product<BitPos<M>> for BitMask<M>
where
    M: BitMemory {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = BitPos<M>> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<M> Product<BitSel<M>> for BitMask<M>
where
    M: BitMemory {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = BitSel<M>> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Enable accumulation of a multi-bit mask from a sequence of position values."]
impl<M> Sum<BitPos<M>> for BitMask<M>
where
    M: BitMemory {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = BitPos<M>> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Enable accumulation of a multi-bit mask from a sequence of selector masks."]
impl<M> Sum<BitSel<M>> for BitMask<M>
where
    M: BitMemory {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = BitSel<M>> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<M> BitAnd<M> for BitMask<M>
where
    M: BitMemory {
    type Output = Self;

    fn bitand(self, rhs: M) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<M> BitAnd<BitPos<M>> for BitMask<M>
where
    M: BitMemory {
    type Output = Self;

    fn bitand(self, rhs: BitPos<M>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<M> BitAnd<BitSel<M>> for BitMask<M>
where
    M: BitMemory {
    type Output = Self;

    fn bitand(self, rhs: BitSel<M>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<M> BitOr<M> for BitMask<M>
where
    M: BitMemory {
    type Output = Self;

    fn bitor(self, rhs: M) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Insert a position value into a multimask."]
impl<M> BitOr<BitPos<M>> for BitMask<M>
where
    M: BitMemory {
    type Output = Self;

    fn bitor(self, rhs: BitPos<M>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Insert a single selector into a multimask."]
impl<M> BitOr<BitSel<M>> for BitMask<M>
where
    M: BitMemory {
    type Output = Self;

    fn bitor(self, rhs: BitSel<M>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<M> Deref for BitMask<M>
where
    M: BitMemory {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<M> Not for BitMask<M>
where
    M: BitMemory {
    type Output = Self;

    fn not(self) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Internal convenience trait for wrapping numbers with appropriate markers.\n\nThis trait must only be used on values that are known to be valid for their\ncontext. It provides an internal-only shorthand for wrapping integer literals\nand known-good values in marker types.\n\nIt is only implemented on `u8`.\n*"]
pub(crate) trait Indexable {
    #[doc= " Wraps a value as a `BitIdx<M>`."]
    fn idx<M>(self) -> BitIdx<M>
    where
        M: BitMemory;
    #[doc= " Wraps a value as a `BitTail<M>`."]
    fn tail<M>(self) -> BitTail<M>
    where
        M: BitMemory;
    #[doc= " Wraps a value as a `BitPos<M>`."]
    fn pos<M>(self) -> BitPos<M>
    where
        M: BitMemory;
}

impl Indexable for u8 {
    fn idx<M>(self) -> BitIdx<M>
    where
        M: BitMemory {
        unsafe {
            BitIdx::<M>::new_unchecked(self)
        }
    }

    fn tail<M>(self) -> BitTail<M>
    where
        M: BitMemory {
        unsafe {
            BitTail::<M>::new_unchecked(self)
        }
    }

    fn pos<M>(self) -> BitPos<M>
    where
        M: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_far_up() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn jump_far_down() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
