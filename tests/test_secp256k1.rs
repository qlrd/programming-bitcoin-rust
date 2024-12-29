use programming_bitcoin_in_rust::primitives::field_element::FieldElement;
use programming_bitcoin_in_rust::primitives::secp256k1::Secp256k1Point;

#[cfg(test)]
mod tests {

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
}
