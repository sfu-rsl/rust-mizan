#![doc= " `bitvec` – `[bool]` in overdrive.\n\nThis crate provides views into slices of bits that are truly `[u1]`. Each bit in\nthe data segment is used, unlike `[bool]` which ignores seven bits out of every\nbyte.\n\n`bitvec`’s data structures provide strong guarantees about, and fine-grained\ncontrol of, the bit-level representation of a sequence of memory. The user is\nempowered to choose the fundamental type underlying the store – `u8`, `u16`,\n`u32`, or `u64` – and the order in which each primitive is traversed – `Msb0`,\nfrom the most significant bit to the least, or `Lsb0`, from the least\nsignificant bit to the most.\n\nThis level of control is not necessary for most use cases where users just want\nto put bits in a sequence, but it is critically important for users making\npackets that leave main memory and hit some external device like a peripheral\ncontroller or a network socket. In order to provide convenience to users for\nwhom the storage details do not matter, `bitvec` types default to using the\nlocal C bitfield ordering and CPU word size.\n\nIn addition to providing compact, efficient, and powerful storage and\nmanipulation of bits in memory, the `bitvec` structures are capable of acting as\na queue, set, or stream of bits. They implement the bit-wise operators for\nBoolean arithmetic, arithmetic operators for 2’s-complement numeric arithmetic,\nread indexing, bit shifts, and access to the underlying storage fundamental\nelements as a slice.\n\n(Write indexing is impossible in Rust semantics.)\n!"]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(debug_assertions, warn(missing_docs))]
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![deny(unconditional_recursion)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub(crate) mod macros;
mod access;
pub(crate) mod domain;
pub(crate) mod index;
pub(crate) mod mem;
pub(crate) mod order;
mod pointer;
pub mod prelude;
pub(crate) mod slice;
pub(crate) mod store;
#[cfg(feature = "alloc")]
pub(crate) mod boxed;
#[cfg(feature = "alloc")]
pub(crate) mod vec;
