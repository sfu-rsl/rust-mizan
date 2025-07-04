#[cfg(feature = "alloc")]
extern crate sample_10013_function as bitvec;

use bitvec::{
	BitBox,
	BitVec,
};


#[cfg(feature = "alloc")]
#[test]
fn preserve_vec_vuln_fn() {
	let len = 8065; // 8065 fails, 8064 passes
	let mut bv = BitVec::with_capacity(len);
	bv.push(false);
	let _: BitBox = bv.into();
}
