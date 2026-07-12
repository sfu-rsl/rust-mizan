pub const KEY_SIZE: usize = 32;
pub const SALT_MIN_SIZE: usize = 16;
pub const SALT_MAX_SIZE: usize = 128;

pub mod pbkdf2 {
    use hmac::Hmac;
    use pbkdf2::pbkdf2;
    use sha2::Sha256;
    use zeroize::Zeroizing;

    use super::{KEY_SIZE, SALT_MAX_SIZE, SALT_MIN_SIZE};

    pub fn derive(salt: &[u8], password: &[u8], iterations: u32) -> Zeroizing<[u8; KEY_SIZE]> {
        debug_assert!(salt.len() >= SALT_MIN_SIZE);
        debug_assert!(salt.len() <= SALT_MAX_SIZE);

        const COCOON_PREFIX: &[u8] = b"cocoon";
        const COCOON_PREFIX_LEN: usize = COCOON_PREFIX.len();

        let mut ext_salt = [0u8; SALT_MAX_SIZE + COCOON_PREFIX_LEN];
        ext_salt[..COCOON_PREFIX_LEN].copy_from_slice(COCOON_PREFIX);
        ext_salt[COCOON_PREFIX_LEN..COCOON_PREFIX_LEN + salt.len()].copy_from_slice(salt);

        // Prepare an output buffer.
        let mut derived_key = [0u8; KEY_SIZE];

        pbkdf2::<Hmac<Sha256>>(
            password,
            &ext_salt[..COCOON_PREFIX_LEN + salt.len()],
            iterations,
            &mut derived_key,
        );

        Zeroizing::new(derived_key)
    }
}
