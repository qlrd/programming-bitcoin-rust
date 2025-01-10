use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Base58 {}

const ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

impl Base58 {
    /// Decode a base58 string into an vector of bytes
    pub fn decode(base58: &str) -> Result<Vec<u8>, String> {
        // Step 1: Decode Base58 string to a big integer
        let mut num = BigUint::zero();
        let base = BigUint::from(58u32);

        for char in base58.chars() {
            let char_index = ALPHABET
                .find(char)
                .ok_or_else(|| format!("Invalid character '{}' in Base58 string", char))?;
            num = num * &base + BigUint::from(char_index as u32);
        }

        // Step 2: Convert the integer to bytes
        let byte_array = num.to_bytes_be();

        // Step 3: Add leading zero bytes for each '1' in the Base58 string
        let leading_zeros = base58.chars().take_while(|&c| c == '1').count();
        let mut full_byte_array = vec![0u8; leading_zeros];
        full_byte_array.extend_from_slice(&byte_array);

        // Step 4: Extract and verify checksum
        if full_byte_array.len() < 4 {
            return Err("Invalid Base58 string: too short to contain a checksum".to_string());
        }
        let (data, checksum) = full_byte_array.split_at(full_byte_array.len() - 4);

        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash_once = hasher.finalize_reset();

        hasher.update(hash_once);
        let hash_twice = hasher.finalize();

        if checksum != &hash_twice[..4] {
            return Err(format!(
                "Invalid checksum '{:x?}' for string '{}'",
                checksum, base58
            ));
        }

        // Step 5: Return decoded data
        Ok(data.to_vec())
    }

    /// Encode bytes to base58 string
    pub fn encode(bytes: Vec<u8>) -> Result<String, String> {
        let base = BigUint::from(58u32);

        // Count leading zero bytes
        let mut count = 0;
        for &b in &bytes {
            if b == 0u8 {
                count += 1;
            } else {
                break;
            }
        }

        // Convert the bytes to a BigUint
        let mut num = BigUint::from_bytes_be(&bytes);

        // Encode into Base58 string
        let mut data = String::new();
        while num > BigUint::zero() {
            let rem = (&num % &base)
                .to_u32()
                .ok_or("Failed to convert BigUint to u32")? as usize;
            num /= &base;
            data.insert(0, ALPHABET.chars().nth(rem).unwrap());
        }

        // Add Base58 '1's for each leading zero byte
        let prefix = "1".repeat(count);

        // Combine the prefix and result
        let result = format!("{}{}", prefix, data);

        Ok(result)
    }
}
