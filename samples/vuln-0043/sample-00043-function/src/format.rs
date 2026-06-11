
use super::{
    header::{CocoonHeader, CocoonVersion, MiniCocoonHeader},
};

const HEADER_SIZE: usize = CocoonHeader::SIZE;
const TAG_SIZE: usize = 16;
const MAX_SIZE: usize = HEADER_SIZE + TAG_SIZE;

const MINI_HEADER_SIZE: usize = MiniCocoonHeader::SIZE;
const MINI_SIZE: usize = MINI_HEADER_SIZE + TAG_SIZE;

pub struct FormatPrefix {
    header: CocoonHeader,
    raw: [u8; MAX_SIZE],
}

impl FormatPrefix {
    pub const SERIALIZE_SIZE: usize = MAX_SIZE;

    pub fn new(header: CocoonHeader) -> Self {
        let mut raw = [0u8; MAX_SIZE];

        match header.version() {
            CocoonVersion::Version1 => {
                header.serialize_into(&mut raw);
            }
        };

        FormatPrefix { header, raw }
    }

    pub fn serialize(mut self, tag: &[u8; TAG_SIZE]) -> [u8; Self::SERIALIZE_SIZE] {
        match self.header().version() {
            CocoonVersion::Version1 => (),
            // _ => panic!("Prefix can be serialized into the latest version only!"),
        }

        self.raw[HEADER_SIZE..HEADER_SIZE + TAG_SIZE].copy_from_slice(tag);
        self.raw
    }

    pub fn header(&self) -> &CocoonHeader {
        &self.header
    }

    pub fn prefix(&self) -> &[u8] {
        &self.raw[..HEADER_SIZE]
    }
    
}

pub struct MiniFormatPrefix {
    header: MiniCocoonHeader,
    raw: [u8; MINI_SIZE],
}

impl MiniFormatPrefix {
    pub const SERIALIZE_SIZE: usize = MINI_SIZE;

    pub fn new(header: MiniCocoonHeader) -> Self {
        let mut raw = [0u8; MINI_SIZE];

        header.serialize_into(&mut raw);

        MiniFormatPrefix { header, raw }
    }

    pub fn serialize(mut self, tag: &[u8; TAG_SIZE]) -> [u8; Self::SERIALIZE_SIZE] {
        self.raw[MINI_HEADER_SIZE..MINI_HEADER_SIZE + TAG_SIZE].copy_from_slice(tag);
        self.raw
    }

    pub fn prefix(&self) -> &[u8] {
        &self.raw[..MINI_HEADER_SIZE]
    }

}
