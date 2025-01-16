use base58::encode_base58check;
use field_element::FieldElement;
use hasher::{hash160, hmac256, MAINNET_PREFIX, TESTNET_PREFIX};
use num_bigint::BigUint;
use num_traits::One;
use secp256k1::{Secp256k1, Secp256k1Point};

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

impl Signature {
    /// create a signature from BigUint
    pub fn from_biguint(r: BigUint, s: BigUint) -> Result<Self, String> {
        if r.to_bytes_be().len() == 32 {
            let r_vec = <[u8; 32]>::try_from(r.to_bytes_be()).unwrap().to_vec();
            let s_vec = <[u8; 32]>::try_from(s.to_bytes_be()).unwrap().to_vec();
            Ok(Signature::new(r_vec, s_vec).unwrap())
        } else {
            let r_vec = <[u8; 33]>::try_from(r.to_bytes_be()).unwrap().to_vec();
            let s_vec = <[u8; 32]>::try_from(s.to_bytes_be()).unwrap().to_vec();
            Ok(Signature::new(r_vec, s_vec).unwrap())
        }
    }

    /// Create a Signature from two vectors.
    /// The `r` value can be 32 or 33 bytes; the `s`
    /// value should be 32 bytes
    pub fn new(r: Vec<u8>, s: Vec<u8>) -> Result<Self, String> {
        println!("len == {}: {:?}", r.len(), r);
        if r.len() != 32 && r.len() != 33 {
            return Err("R value should have 32 or 33 bytes length".to_string());
        }

        Ok(Self { r, s })
    }

    /// Serialize the current Signature struct to bitcoin's DER format
    pub fn der(&self) -> Result<Vec<u8>, String> {
        // start with 0x30 byte, equivalent 48u8
        let mut serialized = vec![48u8];

        let serialize = |element: &Vec<u8>| -> Result<Vec<u8>, String> {
            if element.is_empty() {
                return Err("Signature element cannot be empty.".to_string());
            }

            // Append the 0x02 marker
            let mut res = vec![2u8];

            // Prepend 0x00 if the first byte is >= 0x80 (MSB is set)
            if element[0] >= 128u8 {
                res.push((element.len() + 1) as u8);
                res.push(0u8);
            } else {
                res.push(element.len() as u8);
            }

            // Append the element itself
            res.extend_from_slice(element.as_slice());
            Ok(res)
        };

        let r = serialize(&self.r).map_err(|e| format!("Error serializing 'r': {}", e))?;

        let s = serialize(&self.s).map_err(|e| format!("Error serializing 's': {}", e))?;

        let len = r.len() + s.len();

        if len > 255 {
            return Err(format!(
                "Total serialized length exceeds maximum allowable value: {} bytes.",
                len
            ));
        }

        serialized.extend_from_slice(&[len as u8]);
        serialized.extend_from_slice(&r);
        serialized.extend_from_slice(&s);
        Ok(serialized)
    }
}

/// Implements a struct representation that stores
/// a private key and its correspondent public key
impl Key {
    /// Create a Secp256k1Point from a given private key represented as bytes
    pub fn to_public(private: &[u8; 32]) -> Result<Secp256k1Point, String> {
        let prime = Secp256k1::Prime.as_biguint().to_str_radix(16);
        let p = prime.as_str();
        let private_num = BigUint::from_bytes_be(private).to_str_radix(16);
        let private_fe = FieldElement::new(private_num.as_str(), p).unwrap();
        let g = Secp256k1::Generator.as_point();
        Ok(private_fe.num * g)
    }

    /// Create a Key from a private key represented as 32 bytes
    pub fn from_bytes_be(private: [u8; 32]) -> Result<Self, String> {
        let public = Self::to_public(&private).unwrap();
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
        Self::from_bytes_be(bytes_private)
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

        // Redefine k with byte 00
        k_bytes = hmac256(&k_bytes, &[&v_bytes, &[0u8], &self.private, z])?;
        v_bytes = hmac256(&k_bytes, &[&v_bytes])?;
        k_bytes = hmac256(&k_bytes, &[&v_bytes, &[1u8], &self.private, z])?;
        v_bytes = hmac256(&k_bytes, &[&v_bytes])?;

        loop {
            v_bytes = hmac256(&k_bytes, &[&v_bytes])?;
            let k = BigUint::from_bytes_be(&v_bytes);
            if k >= BigUint::one() && k < ord {
                let result = <[u8; 32]>::try_from(k.to_bytes_be()).unwrap();
                return Ok(result);
            }
            k_bytes = hmac256(&k_bytes, &[&v_bytes, &[0u8]])?;
            v_bytes = hmac256(&k_bytes, &[&v_bytes])?;
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

        Ok(Signature::from_biguint(r_num, s_num).unwrap())
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

    /// Return an address string (P2PKH format)
    pub fn to_pubkey_hash(&self, compressed: bool, testnet: bool) -> Result<String, String> {
        // Get the public key
        let pubkey = &self.public;

        // Generate the SEC (serialized public key) and hash160
        let h160 = if compressed {
            pubkey
                .to_compressed_sec()
                .map_err(|e| format!("Failed to compress public key: {:?}", e))
                .and_then(|sec| {
                    hash160(&sec).map_err(|e| format!("Failed to hash public key: {:?}", e))
                })?
        } else {
            pubkey
                .to_uncompressed_sec()
                .map_err(|e| format!("Failed to uncompress public key: {:?}", e))
                .and_then(|sec| {
                    hash160(&sec).map_err(|e| format!("Failed to hash public key: {:?}", e))
                })?
        };

        // Determine the prefix and construct the address
        let prefix = if testnet {
            TESTNET_PREFIX
        } else {
            MAINNET_PREFIX
        };
        let mut result = vec![prefix];
        result.extend_from_slice(&h160);

        encode_base58check(&result).map_err(|e| format!("Failed to encode address: {:?}", e))
    }
}
