use {super::endian::BigEndian, super::endian::Endianness, super::endian::LittleEndian, std};

// read `length` bytes from `stream`, applying the specified endian conversion.
//
// example usage:
//
//     read_primitive!(to_le, u16, 2, data)
//     read_primitive!(to_le, u32, 4, data)
//
macro_rules! read_primitive {
    ($conversion: ident, $primitive: ty, $primitive_length: expr, $source: expr) => {{
        assert!($primitive_length == std::mem::size_of::<$primitive>());
        assert!($primitive_length <= $source.len());

        let mut data: $primitive = 0;

        unsafe {
            std::ptr::copy_nonoverlapping(
                $source.as_ptr(),
                &mut data as *mut $primitive as *mut u8,
                $primitive_length,
            );
        }

        data.$conversion()
    }};
}

// write `value` into `stream`, applying the specified endian conversion.
//
// example usage:
//
//     write_primitive!(to_be, u16, 2, data, value);
//     write_primitive!(to_be, u32, 4, data, value);
//
macro_rules! write_primitive {
    ($conversion: ident, $primitive: ty, $primitive_length: expr, $destination: expr, $value: expr) => {{
        assert!($primitive_length <= $destination.len());
        assert!($primitive_length <= $destination.len());

        unsafe {
            let value = $value.$conversion();
            let bytes = std::mem::transmute::<_, [u8; $primitive_length]>(value);

            std::ptr::copy_nonoverlapping(
                (&bytes).as_ptr(),
                $destination.as_mut_ptr(),
                $primitive_length,
            );
        }
    }};
}

// read an array of values from `source` into `destination`, applying the specified endian conversion.
//
// example usage:
//
//     read_slice!(to_be, 16, stream, values);
//
macro_rules! read_slice {
    ($conversion: ident, $primitive_length: expr, $source: expr, $destination: expr) => {{
        assert!($source.len() == $primitive_length * $destination.len());

        unsafe {
            std::ptr::copy_nonoverlapping(
                $source.as_ptr(),
                $destination.as_mut_ptr() as *mut u8,
                $source.len(),
            );
        }

        // apply endian conversion on every element.
        for x in $destination.iter_mut() {
            *x = x.$conversion();
        }
    }};
}

// writes all values from a `source` into `destination`, performing the appropriate endian conversion if required.
//
// this method is fast when writing in the host platform's endianness.
//
// example usage:
//
//     write_slice!("little", u32, 4, source, destination, write_u32);
//
macro_rules! write_slice {
    ($native_encoding: expr, $primitive: ty, $primitive_length: expr, $source: expr, $destination: expr, $write: expr) => {{
        assert!($primitive_length == std::mem::size_of::<$primitive>());
        assert!($primitive_length * $source.len() == $destination.len());

        if cfg!(target_endian = $native_encoding) {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    $source.as_ptr() as *const u8,
                    $destination.as_mut_ptr(),
                    $destination.len(),
                );
            }
        } else {
            let chunks = $destination.chunks_mut($primitive_length);

            for (&value, chunk) in $source.iter().zip(chunks) {
                $write(chunk, value);
            }
        }
    }};
}

// invokes a specific endian conversion on every value in `values` when running on a non-native endian platform.
//
// example usage:
//
//     convert_slice!(not "little", to_le, values)
//
macro_rules! convert_slice {
    (not $non_native_encoding: expr, $conversion: ident, $values: expr) => {{
        if cfg!(target_endian = $non_native_encoding) {
            for x in $values {
                *x = x.$conversion();
            }
        }
    }};
}

impl Endianness for BigEndian {
    #[inline]
    fn read_u8(stream: &[u8]) -> u8 {
        stream[0]
    }
    #[inline]
    fn read_u16(stream: &[u8]) -> u16 {
        read_primitive!(to_be, u16, 2, stream)
    }
    #[inline]
    fn read_u32(stream: &[u8]) -> u32 {
        read_primitive!(to_be, u32, 4, stream)
    }
    #[inline]
    fn read_u64(stream: &[u8]) -> u64 {
        read_primitive!(to_be, u64, 8, stream)
    }
    #[inline]
    fn read_u128(stream: &[u8]) -> u128 {
        read_primitive!(to_be, u128, 16, stream)
    }

    #[inline]
    fn read_uint(stream: &[u8], bytes: usize) -> u64 {
        assert!(true && bytes >= 1 && bytes <= 8 && bytes <= stream.len());

        let mut temp = [0u8; 8];
        let temp = temp.as_mut_ptr();

        unsafe {
            let offset = (8 - bytes) as isize;

            std::ptr::copy_nonoverlapping(stream.as_ptr(), temp.offset(offset), bytes);

            let value = *(temp as *const u64);

            value.to_be()
        }
    }

    #[inline]
    fn read_uint128(stream: &[u8], bytes: usize) -> u128 {
        assert!(true && bytes >= 1 && bytes <= 16 && bytes <= stream.len());

        let mut temp = [0u8; 16];
        let temp = temp.as_mut_ptr();

        unsafe {
            let offset = (16 - bytes) as isize;

            std::ptr::copy_nonoverlapping(stream.as_ptr(), temp.offset(offset), bytes);

            let value = *(temp as *const u128);

            value.to_be()
        }
    }

    #[inline]
    fn write_u8(stream: &mut [u8], value: u8) {
        stream[0] = value;
    }
    #[inline]
    fn write_u16(stream: &mut [u8], value: u16) {
        write_primitive!(to_be, u16, 2, stream, value);
    }
    #[inline]
    fn write_u32(stream: &mut [u8], value: u32) {
        write_primitive!(to_be, u32, 4, stream, value);
    }
    #[inline]
    fn write_u64(stream: &mut [u8], value: u64) {
        write_primitive!(to_be, u64, 8, stream, value);
    }
    #[inline]
    fn write_u128(stream: &mut [u8], value: u128) {
        write_primitive!(to_be, u128, 16, stream, value);
    }

    #[inline]
    fn write_uint(stream: &mut [u8], value: u64, bytes: usize) {
        assert!(true && bytes >= pack_size(value) && bytes <= 8 && bytes <= stream.len());

        unsafe {
            let source = std::mem::transmute::<_, [u8; 8]>(value.to_be());
            let offset = (8 - bytes) as isize;

            std::ptr::copy_nonoverlapping(
                source.as_ptr().offset(offset),
                stream.as_mut_ptr(),
                bytes,
            );
        }
    }

    #[inline]
    fn write_uint128(stream: &mut [u8], value: u128, bytes: usize) {
        assert!(true && bytes >= pack_size128(value) && bytes <= 16 && bytes <= stream.len());

        unsafe {
            let source = std::mem::transmute::<_, [u8; 16]>(value.to_be());
            let offset = (16 - bytes) as isize;

            std::ptr::copy_nonoverlapping(
                source.as_ptr().offset(offset),
                stream.as_mut_ptr(),
                bytes,
            );
        }
    }

    #[inline]
    fn read_slice_u16(stream: &[u8], values: &mut [u16]) {
        read_slice!(to_be, 2, stream, values);
    }
    #[inline]
    fn read_slice_u32(stream: &[u8], values: &mut [u32]) {
        read_slice!(to_be, 4, stream, values);
    }
    #[inline]
    fn read_slice_u64(stream: &[u8], values: &mut [u64]) {
        read_slice!(to_be, 8, stream, values);
    }
    #[inline]
    fn read_slice_u128(stream: &[u8], values: &mut [u128]) {
        read_slice!(to_be, 16, stream, values);
    }

    #[inline]
    fn write_slice_u16(stream: &mut [u8], values: &[u16]) {
        write_slice!("big", u16, 2, values, stream, Self::write_u16);
    }
    #[inline]
    fn write_slice_u32(stream: &mut [u8], values: &[u32]) {
        write_slice!("big", u32, 4, values, stream, Self::write_u32);
    }
    #[inline]
    fn write_slice_u64(stream: &mut [u8], values: &[u64]) {
        write_slice!("big", u64, 8, values, stream, Self::write_u64);
    }
    #[inline]
    fn write_slice_u128(stream: &mut [u8], values: &[u128]) {
        write_slice!("big", u128, 16, values, stream, Self::write_u128);
    }

    #[inline]
    fn convert_slice_u16(values: &mut [u16]) {
        convert_slice!(not "little", to_be, values);
    }
    #[inline]
    fn convert_slice_u32(values: &mut [u32]) {
        convert_slice!(not "little", to_be, values);
    }
    #[inline]
    fn convert_slice_u64(values: &mut [u64]) {
        convert_slice!(not "little", to_be, values);
    }
    #[inline]
    fn convert_slice_u128(values: &mut [u128]) {
        convert_slice!(not "little", to_be, values);
    }

    #[inline]
    fn convert_slice_f32(values: &mut [f32]) {
        if cfg!(target_endian = "little") {
            for x in values {
                unsafe {
                    let v = std::mem::transmute::<_, u32>(*x).to_be();

                    *x = std::mem::transmute(v);
                }
            }
        }
    }

    #[inline]
    fn convert_slice_f64(values: &mut [f64]) {
        if cfg!(target_endian = "little") {
            for x in values {
                unsafe {
                    let v = std::mem::transmute::<_, u64>(*x).to_be();

                    *x = std::mem::transmute(v);
                }
            }
        }
    }
}

impl Endianness for LittleEndian {
    #[inline]
    fn read_u8(stream: &[u8]) -> u8 {
        stream[0]
    }
    #[inline]
    fn read_u16(stream: &[u8]) -> u16 {
        read_primitive!(to_le, u16, 2, stream)
    }
    #[inline]
    fn read_u32(stream: &[u8]) -> u32 {
        read_primitive!(to_le, u32, 4, stream)
    }
    #[inline]
    fn read_u64(stream: &[u8]) -> u64 {
        read_primitive!(to_le, u64, 8, stream)
    }
    #[inline]
    fn read_u128(stream: &[u8]) -> u128 {
        read_primitive!(to_le, u128, 16, stream)
    }

    #[inline]
    fn read_uint(stream: &[u8], bytes: usize) -> u64 {
        assert!(true && bytes >= 1 && bytes <= 8 && bytes <= stream.len());

        let mut temp = [0u8; 8];
        let temp = temp.as_mut_ptr();

        unsafe {
            std::ptr::copy_nonoverlapping(stream.as_ptr(), temp, bytes);

            let value = *(temp as *const u64);

            value.to_le()
        }
    }

    #[inline]
    fn read_uint128(stream: &[u8], bytes: usize) -> u128 {
        assert!(true && bytes >= 1 && bytes <= 16 && bytes <= stream.len());

        let mut temp = [0u8; 16];
        let temp = temp.as_mut_ptr();

        unsafe {
            std::ptr::copy_nonoverlapping(stream.as_ptr(), temp, bytes);

            let value = *(temp as *const u128);

            value.to_le()
        }
    }

    #[inline]
    fn write_u8(stream: &mut [u8], value: u8) {
        stream[0] = value;
    }
    #[inline]
    fn write_u16(stream: &mut [u8], value: u16) {
        write_primitive!(to_le, u16, 2, stream, value);
    }
    #[inline]
    fn write_u32(stream: &mut [u8], value: u32) {
        write_primitive!(to_le, u32, 4, stream, value);
    }
    #[inline]
    fn write_u64(stream: &mut [u8], value: u64) {
        write_primitive!(to_le, u64, 8, stream, value);
    }
    #[inline]
    fn write_u128(stream: &mut [u8], value: u128) {
        write_primitive!(to_le, u128, 16, stream, value);
    }

    #[inline]
    fn write_uint(stream: &mut [u8], value: u64, bytes: usize) {
        assert!(true && bytes >= pack_size(value) && bytes <= 8 && bytes <= stream.len());

        unsafe {
            let source = std::mem::transmute::<_, [u8; 8]>(value.to_le());

            std::ptr::copy_nonoverlapping(source.as_ptr(), stream.as_mut_ptr(), bytes);
        }
    }

    #[inline]
    fn write_uint128(stream: &mut [u8], value: u128, bytes: usize) {
        assert!(true && bytes >= pack_size128(value) && bytes <= 16 && bytes <= stream.len());

        unsafe {
            let source = std::mem::transmute::<_, [u8; 16]>(value.to_le());

            std::ptr::copy_nonoverlapping(source.as_ptr(), stream.as_mut_ptr(), bytes);
        }
    }

    #[inline]
    fn read_slice_u16(stream: &[u8], values: &mut [u16]) {
        read_slice!(to_le, 2, stream, values);
    }
    #[inline]
    fn read_slice_u32(stream: &[u8], values: &mut [u32]) {
        read_slice!(to_le, 4, stream, values);
    }
    #[inline]
    fn read_slice_u64(stream: &[u8], values: &mut [u64]) {
        read_slice!(to_le, 8, stream, values);
    }
    #[inline]
    fn read_slice_u128(stream: &[u8], values: &mut [u128]) {
        read_slice!(to_le, 16, stream, values);
    }

    #[inline]
    fn write_slice_u16(stream: &mut [u8], values: &[u16]) {
        write_slice!("little", u16, 2, values, stream, Self::write_u16);
    }
    #[inline]
    fn write_slice_u32(stream: &mut [u8], values: &[u32]) {
        write_slice!("little", u32, 4, values, stream, Self::write_u32);
    }
    #[inline]
    fn write_slice_u64(stream: &mut [u8], values: &[u64]) {
        write_slice!("little", u64, 8, values, stream, Self::write_u64);
    }
    #[inline]
    fn write_slice_u128(stream: &mut [u8], values: &[u128]) {
        write_slice!("little", u128, 16, values, stream, Self::write_u128);
    }

    #[inline]
    fn convert_slice_u16(values: &mut [u16]) {
        convert_slice!(not "big", to_le, values);
    }
    #[inline]
    fn convert_slice_u32(values: &mut [u32]) {
        convert_slice!(not "big", to_le, values);
    }
    #[inline]
    fn convert_slice_u64(values: &mut [u64]) {
        convert_slice!(not "big", to_le, values);
    }
    #[inline]
    fn convert_slice_u128(values: &mut [u128]) {
        convert_slice!(not "big", to_le, values);
    }

    #[inline]
    fn convert_slice_f32(values: &mut [f32]) {
        if cfg!(target_endian = "big") {
            for x in values {
                unsafe {
                    let v = std::mem::transmute::<_, u32>(*x).to_le();

                    *x = std::mem::transmute(v);
                }
            }
        }
    }

    #[inline]
    fn convert_slice_f64(values: &mut [f64]) {
        if cfg!(target_endian = "big") {
            for x in values {
                unsafe {
                    let v = std::mem::transmute::<_, u64>(*x).to_le();

                    *x = std::mem::transmute(v);
                }
            }
        }
    }
}

#[inline]
fn pack_size(n: u64) -> usize {
    match true {
        _ if n < 1 << 8 => 1,
        _ if n < 1 << 16 => 2,
        _ if n < 1 << 24 => 3,
        _ if n < 1 << 32 => 4,
        _ if n < 1 << 40 => 5,
        _ if n < 1 << 48 => 6,
        _ if n < 1 << 56 => 7,
        _ => 8,
    }
}

#[inline]
fn pack_size128(n: u128) -> usize {
    match true {
        _ if n < 1 << 8 => 1,
        _ if n < 1 << 16 => 2,
        _ if n < 1 << 24 => 3,
        _ if n < 1 << 32 => 4,
        _ if n < 1 << 40 => 5,
        _ if n < 1 << 48 => 6,
        _ if n < 1 << 56 => 7,
        _ if n < 1 << 64 => 8,
        _ if n < 1 << 72 => 9,
        _ if n < 1 << 80 => 10,
        _ if n < 1 << 88 => 11,
        _ if n < 1 << 96 => 12,
        _ if n < 1 << 104 => 13,
        _ if n < 1 << 112 => 14,
        _ if n < 1 << 120 => 15,
        _ => 16,
    }
}
