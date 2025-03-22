#![doc= " Internal implementation macros for the public exports.\n\nThe macros in this module are required to be exported from the crate, as the\npublic macros will call them from client contexts (`macro_rules!` expansion\nbodies are not in source crate scope, as they are token expansion rather than\nsymbolic calls). However, they are not part of the public *API* of the crate,\nand are not intended for use anywhere but in the expansion bodies of the\npublic-API constructor macros.\n!"]
#![doc(hidden)]

#[doc= " Ensures that the ordering tokens map to a known ordering type path."]
#[doc(hidden)]
#[macro_export]
macro_rules! __bits_from_slice{
    (Local, $store: ident, $len: expr, $slice: ident) => {
        $crate:: slice:: BitSlice::< $crate:: order:: Local,
        $store >:: from_slice($slice,)[..$len]
    };
    (Lsb0, $store: ident, $len: expr, $slice: ident) => {
        $crate:: slice:: BitSlice::< $crate:: order:: Lsb0,
        $store >:: from_slice($slice,)[..$len]
    };
    (Msb0, $store: ident, $len: expr, $slice: ident) => {
        $crate:: slice:: BitSlice::< $crate:: order:: Msb0,
        $store >:: from_slice($slice,)[..$len]
    };
    ($order: tt, $store: ident, $len: expr, $slice: ident) => {
        $crate:: slice:: BitSlice::< $order,
        $store >:: from_slice($slice)[..$len]
    };
}

#[doc= " Accumulates a stream of bit expressions into a compacted array of elements.\n\nThis macro constructs a well-ordered `[T; N]` array expression usable in `const`\ncontexts. Callers may then use that expression as the source slice over which to\nconstruct `bitvec` types.\n*"]
#[doc(hidden)]
#[macro_export]
macro_rules! __bits_store_array{
    (
        $order: tt,
        usize;
        $($val: expr),
        *
    ) => {
        {
            const LEN: usize = $crate:: mem:: elts::< usize >($crate:: __count !($($val), *),);
            #[cfg(target_pointer_width = "32")] const OUT:[
                usize;
                LEN
            ] = $crate:: __bits_store_array !($order, u32 @ usz; $($val), *);
            #[cfg(target_pointer_width = "64")] const OUT:[
                usize;
                LEN
            ] = $crate:: __bits_store_array !($order, u64 @ usz; $($val), *);
            OUT
        }
    };
    ($order: tt, $store: ident $(@ $usz: ident) ?; $($val: expr), *) => {
        $crate:: __bits_store_array !(
            $order,
            $store $(@ $usz) ?,
            [];
            $($val,) * 0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0
        );
    };
    ($order: tt, $store: ident @ usz, [$(($($elt: tt), *)) *]; $(0), *) => {
        [$($crate:: __elt_from_bits !($order, $store; $($elt), *) as usize), *]
    };
    ($order: tt, $store: ident, [$(($($elt: tt), *)) *]; $(0), *) => {
        [$($crate:: __elt_from_bits !($order, $store; $($elt), *)), *]
    };
    (
        $order: tt,
        u8 $(@ $usz: ident) ?,
        [$($w: tt) *];
        $a0: tt,
        $b0: tt,
        $c0: tt,
        $d0: tt,
        $e0: tt,
        $f0: tt,
        $g0: tt,
        $h0: tt $(, $($t: tt) *) ?
    ) => {
        $crate:: __bits_store_array !(
            $order,
            u8 $(@ $usz) ?,
            [$($w) *($a0, $b0, $c0, $d0, $e0, $f0, $g0, $h0)];
            $($($t) *) ?
        )
    };
    (
        $order: tt,
        u16 $(@ $usz: ident) ?,
        [$($w: tt) *];
        $a0: tt,
        $b0: tt,
        $c0: tt,
        $d0: tt,
        $e0: tt,
        $f0: tt,
        $g0: tt,
        $h0: tt,
        $a1: tt,
        $b1: tt,
        $c1: tt,
        $d1: tt,
        $e1: tt,
        $f1: tt,
        $g1: tt,
        $h1: tt $(, $($t: tt) *) ?
    ) => {
        $crate:: __bits_store_array !(
            $order,
            u16 $(@ $usz) ?,
            [$($w) *($a0, $b0, $c0, $d0, $e0, $f0, $g0, $h0, $a1, $b1, $c1, $d1, $e1, $f1, $g1, $h1)];
            $($($t) *) ?
        )
    };
    (
        $order: tt,
        u32 $(@ $usz: ident) ?,
        [$($w: tt) *];
        $a0: tt,
        $b0: tt,
        $c0: tt,
        $d0: tt,
        $e0: tt,
        $f0: tt,
        $g0: tt,
        $h0: tt,
        $a1: tt,
        $b1: tt,
        $c1: tt,
        $d1: tt,
        $e1: tt,
        $f1: tt,
        $g1: tt,
        $h1: tt,
        $a2: tt,
        $b2: tt,
        $c2: tt,
        $d2: tt,
        $e2: tt,
        $f2: tt,
        $g2: tt,
        $h2: tt,
        $a3: tt,
        $b3: tt,
        $c3: tt,
        $d3: tt,
        $e3: tt,
        $f3: tt,
        $g3: tt,
        $h3: tt $(, $($t: tt) *) ?
    ) => {
        $crate:: __bits_store_array !(
            $order,
            u32 $(@ $usz) ?,
            [
                $(
                    $w
                ) *(
                    $a0,
                    $b0,
                    $c0,
                    $d0,
                    $e0,
                    $f0,
                    $g0,
                    $h0,
                    $a1,
                    $b1,
                    $c1,
                    $d1,
                    $e1,
                    $f1,
                    $g1,
                    $h1,
                    $a2,
                    $b2,
                    $c2,
                    $d2,
                    $e2,
                    $f2,
                    $g2,
                    $h2,
                    $a3,
                    $b3,
                    $c3,
                    $d3,
                    $e3,
                    $f3,
                    $g3,
                    $h3
                )
            ];
            $($($t) *) ?
        )
    };
    (
        $order: tt,
        u64 $(@ $usz: ident) ?,
        [$($w: tt) *];
        $a0: tt,
        $b0: tt,
        $c0: tt,
        $d0: tt,
        $e0: tt,
        $f0: tt,
        $g0: tt,
        $h0: tt,
        $a1: tt,
        $b1: tt,
        $c1: tt,
        $d1: tt,
        $e1: tt,
        $f1: tt,
        $g1: tt,
        $h1: tt,
        $a2: tt,
        $b2: tt,
        $c2: tt,
        $d2: tt,
        $e2: tt,
        $f2: tt,
        $g2: tt,
        $h2: tt,
        $a3: tt,
        $b3: tt,
        $c3: tt,
        $d3: tt,
        $e3: tt,
        $f3: tt,
        $g3: tt,
        $h3: tt,
        $a4: tt,
        $b4: tt,
        $c4: tt,
        $d4: tt,
        $e4: tt,
        $f4: tt,
        $g4: tt,
        $h4: tt,
        $a5: tt,
        $b5: tt,
        $c5: tt,
        $d5: tt,
        $e5: tt,
        $f5: tt,
        $g5: tt,
        $h5: tt,
        $a6: tt,
        $b6: tt,
        $c6: tt,
        $d6: tt,
        $e6: tt,
        $f6: tt,
        $g6: tt,
        $h6: tt,
        $a7: tt,
        $b7: tt,
        $c7: tt,
        $d7: tt,
        $e7: tt,
        $f7: tt,
        $g7: tt,
        $h7: tt $(, $($t: tt) *) ?
    ) => {
        $crate:: __bits_store_array !(
            $order,
            u64 $(@ $usz) ?,
            [
                $(
                    $w
                ) *(
                    $a0,
                    $b0,
                    $c0,
                    $d0,
                    $e0,
                    $f0,
                    $g0,
                    $h0,
                    $a1,
                    $b1,
                    $c1,
                    $d1,
                    $e1,
                    $f1,
                    $g1,
                    $h1,
                    $a2,
                    $b2,
                    $c2,
                    $d2,
                    $e2,
                    $f2,
                    $g2,
                    $h2,
                    $a3,
                    $b3,
                    $c3,
                    $d3,
                    $e3,
                    $f3,
                    $g3,
                    $h3,
                    $a4,
                    $b4,
                    $c4,
                    $d4,
                    $e4,
                    $f4,
                    $g4,
                    $h4,
                    $a5,
                    $b5,
                    $c5,
                    $d5,
                    $e5,
                    $f5,
                    $g5,
                    $h5,
                    $a6,
                    $b6,
                    $c6,
                    $d6,
                    $e6,
                    $f6,
                    $g6,
                    $h6,
                    $a7,
                    $b7,
                    $c7,
                    $d7,
                    $e7,
                    $f7,
                    $g7,
                    $h7
                )
            ];
            $($($t) *) ?
        )
    };
}

#[doc= " Counts the number of repetitions inside a `$()*` sequence."]
#[doc(hidden)]
#[macro_export]
macro_rules! __count{
    (@ $val: expr) => {
        1
    };
    ($($val: expr), *) => {
        {
            const LEN: usize = 0usize $(+ $crate:: __count !(@ $val)) *;
            LEN
        }
    };
}

#[doc= " Construct a `T` element from an array of `u8`."]
#[doc(hidden)]
#[macro_export]
macro_rules! __elt_from_bits{
    (Lsb0, $store: ident; $($a: tt, $b: tt, $c: tt, $d: tt, $e: tt, $f: tt, $g: tt, $h: tt), *) => {
        $crate:: __ty_from_bytes !(
            Lsb0,
            $store,
            [
                $(
                    $crate:: macros:: internal:: u8_from_le_bits(
                        $a != 0,
                        $b != 0,
                        $c != 0,
                        $d != 0,
                        $e != 0,
                        $f != 0,
                        $g != 0,
                        $h != 0,
                    )
                ),
                *
            ]
        )
    };
    (Msb0, $store: ident; $($a: tt, $b: tt, $c: tt, $d: tt, $e: tt, $f: tt, $g: tt, $h: tt), *) => {
        $crate:: __ty_from_bytes !(
            Msb0,
            $store,
            [
                $(
                    $crate:: macros:: internal:: u8_from_be_bits(
                        $a != 0,
                        $b != 0,
                        $c != 0,
                        $d != 0,
                        $e != 0,
                        $f != 0,
                        $g != 0,
                        $h != 0,
                    )
                ),
                *
            ]
        )
    };
    (Local, $store: ident; $($a: tt, $b: tt, $c: tt, $d: tt, $e: tt, $f: tt, $g: tt, $h: tt), *) => {
        $crate:: __ty_from_bytes !(
            Local,
            $store,
            [
                $(
                    $crate:: macros:: internal:: u8_from_ne_bits(
                        $a != 0,
                        $b != 0,
                        $c != 0,
                        $d != 0,
                        $e != 0,
                        $f != 0,
                        $g != 0,
                        $h != 0,
                    )
                ),
                *
            ]
        )
    };
    ($order: tt, $store: ident; $($a: tt), *) => {
        {
            compile_error!(
                "The ordering argument you provided is unrecognized, \
			and as such cannot be used in const contexts."
            );
            let mut value: $store = 0;
            let mut _idx = 0u8;
            $(
                $crate:: store:: BitStore:: set::< $order >(&mut value, unsafe {
                    $crate:: index:: BitIdx:: new_unchecked(_idx)
                }, $a != 0,);
                _idx += 1;
            ) * value
        }
    };
}

#[doc= " Extend a single bit to fill an element."]
#[doc(hidden)]
#[macro_export]
macro_rules! __extend_bool{
    ($val: expr, $typ: ident) => {
        (0 as $typ).wrapping_sub(($val != 0) as $typ)
    };
}

#[doc= " Implement the shift operators on `BitSlice` with other types than `usize`."]
#[doc(hidden)]
macro_rules! __bitslice_shift{
    ($($t: ty), +) => {
        $(
            #[doc(hidden)] impl < O,
            T > core:: ops:: ShlAssign < $t > for $crate:: prelude:: BitSlice < O,
            T > where O: $crate:: order:: BitOrder,
            T: $crate:: store:: BitStore {
                fn shl_assign(&mut self, shamt: $t) {
                    core::ops::ShlAssign::<usize>::shl_assign(self, shamt as usize)
                }
            }
            #[doc(hidden)] impl < O,
            T > core:: ops:: ShrAssign < $t > for $crate:: prelude:: BitSlice < O,
            T > where O: $crate:: order:: BitOrder,
            T: $crate:: store:: BitStore {
                fn shr_assign(&mut self, shamt: $t) {
                    core::ops::ShrAssign::<usize>::shr_assign(self, shamt as usize)
                }
            }
        ) +
    };
}

#[doc= " Implement the shift operators on `BitVec` with other types than `usize`."]
#[doc(hidden)]
#[cfg(feature = "alloc")]
macro_rules! __bitvec_shift{
    ($($t: ty), +) => {
        $(
            #[doc(hidden)] impl < O,
            T > core:: ops:: Shl < $t > for $crate:: vec:: BitVec < O,
            T > where O: $crate:: order:: BitOrder,
            T: $crate:: store:: BitStore {
                type Output = <Self as core::ops::Shl<usize>>::Output;
                fn shl(self, shamt: $t) -> Self:: Output {
                    core::ops::Shl::<usize>::shl(self, shamt as usize)
                }
            }
            #[doc(hidden)] impl < O,
            T > core:: ops:: ShlAssign < $t > for $crate:: vec:: BitVec < O,
            T > where O: $crate:: order:: BitOrder,
            T: $crate:: store:: BitStore {
                fn shl_assign(&mut self, shamt: $t) {
                    core::ops::ShlAssign::<usize>::shl_assign(self, shamt as usize)
                }
            }
            #[doc(hidden)] impl < O,
            T > core:: ops:: Shr < $t > for $crate:: vec:: BitVec < O,
            T > where O: $crate:: order:: BitOrder,
            T: $crate:: store:: BitStore {
                type Output = <Self as core::ops::Shr<usize>>::Output;
                fn shr(self, shamt: $t) -> Self:: Output {
                    core::ops::Shr::<usize>::shr(self, shamt as usize)
                }
            }
            #[doc(hidden)] impl < O,
            T > core:: ops:: ShrAssign < $t > for $crate:: vec:: BitVec < O,
            T > where O: $crate:: order:: BitOrder,
            T: $crate:: store:: BitStore {
                fn shr_assign(&mut self, shamt: $t) {
                    core::ops::ShrAssign::<usize>::shr_assign(self, shamt as usize)
                }
            }
        ) +
    };
}

#[doc= " Constructs a fundamental integer from a list of bytes."]
#[doc(hidden)]
#[macro_export]
macro_rules! __ty_from_bytes{
    (Msb0, u8, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u8_from_be_bytes([$($byte), *])
    };
    (Lsb0, u8, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u8_from_le_bytes([$($byte), *])
    };
    (Local, u8, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u8_from_ne_bytes([$($byte), *])
    };
    (Msb0, u16, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u16_from_be_bytes([$($byte), *])
    };
    (Lsb0, u16, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u16_from_le_bytes([$($byte), *])
    };
    (Local, u16, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u16_from_ne_bytes([$($byte), *])
    };
    (Msb0, u32, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u32_from_be_bytes([$($byte), *])
    };
    (Lsb0, u32, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u32_from_le_bytes([$($byte), *])
    };
    (Local, u32, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u32_from_ne_bytes([$($byte), *])
    };
    (Msb0, u64, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u64_from_be_bytes([$($byte), *])
    };
    (Lsb0, u64, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u64_from_le_bytes([$($byte), *])
    };
    (Local, u64, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: u64_from_ne_bytes([$($byte), *])
    };
    (Msb0, usize, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: usize_from_be_bytes([$($byte), *])
    };
    (Lsb0, usize, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: usize_from_le_bytes([$($byte), *])
    };
    (Local, usize, [$($byte: expr), *]) => {
        $crate:: macros:: internal:: usize_from_ne_bytes([$($byte), *])
    };
}

#[doc= " Construct a `u8` from bits applied in Lsb0-order."]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::too_many_arguments)]
pub(crate) const fn u8_from_le_bits(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool, h: bool) -> u8 {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc= " Construct a `u8` from bits applied in Msb0-order."]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::too_many_arguments)]
pub(crate) const fn u8_from_be_bits(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool, h: bool) -> u8 {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use self::u8_from_be_bits as u8_from_ne_bits;

#[doc(hidden)]
pub(crate) const fn u8_from_be_bytes(bytes: [u8; 1]) -> u8 {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
pub(crate) const fn u8_from_le_bytes(bytes: [u8; 1]) -> u8 {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
pub(crate) const fn u16_from_be_bytes(bytes: [u8; 2]) -> u16 {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
pub(crate) const fn u16_from_le_bytes(bytes: [u8; 2]) -> u16 {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
pub(crate) const fn u32_from_be_bytes(bytes: [u8; 4]) -> u32 {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
pub(crate) const fn u32_from_le_bytes(bytes: [u8; 4]) -> u32 {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
pub(crate) const fn u64_from_be_bytes(bytes: [u8; 8]) -> u64 {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
pub(crate) const fn u64_from_le_bytes(bytes: [u8; 8]) -> u64 {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
#[cfg(target_pointer_width = "32")]
pub(crate) const fn usize_from_be_bytes(bytes: [u8; 4]) -> usize {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
#[cfg(target_pointer_width = "32")]
pub(crate) const fn usize_from_le_bytes(bytes: [u8; 4]) -> usize {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
#[cfg(target_pointer_width = "64")]
pub(crate) const fn usize_from_be_bytes(bytes: [u8; 8]) -> usize {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
#[cfg(target_pointer_width = "64")]
pub(crate) const fn usize_from_le_bytes(bytes: [u8; 8]) -> usize {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use u8_from_be_bytes as u8_from_ne_bytes;
#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use u16_from_be_bytes as u16_from_ne_bytes;
#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use u32_from_be_bytes as u32_from_ne_bytes;
#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use u64_from_be_bytes as u64_from_ne_bytes;
#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use usize_from_be_bytes as usize_from_ne_bytes;
