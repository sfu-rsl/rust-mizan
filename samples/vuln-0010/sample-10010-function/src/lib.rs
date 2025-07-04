use std::ops::Range;
use typenum::*;
use sized_chunks as sc;
use std::rc::Rc as RRc;
use std::sync::Arc as RArc;

enum Size {
    Size(usize),
    Table(PoolRef<Chunk<usize>>),
}

// Invariants: Nodes only at level > 0, Values/Empty only at level = 0
enum Entry<A> {
    Nodes(Size, PoolRef<Chunk<Node<A>>>),
    Values(PoolRef<Chunk<A>>),
    Empty,
}

pub(crate) struct Node<A> {
    children: Entry<A>,
}

pub(crate) type VectorChunkSize = U64;
pub(crate) type Chunk<A> = sc::sized_chunk::Chunk<A, VectorChunkSize>;

// `Ref` == `Arc` when threadsafe
#[cfg(threadsafe)]
pub(crate) type Ref<A> = std::sync::Arc<A>;

#[derive(Default)]
pub(crate) struct Rc<A>(RRc<A>);

#[cfg(all(not(threadsafe), not(feature = "pool")))]
pub(crate) use crate::{Rc as PoolRef};

// `Rc` with refpool
#[cfg(all(not(threadsafe), feature = "pool"))]
pub(crate) type PoolRef<A> = refpool::PoolRef<A>;

// `Ref` == `Rc` when not threadsafe
#[cfg(not(threadsafe))]
pub(crate) type Ref<A> = std::rc::Rc<A>;

#[doc(hidden)]
pub struct Rrb<A> {
    length: usize,
    middle_level: usize,
    outer_f: PoolRef<Chunk<A>>,
    inner_f: PoolRef<Chunk<A>>,
    middle: Ref<Node<A>>,
    inner_b: PoolRef<Chunk<A>>,
    outer_b: PoolRef<Chunk<A>>,
}

pub struct TreeFocus<A> {
    tree: Rrb<A>,
    view: Range<usize>,
    middle_range: Range<usize>,
    target_range: Range<usize>,
    target_ptr: *const Chunk<A>,
}

#[allow(unsafe_code)]
#[cfg(threadsafe)]
unsafe impl<A: Send> Send for TreeFocus<A> {}
#[allow(unsafe_code)]
#[cfg(threadsafe)]
unsafe impl<A: Sync> Sync for TreeFocus<A> {}
