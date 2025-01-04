use crate::primitives::field_element::FieldElement;
use crate::primitives::secp256k1::{Secp256k1, Secp256k1Point};
use hex;
use hmac::{Hmac, Mac};
use num_bigint::BigUint;
use num_traits::One;
use sha2::{Digest, Sha256};
use std::array::TryFromSliceError;
use std::convert::TryFrom;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct Key {
    private: [u8; 32],
    pub public: Secp256k1Point,
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub r: Vec<u8>,
    pub s: Vec<u8>,
}

/// Implements a struct representation that stores
/// a private key and its correspondent public key
impl Key {
    /// Create a Secp256k1Point from a given private key represented as bytes
    pub fn public_from(private: &[u8; 32]) -> Result<Secp256k1Point, String> {
        let prime = Secp256k1::Prime.as_biguint().to_str_radix(16);
        let p = prime.as_str();
        let private_num = BigUint::from_bytes_be(private).to_str_radix(16);
        let private_fe = FieldElement::new(private_num.as_str(), p).unwrap();
        let g = Secp256k1::Generator.as_point();
        Ok(private_fe.num * g)
    }

    /// Create a Key from a private key represented as 32 bytes
    pub fn new(private: [u8; 32]) -> Result<Self, String> {
        let public = Self::public_from(&private).unwrap();
        Ok(Self { private, public })
    }

    /// Create a Key from a private key represented as 32 bytes hexstring
    pub fn from_hexstr(private: &str) -> Result<Self, String> {
        // Decode the hexadecimal string into a Vec<u8>
        let decoded = match hex::decode(private) {
            Ok(bytes) => bytes,
            Err(_) => return Err("Invalid hexadecimal string".to_string()),
        };

        // Ensure the decoded vector is exactly 32 bytes long
        let bytes_private: [u8; 32] = match <[u8; 32]>::try_from(decoded.as_slice()) {
            Ok(arr) => arr,
            Err(_) => return Err("Hexadecimal string does not decode to 32 bytes".to_string()),
        };
        Self::new(bytes_private)
    }

    /// Apply sha256 hash to a given slice of bytes
    pub fn sha256(message: &[u8]) -> Result<[u8; 32], TryFromSliceError> {
        let mut hasher = Sha256::new();
        hasher.update(message);
        <[u8; 32]>::try_from(hasher.finalize().as_slice())
    }

    /// Apply double sha256 hash to a given slice of bytes
    pub fn double_sha256(message: &[u8]) -> Result<[u8; 32], TryFromSliceError> {
        let first_hash = Self::sha256(message)?; // First hash
        let slice_hash = first_hash.as_slice();
        Self::sha256(slice_hash)
    }

    /// Apply RFC6979
    /// Deterministic Usage of the Digital Signature Algorithm (DSA)
    /// and Elliptic Curve Digital Signature Algorithm (ECDSA)
    pub fn deterministic_k(&self, z: &[u8; 32]) -> Result<[u8; 32], String> {
        // Define constants
        let ord = Secp256k1::Order.as_biguint();

        // Define byte variables
        let mut k_bytes = vec![0u8; 32];
        let mut v_bytes = vec![1u8; 32];

        // Closure to update HMAC
        let update_hmac = |key: &[u8], data: &[&[u8]]| -> Result<Vec<u8>, String> {
            let mut mac = HmacSha256::new_from_slice(key)
                .map_err(|e| format!("Failed to init HMAC: {}", e))?;
            for part in data {
                mac.update(part);
            }
            Ok(mac.finalize().into_bytes().to_vec())
        };

        // Redefine k with byte 00
        k_bytes = update_hmac(&k_bytes, &[&v_bytes, &[0u8], &self.private, z])?;
        v_bytes = update_hmac(&k_bytes, &[&v_bytes])?;
        k_bytes = update_hmac(&k_bytes, &[&v_bytes, &[1u8], &self.private, z])?;
        v_bytes = update_hmac(&k_bytes, &[&v_bytes])?;

        loop {
            v_bytes = update_hmac(&k_bytes, &[&v_bytes])?;
            let k = BigUint::from_bytes_be(&v_bytes);
            if k >= BigUint::one() && k < ord {
                let result = <[u8; 32]>::try_from(k.to_bytes_be()).unwrap();
                return Ok(result);
            }
            k_bytes = update_hmac(&k_bytes, &[&v_bytes, &[0u8]])?;
            v_bytes = update_hmac(&k_bytes, &[&v_bytes])?;
        }
    }

    /// Sign a BIP 62 compliant hashed message
    pub fn sign(&self, z: [u8; 32]) -> Result<Signature, String> {
        // Extract some required constants
        let g = Secp256k1::Generator.as_point();
        let two = BigUint::from(2u32);
        let ord = Secp256k1::Order.as_biguint();

        // convert z to num
        let z_num = BigUint::from_bytes_be(&z);
        let e_num = BigUint::from_bytes_be(&self.private);

        // Generate deterministic k
        let k = self.deterministic_k(&z)?;
        let k_num = BigUint::from_bytes_be(&k);

        // Calculate r = (k * G).x
        let r_point = &k_num * &g;
        let r_num = r_point.x.unwrap().num % &ord;

        // Calculate k_inv = k^(ord-2) mod ord
        let k_inv = &k_num.modpow(&(&ord - &two), &ord);

        // Calculate s = k_inv * (z + r * private_key) mod ord
        let mut s_num = (k_inv * (&z_num + (&r_num * &e_num) % &ord)) % &ord;

        // Ensure low-S compliance
        if s_num > (&ord / &two) {
            s_num = &ord - &s_num;
        }

        if r_num.to_bytes_be().len() == 32 {
            let r = <[u8; 32]>::try_from(r_num.to_bytes_be()).unwrap();
            let s = <[u8; 32]>::try_from(s_num.to_bytes_be()).unwrap();
            Ok(Signature {
                r: r.to_vec(),
                s: s.to_vec(),
            })
        } else {
            let r = <[u8; 33]>::try_from(r_num.to_bytes_be()).unwrap();
            let s = <[u8; 32]>::try_from(s_num.to_bytes_be()).unwrap();
            Ok(Signature {
                r: r.to_vec(),
                s: s.to_vec(),
            })
        }
    }

    /// Apply signature verification from a given hashed message
    pub fn verify(&self, z: &[u8; 32], signature: &Signature) -> bool {
        // define some "constants"
        let two = BigUint::from(2u32);
        let ord = Secp256k1::Order.as_biguint();
        let generator = Secp256k1::Generator.as_point();

        let z_num = BigUint::from_bytes_be(z);
        let s_num = BigUint::from_bytes_be(signature.s.as_slice());
        let r_num = BigUint::from_bytes_be(signature.r.as_slice());

        let exp = &ord - &two;
        let s_inv = s_num.modpow(&exp, &ord);

        let u = (&z_num * &s_inv) % &ord;
        let v = (&r_num * &s_inv) % ord;

        let u_g = u * generator;
        let v_p = v * &self.public;
        let total = u_g + v_p;

        total.x.unwrap().num == r_num
    }
}
