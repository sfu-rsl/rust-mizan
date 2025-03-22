#![doc= " Parallel bitfield access.\n\nThis module provides parallel, multiple-bit, access to a `BitSlice`. This\nfunctionality permits the use of `BitSlice` as a library-level implementation of\nthe bitfield language feature found in C and C++.\n\nThe `BitField` trait is not sealed against client implementation, as there is no\nuseful way to automatically use a `BitOrder` implementation to provide a\nuniversal behavior. As such, the trait has some requirements that the compiler\ncannot enforce for client implementations.\n\n# Batch Behavior\n\nThe purpose of this trait is to provide access to arbitrary bit regions as if\nthey were an ordinary memory location. As such, it is important for\nimplementations of this trait to provide shift/mask register transfer behavior\nwhere possible, for as wide a span as possible in each action. Implementations\nof this trait should *not* use bit-by-bit iteration.\n\n# Register Bit Order Preservation\n\nAs a default assumption – user orderings *may* violate this, but *should* not –\neach element of slice memory used to store part of a value should not reorder\nthe value bits. Transfer between slice memory and a CPU register should solely\nbe an ordinary value load or store between memory and the register, and a\nshift/mask operation to select the part of the value that is live.\n\n# Endianness\n\nThe `_le` and `_be` methods of `BitField` refer to the order in which\n`T: BitStore` elements of the slice are assigned significance when containing\nfragments of a stored data value. Within any `T` element, the order of its\nconstituent bytes is *not* governed by the `BitField` trait method.\n\nThe provided `BitOrder` implementors `Lsb0` and `Msb0` use the local machine’s\nbyte ordering. Other cursors *may* implement ordering of bytes within `T`\nelements differently, for instance by calling `.to_be_bytes` before store and\n`from_be_bytes` after load.\n!"]

use crate::{
    mem::BitMemory,
    order::{
        BitOrder,
        Lsb0,
        Msb0,
    },
    slice::BitSlice,
    store::BitStore,
};
#[cfg(feature = "alloc")]
use crate::{
    boxed::BitBox,
    vec::BitVec,
};
