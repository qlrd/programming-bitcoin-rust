use hmac::{Hmac, Mac};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256, Sha512};
use std::array::TryFromSliceError;

pub const MAINNET_PREFIX: u8 = 0u8; // 0x00
pub const TESTNET_PREFIX: u8 = 111u8; // 0x6F

/// Alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

/// Alias for HMAC-SHA256
type HmacSha512 = Hmac<Sha512>;

/// Apply sha256 hash to a given slice of bytes
pub fn sha256(message: &[u8]) -> Result<[u8; 32], TryFromSliceError> {
    let mut hasher = Sha256::new();
    hasher.update(message);
    <[u8; 32]>::try_from(hasher.finalize().as_slice())
}

/// Apply double sha256 hash to a given slice of bytes
pub fn double_sha256(message: &[u8]) -> Result<[u8; 32], TryFromSliceError> {
    let first_hash = sha256(message)?;

    // First hash
    let slice_hash = first_hash.as_slice();

    // Second hash
    sha256(slice_hash)
}

/// Apply ripemd160 hash to a given slice of bytes
pub fn ripemd160(message: &[u8]) -> Result<[u8; 20], std::array::TryFromSliceError> {
    let mut hasher = Ripemd160::new();
    hasher.update(message);
    <[u8; 20]>::try_from(hasher.finalize().as_slice())
}

/// Apply hash160 hash to a given slice of bytes
pub fn hash160(message: &[u8]) -> Result<[u8; 20], TryFromSliceError> {
    let first_hash = double_sha256(message)?;

    // First hash
    let slice_hash = first_hash.as_slice();

    // Second hash
    ripemd160(slice_hash)
}

/// Update some key with data to convert it in a secure result
/// Mainly used in deterministic usage of Digital Signature Algorithm
/// and Elliptc Curve Digital Signature Algorithm
/// (RFC6979)
pub fn hmac256(key: &[u8], data: &[&[u8]]) -> Result<Vec<u8>, String> {
    let mut mac =
        HmacSha256::new_from_slice(key).map_err(|e| format!("Failed to init HMAC: {}", e))?;
    for part in data {
        mac.update(part);
    }
    Ok(mac.finalize().into_bytes().to_vec())
}

/// Update some key with data to convert it in a secure result
///
/// Mainly used in derivation of paths as specified in BIP32
/// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#user-content-Private_parent_key_rarr_private_child_key
pub fn hmac512(key: &[u8], data: &[&[u8]]) -> Result<Vec<u8>, String> {
    let mut mac =
        HmacSha512::new_from_slice(key).map_err(|e| format!("Failed to init HMAC: {}", e))?;
    for part in data {
        mac.update(part);
    }
    Ok(mac.finalize().into_bytes().to_vec())
}
