#![doc= " Iteration processes for `BitSlice`."]

use super::*;
use core::{
    fmt::{
        self,
        Debug,
        Formatter,
    },
    iter::FusedIterator,
};


#[doc= " Immutable slice iterator\n\nThis struct is created by the [`iter`] method on [`BitSlice`]s.\n\n# Examples\n\nBasic usage:\n\n```rust\n# use bitvec::prelude::*;\nlet data = 5u8;\nlet bits = data.bits::<Lsb0>();\n\nfor bit in bits[.. 4].iter() {\n  println!(\"{}\", bit);\n}\n```\n\n[`BitSlice`]: struct.BitSlice.html\n[`iter`]: struct.BitSlice.html#method.iter\n*"]
#[derive(Clone, Debug)]
pub struct Iter<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " The `BitSlice` undergoing iteration."]
    pub(super) inner: &'a BitSlice<O, T>,
}

impl<'a, O, T> Iterator for Iter<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    type Item = &'a bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn count(self) -> usize {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Mutable slice iterator.\n\nThis struct is created by the [`iter_mut`] method on [`BitSlice`]s.\n\n# API Differences\n\nThis is required to return references marked as aliasing, as you are permitted\nto keep the returned references alive in parallel.\n\n# Examples\n\nBasic usage:\n\n```rust\n# use bitvec::prelude::*;\nlet mut data = 0u8;\nlet bits = data.bits_mut::<Msb0>();\nassert!(bits.not_any());\nfor mut bit in bits.iter_mut() {\n  *bit = true;\n}\nassert!(bits.all());\n```\n\n[`BitSlice`]: struct.BitSlice.html\n[`iter_mut`]: struct.BitSlice.html#method.iter_mut\n*"]
#[derive(Debug)]
pub struct IterMut<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " The `BitSlice` undergoing iteration."]
    pub(super) inner: &'a mut BitSlice<O, T::Alias>,
}

#[doc= " An iterator over a slice in (non-overlapping) chunks (`width` bits at a\ntime), starting at the beginning of the slice.\n\nWhen the slice length is not evenly divided by the chunk size, the last slice of\nthe iteration will be the remainder.\n\nThis struct is created by the [`chunks`] method on [`BitSlice`]s.\n\n[`BitSlice`]: struct.BitSlice.html\n[`chunks`]: struct.BitSlice.html#method.chunks\n*"]
#[derive(Clone, Debug)]
pub(crate) struct Chunks<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " The `BitSlice` undergoing iteration."]
    pub(super) inner: &'a BitSlice<O, T>,
}

impl<'a, O, T> Iterator for Chunks<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    type Item = &'a BitSlice<O, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn count(self) -> usize {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " An iterator over a slice in (non-overlapping) chunks (`width` bits at a\ntime), starting at the beginning of the slice.\n\nWhen the slice length is not evenly divided by the chunk size, the last up to\n`width - 1` bits will be omitted but can be retrieved from the\n[`remainder`] function from the iterator.\n\nThis struct is created by the [`chunks_exact`] method on [`BitSlice`]s.\n\n[`BitSlice`]: struct.BitSlice.html\n[`chunks_exact`]: struct.BitSlice.html#method.chunks_exact\n[`remainder`]: #method.remainder\n*"]
#[derive(Clone, Debug)]
pub(crate) struct ChunksExact<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " Remainder of the original `BitSlice`."]
    pub(super) extra: &'a BitSlice<O, T>,
}

#[doc= " An iterator over a slice in (non-ovelapping) mutable chunks (`width` bits at\na time), starting at the beginning of the slice.\n\nWhen the slice length is not evenly divided by the chunk size, the last up to\n`width - 1` elements will be omitted but can be retrieved from the\n[`into_remainder`] function from the iterator.\n\nThis struct is created by the [`chunks_exact_mut`] method on [`BitSlice`]s.\n\n# API Differences\n\nThis is required to return references marked as aliasing, as you are permitted\nto keep the returned references alive in parallel.\n\n[`BitSlice`]: struct.BitSlice.html\n[`chunks_exact_mut`]: struct.BitSlice.html#method.chunks_exact_mut\n[`into_remainder`]: #method.into_remainder\n*"]
#[derive(Debug)]
pub(crate) struct ChunksExactMut<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " Remainder of the original `BitSlice`."]
    pub(super) extra: &'a mut BitSlice<O, T::Alias>,
}

#[doc= " An iterator over a slice in (non-overlapping) mutable chunks (`width` bits\nat a time), starting at the beginning of the slice.\n\nWhen the slice length is not evenly divided by the chunk size, the last slice of\nthe iteration will be the remainder.\n\nThis struct is created by the [`chunks_mut`] method on [`BitSlice`]s.\n\n# API Differences\n\nThis is required to return references marked as aliasing, as you are permitted\nto keep the returned references alive in parallel.\n\n[`BitSlice`]: struct.BitSlice.html\n[`chunks_mut`]: struct.BitSlice.html#chunks_mut\n*"]
#[derive(Debug)]
pub(crate) struct ChunksMut<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " The `BitSlice` undergoing iteration."]
    pub(super) inner: &'a mut BitSlice<O, T::Alias>,
}

#[doc= " An iterator over a slice in (non-overlapping) chunks (`width` bits at a\ntime), starting at the end of the slice.\n\nWhen the slice length is not evenly divided by the chunk size, the last slice of\nthe iteration will be the remainder.\n\nThis struct is created by the [`rchunks`] method on [`BitSlice`]s.\n\n[`BitSlice`]: struct.BitSlice.html\n[`rchunks`]: struct.BitSlice.html#method.rchunks\n*"]
#[derive(Clone, Debug)]
pub(crate) struct RChunks<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " The `BitSlice` undergoing iteration."]
    pub(super) inner: &'a BitSlice<O, T>,
}

#[doc= " An iterator over a slice in (non-overlapping) chunks (`width` bits\nat a time), starting at the end of the slice.\n\nWhen the slice length is not evenly divided by the chunk size, the last up to\n`width - 1` bits will be omitted but can be retrieved from the [`remainder`]\nfunction from the iterator.\n\nThis struct is created by the [`rchunks_exact`] method on [`BitSlice`]s.\n\n[`BitSlice`]: struct.BitSlice.html\n[`rchunks_exact`]: struct.BitSlice.html#method.rchunks_exact\n[`remainder`]: #method.remainder\n*"]
#[derive(Clone, Debug)]
pub(crate) struct RChunksExact<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " Remainder of the original `BitSlice`."]
    pub(super) extra: &'a BitSlice<O, T>,
}

#[doc= " An iterator over a slice in (non-overlapping) mutable chunks (`width` bits\nat a time), starting at the end of the slice.\n\nWhen the slice len is not evenly divided by the chunk size, the last up to\n`width - 1` bits will be omitted but can be retrieved from the\n[`into_remainder`] function from the iterator.\n\nThis struct is created by the [`rchunks_exact_mut`] method on [`BitSlice`]s.\n\n# API Differences\n\nThis is required to return references marked as aliasing, as you are permitted\nto keep the returned references alive in parallel.\n\n[`BitSlice`]: struct.BitSlice.html\n[`into_remainder`]: #method.into_remainder\n[`rchunks_exact_mut`]: struct.BitSlice.html#rchunks_exact_mut\n*"]
#[derive(Debug)]
pub(crate) struct RChunksExactMut<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " Remainder of the original BitSlice`."]
    pub(super) extra: &'a mut BitSlice<O, T::Alias>,
}

#[doc= " An iterator over a slice in (non-overlapping) mutable chunks (`width` bits\nat a time), starting at the end of the slice.\n\nWhen the slice length is not evenly divided by the chunk size, the last slice of\nthe iteration will be the remainder.\n\nThis struct is created by the [`rchunks_mut`] method on [`BitSlice`]s.\n\n# API Differences\n\nThis is required to return references marked as aliasing, as you are permitted\nto keep the returned references alive in parallel.\n\n[`BitSlice`]: struct.BitSlice.html\n[`rchunks_mut`]: struct.BitSlice.html#method.rchunks_mut\n*"]
#[derive(Debug)]
pub(crate) struct RChunksMut<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " The `BitSlice` undergoing iteration."]
    pub(super) inner: &'a mut BitSlice<O, T::Alias>,
}

#[doc= " An internal abstraction over the splitting iterators, so that\n`{,r}splitn{,_mut}` can have a single implementation.\n*"]
#[doc(hidden)]
pub(super) trait SplitIter: DoubleEndedIterator {
    #[doc= " Marks the underlying iterator as complete, extracting the remaining"]
    #[doc= " portion of the slice."]
    fn finish(&mut self) -> Option<Self::Item>;
}

#[doc= " An iterator over subslices separated by bits that satisfy a predicate\nfunction.\n\nThis struct is created by the [`split`] method on [`BitSlice`]s.\n\n[`BitSlice`]: struct.BitSlice.html\n[`split`]: struct.BitSlice.html#method.split\n*"]
#[derive(Clone)]
pub(crate) struct Split<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[doc= " The `BitSlice` undergoing iteration."]
    pub(super) inner: &'a BitSlice<O, T>,
    #[doc= " The testing function."]
    pub(super) func: F,
}

impl<'a, O, T, F> SplitIter for Split<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[inline]
    fn finish(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T, F> Iterator for Split<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    type Item = &'a BitSlice<O, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T, F> DoubleEndedIterator for Split<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " An iterator over subslices separated by positions that satisfy a predicate\nfunction.\n\nThis struct is created by the [`split_mut`] method on [`BitSlice`]s.\n\n# API Differences\n\nThis is required to return references marked as aliasing, as you are permitted\nto keep the returned references alive in parallel.\n\n[`BitSlice`]: struct.BitSlice.html\n[`split_mut`]: struct.BitSlice.html#method.split_mut\n*"]
pub(crate) struct SplitMut<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[doc= " The `BitSlice` undergoing iteration."]
    pub(super) inner: &'a mut BitSlice<O, T::Alias>,
    #[doc= " The testing function."]
    pub(super) func: F,
}

impl<'a, O, T, F> SplitIter for SplitMut<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[inline]
    fn finish(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T, F> Iterator for SplitMut<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    type Item = &'a mut BitSlice<O, T::Alias>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T, F> DoubleEndedIterator for SplitMut<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " An iterator over subslices separated by bits that satisfy a predicate\nfunction, starting from the end of the slice.\n\nThis struct is created by the [`rsplit`] method on [`BitSlice`]s.\n\n[`BitSlice`]: struct.BitSlice.html\n[`rsplit`]: struct.BitSlice.html#rsplit\n*"]
#[derive(Clone)]
pub(crate) struct RSplit<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[doc= " This delegates to `Split`, and switches `next` and `next_back`."]
    pub(super) inner: Split<'a, O, T, F>,
}

impl<'a, O, T, F> SplitIter for RSplit<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    fn finish(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T, F> Iterator for RSplit<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    type Item = &'a BitSlice<O, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T, F> DoubleEndedIterator for RSplit<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " An iterator over subslices separated by bits that satisfy a predicate\nfunction, starting from the end of the slice.\n\nThis struct is created by the [`rsplit_mut`] method on [`BitSlice`]s.\n\n# API Differences\n\nThis is required to return references marked as aliasing, as you are permitted\nto keep the returned references alive in parallel.\n\n[`BitSlice`]: struct.BitSlice.html\n[`rsplit_mut`]: struct.BitSlice.html#rsplit_mut\n*"]
pub(crate) struct RSplitMut<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[doc= " This delegates to `SplitMut`, and switches `next` and `next_back`."]
    pub(super) inner: SplitMut<'a, O, T, F>,
}

impl<'a, O, T, F> SplitIter for RSplitMut<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    fn finish(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T, F> Iterator for RSplitMut<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    type Item = &'a mut BitSlice<O, T::Alias>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T, F> DoubleEndedIterator for RSplitMut<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

pub(super) struct GenericSplitN<I>
where
    I: SplitIter {
    #[doc= " Some splitting wrapper."]
    pub(super) inner: I,
}

#[doc= " An iterator over subslices separated by positions that satisfy a predicate\nfunction, limited to a given number of splits.\n\nThis struct is created by the [`splitn`] method on [`BitSlice`]s.\n\n[`BitSlice`]: struct.BitSlice.html\n[`splitn`]: struct.BitSlice.html#method.splitn\n*"]
pub(crate) struct SplitN<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[doc= " The interior splitter."]
    pub(super) inner: GenericSplitN<Split<'a, O, T, F>>,
}

#[doc= " An iterator over mutable subslices separated by positions that satisfy a\npredicate function, limited to a given number of splits.\n\nThis struct is created by the [`splitn_mut`] method on [`BitSlice`]s.\n\n# API Differences\n\nThis is required to return references marked as aliasing, as you are permitted\nto keep the returned references alive in parallel.\n\n[`BitSlice`]: struct.BitSlice.html\n[`splitn_mut`]: struct.BitSlice.html#method.splitn_mut\n*"]
pub(crate) struct SplitNMut<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[doc= " The interior splitter."]
    pub(super) inner: GenericSplitN<SplitMut<'a, O, T, F>>,
}

#[doc= " An iterator over subslices separated by positions that satisfy a predicate\nfunction, limited to a given number of splits, starting from the end of the\nslice.\n\nThis struct is created by the [`rsplitn`] method on [`BitSlice`]s.\n\n[`BitSlice`]: struct.BitSlice.html\n[`rsplitn`]: struct.BitSlice.html#method.rsplitn\n*"]
pub(crate) struct RSplitN<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[doc= " The interior splitter."]
    pub(super) inner: GenericSplitN<RSplit<'a, O, T, F>>,
}

#[doc= " An iterator over mutable subslices separated by positions that satisfy a\npredicate function, limited to a given number of splits, starting from the end\nof the slice.\n\nThis struct is created by the [`rsplitn_mut`] method on [`BitSlice`]s.\n\n# API Differences\n\nThis is required to return references marked as aliasing, as you are permitted\nto keep the returned references alive in parallel.\n\n[`BitSlice`]: struct.BitSlice.html\n[`rsplitn_mut`]: struct.BitSlice.html#method.rsplitn_mut\n*"]
pub(crate) struct RSplitNMut<'a, O, T, F>
where
    O: BitOrder,
    T: 'a + BitStore,
    F: FnMut(usize, &bool) -> bool {
    #[doc= " The interior splitter."]
    pub(super) inner: GenericSplitN<RSplitMut<'a, O, T, F>>,
}

#[doc= " An iterator over overlapping subslices of some width.\n\nThis struct is created by the [`windows`] method on [`BitSlice`]s.\n\n[`BitSlice`]: struct.BitSlice.html\n[`windows`]: struct.BitSlice.html#method.windows\n*"]
#[derive(Clone, Debug)]
pub(crate) struct Windows<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " The `BitSlice` undergoing iteration."]
    pub(super) inner: &'a BitSlice<O, T>,
}
