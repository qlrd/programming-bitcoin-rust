use field_element::FieldElement;
use hasher::{double_sha256, sha256};
use key::{Key, Signature};
use secp256k1::{Secp256k1Point, PRIME};

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::{FromPrimitive, Num};

    use super::*;

    #[test]
    fn test_to_public() {
        let prv: [u8; 32] = [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
        ];

        let public = Key::to_public(&prv).unwrap();
        let x = FieldElement::new(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            PRIME,
        )
        .unwrap();

        let y = FieldElement::new(
            "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            PRIME,
        )
        .unwrap();

        let p = Secp256k1Point::new(Some(x), Some(y)).unwrap();
        assert_eq!(public, p);
    }

    #[test]
    fn test_from_bytes_be() {
        let prv: [u8; 32] = [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
        ];

        assert!(Key::from_bytes_be(prv).is_ok());
    }

    #[test]
    fn test_from_hexstr() {
        let prv = "0000000000000000000000000000000000000000000000000000000000000001";
        assert!(Key::from_hexstr(prv).is_ok());
    }

    #[test]
    fn test_sha256() {
        let expected_sha = [
            74u8, 231u8, 195u8, 182u8, 172u8, 11u8, 239u8, 246u8, 113u8, 239u8, 168u8, 207u8, 87u8,
            56u8, 97u8, 81u8, 192u8, 110u8, 88u8, 202u8, 83u8, 167u8, 141u8, 131u8, 243u8, 97u8,
            7u8, 49u8, 108u8, 236u8, 18u8, 95u8,
        ];
        let message = b"Hello, world";
        let sha = sha256(message).unwrap();
        assert_eq!(sha, expected_sha);
    }

    #[test]
    fn test_double_sha256() {
        let expected_sha = [
            217u8, 235u8, 182u8, 14u8, 223u8, 24u8, 253u8, 91u8, 188u8, 200u8, 240u8, 29u8, 219u8,
            19u8, 11u8, 65u8, 34u8, 247u8, 76u8, 202u8, 201u8, 161u8, 55u8, 76u8, 133u8, 179u8,
            16u8, 169u8, 160u8, 221u8, 221u8, 202u8,
        ];
        let message = b"Hello, world";
        let sha = double_sha256(message).unwrap();
        assert_eq!(sha, expected_sha);
    }

    #[test]
    fn test_deterministic_k_from_sha256_message() {
        let expected_k = [
            143u8, 199u8, 121u8, 85u8, 102u8, 199u8, 51u8, 79u8, 175u8, 233u8, 118u8, 36u8, 182u8,
            85u8, 226u8, 228u8, 34u8, 57u8, 136u8, 161u8, 80u8, 119u8, 50u8, 67u8, 247u8, 92u8,
            75u8, 93u8, 151u8, 116u8, 247u8, 64u8,
        ];

        let prv = "0000000000000000000000000000000000000000000000000000000000000001";
        let key = Key::from_hexstr(prv).unwrap();
        let message = b"Hello, world";

        let z = sha256(message).unwrap();
        let k = key.deterministic_k(&z).unwrap();
        assert_eq!(k, expected_k);
    }

    #[test]
    fn test_deterministic_k_from_double_sha256_message() {
        let expected_k = [
            161u8, 62u8, 173u8, 186u8, 41u8, 172u8, 129u8, 57u8, 11u8, 138u8, 196u8, 36u8, 51u8,
            152u8, 205u8, 7u8, 44u8, 40u8, 100u8, 94u8, 155u8, 121u8, 14u8, 35u8, 173u8, 70u8,
            66u8, 209u8, 72u8, 189u8, 173u8, 87u8,
        ];

        let prv = "0000000000000000000000000000000000000000000000000000000000000001";
        let key = Key::from_hexstr(prv).unwrap();
        let message = b"Hello, world";

        let z = double_sha256(message).unwrap();
        let k = key.deterministic_k(&z).unwrap();
        assert_eq!(k, expected_k);
    }

    #[test]
    fn test_serialized_from_prv_5001() {
        // 0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1
        let expected_sec = [
            3u8, 87u8, 164u8, 243u8, 104u8, 134u8, 138u8, 138u8, 109u8, 87u8, 41u8, 145u8, 228u8,
            132u8, 230u8, 100u8, 129u8, 15u8, 241u8, 76u8, 5u8, 192u8, 250u8, 2u8, 50u8, 117u8,
            37u8, 17u8, 81u8, 254u8, 14u8, 83u8, 209u8,
        ];

        let n = BigUint::from_u32(5001u32).unwrap().to_bytes_be();
        let mut prv = [0u8; 32];
        prv[(32 - n.len())..].copy_from_slice(&n);
        let key = Key::from_bytes_be(prv).unwrap();
        let sec = key.public.to_compressed_sec().unwrap();
        assert_eq!(sec, expected_sec);
    }

    #[test]
    fn test_serialized_from_prv_2019_pow_5() {
        // 02933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb8701
        let expected_sec = [
            2u8, 147u8, 62u8, 194u8, 210u8, 177u8, 17u8, 185u8, 39u8, 55u8, 236u8, 18u8, 241u8,
            197u8, 210u8, 15u8, 50u8, 51u8, 160u8, 173u8, 33u8, 205u8, 139u8, 54u8, 208u8, 188u8,
            167u8, 160u8, 207u8, 165u8, 203u8, 135u8, 1u8,
        ];

        let n = BigUint::from_u64(2019u64.pow(5)).unwrap().to_bytes_be();
        let mut prv = [0u8; 32];
        prv[(32 - n.len())..].copy_from_slice(&n);
        let key = Key::from_bytes_be(prv).unwrap();
        let sec = key.public.to_compressed_sec().unwrap();
        assert_eq!(sec, expected_sec);
    }

    #[test]
    fn test_serialized_from_prv_0xdeadbeef54321() {
        // 0296be5b1292f6c856b3c5654e886fc13511462059089cdf9c479623bfcbe77690
        let expected_sec = [
            2u8, 150u8, 190u8, 91u8, 18u8, 146u8, 246u8, 200u8, 86u8, 179u8, 197u8, 101u8, 78u8,
            136u8, 111u8, 193u8, 53u8, 17u8, 70u8, 32u8, 89u8, 8u8, 156u8, 223u8, 156u8, 71u8,
            150u8, 35u8, 191u8, 203u8, 231u8, 118u8, 144u8,
        ];

        let n = BigUint::from_str_radix("deadbeef54321", 16)
            .unwrap()
            .to_bytes_be();
        let mut prv = [0u8; 32];
        prv[(32 - n.len())..].copy_from_slice(&n);
        let key = Key::from_bytes_be(prv).unwrap();
        let sec = key.public.to_compressed_sec().unwrap();
        assert_eq!(sec, expected_sec);
    }

    #[test]
    fn test_signature_from_sha256_message() {
        let r = vec![
            40u8, 107u8, 87u8, 112u8, 240u8, 25u8, 6u8, 39u8, 181u8, 83u8, 183u8, 154u8, 43u8,
            127u8, 127u8, 175u8, 52u8, 105u8, 108u8, 205u8, 46u8, 240u8, 85u8, 137u8, 56u8, 234u8,
            129u8, 129u8, 191u8, 7u8, 127u8, 237u8,
        ];

        let s = vec![
            125u8, 60u8, 106u8, 138u8, 65u8, 176u8, 36u8, 151u8, 84u8, 44u8, 215u8, 70u8, 155u8,
            79u8, 28u8, 34u8, 140u8, 221u8, 124u8, 68u8, 48u8, 11u8, 130u8, 76u8, 114u8, 22u8,
            42u8, 8u8, 251u8, 16u8, 30u8, 111u8,
        ];

        let prv = "0000000000000000000000000000000000000000000000000000000000000001";
        let key = Key::from_hexstr(prv).unwrap();
        let message = b"Hello, world";

        let z = sha256(message).unwrap();
        let signature = key.sign(z).unwrap();

        assert_eq!(signature.r, r);
        assert_eq!(signature.s, s);
    }

    #[test]
    fn test_signature_from_double_sha256_message() {
        let r = vec![
            195u8, 43u8, 42u8, 42u8, 128u8, 218u8, 116u8, 116u8, 27u8, 233u8, 62u8, 132u8, 250u8,
            197u8, 16u8, 101u8, 227u8, 218u8, 223u8, 189u8, 130u8, 76u8, 81u8, 168u8, 183u8, 71u8,
            183u8, 80u8, 155u8, 52u8, 151u8, 190u8,
        ];
        let s = vec![
            11u8, 240u8, 58u8, 115u8, 126u8, 155u8, 90u8, 101u8, 173u8, 12u8, 41u8, 127u8, 168u8,
            74u8, 169u8, 124u8, 150u8, 196u8, 19u8, 52u8, 13u8, 50u8, 221u8, 71u8, 112u8, 76u8,
            99u8, 217u8, 69u8, 172u8, 217u8, 184u8,
        ];
        let prv = "0000000000000000000000000000000000000000000000000000000000000001";
        let key = Key::from_hexstr(prv).unwrap();
        let message = b"Hello, world";

        let z = double_sha256(message).unwrap();
        let signature = key.sign(z).unwrap();

        assert_eq!(signature.r, r);
        assert_eq!(signature.s, s);
    }

    #[test]
    fn test_verify_from_sha256_message() {
        let prv = "0000000000000000000000000000000000000000000000000000000000000001";
        let key = Key::from_hexstr(prv).unwrap();
        let message = b"Hello, world";

        let z = sha256(message).unwrap();
        let signature = key.sign(z).unwrap();

        assert!(key.verify(&z, &signature));
    }

    #[test]
    fn test_verify_from_double_sha256_message() {
        let prv = "0000000000000000000000000000000000000000000000000000000000000001";
        let key = Key::from_hexstr(prv).unwrap();
        let message = b"Hello, world";

        let z = double_sha256(message).unwrap();
        let signature = key.sign(z).unwrap();

        assert!(key.verify(&z, &signature));
    }

    #[test]
    fn test_der() {
        let r = BigUint::from_str_radix(
            "37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            16,
        )
        .unwrap();

        let s = BigUint::from_str_radix(
            "8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            16,
        )
        .unwrap();

        let expected_der = vec![
            48u8, 69u8, 2u8, 32u8, 55u8, 32u8, 106u8, 6u8, 16u8, 153u8, 92u8, 88u8, 7u8, 73u8,
            153u8, 203u8, 151u8, 103u8, 184u8, 122u8, 244u8, 196u8, 151u8, 141u8, 182u8, 140u8,
            6u8, 232u8, 230u8, 232u8, 29u8, 40u8, 32u8, 71u8, 167u8, 198u8, 2u8, 33u8, 0u8, 140u8,
            166u8, 55u8, 89u8, 193u8, 21u8, 126u8, 190u8, 174u8, 192u8, 208u8, 60u8, 236u8, 202u8,
            17u8, 159u8, 201u8, 167u8, 91u8, 248u8, 230u8, 208u8, 250u8, 101u8, 200u8, 65u8, 200u8,
            226u8, 115u8, 140u8, 218u8, 236u8,
        ];

        let signature = Signature::from_biguint(r, s).unwrap();
        let der = signature.der().unwrap();
        assert_eq!(der, expected_der);
    }
}
