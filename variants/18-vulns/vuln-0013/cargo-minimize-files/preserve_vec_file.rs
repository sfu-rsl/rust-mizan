#[cfg(feature = "alloc")]
extern crate vuln_0013_fixed_file as bitvec;

#[cfg(test)]
use bitvec::prelude::*;

#[cfg(feature = "alloc")]
#[test]
fn preserve_vec_file() {
	{
		let len = 8065; // 8065 fails, 8064 passes
		let mut bv = BitVec::with_capacity(len);

		bv.push(false);

		let _: BitBox = bv.into();
	}

	let mut bv: BitVec<Msb0, u8> = BitVec::with_capacity(0);
	bv.push(false);
	let mut bv: BitVec<Msb0, u8> = BitVec::with_capacity(2);
	bv.push(false);
	bv.push(true);
	bv.push(true);
	let mut bv: BitVec<Local, u8> = BitVec::with_capacity(3);
	bv.push(false);
	let bv = BitVec::<Msb0, u8>::from_element(3);
	let mut bv = BitVec::<Local, u8>::repeat(false, 1);
	let mut bv = BitVec::<Local, u8>::repeat(true, 1);
	let bv = BitVec::<Msb0, u8>::from_slice(&vec![1, 2, 4, 8]);
	let bv = BitVec::<Msb0, u8>::from_vec(vec![1, 2, 4, 8]);
	{
		let bv0 = BitVec::<Local, u8>::repeat(false, 1);
		let bs = bv0.as_bitslice();
		let bv = BitVec::from_bitslice(bs);
	}
	let bv = BitVec::<Local, u8>::repeat(false, 1);
	let bs = bv.as_bitslice();

	let mut bv = BitVec::<Local, u8>::repeat(false, 1);
	let bs = bv.as_mut_bitslice();

	let mut bv = BitVec::<Local, u8>::repeat(false, 1);
	bv.set_elements(0xA5);

	let mut bv = BitVec::<Local, u8>::repeat(false, 1);
	let bv = bv.change_order::<Lsb0>();

	let bb = bv.into_boxed_bitslice();
	let bv = BitVec::from_boxed_bitslice(bb);

	let bv = BitVec::<Local, u8>::repeat(false, 1);
	let v = bv.into_vec();

	let mut bv = BitVec::<Local, u8>::repeat(false, 1);
	bv.force_align();
	{
		let src_base = BitVec::<Msb0, u8>::from_slice(&[182]);
		let src = &src_base[1..7];
		// assert_eq!(src.len(), 6);
		let mut bv = BitVec::from_bitslice(src);
		// assert_eq!(bv.as_slice()[0], 0xB6);
		bv.force_align();
		// assert_eq!(bv.as_slice()[0], 0x6E);
	}
}