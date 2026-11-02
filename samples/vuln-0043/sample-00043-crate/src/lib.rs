
#![forbid(unsafe_code)]
#![warn(missing_docs, unused_qualifications)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docs_rs, feature(doc_cfg))]

mod error;
mod format;
mod header;
mod kdf;
mod mini;

#[cfg(feature = "alloc")]
extern crate alloc;

use aes_gcm::{Aes256Gcm, KeyInit};
use chacha20poly1305::{
    aead::{generic_array::GenericArray, AeadInPlace},
    ChaCha20Poly1305,
};
#[cfg(feature = "std")]
use rand::rngs::ThreadRng;
use rand::{
    rngs::StdRng,
    {RngCore, SeedableRng},
};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::io::{Read, Write};

use format::FormatPrefix;
use header::{CocoonConfig, CocoonHeader};

// Enumeration is needed to avoid dynamic allocation (important for "nostd" build).
#[allow(clippy::large_enum_variant)]
enum RngVariant {
    #[cfg(feature = "std")]
    Thread(ThreadRng),
    Std(StdRng),
    None,
}

pub use error::Error;
pub use header::{CocoonCipher, CocoonKdf};

#[doc(hidden)]
pub struct Creation;

#[doc(hidden)]
pub struct Parsing;

pub const PREFIX_SIZE: usize = FormatPrefix::SERIALIZE_SIZE;

pub use mini::*;

pub struct Cocoon<'a, M> {
    password: &'a [u8],
    rng: RngVariant,
    config: CocoonConfig,
    _methods_marker: PhantomData<M>,
}

#[cfg(feature = "std")]
#[cfg_attr(docs_rs, doc(cfg(feature = "std")))]
impl<'a> Cocoon<'a, Creation> {
    
    pub fn new(password: &'a [u8]) -> Self {
        Cocoon {
            password,
            rng: RngVariant::Thread(ThreadRng::default()),
            config: CocoonConfig::default(),
            _methods_marker: PhantomData,
        }
    }
}

impl<'a> Cocoon<'a, Creation> {
   
    pub fn from_seed(password: &'a [u8], seed: [u8; 32]) -> Self {
        Cocoon {
            password,
            rng: RngVariant::Std(StdRng::from_seed(seed)),
            config: CocoonConfig::default(),
            _methods_marker: PhantomData,
        }
    }

    pub fn from_rng<R: RngCore>(password: &'a [u8], rng: R) -> Result<Self, rand::Error> {
        Ok(Cocoon {
            password,
            rng: RngVariant::Std(StdRng::from_rng(rng)?),
            config: CocoonConfig::default(),
            _methods_marker: PhantomData,
        })
    }

    #[cfg(any(feature = "getrandom", test))]
    #[cfg_attr(docs_rs, doc(cfg(feature = "getrandom")))]
    pub fn from_entropy(password: &'a [u8]) -> Self {
        Cocoon {
            password,
            rng: RngVariant::Std(StdRng::from_entropy()),
            config: CocoonConfig::default(),
            _methods_marker: PhantomData,
        }
    }
}

impl<'a> Cocoon<'a, Parsing> {
   
    pub fn parse_only(password: &'a [u8]) -> Self {
        Cocoon {
            password,
            rng: RngVariant::None,
            config: CocoonConfig::default(),
            _methods_marker: PhantomData,
        }
    }
}

// Wrapping/encryption methods are accessible only when random generator is accessible.
impl<'a> Cocoon<'a, Creation> {
    
    pub fn with_cipher(mut self, cipher: CocoonCipher) -> Self {
        self.config = self.config.with_cipher(cipher);
        self
    }

    pub fn with_weak_kdf(mut self) -> Self {
        self.config = self.config.with_weak_kdf();
        self
    }

    #[cfg(feature = "alloc")]
    #[cfg_attr(docs_rs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn wrap(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        // Allocation is needed because there is no way to prefix encrypted
        // data with a header without an allocation. It means that we need
        // to copy data at least once. It's necessary to avoid any further copying.
        let mut container = Vec::with_capacity(PREFIX_SIZE + data.len());
        container.extend_from_slice(&[0; PREFIX_SIZE]);
        container.extend_from_slice(data);

        let body = &mut container[PREFIX_SIZE..];

        // Encrypt in place and get a prefix part.
        let detached_prefix = self.encrypt(body)?;

        container[..PREFIX_SIZE].copy_from_slice(&detached_prefix);

        Ok(container)
    }

    #[cfg(feature = "std")]
    #[cfg_attr(docs_rs, doc(cfg(feature = "std")))]
    pub fn dump(&self, mut data: Vec<u8>, writer: &mut impl Write) -> Result<(), Error> {
        let detached_prefix = self.encrypt(&mut data)?;

        writer.write_all(&detached_prefix)?;
        writer.write_all(&data)?;

        Ok(())
    }

    pub fn encrypt(&self, data: &mut [u8]) -> Result<[u8; PREFIX_SIZE], Error> {
        let mut salt = [0u8; 16];
        let mut nonce = [0u8; 12];

        match &self.rng {
            #[cfg(feature = "std")]
            RngVariant::Thread(rng) => {
                let mut rng = rng.clone();
                rng.fill_bytes(&mut salt);
                rng.fill_bytes(&mut nonce);
            }
            RngVariant::Std(rng) => {
                let mut rng = rng.clone();
                rng.fill_bytes(&mut salt);
                rng.fill_bytes(&mut nonce);
            }
            RngVariant::None => unreachable!(),
        }

        let header = CocoonHeader::new(self.config.clone(), salt, nonce, data.len());
        let prefix = FormatPrefix::new(header);

        let master_key = match self.config.kdf() {
            CocoonKdf::Pbkdf2 => {
                kdf::pbkdf2::derive(&salt, self.password, self.config.kdf_iterations())
            }
        };

        let nonce = GenericArray::from_slice(&nonce);
        let master_key = GenericArray::clone_from_slice(master_key.as_ref());

        let tag: [u8; 16] = match self.config.cipher() {
            CocoonCipher::Chacha20Poly1305 => {
                let cipher = ChaCha20Poly1305::new(&master_key);
                cipher.encrypt_in_place_detached(nonce, prefix.prefix(), data)
            }
            CocoonCipher::Aes256Gcm => {
                let cipher = Aes256Gcm::new(&master_key);
                cipher.encrypt_in_place_detached(nonce, prefix.prefix(), data)
            }
        }
        .map_err(|_| Error::Cryptography)?
        .into();

        Ok(prefix.serialize(&tag))
    }
}

/// Parsing methods are always accessible. They don't need random generator in general.
impl<'a, M> Cocoon<'a, M> {
   
    #[cfg(feature = "alloc")]
    #[cfg_attr(docs_rs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn unwrap(&self, container: &[u8]) -> Result<Vec<u8>, Error> {
        let prefix = FormatPrefix::deserialize(container)?;
        let header = prefix.header();

        if container.len() < prefix.size() + header.data_length() {
            return Err(Error::TooShort);
        }

        let mut body = Vec::with_capacity(header.data_length());
        body.extend_from_slice(&container[prefix.size()..prefix.size() + body.capacity()]);

        self.decrypt_parsed(&mut body, &prefix)?;

        Ok(body)
    }

    #[cfg(feature = "std")]
    #[cfg_attr(docs_rs, doc(cfg(feature = "std")))]
    pub fn parse(&self, reader: &mut impl Read) -> Result<Vec<u8>, Error> {
        let prefix = FormatPrefix::deserialize_from(reader)?;
        let mut body = vec![0; prefix.header().data_length()];

        // Too short error can be thrown right from here.
        reader.read_exact(&mut body)?;

        self.decrypt_parsed(&mut body, &prefix)?;

        Ok(body)
    }

    pub fn decrypt(&self, data: &mut [u8], detached_prefix: &[u8]) -> Result<(), Error> {
        let prefix = FormatPrefix::deserialize(detached_prefix)?;

        self.decrypt_parsed(data, &prefix)
    }

    fn decrypt_parsed(&self, data: &mut [u8], detached_prefix: &FormatPrefix) -> Result<(), Error> {
        let mut salt = [0u8; 16];
        let mut nonce = [0u8; 12];

        let header = detached_prefix.header();

        if data.len() < header.data_length() {
            return Err(Error::TooShort);
        }

        let data = &mut data[..header.data_length()];

        salt.copy_from_slice(header.salt());
        nonce.copy_from_slice(header.nonce());

        let master_key = match header.config().kdf() {
            CocoonKdf::Pbkdf2 => {
                kdf::pbkdf2::derive(&salt, self.password, header.config().kdf_iterations())
            }
        };

        let nonce = GenericArray::from_slice(&nonce);
        let master_key = GenericArray::clone_from_slice(master_key.as_ref());
        let tag = GenericArray::from_slice(detached_prefix.tag());

        match header.config().cipher() {
            CocoonCipher::Chacha20Poly1305 => {
                let cipher = ChaCha20Poly1305::new(&master_key);
                cipher.decrypt_in_place_detached(nonce, detached_prefix.prefix(), data, tag)
            }
            CocoonCipher::Aes256Gcm => {
                let cipher = Aes256Gcm::new(&master_key);
                cipher.decrypt_in_place_detached(nonce, detached_prefix.prefix(), data, tag)
            }
        }
        .map_err(|_| Error::Cryptography)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Cursor;

    use super::*;

    #[test]
    fn cocoon_create() {
        Cocoon::new(b"password").with_cipher(CocoonCipher::Aes256Gcm);
        Cocoon::from_seed(b"another password", [0; 32]).with_weak_kdf();
        Cocoon::from_entropy(b"new password");
        Cocoon::from_rng(b"password", rand::thread_rng()).unwrap();
        Cocoon::parse_only(b"password");
    }

    #[test]
    fn cocoon_encrypt() {
        let cocoon = Cocoon::from_seed(b"password", [0; 32]).with_weak_kdf();
        let mut data = "my secret data".to_owned().into_bytes();

        let detached_prefix = cocoon.encrypt(&mut data).unwrap();

        assert_eq!(
            &[
                127, 192, 10, 1, 1, 1, 2, 0, 155, 244, 154, 106, 7, 85, 249, 83, 129, 31, 206, 18,
                95, 38, 131, 213, 4, 41, 195, 187, 73, 224, 116, 20, 126, 0, 137, 165, 0, 0, 0, 0,
                0, 0, 0, 14, 114, 102, 232, 234, 188, 49, 190, 30, 41, 136, 238, 190, 46, 182, 211,
                244
            ][..],
            &detached_prefix[..]
        );

        assert_eq!(
            &[186, 240, 214, 29, 4, 147, 205, 72, 210, 7, 167, 234, 199, 53],
            &data[..]
        );
    }

    #[test]
    fn cocoon_encrypt_aes() {
        let cocoon = Cocoon::from_seed(b"password", [0; 32])
            .with_weak_kdf()
            .with_cipher(CocoonCipher::Aes256Gcm);
        let mut data = "my secret data".to_owned().into_bytes();

        let detached_prefix = cocoon.encrypt(&mut data).unwrap();

        assert_eq!(
            &[
                127, 192, 10, 1, 2, 1, 2, 0, 155, 244, 154, 106, 7, 85, 249, 83, 129, 31, 206, 18,
                95, 38, 131, 213, 4, 41, 195, 187, 73, 224, 116, 20, 126, 0, 137, 165, 0, 0, 0, 0,
                0, 0, 0, 14, 103, 127, 175, 154, 15, 80, 248, 145, 128, 241, 138, 15, 154, 128,
                201, 157
            ][..],
            &detached_prefix[..]
        );

        assert_eq!(
            &[88, 183, 11, 7, 192, 224, 203, 107, 144, 162, 48, 78, 61, 223],
            &data[..]
        );
    }

    #[test]
    fn cocoon_decrypt() {
        let detached_prefix = [
            127, 192, 10, 1, 1, 1, 1, 0, 118, 184, 224, 173, 160, 241, 61, 144, 64, 93, 106, 229,
            83, 134, 189, 40, 189, 210, 25, 184, 160, 141, 237, 26, 168, 54, 239, 204, 0, 0, 0, 0,
            0, 0, 0, 14, 53, 9, 86, 247, 53, 186, 123, 217, 156, 132, 173, 200, 208, 134, 179, 12,
        ];
        let mut data = [
            244, 85, 222, 144, 119, 169, 144, 11, 178, 216, 4, 57, 17, 47,
        ];
        let cocoon = Cocoon::parse_only(b"password");

        cocoon
            .decrypt(&mut data, &detached_prefix)
            .expect("Decrypted data");

        assert_eq!(b"my secret data", &data);
    }

    #[test]
    fn cocoon_decrypt_aes() {
        let detached_prefix = [
            127, 192, 10, 1, 2, 1, 2, 0, 155, 244, 154, 106, 7, 85, 249, 83, 129, 31, 206, 18, 95,
            38, 131, 213, 4, 41, 195, 187, 73, 224, 116, 20, 126, 0, 137, 165, 0, 0, 0, 0, 0, 0, 0,
            14, 103, 127, 175, 154, 15, 80, 248, 145, 128, 241, 138, 15, 154, 128, 201, 157,
        ];
        let mut data = [
            88, 183, 11, 7, 192, 224, 203, 107, 144, 162, 48, 78, 61, 223,
        ];
        let cocoon = Cocoon::parse_only(b"password");

        cocoon
            .decrypt(&mut data, &detached_prefix)
            .expect("Decrypted data");

        assert_eq!(b"my secret data", &data);
    }

    #[test]
    fn cocoon_wrap() {
        let cocoon = Cocoon::from_seed(b"password", [0; 32]);
        let wrapped = cocoon.wrap(b"data").expect("Wrapped container");

        assert_eq!(wrapped[wrapped.len() - 4..], [27, 107, 178, 181]);
    }

    #[test]
    fn cocoon_wrap_unwrap() {
        let cocoon = Cocoon::from_seed(b"password", [0; 32]);
        let wrapped = cocoon.wrap(b"data").expect("Wrapped container");
        let original = cocoon.unwrap(&wrapped).expect("Unwrapped container");

        assert_eq!(original, b"data");
    }

    #[test]
    fn cocoon_wrap_unwrap_corrupted() {
        let cocoon = Cocoon::from_seed(b"password", [0; 32]);
        let mut wrapped = cocoon.wrap(b"data").expect("Wrapped container");

        let last = wrapped.len() - 1;
        wrapped[last] += 1;
        cocoon.unwrap(&wrapped).expect_err("Unwrapped container");
    }

    #[test]
    fn cocoon_unwrap_larger_is_ok() {
        let cocoon = Cocoon::from_seed(b"password", [0; 32]);
        let mut wrapped = cocoon.wrap(b"data").expect("Wrapped container");

        wrapped.push(0);
        let original = cocoon.unwrap(&wrapped).expect("Unwrapped container");

        assert_eq!(original, b"data");
    }

    #[test]
    fn cocoon_unwrap_too_short() {
        let cocoon = Cocoon::from_seed(b"password", [0; 32]);
        let mut wrapped = cocoon.wrap(b"data").expect("Wrapped container");

        wrapped.pop();
        cocoon.unwrap(&wrapped).expect_err("Too short");
    }

    #[test]
    fn cocoon_decrypt_wrong_sizes() {
        let detached_prefix = [
            127, 192, 10, 1, 1, 1, 1, 0, 118, 184, 224, 173, 160, 241, 61, 144, 64, 93, 106, 229,
            83, 134, 189, 40, 189, 210, 25, 184, 160, 141, 237, 26, 168, 54, 239, 204, 0, 0, 0, 0,
            0, 0, 0, 14, 53, 9, 86, 247, 53, 186, 123, 217, 156, 132, 173, 200, 208, 134, 179, 12,
        ];
        let mut data = [
            244, 85, 222, 144, 119, 169, 144, 11, 178, 216, 4, 57, 17, 47, 0,
        ];
        let cocoon = Cocoon::parse_only(b"password");

        cocoon
            .decrypt(&mut data, &detached_prefix)
            .expect("Decrypted data");

        assert_eq!(b"my secret data\0", &data);

        cocoon
            .decrypt(&mut data[..4], &detached_prefix)
            .expect_err("Too short");
    }

    #[test]
    fn cocoon_dump_parse() {
        let buf = vec![0; 100];
        let mut file = Cursor::new(buf);
        let cocoon = Cocoon::from_seed(b"password", [0; 32]).with_weak_kdf();

        // Prepare data inside of `Vec` container.
        let data = b"my data".to_vec();

        cocoon.dump(data, &mut file).expect("Dumped container");
        assert_ne!(b"my data", file.get_ref().as_slice());

        // "Re-open" the file.
        file.set_position(0);

        let original = cocoon.parse(&mut file).expect("Parsed container");
        assert_eq!(b"my data", original.as_slice());
    }

    #[test]
    fn cocoon_dump_io_error() {
        let read_only_file =
            std::env::var("CARGO_TARGET_DIR").unwrap_or("target".into()) + "/read_only.txt";

        File::create(read_only_file.clone()).expect("Test file");
        let mut file = File::open(read_only_file).expect("Test file");

        let cocoon = Cocoon::from_seed(b"password", [0; 32]).with_weak_kdf();

        // Prepare data inside of `Vec` container.
        let data = b"my data".to_vec();

        match cocoon.dump(data, &mut file) {
            Err(e) => match e {
                Error::Io(_) => (),
                _ => panic!("Only unexpected I/O error is expected :)"),
            },
            _ => panic!("Success is not expected"),
        }
    }

    #[test]
    fn cocoon_parse_io_error() {
        let read_only_file =
            std::env::var("CARGO_TARGET_DIR").unwrap_or("target".into()) + "/read_only.txt";

        File::create(read_only_file.clone()).expect("Test file");
        let mut file = File::open(read_only_file).expect("Test file");

        let cocoon = Cocoon::from_seed(b"password", [0; 32]).with_weak_kdf();

        match cocoon.parse(&mut file) {
            Err(e) => match e {
                Error::TooShort => (),
                _ => panic!("TooShort is expected for an empty file"),
            },
            _ => panic!("Success is not expected"),
        }
    }
}
