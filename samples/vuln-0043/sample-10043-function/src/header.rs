use core::convert::{TryFrom, TryInto};

use super::error::Error;

macro_rules! match_enum {
    ($m:expr, $($variant:expr),+) => {
        match $m {
            $(v if v == $variant as u8 => $variant),+,
            _ => return Err(Error::UnrecognizedFormat),
        }
    };
}

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum CocoonCipher {
    Chacha20Poly1305 = 1,
    Aes256Gcm,
}

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum CocoonKdf {
    Pbkdf2 = 1,
}

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
enum CocoonKdfVariant {
    Strong = 1,
    Weak,
}

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum CocoonVersion {
    Version1 = 1,
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct CocoonConfig {
    cipher: CocoonCipher,
    kdf: CocoonKdf,
    kdf_variant: CocoonKdfVariant,
    _reserved: u8,
}

impl Default for CocoonConfig {
    fn default() -> CocoonConfig {
        CocoonConfig {
            cipher: CocoonCipher::Chacha20Poly1305,
            kdf: CocoonKdf::Pbkdf2,
            kdf_variant: CocoonKdfVariant::Strong,
            _reserved: Default::default(),
        }
    }
}

impl CocoonConfig {
    pub fn cipher(&self) -> CocoonCipher {
        self.cipher
    }
    pub fn kdf(&self) -> CocoonKdf {
        self.kdf
    }

    pub fn kdf_iterations(&self) -> u32 {
        match self.kdf {
            CocoonKdf::Pbkdf2 => match self.kdf_variant {
                CocoonKdfVariant::Weak => 10_000,
                CocoonKdfVariant::Strong => 100_000,
            },
        }
    }
    fn serialize(&self) -> [u8; 4] {
        let mut buf = [0u8; 4];
        buf[0] = self.cipher as u8;
        buf[1] = self.kdf as u8;
        buf[2] = self.kdf_variant as u8;
        buf[3] = Default::default();
        buf
    }
}

pub struct CocoonHeader {
    magic: [u8; 3],
    version: CocoonVersion,
    config: CocoonConfig,
    salt: [u8; 16],
    nonce: [u8; 12],
    length: usize,
}

impl CocoonHeader {
    const MAGIC: [u8; 3] = [0x7f, 0xc0, b'\n'];

    pub const SIZE: usize = 44;

    pub fn new(config: CocoonConfig, salt: [u8; 16], nonce: [u8; 12], length: usize) -> Self {
        CocoonHeader {
            magic: CocoonHeader::MAGIC,
            version: CocoonVersion::Version1,
            config,
            salt,
            nonce,
            length,
        }
    }
    pub fn version(&self) -> CocoonVersion {
        self.version
    }
    pub fn serialize_into(&self, buf: &mut [u8]) {
        debug_assert!(buf.len() >= Self::SIZE);
        let length = u64::try_from(self.length).expect("Data too large");

        buf[..3].copy_from_slice(&self.magic);
        buf[3] = self.version as u8;
        buf[4..8].copy_from_slice(&self.config.serialize());
        buf[8..24].copy_from_slice(&self.salt);
        buf[24..36].copy_from_slice(&self.nonce);
        buf[36..Self::SIZE].copy_from_slice(&length.to_be_bytes());
    }
}

pub struct MiniCocoonHeader {
    nonce: [u8; 12],
    length: usize,
}

impl MiniCocoonHeader {
    pub const SIZE: usize = 20;

    pub fn new(nonce: [u8; 12], length: usize) -> Self {
        MiniCocoonHeader { nonce, length }
    }
    pub fn serialize_into(&self, buf: &mut [u8]) {
        debug_assert!(buf.len() >= Self::SIZE);
        let length = u64::try_from(self.length).expect("Data too large");

        buf[..12].copy_from_slice(&self.nonce);
        buf[12..Self::SIZE].copy_from_slice(&length.to_be_bytes());
    }
}
