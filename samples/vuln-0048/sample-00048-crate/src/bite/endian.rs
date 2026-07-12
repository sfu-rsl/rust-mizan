use {
    std,
    std::fmt::Debug,
    std::hash::Hash,
};



#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)] pub enum BigEndian { }
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)] pub enum LittleEndian { }

pub type NetworkEndian = BigEndian;

#[cfg(target_endian = "big")]    pub type NativeEndian = BigEndian;
#[cfg(target_endian = "little")] pub type NativeEndian = LittleEndian;



// a trait that defines basic io operations in some endian encoding.
//
// convenience extension traits exist for reading and writing bytes for `std::io::read` and `std::io::write` streams.
pub trait Endianness: Clone + Copy + Debug + Eq + Hash + Ord + PartialEq + PartialOrd {
    fn read_u8           (stream: &[u8])               -> u8;
    fn read_u16          (stream: &[u8])               -> u16;
    fn read_u32          (stream: &[u8])               -> u32;
    fn read_u64          (stream: &[u8])               -> u64;
    fn read_u128         (stream: &[u8])               -> u128;
    fn read_uint         (stream: &[u8], bytes: usize) -> u64;
    fn read_uint128      (stream: &[u8], bytes: usize) -> u128;

    fn write_u8          (stream: &mut [u8], value: u8);
    fn write_u16         (stream: &mut [u8], value: u16);
    fn write_u32         (stream: &mut [u8], value: u32);
    fn write_u64         (stream: &mut [u8], value: u64);
    fn write_u128        (stream: &mut [u8], value: u128);
    fn write_uint        (stream: &mut [u8], value: u64, bytes: usize);
    fn write_uint128     (stream: &mut [u8], value: u128, bytes: usize);

    fn convert_slice_u16 (values: &mut [u16]);
    fn convert_slice_u32 (values: &mut [u32]);
    fn convert_slice_u64 (values: &mut [u64]);
    fn convert_slice_u128(values: &mut [u128]);
    fn convert_slice_f32 (values: &mut [f32]);
    fn convert_slice_f64 (values: &mut [f64]);

    fn read_slice_u16    (stream: &[u8], values: &mut [u16]);
    fn read_slice_u32    (stream: &[u8], values: &mut [u32]);
    fn read_slice_u64    (stream: &[u8], values: &mut [u64]);
    fn read_slice_u128   (stream: &[u8], values: &mut [u128]);

    fn write_slice_u16   (stream: &mut [u8], values: &[u16]);
    fn write_slice_u32   (stream: &mut [u8], values: &[u32]);
    fn write_slice_u64   (stream: &mut [u8], values: &[u64]);
    fn write_slice_u128  (stream: &mut [u8], values: &[u128]);



    #[inline] fn read_u24          (stream: &[u8])               -> u32           { Self::read_uint(stream, 3) as u32                                       }
    #[inline] fn read_i24          (stream: &[u8])               -> i32           { Self::read_uint(stream, 3) as i32                                       }

    #[inline] fn read_i8           (stream: &[u8])               -> i8            { Self::read_u8  (stream) as i8                                           }
    #[inline] fn read_i16          (stream: &[u8])               -> i16           { Self::read_u16 (stream) as i16                                          }
    #[inline] fn read_i32          (stream: &[u8])               -> i32           { Self::read_u32 (stream) as i32                                          }
    #[inline] fn read_i64          (stream: &[u8])               -> i64           { Self::read_u64 (stream) as i64                                          }
    #[inline] fn read_i128         (stream: &[u8])               -> i128          { Self::read_u128(stream) as i128                                         }
    #[inline] fn read_int          (stream: &[u8], bytes: usize) -> i64           { extend_sign   (Self::read_uint   (stream, bytes), bytes)                }
    #[inline] fn read_int128       (stream: &[u8], bytes: usize) -> i128          { extend_sign128(Self::read_uint128(stream, bytes), bytes)                }
    #[inline] fn read_f32          (stream: &[u8])               -> f32           { unsafe { std::mem::transmute(Self::read_u32(stream)) }                  }
    #[inline] fn read_f64          (stream: &[u8])               -> f64           { unsafe { std::mem::transmute(Self::read_u64(stream)) }                  }

    #[inline] fn write_u24         (stream: &mut [u8], value: u32)                { Self::write_uint   (stream, value as u64, 3)                            }
    #[inline] fn write_i24         (stream: &mut [u8], value: i32)                { Self::write_int    (stream, value as i64, 3)                            }

    #[inline] fn write_i16         (stream: &mut [u8], value: i16)                { Self::write_u16    (stream, value as u16)                               }
    #[inline] fn write_i32         (stream: &mut [u8], value: i32)                { Self::write_u32    (stream, value as u32)                               }
    #[inline] fn write_i64         (stream: &mut [u8], value: i64)                { Self::write_u64    (stream, value as u64)                               }
    #[inline] fn write_i128        (stream: &mut [u8], value: i128)               { Self::write_u128   (stream, value as u128)                              }
    #[inline] fn write_int         (stream: &mut [u8], value: i64,  bytes: usize) { Self::write_uint   (stream, unextend_sign(value, bytes), bytes)         }
    #[inline] fn write_int128      (stream: &mut [u8], value: i128, bytes: usize) { Self::write_uint128(stream, unextend_sign128(value, bytes), bytes)      }
    #[inline] fn write_f32         (stream: &mut [u8], value: f32)                { Self::write_u32    (stream, unsafe { std::mem::transmute(value) })      }
    #[inline] fn write_f64         (stream: &mut [u8], value: f64)                { Self::write_u64    (stream, unsafe { std::mem::transmute(value) })      }

    #[inline] fn read_slice_i16    (stream: &[u8], values: &mut [i16])            { Self::read_slice_u16 (stream, unsafe { std::mem::transmute(values) });  }
    #[inline] fn read_slice_i32    (stream: &[u8], values: &mut [i32])            { Self::read_slice_u32 (stream, unsafe { std::mem::transmute(values) });  }
    #[inline] fn read_slice_i64    (stream: &[u8], values: &mut [i64])            { Self::read_slice_u64 (stream, unsafe { std::mem::transmute(values) });  }
    #[inline] fn read_slice_i128   (stream: &[u8], values: &mut [i128])           { Self::read_slice_u128(stream, unsafe { std::mem::transmute(values) });  }

    #[inline] fn read_slice_f32_unchecked(stream: &[u8], values: &mut [f32])      { Self::read_slice_u32(stream, unsafe { std::mem::transmute(values) });   }
    #[inline] fn read_slice_f64_unchecked(stream: &[u8], values: &mut [f64])      { Self::read_slice_u64(stream, unsafe { std::mem::transmute(values) });   }

    #[inline] fn write_slice_i16   (stream: &mut [u8], values: &[i16])            { Self::write_slice_u16 (stream, unsafe { std::mem::transmute(values) }); }
    #[inline] fn write_slice_i32   (stream: &mut [u8], values: &[i32])            { Self::write_slice_u32 (stream, unsafe { std::mem::transmute(values) }); }
    #[inline] fn write_slice_i64   (stream: &mut [u8], values: &[i64])            { Self::write_slice_u64 (stream, unsafe { std::mem::transmute(values) }); }
    #[inline] fn write_slice_i128  (stream: &mut [u8], values: &[i128])           { Self::write_slice_u128(stream, unsafe { std::mem::transmute(values) }); }

    #[inline] fn write_slice_f32   (stream: &mut [u8], values: &[f32])            { Self::write_slice_u32(stream, unsafe { std::mem::transmute(values) });  }
    #[inline] fn write_slice_f64   (stream: &mut [u8], values: &[f64])            { Self::write_slice_u64(stream, unsafe { std::mem::transmute(values) });  }

    #[inline] fn convert_slice_i16 (values: &mut [i16])                           { Self::convert_slice_u16 (unsafe { std::mem::transmute(values) });       }
    #[inline] fn convert_slice_i32 (values: &mut [i32])                           { Self::convert_slice_u32 (unsafe { std::mem::transmute(values) });       }
    #[inline] fn convert_slice_i64 (values: &mut [i64])                           { Self::convert_slice_u64 (unsafe { std::mem::transmute(values) });       }
    #[inline] fn convert_slice_i128(values: &mut [i128])                          { Self::convert_slice_u128(unsafe { std::mem::transmute(values) });       }
}



#[inline]
fn extend_sign(value: u64, bytes: usize) -> i64 {
    let shift = (8 - bytes) * 8;
    (value << shift) as i64 >> shift
}

#[inline]
fn extend_sign128(value: u128, bytes: usize) -> i128 {
    let shift = (16 - bytes) * 8;
    (value << shift) as i128 >> shift
}

#[inline]
fn unextend_sign(value: i64, bytes: usize) -> u64 {
    let shift = (8 - bytes) * 8;
    (value << shift) as u64 >> shift
}

#[inline]
fn unextend_sign128(value: i128, bytes: usize) -> u128 {
    let shift = (16 - bytes) * 8;
    (value << shift) as u128 >> shift
}
