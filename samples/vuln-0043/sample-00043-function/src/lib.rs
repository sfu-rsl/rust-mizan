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
}

impl<'a> Cocoon<'a, Creation> {
   
    #[cfg(feature = "alloc")]
    #[cfg_attr(docs_rs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn wrap(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
    
        let mut container = Vec::with_capacity(PREFIX_SIZE + data.len());
        container.extend_from_slice(&[0; PREFIX_SIZE]);
        container.extend_from_slice(data);

        let body = &mut container[PREFIX_SIZE..];

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
