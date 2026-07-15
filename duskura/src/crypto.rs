//! Cryptographic utilities for Duškura

use crate::error::{DuskuraError, Result};
use sha2::{Sha256, Digest};
use rand::Rng;
use base64::{engine::general_purpose, Engine as _};

/// Generate SHA-256 hash of data
pub fn hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Verify hash chain integrity
pub fn verify_hash_chain(entries: &[(String, String)]) -> Result<()> {
    if entries.is_empty() {
        return Ok(());
    }

    let mut prev_hash = String::new();
    
    for (current_hash, prev_in_entry) in entries {
        if !prev_hash.is_empty() && prev_hash != *prev_in_entry {
            return Err(DuskuraError::HashVerificationFailed);
        }
        prev_hash = current_hash.clone();
    }

    Ok(())
}

/// Generate cryptographic random bytes
pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen()).collect()
}

/// Generate random hex string
pub fn random_hex(len: usize) -> String {
    hex::encode(random_bytes(len))
}

/// Derive key from password using Argon2
pub fn derive_key(password: &str, salt: &[u8]) -> Result<Vec<u8>> {
    use argon2::{Argon2, PasswordHasher, ParamString};
    use argon2::password_hash::{SaltString, PasswordHash};

    let salt_string = SaltString::encode_b64(salt)
        .map_err(|e| DuskuraError::EncryptionFailed(e.to_string()))?;
    
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt_string)
        .map_err(|e| DuskuraError::EncryptionFailed(e.to_string()))?;

    let hash_str = password_hash.hash
        .ok_or_else(|| DuskuraError::EncryptionFailed("No hash generated".to_string()))?
        .as_str();
    
    Ok(general_purpose::STANDARD.decode(hash_str)
        .map_err(|e| DuskuraError::EncryptionFailed(e.to_string()))?)
}

/// XOR encryption (simple, for demonstration)
pub fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, byte)| byte ^ key[i % key.len()])
        .collect()
}

/// XOR decryption
pub fn xor_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    xor_encrypt(data, key)
}

/// Encode bytes to base64
pub fn encode_base64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

/// Decode base64 to bytes
pub fn decode_base64(data: &str) -> Result<Vec<u8>> {
    general_purpose::STANDARD
        .decode(data)
        .map_err(|e| DuskuraError::EncryptionFailed(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_consistency() {
        let data = b"test data";
        let hash1 = hash(data);
        let hash2 = hash(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_xor_encrypt_decrypt() {
        let data = b"secret message";
        let key = b"password";
        let encrypted = xor_encrypt(data, key);
        let decrypted = xor_decrypt(&encrypted, key);
        assert_eq!(data, &decrypted[..]);
    }

    #[test]
    fn test_random_bytes() {
        let bytes1 = random_bytes(16);
        let bytes2 = random_bytes(16);
        assert_eq!(bytes1.len(), 16);
        assert_eq!(bytes2.len(), 16);
        assert_ne!(bytes1, bytes2);
    }
}
