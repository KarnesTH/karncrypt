use ring::{
    aead::{self, Nonce},
    pbkdf2,
    rand::{SecureRandom, SystemRandom},
};
use std::num::NonZeroU32;

#[derive(Clone)]
pub struct Encryption {
    key: aead::LessSafeKey,
}

impl Encryption {
    pub fn new(master_password: &str, salt: &[u8; 16]) -> Self {
        let mut key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000).unwrap(),
            salt,
            master_password.as_bytes(),
            &mut key,
        );

        let unbound_key =
            aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &key).expect("Failed to create key");

        Self {
            key: aead::LessSafeKey::new(unbound_key),
        }
    }

    pub fn encrypt(&self, data: &str) -> Result<Vec<u8>, ring::error::Unspecified> {
        let rng = SystemRandom::new();
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes)?;
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);

        let mut in_out = data.as_bytes().to_vec();
        self.key
            .seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)?;

        let mut result = Vec::with_capacity(nonce_bytes.len() + in_out.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&in_out);

        Ok(result)
    }

    pub fn decrypt(&self, encryted_data: &[u8]) -> Result<String, ring::error::Unspecified> {
        if encryted_data.len() < 12 {
            return Err(ring::error::Unspecified);
        }

        let nonce = Nonce::assume_unique_for_key(
            encryted_data[..12]
                .try_into()
                .map_err(|_| ring::error::Unspecified)?,
        );
        let mut in_out = encryted_data[12..].to_vec();

        let plain_text = self
            .key
            .open_in_place(nonce, aead::Aad::empty(), &mut in_out)?;

        String::from_utf8(plain_text.to_vec()).map_err(|_| ring::error::Unspecified)
    }

    pub fn get_key(&self, master_password: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut key_bytes = [0u8; 32];

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000).unwrap(),
            b"db_encryption",
            master_password.as_bytes(),
            &mut key_bytes,
        );

        let hex_string = key_bytes
            .iter()
            .fold(String::with_capacity(64), |mut acc, b| {
                use std::fmt::Write;
                write!(&mut acc, "{:02x}", b).expect("Failed to write to string");
                acc
            });

        Ok(hex_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_salt() -> [u8; 16] {
        let mut salt = [0u8; 16];
        let rng = SystemRandom::new();
        rng.fill(&mut salt).unwrap();
        salt
    }

    #[test]
    fn test_encrypt_decrypt() {
        let master_password = "password";
        let data = "data";
        let salt = create_test_salt();

        let encryption = Encryption::new(master_password, &salt);
        let encrypted_data = encryption.encrypt(data).unwrap();
        let decrypted_data = encryption.decrypt(&encrypted_data).unwrap();

        assert_eq!(data, decrypted_data);
    }

    #[test]
    fn test_decrypt_invalid_data() {
        let salt = create_test_salt();
        let encryption = Encryption::new("password", &salt);
        let result = encryption.decrypt(&[0u8; 8]);
        assert!(result.is_err());
    }

    #[test]
    fn test_different_passwords() {
        let data = "data";
        let salt = create_test_salt();
        let encryption1 = Encryption::new("password1", &salt);
        let encryption2 = Encryption::new("password2", &salt);

        let encrypted_data = encryption1.encrypt(data).unwrap();
        let result = encryption2.decrypt(&encrypted_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_same_password_different_salt() {
        let data = "data";
        let password = "password";
        let salt1 = create_test_salt();
        let salt2 = create_test_salt();

        let encryption1 = Encryption::new(password, &salt1);
        let encryption2 = Encryption::new(password, &salt2);

        let encrypted_data = encryption1.encrypt(data).unwrap();
        let result = encryption2.decrypt(&encrypted_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_key() {
        let salt = create_test_salt();
        let encryption = Encryption::new("password", &salt);

        let key1 = encryption.get_key("testpass").unwrap();
        let key2 = encryption.get_key("testpass").unwrap();
        assert_eq!(key1, key2);

        let key3 = encryption.get_key("different").unwrap();
        assert_ne!(key1, key3);

        assert_eq!(key1.len(), 64);
    }

    #[test]
    fn test_empty_data() {
        let salt = create_test_salt();
        let encryption = Encryption::new("password", &salt);

        let encrypted = encryption.encrypt("").unwrap();
        let decrypted = encryption.decrypt(&encrypted).unwrap();
        assert_eq!("", decrypted);
    }

    #[test]
    fn test_large_data() {
        let salt = create_test_salt();
        let encryption = Encryption::new("password", &salt);

        let large_data = "x".repeat(1000);
        let encrypted = encryption.encrypt(&large_data).unwrap();
        let decrypted = encryption.decrypt(&encrypted).unwrap();
        assert_eq!(large_data, decrypted);
    }

    #[test]
    fn test_encryption_determinism() {
        let salt = create_test_salt();
        let encryption = Encryption::new("password", &salt);

        let data = "test";
        let enc1 = encryption.encrypt(data).unwrap();
        let enc2 = encryption.encrypt(data).unwrap();
        assert_ne!(enc1, enc2);

        assert_eq!(encryption.decrypt(&enc1).unwrap(), data);
        assert_eq!(encryption.decrypt(&enc2).unwrap(), data);
    }
}
