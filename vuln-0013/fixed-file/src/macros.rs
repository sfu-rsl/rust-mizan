#![doc= " Utility macros for constructing data structures and implementing bulk types.\n\nThe public macros are `bits!`, `bitvec!`, and `bitbox!`.\n!"]

#[macro_use]
#[doc(hidden)]
pub(crate) mod internal;

#[doc= " Construct a `&BitSlice` out of a literal array in source code, like `vec!`.\n\n`bits!` can be invoked in a number of ways. It takes the name of a `BitOrder`\nimplementation, the name of a `BitStore`-implementing fundamental (which must be\none of `u8`, `u16`, `u32`, or `u64`), and zero or more fundamentals (integer,\nfloating-point) which are used to build the bits. Each fundamental literal\ncorresponds to one bit, and is considered to represent `1` if it is any other\nvalue than exactly zero.\n\n`bits!` can be invoked with no specifiers, a `BitOrder` specifier, or a\n`BitOrder` and a `BitStore` specifier. It cannot be invoked with a `BitStore`\nspecifier but no `BitOrder` specifier, due to overlap in how those tokens are\nmatched by the macro system.\n\nLike `vec!`, `bits!` supports bit lists `[0, 1, …]` and repetition markers\n`[1; n]`.\n\n# Examples\n\n```rust\nuse bitvec::prelude::*;\n\nbits![Msb0, u8; 0, 1];\nbits![Lsb0, u8; 0, 1,];\nbits![Msb0; 0, 1];\nbits![Lsb0; 0, 1,];\nbits![0, 1];\nbits![0, 1,];\nbits![0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0];\nbits![Msb0, u8; 1; 5];\nbits![Lsb0; 0; 5];\nbits![1; 5];\nbits![Local; 0, 1,];\n```\n*"]
#[macro_export]
macro_rules! bits{
    ($order: ident, $store: ident; $($val: expr), * $(,) ?) => {
        {
            static DATA:&[$store] =& $crate:: __bits_store_array !($order, $store; $($val), *);
            & $crate:: __bits_from_slice !($order, $store, $crate:: __count !($($val), *), DATA)
        }
    };
    ($order: path, $store: ident; $($val: expr), * $(,) ?) => {
        {
            static DATA:&[$store] =& $crate:: __bits_store_array !($order, $store; $($val), *);
            & $crate:: __bits_from_slice !($order, $store, $crate:: __count !($($val), *), DATA)
        }
    };
    ($order: ident; $($val: expr), * $(,) ?) => {
        $crate:: bits !(
            $order,
            usize;
            $($val),
            *
        )
    };
    ($order: path; $($val: expr), * $(,) ?) => {
        $crate:: bits !(
            $order,
            usize;
            $($val),
            *
        )
    };
    ($($val: expr), * $(,) ?) => {
        $crate:: bits !(
            Local,
            usize;
            $($val),
            *
        )
    };
    ($order: ident, $store: ident; $val: expr; $len: expr) => {
        {
            static DATA:&[$store] =&[$crate:: __extend_bool !($val, $store); $crate:: mem:: elts::< $store >($len)];
            & $crate:: __bits_from_slice !($order, $store, $len, DATA)
        }
    };
    ($order: path, $store: ident; $val: expr; $len: expr) => {
        {
            static DATA:&[$store] =&[$crate:: __extend_bool !($val, $store); $crate:: mem:: elts::< $store >($len)];
            & $crate:: __bits_from_slice !($order, $store, $len, DATA)
        }
    };
    ($order: ident; $val: expr; $len: expr) => {
        $crate:: bits !(
            $order,
            usize;
            $val;
            $len
        )
    };
    ($order: path; $val: expr; $len: expr) => {
        $crate:: bits !(
            $order,
            usize;
            $val;
            $len
        )
    };
    ($val: expr; $len: expr) => {
        $crate:: bits !(
            Local,
            usize;
            $val;
            $len
        )
    };
}

#[doc= " Construct a `BitVec` out of a literal array in source code, like `vec!`.\n\n`bitvec!` can be invoked in a number of ways. It takes the name of a `BitOrder`\nimplementation, the name of a `BitStore`-implementing fundamental, and zero or\nmore fundamentals (integer, floating-point, or boolean) which are used to build\nthe bits. Each fundamental literal corresponds to one bit, and is considered to\nrepresent `1` if it is any other value than exactly zero.\n\n`bitvec!` can be invoked with no specifiers, a `BitOrder` specifier, or a\n`BitOrder` and a `BitStore` specifier. It cannot be invoked with a `BitStore`\nspecifier but no `BitOrder` specifier, due to overlap in how those tokens are\nmatched by the macro system.\n\nLike `vec!`, `bitvec!` supports bit lists `[0, 1, …]` and repetition markers\n`[1; n]`.\n\n# Examples\n\n```rust\nuse bitvec::prelude::*;\n\nbitvec![Msb0, u8; 0, 1];\nbitvec![Lsb0, u8; 0, 1,];\nbitvec![Msb0; 0, 1];\nbitvec![Lsb0; 0, 1,];\nbitvec![0, 1];\nbitvec![0, 1,];\nbitvec![Msb0, u8; 1; 5];\nbitvec![Lsb0; 0; 5];\nbitvec![1; 5];\n```\n*"]
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! bitvec{
    ($order: ty, $store: ident; $val: expr; $rep: expr) => {
        $crate:: vec:: BitVec::< $order,
        $store >:: repeat($val != 0, $rep)
    };
    ($order: ty; $val: expr; $rep: expr) => {
        $crate:: bitvec !(
            $order,
            usize;
            $val;
            $rep
        )
    };
    ($val: expr; $rep: expr) => {
        $crate:: bitvec !(
            $crate:: order:: Local,
            usize;
            $val;
            $rep
        )
    };
    ($($arg: tt) *) => {
        {
            let bits:& 'static $crate:: slice:: BitSlice::< _,
            _ >= $crate:: bits !($($arg) *);
            $crate:: vec:: BitVec:: from_bitslice(bits)
        }
    };
}

#[doc= " Construct a `BitBox` out of a literal array in source code, like `bitvec!`.\n\nThis has exactly the same syntax as [`bitvec!`], and in fact is a thin wrapper\naround `bitvec!` that calls `.into_boxed_slice()` on the produced `BitVec` to\nfreeze it.\n\n[`bitvec!`]: #macro.bitvec\n*"]
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! bitbox{
    ($($arg: tt) *) => {
        $crate:: bitvec ![$($arg) *].into_boxed_bitslice()
    };
}
