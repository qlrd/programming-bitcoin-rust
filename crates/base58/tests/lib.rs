use base58::{decode_base58, encode_base58, encode_base58check};
use num_bigint::BigUint;
use num_traits::Num;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode() {
        let prv = "tprv8ZgxMBicQKsPf42QMo57FTLmVCgwZfQeXnWcTG2s45A47SKWqekmQZnFy33h8XUEEAnyzVgoiakvREbekg5ZCZmDg4jDhwFm5miSwWg8w67";

        let expected = [
            4u8, 53u8, 131u8, 148u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 218u8, 244u8,
            13u8, 211u8, 41u8, 247u8, 145u8, 56u8, 166u8, 184u8, 92u8, 167u8, 163u8, 37u8, 113u8,
            112u8, 101u8, 213u8, 213u8, 254u8, 45u8, 109u8, 205u8, 209u8, 61u8, 237u8, 240u8,
            137u8, 58u8, 238u8, 227u8, 157u8, 0u8, 169u8, 186u8, 198u8, 181u8, 78u8, 30u8, 232u8,
            104u8, 108u8, 158u8, 136u8, 85u8, 145u8, 114u8, 12u8, 107u8, 72u8, 153u8, 206u8, 241u8,
            114u8, 156u8, 72u8, 92u8, 166u8, 126u8, 65u8, 72u8, 237u8, 236u8, 248u8, 185u8,
        ];

        let result = decode_base58(prv).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_encode() {
        // Programming bitcoin chapter 4 exercise 4
        let hexs = [
            "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
            "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
            "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
        ];

        let expected = [
            "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6",
            "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd",
            "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7",
        ];

        for i in 0..hexs.len() {
            let num = BigUint::from_str_radix(hexs[i], 16).unwrap();
            let bytes = num.to_bytes_be();
            let result = encode_base58(bytes).unwrap();
            assert_eq!(result, expected[i]);
        }
    }

    #[test]
    fn test_encode_with_checksum() {
        // Programming bitcoin chapter 4 exercise 4
        let hexs = [
            "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
            "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
            "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
        ];

        let expected = [
            "wdA2ffYs5cudrdkhFm5Ym94AuLvavacapuDBL2CAcvqXHcM56",
            "Qwj1mwXNifQmo5VV2s587usAy4QRUviQsBxoe4EJXyb5CAhV",
            "2WhRyzK3iKFveq4hvQ3VR9uau26t6qZCMhADPAVMeMR6S5dV2q",
        ];

        for i in 0..hexs.len() {
            let num = BigUint::from_str_radix(hexs[i], 16).unwrap();
            let bytes = num.to_bytes_be();
            let result = encode_base58check(&bytes).unwrap();
            assert_eq!(result, expected[i]);
        }
    }
}
