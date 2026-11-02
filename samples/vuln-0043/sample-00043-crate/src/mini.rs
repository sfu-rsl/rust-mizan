use aes_gcm::{
    aead::{generic_array::GenericArray, KeyInit},
    AeadInPlace, Aes256Gcm,
};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use chacha20poly1305::ChaCha20Poly1305;
use rand::{rngs::StdRng, RngCore, SeedableRng};
#[cfg(feature = "std")]
use std::io::{Read, Write};
use zeroize::Zeroizing;

use super::{
    error::Error,
    format::MiniFormatPrefix,
    header::{CocoonCipher, CocoonConfig, CocoonKdf, MiniCocoonHeader},
    kdf::{self, KEY_SIZE},
};

pub const MINI_PREFIX_SIZE: usize = MiniFormatPrefix::SERIALIZE_SIZE;

pub struct MiniCocoon {
    key: Zeroizing<[u8; KEY_SIZE]>,
    rng: StdRng,
    config: CocoonConfig,
}


impl MiniCocoon {
    
    pub fn from_key(key: &[u8], seed: &[u8]) -> Self {
        let mut k = [0u8; KEY_SIZE];
        let mut s = [0u8; KEY_SIZE];

        k.copy_from_slice(key);
        s.copy_from_slice(seed);

        let key = Zeroizing::new(k);
        let rng = StdRng::from_seed(s);

        MiniCocoon {
            key,
            rng,
            config: CocoonConfig::default(),
        }
    }

    pub fn from_password(password: &[u8], seed: &[u8]) -> Self {
        let config = CocoonConfig::default();
        let key = match config.kdf() {
            CocoonKdf::Pbkdf2 => kdf::pbkdf2::derive(seed, password, config.kdf_iterations()),
        };

        let mut s = [0u8; KEY_SIZE];
        s.copy_from_slice(seed);

        let rng = StdRng::from_seed(s);

        MiniCocoon { key, rng, config }
    }

    pub fn with_cipher(mut self, cipher: CocoonCipher) -> Self {
        self.config = self.config.with_cipher(cipher);
        self
    }

    #[cfg(feature = "alloc")]
    #[cfg_attr(docs_rs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn wrap(&self, data: &[u8]) -> Result<Vec<u8>, Error> {

        let mut container = Vec::with_capacity(MINI_PREFIX_SIZE + data.len());
        container.extend_from_slice(&[0; MINI_PREFIX_SIZE]);
        container.extend_from_slice(data);

        let body = &mut container[MINI_PREFIX_SIZE..];

        // Encrypt in place and get a prefix part.
        let detached_prefix = self.encrypt(body)?;

        container[..MINI_PREFIX_SIZE].copy_from_slice(&detached_prefix);

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

    pub fn encrypt(&self, data: &mut [u8]) -> Result<[u8; MINI_PREFIX_SIZE], Error> {
        let mut rng = self.rng.clone();

        let mut nonce = [0u8; 12];
        rng.fill_bytes(&mut nonce);

        let header = MiniCocoonHeader::new(nonce, data.len());
        let prefix = MiniFormatPrefix::new(header);

        let nonce = GenericArray::from_slice(&nonce);
        let key = GenericArray::clone_from_slice(self.key.as_ref());

        let tag: [u8; 16] = match self.config.cipher() {
            CocoonCipher::Chacha20Poly1305 => {
                let cipher = ChaCha20Poly1305::new(&key);
                cipher.encrypt_in_place_detached(nonce, prefix.prefix(), data)
            }
            CocoonCipher::Aes256Gcm => {
                let cipher = Aes256Gcm::new(&key);
                cipher.encrypt_in_place_detached(nonce, prefix.prefix(), data)
            }
        }
        .map_err(|_| Error::Cryptography)?
        .into();

        Ok(prefix.serialize(&tag))
    }

    #[cfg(feature = "alloc")]
    #[cfg_attr(docs_rs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn unwrap(&self, container: &[u8]) -> Result<Vec<u8>, Error> {
        let prefix = MiniFormatPrefix::deserialize(container)?;
        let header = prefix.header();

        if container.len() < MINI_PREFIX_SIZE + header.data_length() {
            return Err(Error::TooShort);
        }

        let mut body = Vec::with_capacity(header.data_length());
        body.extend_from_slice(&container[MINI_PREFIX_SIZE..MINI_PREFIX_SIZE + body.capacity()]);

        self.decrypt_parsed(&mut body, &prefix)?;

        Ok(body)
    }

    #[cfg(feature = "std")]
    #[cfg_attr(docs_rs, doc(cfg(feature = "std")))]
    pub fn parse(&self, reader: &mut impl Read) -> Result<Vec<u8>, Error> {
        let prefix = MiniFormatPrefix::deserialize_from(reader)?;
        let mut body = vec![0; prefix.header().data_length()];

        // Too short error can be thrown right from here.
        reader.read_exact(&mut body)?;

        self.decrypt_parsed(&mut body, &prefix)?;

        Ok(body)
    }

    pub fn decrypt(&self, data: &mut [u8], detached_prefix: &[u8]) -> Result<(), Error> {
        let prefix = MiniFormatPrefix::deserialize(detached_prefix)?;

        self.decrypt_parsed(data, &prefix)
    }

    fn decrypt_parsed(
        &self,
        data: &mut [u8],
        detached_prefix: &MiniFormatPrefix,
    ) -> Result<(), Error> {
        let mut nonce = [0u8; 12];

        let header = detached_prefix.header();

        if data.len() < header.data_length() {
            return Err(Error::TooShort);
        }

        let data = &mut data[..header.data_length()];

        nonce.copy_from_slice(header.nonce());

        let nonce = GenericArray::from_slice(&nonce);
        let master_key = GenericArray::clone_from_slice(self.key.as_ref());
        let tag = GenericArray::from_slice(detached_prefix.tag());

        match self.config.cipher() {
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
    fn mini_cocoon_create() {
        MiniCocoon::from_password(b"password", &[0; 32]);
        MiniCocoon::from_key(&[1; 32], &[0; 32]);
    }

    #[test]
    fn mini_cocoon_encrypt() {
        let cocoon = MiniCocoon::from_password(b"password", &[0; 32]);
        let mut data = "my secret data".to_owned().into_bytes();

        let detached_prefix = cocoon.encrypt(&mut data).unwrap();

        assert_eq!(
            &[
                155, 244, 154, 106, 7, 85, 249, 83, 129, 31, 206, 18, 0, 0, 0, 0, 0, 0, 0, 14, 88,
                114, 102, 98, 71, 228, 153, 231, 144, 157, 177, 113, 160, 209, 154, 83
            ][..],
            &detached_prefix[..]
        );

        assert_eq!(
            &[98, 34, 35, 62, 28, 121, 71, 223, 170, 151, 215, 104, 52, 187],
            &data[..]
        );
    }

    #[test]
    fn mini_cocoon_encrypt_aes() {
        let cocoon =
            MiniCocoon::from_password(b"password", &[0; 32]).with_cipher(CocoonCipher::Aes256Gcm);
        let mut data = "my secret data".to_owned().into_bytes();

        let detached_prefix = cocoon.encrypt(&mut data).unwrap();

        assert_eq!(
            &[
                155, 244, 154, 106, 7, 85, 249, 83, 129, 31, 206, 18, 0, 0, 0, 0, 0, 0, 0, 14, 95,
                1, 247, 191, 121, 127, 53, 49, 59, 241, 134, 122, 122, 207, 110, 138
            ][..],
            &detached_prefix[..]
        );

        assert_eq!(
            &[41, 58, 226, 219, 28, 132, 21, 216, 165, 46, 246, 120, 10, 92],
            &data[..]
        );
    }

    #[test]
    fn mini_cocoon_decrypt() {
        let detached_prefix = [
            118, 184, 224, 173, 160, 241, 61, 144, 64, 93, 106, 229, 0, 0, 0, 0, 0, 0, 0, 14, 159,
            31, 100, 63, 43, 219, 99, 46, 201, 213, 205, 233, 174, 235, 43, 24,
        ];
        let mut data = [
            224, 50, 239, 254, 30, 140, 44, 135, 217, 94, 127, 67, 78, 31,
        ];
        let cocoon = MiniCocoon::from_password(b"password", &[0; 32]);

        cocoon
            .decrypt(&mut data, &detached_prefix)
            .expect("Decrypted data");

        assert_eq!(b"my secret data", &data);
    }

    #[test]
    fn mini_cocoon_decrypt_aes() {
        let detached_prefix = [
            118, 184, 224, 173, 160, 241, 61, 144, 64, 93, 106, 229, 0, 0, 0, 0, 0, 0, 0, 14, 165,
            83, 248, 230, 121, 148, 146, 253, 98, 153, 208, 174, 129, 31, 162, 13,
        ];
        let mut data = [
            178, 119, 26, 64, 67, 5, 235, 21, 238, 150, 245, 172, 197, 114,
        ];
        let cocoon =
            MiniCocoon::from_password(b"password", &[0; 32]).with_cipher(CocoonCipher::Aes256Gcm);

        cocoon
            .decrypt(&mut data, &detached_prefix)
            .expect("Decrypted data");

        assert_eq!(b"my secret data", &data);
    }

    #[test]
    fn mini_cocoon_wrap() {
        let cocoon = MiniCocoon::from_password(b"password", &[0; 32]);
        let wrapped = cocoon.wrap(b"data").expect("Wrapped container");

        assert_eq!(wrapped[wrapped.len() - 4..], [107, 58, 119, 44]);
    }

    #[test]
    fn mini_cocoon_wrap_unwrap() {
        let cocoon = MiniCocoon::from_key(&[1; 32], &[0; 32]);
        let wrapped = cocoon.wrap(b"data").expect("Wrapped container");
        let original = cocoon.unwrap(&wrapped).expect("Unwrapped container");

        assert_eq!(original, b"data");
    }

    #[test]
    fn mini_cocoon_wrap_unwrap_corrupted() {
        let cocoon = MiniCocoon::from_key(&[1; 32], &[0; 32]);
        let mut wrapped = cocoon.wrap(b"data").expect("Wrapped container");

        let last = wrapped.len() - 1;
        wrapped[last] += 1;
        cocoon.unwrap(&wrapped).expect_err("Unwrapped container");
    }

    #[test]
    fn mini_cocoon_unwrap_larger_is_ok() {
        let cocoon = MiniCocoon::from_key(&[1; 32], &[0; 32]);
        let mut wrapped = cocoon.wrap(b"data").expect("Wrapped container");

        wrapped.push(0);
        let original = cocoon.unwrap(&wrapped).expect("Unwrapped container");

        assert_eq!(original, b"data");
    }

    #[test]
    fn mini_cocoon_unwrap_too_short() {
        let cocoon = MiniCocoon::from_key(&[1; 32], &[0; 32]);
        let mut wrapped = cocoon.wrap(b"data").expect("Wrapped container");

        wrapped.pop();
        cocoon.unwrap(&wrapped).expect_err("Too short");
    }

    #[test]
    fn cocoon_decrypt_wrong_sizes() {
        let detached_prefix = [
            118, 184, 224, 173, 160, 241, 61, 144, 64, 93, 106, 229, 0, 0, 0, 0, 0, 0, 0, 14, 165,
            83, 248, 230, 121, 148, 146, 253, 98, 153, 208, 174, 129, 31, 162, 13,
        ];
        let mut data = [
            178, 119, 26, 64, 67, 5, 235, 21, 238, 150, 245, 172, 197, 114, 0,
        ];
        let cocoon =
            MiniCocoon::from_password(b"password", &[0; 32]).with_cipher(CocoonCipher::Aes256Gcm);

        cocoon
            .decrypt(&mut data, &detached_prefix)
            .expect("Decrypted data");

        assert_eq!(b"my secret data\0", &data);

        cocoon
            .decrypt(&mut data[..4], &detached_prefix)
            .expect_err("Too short");
    }

    #[test]
    fn mini_cocoon_dump_parse() {
        let buf = vec![0; 100];
        let mut file = Cursor::new(buf);
        let cocoon = MiniCocoon::from_key(&[1; 32], &[0; 32]);

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
    fn mini_cocoon_dump_io_error() {
        let read_only_file =
            std::env::var("CARGO_TARGET_DIR").unwrap_or("target".into()) + "/read_only.txt";

        File::create(read_only_file.clone()).expect("Test file");
        let mut file = File::open(read_only_file).expect("Test file");

        let cocoon = MiniCocoon::from_key(&[1; 32], &[0; 32]);

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
    fn mini_cocoon_parse_io_error() {
        let read_only_file =
            std::env::var("CARGO_TARGET_DIR").unwrap_or("target".into()) + "/read_only.txt";

        File::create(read_only_file.clone()).expect("Test file");
        let mut file = File::open(read_only_file).expect("Test file");

        let cocoon = MiniCocoon::from_key(&[1; 32], &[0; 32]);

        match cocoon.parse(&mut file) {
            Err(e) => match e {
                Error::TooShort => (),
                _ => panic!("TooShort is expected for an empty file"),
            },
            _ => panic!("Success is not expected"),
        }
    }
}
