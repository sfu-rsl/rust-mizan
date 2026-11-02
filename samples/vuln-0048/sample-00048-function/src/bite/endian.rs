use {std, std::fmt::Debug, std::hash::Hash};

// a trait that defines basic io operations in some endian encoding.
//
// convenience extension traits exist for reading and writing bytes for `std::io::read` and `std::io::write` streams.
pub trait Endianness: Clone + Copy + Debug + Eq + Hash + Ord + PartialEq + PartialOrd {
    fn read_u32(stream: &[u8]) -> u32;
}
