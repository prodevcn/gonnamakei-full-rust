use std::num::NonZeroU32;

use ring::digest::SHA256_OUTPUT_LEN;
use ring::pbkdf2;
use ring::pbkdf2::PBKDF2_HMAC_SHA256;
use tindercrypt::cryptors::RingCryptor;

use crate::error::AppResult;

const RANDOM_SALT_LENGTH: usize = 16;

/// Encode as hexadecimal.
pub fn encode_to_hex(value: &[u8]) -> String {
    data_encoding::HEXLOWER.encode(value)
}

/// Encode as base64.
pub fn encode_to_base64(value: &[u8]) -> String {
    data_encoding::BASE64.encode(value)
}

/// Decode from base64.
pub fn decode_from_base64(value: &[u8]) -> AppResult<Vec<u8>> {
    Ok(data_encoding::BASE64.decode(value)?)
}

/// Encode as base58.
pub fn encode_to_base58(value: &[u8]) -> String {
    solana_sdk::bs58::encode(value).into_string()
}

/// Decode from base58.
pub fn decode_from_base58(value: &[u8]) -> AppResult<Vec<u8>> {
    Ok(solana_sdk::bs58::decode(value).into_vec()?)
}

/// Encode as base64 in url format.
pub fn encode_to_url(value: &[u8]) -> String {
    data_encoding::BASE64URL.encode(value)
}

/// Decode from base64 in url format.
pub fn decode_from_url(value: &[u8]) -> AppResult<Vec<u8>> {
    Ok(data_encoding::BASE64URL.decode(value)?)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Hashes a password in order to store it and then be verified.
pub fn hash_password(db_salt: &[u8], password: &str) -> String {
    let (mut salt, random_salt) = salt(db_salt);
    let mut output_slice = [0u8; SHA256_OUTPUT_LEN];

    pbkdf2::derive(
        PBKDF2_HMAC_SHA256,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        password.as_bytes(),
        &mut output_slice,
    );

    salt.clear();
    salt.extend(&random_salt);
    salt.extend(&output_slice);

    encode_to_base64(&salt)
}

/// Validates a password previously encrypted by `hash_password`.
pub fn verify_password(db_salt: &[u8], attempted_password: &str, password: &str) -> bool {
    let password_and_random_salt = match decode_from_base64(password.as_bytes()) {
        Ok(v) => v,
        Err(_) => return false,
    };
    let random_salt = &password_and_random_salt[..RANDOM_SALT_LENGTH];
    let password = &password_and_random_salt[RANDOM_SALT_LENGTH..];

    let mut salt = Vec::with_capacity(db_salt.len() + random_salt.len());
    salt.extend(db_salt);
    salt.extend(random_salt);

    pbkdf2::verify(
        PBKDF2_HMAC_SHA256,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        attempted_password.as_bytes(),
        password,
    )
    .is_ok()
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Hashes a password in order to store it and then be verified.
pub fn encrypt(db_secret: &[u8], value: &[u8]) -> Vec<u8> {
    let cryptor = RingCryptor::new();
    cryptor.seal_with_passphrase(db_secret, value).unwrap()
}

/// Decrypts a data using a password.
pub fn decrypt(db_secret: &[u8], value: &[u8]) -> AppResult<Vec<u8>> {
    let cryptor = RingCryptor::new();
    Ok(cryptor.open(db_secret, value)?)
}

/// Hashes a password in order to store it and then be verified.
pub fn encrypt_to_base64(db_secret: &[u8], value: &[u8]) -> String {
    encode_to_base64(&encrypt(db_secret, value))
}

/// Decrypts a data using a password.
pub fn decrypt_from_base64(db_secret: &[u8], value: &str) -> AppResult<Vec<u8>> {
    let cryptor = RingCryptor::new();
    let result = cryptor.open(db_secret, &decode_from_base64(value.as_bytes())?)?;
    Ok(result)
}

/// Hashes a password in order to send it in a URL and then be verified.
pub fn encrypt_to_url(db_secret: &[u8], value: &[u8]) -> String {
    encode_to_url(&encrypt(db_secret, value))
}

/// Decrypts a data from a URL using a password.
pub fn decrypt_from_url(db_secret: &[u8], value: &str) -> AppResult<Vec<u8>> {
    let cryptor = RingCryptor::new();
    let result = cryptor.open(db_secret, &decode_from_url(value.as_bytes())?)?;
    Ok(result)
}

fn salt(db_salt: &[u8]) -> (Vec<u8>, [u8; RANDOM_SALT_LENGTH]) {
    let mut random_salt = [0u8; RANDOM_SALT_LENGTH];
    tindercrypt::rand::fill_buf(&mut random_salt);

    let mut salt = Vec::with_capacity(db_salt.len() + random_salt.len());
    salt.extend(db_salt);
    salt.extend(&random_salt);
    (salt, random_salt)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn password_verification() {
        let db_salt = &[10, 20, 30];
        let password = "test password";

        let hashed_password = hash_password(db_salt, password);

        assert!(
            verify_password(db_salt, "test password", hashed_password.as_str()),
            "Must be correct"
        );
        assert!(
            !verify_password(db_salt, "incorrect password", hashed_password.as_str()),
            "Must be incorrect"
        );
    }

    #[test]
    fn encrypt_decrypt_base64() {
        let password = "test password";
        let value = "Test value";

        let encrypted = encrypt_to_base64(password.as_bytes(), value.as_bytes());
        let decrypted = decrypt_from_base64(password.as_bytes(), encrypted.as_str())
            .expect("The decryption must success");

        assert_eq!(
            decrypted.as_slice(),
            value.as_bytes(),
            "The decrypted value is incorrect"
        );

        let decrypted = decrypt_from_base64("incorrect password".as_bytes(), encrypted.as_str());

        assert!(decrypted.is_err(), "The decryption cannot success");
    }

    #[test]
    fn encrypt_decrypt_url() {
        let password = "test password";
        let value = "Test value";

        let encrypted = encrypt_to_url(password.as_bytes(), value.as_bytes());
        let decrypted = decrypt_from_url(password.as_bytes(), encrypted.as_str())
            .expect("The decryption must success");

        assert_eq!(
            decrypted.as_slice(),
            value.as_bytes(),
            "The decrypted value is incorrect"
        );

        let decrypted = decrypt_from_base64("incorrect password".as_bytes(), encrypted.as_str());

        assert!(decrypted.is_err(), "The decryption cannot success");
    }
}
