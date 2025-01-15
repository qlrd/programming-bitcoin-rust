use field_element::FieldElement;
use num_bigint::BigInt;
use num_traits::Num;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_field_element() {
        let fe = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        );
        assert!(fe.is_ok());
    }

    #[test]
    fn test_create_field_element_fail_greater_than_prime() {
        let fe = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC30",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        );
        assert!(fe.is_err());
    }

    #[test]
    fn test_create_field_element_fail_equal_to_prime() {
        let fe = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        );
        assert!(fe.is_err());
    }

    #[test]
    fn test_equality_between_2_field_elements_in_same_field() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1, fe_2);
    }

    #[test]
    fn test_inequality_between_2_field_elements_in_same_field() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000010",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_ne!(fe_1, fe_2);
    }

    #[test]
    fn test_inequality_between_2_field_elements_not_in_same_field() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_ne!(fe_1, fe_2);
    }

    #[test]
    fn test_add_between_2_field_elements_in_same_field() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_expected = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000002",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1 + fe_2, fe_expected);
    }

    #[test]
    fn test_add_between_2_field_elements_in_same_field_extrem() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_expected = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000000",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1 + fe_2, fe_expected);
    }

    #[test]
    #[should_panic]
    fn test_add_between_2_field_elements_in_different_field() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC3F",
        )
        .unwrap();

        let _ = fe_1 + fe_2;
    }

    #[test]
    fn test_sub_between_2_field_elements_in_same_field() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_expected = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000000",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1 - fe_2, fe_expected);
    }

    #[test]
    fn test_sub_between_2_field_elements_in_same_field_extrem() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000000",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_expected = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1 - fe_2, fe_expected);
    }

    #[test]
    #[should_panic]
    fn test_sub_between_2_field_elements_in_different_field() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC3F",
        )
        .unwrap();

        let _ = fe_1 - fe_2;
    }

    #[test]
    fn test_mul_between_2_field_elements_in_same_field() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000002",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000002",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_expected = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000004",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1 * fe_2, fe_expected);
    }

    #[test]
    fn test_mul_between_2_field_elements_in_same_field_extrem() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000002",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_expected = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2D",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1 * fe_2, fe_expected);
    }

    #[test]
    #[should_panic]
    fn test_mul_between_2_field_elements_in_different_field() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC3F",
        )
        .unwrap();

        let _ = fe_1 * fe_2;
    }

    #[test]
    fn test_div_between_2_field_elements_in_same_field() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000002",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000002",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_expected = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1 / fe_2, fe_expected);
    }
    #[test]
    fn test_div_between_2_field_elements_in_same_field_2_by_extrem() {
        let fe_1 = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000002",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_expected = FieldElement::new(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7FFFFE17",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1 / fe_2, fe_expected);
    }

    #[test]
    fn test_div_between_2_field_elements_in_same_field_extrem_by_2() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000002",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_expected = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2D",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1 / fe_2, fe_expected);
    }

    #[test]
    #[should_panic]
    fn test_div_between_2_field_elements_in_different_field() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC3F",
        )
        .unwrap();

        let _ = fe_1 / fe_2;
    }

    #[test]
    #[should_panic]
    fn test_div_by_0_() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_2 = FieldElement::new(
            "000000000000000000000000000000000000000000000000000000000000000",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC3F",
        )
        .unwrap();

        let _ = fe_1 / fe_2;
    }

    #[test]
    fn test_pow() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000002",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let exponent = BigInt::from_str_radix("3", 16).unwrap();
        let fe_expected = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000008",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1.pow(&exponent), fe_expected);
    }

    #[test]
    fn test_pow_extreme() {
        let fe_1 = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let exponent = BigInt::from_str_radix("2", 16).unwrap();
        let fe_expected = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        assert_eq!(fe_1.pow(&exponent), fe_expected);
    }

    #[test]
    fn test_sqrt() {
        let fe_1 = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000004",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        let fe_expected = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000002",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();

        assert_eq!(fe_1.sqrt(), fe_expected);
    }

    #[test]
    fn test_sqrt_extreme() {
        let fe_1 = FieldElement::new(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        let fe_expected = FieldElement::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
        .unwrap();
        assert_eq!(fe_1.sqrt(), fe_expected);
    }
}
