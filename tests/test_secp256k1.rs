use programming_bitcoin_in_rust::primitives::field_element::FieldElement;
use programming_bitcoin_in_rust::primitives::secp256k1::{Secp256k1, Secp256k1Point, ORDER, PRIME};

#[cfg(test)]
mod tests {

    use num_bigint::BigUint;

    use super::*;

    #[test]
    fn test_new_infinity() {
        let p = Secp256k1Point::new(None, None);
        assert!(p.is_ok());
    }

    #[test]
    fn test_new_infinity_fail() {
        let y = FieldElement::new(
            "4218F20AE6C646B363DB68605822FB14264CA8D2587FDD6FBC750D587E76A7EE",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let p = Secp256k1Point::new(None, Some(y));
        assert!(p.is_err());
    }

    #[test]
    fn test_new_infinity_y_fail() {
        let x = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let p = Secp256k1Point::new(Some(x), None);
        assert!(p.is_err());
    }

    #[test]
    fn test_new_fail() {
        let x = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let y = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let p = Secp256k1Point::new(Some(x), Some(y));
        assert!(p.is_err());
    }

    #[test]
    fn test_new() {
        let x1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let y1 = FieldElement::new(
            "4218F20AE6C646B363DB68605822FB14264CA8D2587FDD6FBC750D587E76A7EE",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let p1 = Secp256k1Point::new(Some(x1), Some(y1));
        assert!(p1.is_ok());
    }

    #[test]
    fn test_add_two_infinity() {
        let p1 = Secp256k1Point::new(None, None).unwrap();
        let p2 = Secp256k1Point::new(None, None).unwrap();

        assert_eq!(p1 + p2, Secp256k1Point::new(None, None).unwrap())
    }

    #[test]
    fn test_add_p1_infinity() {
        let p1 = Secp256k1Point::new(None, None).unwrap();

        let x2 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let y2 = FieldElement::new(
            "4218F20AE6C646B363DB68605822FB14264CA8D2587FDD6FBC750D587E76A7EE",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let p2 = Secp256k1Point::new(Some(x2), Some(y2)).unwrap();

        assert_eq!(p1 + p2.clone(), p2);
    }

    #[test]
    fn test_add_p2_infinity() {
        let x1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let y1 = FieldElement::new(
            "4218F20AE6C646B363DB68605822FB14264CA8D2587FDD6FBC750D587E76A7EE",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let p1 = Secp256k1Point::new(Some(x1), Some(y1)).unwrap();
        let p2 = Secp256k1Point::new(None, None).unwrap();

        assert_eq!(p1.clone() + p2, p1);
    }

    #[test]
    fn test_add_different_x() {
        let x1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let y1 = FieldElement::new(
            "4218F20AE6C646B363DB68605822FB14264CA8D2587FDD6FBC750D587E76A7EE",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let x2 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000002",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let y2 = FieldElement::new(
            "990418D84D45F61F60A56728F5A10317BDB3A05BDA4425E3AEE079F8A847A8D1",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let x3 = FieldElement::new(
            "F23A2D865C24C99CC9E7B99BD907FB93EBD6CCCE106BCCCB0082ACF8315E67BE",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let y3 = FieldElement::new(
            "791DFC78B49C9B5882867776F18BA7883ED0BAE1C0A856D26D41D38FB47345B4",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let p1 = Secp256k1Point::new(Some(x1), Some(y1)).unwrap();
        let p2 = Secp256k1Point::new(Some(x2), Some(y2)).unwrap();
        let p3 = Secp256k1Point::new(Some(x3), Some(y3)).unwrap();

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_add_equal_x() {
        let x1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let y1 = FieldElement::new(
            "4218F20AE6C646B363DB68605822FB14264CA8D2587FDD6FBC750D587E76A7EE",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let x3 = FieldElement::new(
            "C7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF37FFFD03",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let y3 = FieldElement::new(
            "4298C557A7DDCC570E8BF054C4CAD9E99F396B3CE19D50F1B91C9DF4BB00D333",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let p1 = Secp256k1Point::new(Some(x1), Some(y1)).unwrap();
        let p2 = p1.clone();
        let p3 = Secp256k1Point::new(Some(x3), Some(y3)).unwrap();

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_generator_as_point() {
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

        let g = Secp256k1::Generator.as_point();

        assert_eq!(g, p);
    }

    #[test]
    fn test_ininity_as_point() {
        let p = Secp256k1Point::new(None, None).unwrap();
        let i = Secp256k1::Infinity.as_point();
        assert_eq!(i, p);
    }

    #[test]
    fn test_order_mul_generator_is_infinity() {
        let g = Secp256k1::Generator.as_point();
        let o = Secp256k1::Order.as_biguint();
        let i = Secp256k1::Infinity.as_point();

        assert_eq!(&g * &o, i);
        assert_eq!(o * g, i);
    }

    #[test]
    fn test_mul_double_g() {
        let g = Secp256k1::Generator.as_point();
        let x = FieldElement::new(
            "C6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE5",
            PRIME,
        )
        .unwrap();
        let y = FieldElement::new(
            "1AE168FEA63DC339A3C58419466CEAEEF7F632653266D0E1236431A950CFE52A",
            PRIME,
        )
        .unwrap();
        let p = Secp256k1Point::new(Some(x), Some(y)).unwrap();
        let two = BigUint::from(2u32);
        assert_eq!(two * g, p);
    }

    #[test]
    fn test_mul_triple_g() {
        let g = Secp256k1::Generator.as_point();
        let x = FieldElement::new(
            "F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9",
            PRIME,
        )
        .unwrap();
        let y = FieldElement::new(
            "388F7B0F632DE8140FE337E62A37F3566500A99934C2231B6CB9FD7584B8E672",
            PRIME,
        )
        .unwrap();
        let p = Secp256k1Point::new(Some(x), Some(y)).unwrap();
        let three = BigUint::from(3u32);
        assert_eq!(three * g, p);
    }
}
