use num_bigint::BigUint;
use programming_bitcoin_in_rust::primitives::key::Key;
use programming_bitcoin_in_rust::primitives::signature::Signature;

#[cfg(test)]
mod tests {
    use num_traits::Num;

    use super::*;

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

        let z = Key::sha256(message).unwrap();
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

        let z = Key::double_sha256(message).unwrap();
        let signature = key.sign(z).unwrap();

        assert_eq!(signature.r, r);
        assert_eq!(signature.s, s);
    }

    #[test]
    fn test_verify_from_sha256_message() {
        let prv = "0000000000000000000000000000000000000000000000000000000000000001";
        let key = Key::from_hexstr(prv).unwrap();
        let message = b"Hello, world";

        let z = Key::sha256(message).unwrap();
        let signature = key.sign(z).unwrap();

        assert!(key.verify(&z, &signature));
    }

    #[test]
    fn test_verify_from_double_sha256_message() {
        let prv = "0000000000000000000000000000000000000000000000000000000000000001";
        let key = Key::from_hexstr(prv).unwrap();
        let message = b"Hello, world";

        let z = Key::double_sha256(message).unwrap();
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
