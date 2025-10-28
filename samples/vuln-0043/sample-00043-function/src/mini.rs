use aes_gcm::{
    aead::{generic_array::GenericArray, KeyInit},
    AeadInPlace, Aes256Gcm,
};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use chacha20poly1305::ChaCha20Poly1305;
use rand::{rngs::StdRng, RngCore, SeedableRng};
#[cfg(feature = "std")]
use std::io::{ Write};
use zeroize::Zeroizing;

use super::{
    error::Error,
    format::MiniFormatPrefix,
    header::{CocoonCipher, CocoonConfig, MiniCocoonHeader},
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

    #[cfg(feature = "alloc")]
    #[cfg_attr(docs_rs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn wrap(&self, data: &[u8]) -> Result<Vec<u8>, Error> {

        let mut container = Vec::with_capacity(MINI_PREFIX_SIZE + data.len());
        container.extend_from_slice(&[0; MINI_PREFIX_SIZE]);
        container.extend_from_slice(data);

        let body = &mut container[MINI_PREFIX_SIZE..];

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

}
