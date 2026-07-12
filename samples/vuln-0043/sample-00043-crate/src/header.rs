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

    pub fn with_cipher(mut self, cipher: CocoonCipher) -> Self {
        self.cipher = cipher;
        self
    }

    pub fn with_weak_kdf(mut self) -> Self {
        self.kdf_variant = CocoonKdfVariant::Weak;
        self
    }

    fn serialize(&self) -> [u8; 4] {
        let mut buf = [0u8; 4];
        buf[0] = self.cipher as u8;
        buf[1] = self.kdf as u8;
        buf[2] = self.kdf_variant as u8;
        buf[3] = Default::default();
        buf
    }

    fn deserialize(buf: &[u8]) -> Result<Self, Error> {
        if buf.len() < 4 {
            return Err(Error::UnrecognizedFormat);
        }

        #[rustfmt::skip]
        let cipher = match_enum!(buf[0], CocoonCipher::Chacha20Poly1305, CocoonCipher::Aes256Gcm);
        let kdf = match_enum!(buf[1], CocoonKdf::Pbkdf2);
        let kdf_variant = match_enum!(buf[2], CocoonKdfVariant::Weak, CocoonKdfVariant::Strong);

        Ok(CocoonConfig {
            cipher,
            kdf,
            kdf_variant,
            _reserved: Default::default(),
        })
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

    pub const SIZE: usize = 44; // Don't use size_of::<Self>() here because of #[repr(Rust)].

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

    pub fn config(&self) -> &CocoonConfig {
        &self.config
    }

    pub fn data_length(&self) -> usize {
        self.length
    }

    pub fn salt(&self) -> &[u8] {
        &self.salt
    }

    pub fn nonce(&self) -> &[u8] {
        &self.nonce
    }

    pub fn version(&self) -> CocoonVersion {
        self.version
    }

    #[cfg(test)]
    pub fn serialize(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        self.serialize_into(&mut buf);
        buf
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

    pub fn deserialize(buf: &[u8]) -> Result<CocoonHeader, Error> {
        if buf.len() < Self::SIZE {
            return Err(Error::UnrecognizedFormat);
        }

        let mut magic = [0u8; 3];
        magic.copy_from_slice(&buf[..3]);
        if magic != Self::MAGIC {
            return Err(Error::UnrecognizedFormat);
        }

        let version = match_enum!(buf[3], CocoonVersion::Version1);
        let config = CocoonConfig::deserialize(&buf[4..8])?;
        let mut salt = [0u8; 16];
        salt.copy_from_slice(&buf[8..24]);
        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&buf[24..36]);

        let mut length_bytes = [0u8; 8];
        length_bytes.copy_from_slice(&buf[36..Self::SIZE]);

        // Covert to usize, that may fail on 32-bit platform.
        let length: usize = u64::from_be_bytes(length_bytes)
            .try_into()
            .map_err(|_| Error::TooLarge)?;

        Ok(CocoonHeader {
            magic,
            version,
            config,
            salt,
            nonce,
            length,
        })
    }
}


pub struct MiniCocoonHeader {
    nonce: [u8; 12],
    length: usize,
}

impl MiniCocoonHeader {
    pub const SIZE: usize = 20; // Don't use size_of::<Self>() here because of #[repr(Rust)].

    pub fn new(nonce: [u8; 12], length: usize) -> Self {
        MiniCocoonHeader { nonce, length }
    }

    pub fn data_length(&self) -> usize {
        self.length
    }

    pub fn nonce(&self) -> &[u8] {
        &self.nonce
    }

    #[cfg(test)]
    pub fn serialize(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        self.serialize_into(&mut buf);
        buf
    }

    pub fn serialize_into(&self, buf: &mut [u8]) {
        debug_assert!(buf.len() >= Self::SIZE);
        let length = u64::try_from(self.length).expect("Data too large");

        buf[..12].copy_from_slice(&self.nonce);
        buf[12..Self::SIZE].copy_from_slice(&length.to_be_bytes());
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, Error> {
        if buf.len() < Self::SIZE {
            return Err(Error::UnrecognizedFormat);
        }

        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&buf[..12]);

        let mut length_bytes = [0u8; 8];
        length_bytes.copy_from_slice(&buf[12..Self::SIZE]);
        let length = u64::from_be_bytes(length_bytes)
            .try_into()
            .map_err(|_| Error::TooLarge)?;

        Ok(MiniCocoonHeader { nonce, length })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn header_config_default() {
        let config = CocoonConfig::default();
        assert_eq!(config.kdf(), CocoonKdf::Pbkdf2);
        assert_eq!(config.cipher(), CocoonCipher::Chacha20Poly1305);
        assert_eq!(config.kdf_iterations(), 100_000);
    }

    #[test]
    fn header_config_modify() {
        let config = CocoonConfig::default()
            .with_cipher(CocoonCipher::Aes256Gcm)
            .with_weak_kdf();
        assert_eq!(config.cipher(), CocoonCipher::Aes256Gcm);
        assert_eq!(config.kdf_iterations(), 10_000);
    }

    #[test]
    fn header_config_serialize() {
        let config = CocoonConfig::default();
        assert_eq!(config.serialize(), [0x01, 0x01, 0x01, 0x00]);

        let config = config.with_cipher(CocoonCipher::Aes256Gcm);
        assert_eq!(config.serialize(), [0x02, 0x01, 0x01, 0x00]);

        let config = config.with_weak_kdf();
        assert_eq!(config.serialize(), [0x02, 0x01, 0x02, 0x00]);
    }

    #[test]
    fn header_config_deserialize() {
        let config = CocoonConfig::default().serialize();

        for i in 0..3 {
            match CocoonConfig::deserialize(&config[0..i]) {
                Err(e) => match e {
                    Error::UnrecognizedFormat => (),
                    _ => panic!("Unexpected error, UnrecognizedFormat is expected only"),
                },
                _ => panic!("Success is not expected"),
            }
        }

        CocoonConfig::deserialize(&config[0..4]).expect("Deserialized config");
    }

    #[test]
    fn header_config_deserialize_with_options() {
        let config = CocoonConfig::default()
            .with_weak_kdf()
            .with_cipher(CocoonCipher::Aes256Gcm)
            .serialize();
        CocoonConfig::deserialize(&config).expect("Deserialized config");
    }

    #[test]
    fn header_new() {
        let header = CocoonHeader::new(CocoonConfig::default(), [0; 16], [0; 12], std::usize::MAX);
        assert_eq!(header.config(), &CocoonConfig::default());
        assert_eq!(header.salt(), [0; 16]);
        assert_eq!(header.nonce(), [0; 12]);
        assert_eq!(header.data_length(), std::usize::MAX);
        assert_eq!(header.version(), CocoonVersion::Version1);
    }

    #[test]
    fn header_serialize() {
        let header = CocoonHeader::new(
            CocoonConfig::default().with_cipher(CocoonCipher::Aes256Gcm),
            [1; 16],
            [2; 12],
            std::usize::MAX,
        );

        assert_eq!(
            header.serialize()[..],
            [
                0x7f, 0xc0, b'\n', 1, 2, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 255, 255, 255, 255, 255, 255, 255, 255
            ][..]
        );
    }

    #[test]
    fn header_deserialize() {
        let header = CocoonHeader::new(CocoonConfig::default(), [1; 16], [2; 12], 50);
        let header = match CocoonHeader::deserialize(&header.serialize()) {
            Ok(h) => h,
            Err(e) => panic!("Cannot deserialize serialized: {:?}", e),
        };

        assert_eq!(header.config(), &CocoonConfig::default());
        assert_eq!(header.salt(), [1; 16]);
        assert_eq!(header.nonce(), [2; 12]);
        assert_eq!(header.data_length(), 50);
        assert_eq!(header.version(), CocoonVersion::Version1);
    }

    #[test]
    fn header_deserialize_small() {
        let raw_header = [0u8; CocoonHeader::SIZE - 1];
        match CocoonHeader::deserialize(&raw_header) {
            Err(e) => match e {
                Error::UnrecognizedFormat => (),
                _ => panic!("Unexpected error, UnrecognizedFormat is expected only"),
            },
            _ => panic!("Success is not expected"),
        }
    }

    #[test]
    fn header_deserialize_modified() {
        let header = CocoonHeader::new(CocoonConfig::default(), [1; 16], [2; 12], 50);

        // Corrupt header: one byte per time.
        for i in 0..7 {
            let mut raw_header = header.serialize();
            raw_header[i] = 0xff;
            match CocoonHeader::deserialize(&raw_header) {
                Ok(_) => panic!("Header must not be deserialized on byte #{}", i),
                Err(e) => match e {
                    Error::UnrecognizedFormat => (),
                    _ => panic!("Invalid error, expected Error::UnrecognizedFormat"),
                },
            }
        }

        // Corrupt header: the reserved byte is ignored, and random data and length can be any.
        for i in 7..CocoonHeader::SIZE {
            let mut raw_header = header.serialize();
            raw_header[i] = 0xff;
            CocoonHeader::deserialize(&raw_header).expect("Header must be deserialized");
        }
    }

    #[test]
    fn mini_header_new() {
        let header = MiniCocoonHeader::new([1; 12], std::usize::MAX);
        assert_eq!(header.nonce(), [1; 12]);
        assert_eq!(header.data_length(), std::usize::MAX);
    }

    #[test]
    fn mini_header_serialize() {
        let header = MiniCocoonHeader::new([2; 12], std::usize::MAX);

        assert_eq!(
            header.serialize()[..],
            [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 255, 255, 255, 255, 255, 255, 255, 255][..]
        );
    }

    #[test]
    fn mini_header_deserialize() {
        let header = MiniCocoonHeader::new([2; 12], 50);
        let header = match MiniCocoonHeader::deserialize(&header.serialize()) {
            Ok(h) => h,
            Err(e) => panic!("Cannot deserialize serialized: {:?}", e),
        };

        assert_eq!(header.nonce(), [2; 12]);
        assert_eq!(header.data_length(), 50);
    }

    #[test]
    fn mini_header_deserialize_modified() {
        let header = MiniCocoonHeader::new([2; 12], 50);

        // Corrupt header: random data and length can be any.
        for i in 0..MiniCocoonHeader::SIZE {
            let mut raw_header = header.serialize();
            raw_header[i] = 0xff;
            MiniCocoonHeader::deserialize(&raw_header).expect("Header must be deserialized");
        }
    }

    #[test]
    fn mini_header_deserialize_short() {
        let header = [0u8; MiniCocoonHeader::SIZE];

        for i in 0..header.len() - 1 {
            match MiniCocoonHeader::deserialize(&header[0..i]) {
                Err(e) => match e {
                    Error::UnrecognizedFormat => (),
                    _ => panic!("Unexpected error, UnrecognizedFormat is expected only"),
                },
                _ => panic!("Success is not expected"),
            }
        }
    }

    #[test]
    fn mini_header_deserialize_long() {
        let header = [0u8; MiniCocoonHeader::SIZE + 1];
        MiniCocoonHeader::deserialize(&header).expect("Header must be deserialized");
    }
}
