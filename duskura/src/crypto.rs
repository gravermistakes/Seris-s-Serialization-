//! Cryptographic utilities for Duškura

use crate::error::{DuskuraError, Result};
use sha2::{Sha256, Digest};
use rand::Rng;
use base64::{engine::general_purpose, Engine as _};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};

/// Bytes of nonce prepended to every ciphertext produced by `encrypt`.
const NONCE_LEN: usize = 12;

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

/// Derive a 32-byte key from a password using Argon2id.
///
/// Returns the raw derived key bytes (not a PHC hash string) — this is a KDF
/// call, not a password-verification hash, so there's nothing to base64-decode.
pub fn derive_key(password: &str, salt: &[u8]) -> Result<Vec<u8>> {
    use argon2::{Argon2, PasswordHasher};
    use argon2::password_hash::SaltString;

    let salt_string = SaltString::encode_b64(salt)
        .map_err(|e| DuskuraError::EncryptionFailed(e.to_string()))?;

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| DuskuraError::EncryptionFailed(e.to_string()))?;

    let output = password_hash
        .hash
        .ok_or_else(|| DuskuraError::EncryptionFailed("No hash generated".to_string()))?;

    Ok(output.as_bytes().to_vec())
}

/// Derive the memory-encryption key for an identity from the operator-supplied
/// master passphrase (`DUSKURA_MASTER_KEY`), salted per-identity so a leaked
/// key for one identity doesn't expose another's.
pub fn master_key(identity_id: &str) -> Result<Vec<u8>> {
    let passphrase = std::env::var("DUSKURA_MASTER_KEY").map_err(|_| {
        DuskuraError::EncryptionFailed(
            "DUSKURA_MASTER_KEY environment variable is not set".to_string(),
        )
    })?;

    derive_key(&passphrase, identity_id.as_bytes())
}

/// Encrypt data with AES-256-GCM. A fresh random nonce is generated per call
/// and prepended to the returned ciphertext.
pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let nonce_bytes = random_bytes(NONCE_LEN);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| DuskuraError::EncryptionFailed(e.to_string()))?;

    let mut out = nonce_bytes;
    out.extend(ciphertext);
    Ok(out)
}

/// Decrypt data produced by `encrypt`.
pub fn decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    if data.len() < NONCE_LEN {
        return Err(DuskuraError::DecryptionFailed(
            "ciphertext shorter than nonce".to_string(),
        ));
    }
    let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);

    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| DuskuraError::DecryptionFailed(e.to_string()))
}

/// Encrypt a UTF-8 string for an identity and base64-encode it for storage
/// in a TEXT column.
pub fn encrypt_content(content: &str, identity_id: &str) -> Result<String> {
    let key = master_key(identity_id)?;
    let ciphertext = encrypt(content.as_bytes(), &key)?;
    Ok(encode_base64(&ciphertext))
}

/// Reverse of `encrypt_content`.
pub fn decrypt_content(stored: &str, identity_id: &str) -> Result<String> {
    let key = master_key(identity_id)?;
    let ciphertext = decode_base64(stored)?;
    let plaintext = decrypt(&ciphertext, &key)?;
    String::from_utf8(plaintext).map_err(|e| DuskuraError::DecryptionFailed(e.to_string()))
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
    fn test_derive_key_length() {
        let key = derive_key("test passphrase", b"some-identity-id-salt").unwrap();
        assert_eq!(key.len(), 32); // Aes256Gcm key size
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = derive_key("test passphrase", b"some-identity-id-salt").unwrap();
        let data = b"secret memory content";
        let encrypted = encrypt(data, &key).unwrap();
        assert_ne!(encrypted[NONCE_LEN..], data[..]); // ciphertext isn't plaintext
        let decrypted = decrypt(&encrypted, &key).unwrap();
        assert_eq!(data, &decrypted[..]);
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let key1 = derive_key("passphrase one", b"some-identity-id-salt").unwrap();
        let key2 = derive_key("passphrase two", b"some-identity-id-salt").unwrap();
        let encrypted = encrypt(b"secret", &key1).unwrap();
        assert!(decrypt(&encrypted, &key2).is_err());
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
