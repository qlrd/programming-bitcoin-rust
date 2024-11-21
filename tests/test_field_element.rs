use programming_bitcoin_in_rust::primitives::field_element;

#[cfg(test)]
mod field_element_test {

    use super::*;
    use field_element::FieldElement;
    use num_bigint::{BigInt, BigUint};
    use num_traits::Num;

    #[test]
    fn test_new() {
        let fe = FieldElement::new("2", "7").unwrap();
        let two = BigUint::from_str_radix("2", 16).unwrap();
        let seven = BigUint::from_str_radix("7", 16).unwrap();
        assert_eq!(fe.num, two);
        assert_eq!(fe.prime, seven);
    }

    #[test]
    #[should_panic(expected = "7 isnt in the field [0..6)")]
    fn test_new_fail_equal_to_prime() {
        let _ = FieldElement::new("7", "7").unwrap();
    }

    #[test]
    #[should_panic(expected = "9 isnt in the field [0..6)")]
    fn test_new_fail_greater_than_prime() {
        let _ = FieldElement::new("9", "7").unwrap();
    }

    #[test]
    fn test_equality_same_field() {
        let fe1 = FieldElement::new("2", "7").unwrap();
        let fe2 = FieldElement::new("2", "7").unwrap();
        assert_eq!(fe1, fe2);
    }

    #[test]
    fn test_inequality_same_field() {
        let fe1 = FieldElement::new("2", "7").unwrap();
        let fe2 = FieldElement::new("3", "7").unwrap();

        assert_ne!(fe1, fe2);
    }

    #[test]
    fn test_inequality_different_field() {
        let fe1 = FieldElement::new("2", "7").unwrap();
        let fe2 = FieldElement::new("2", "13").unwrap();
        assert_ne!(&fe1, &fe2);
    }

    #[test]
    fn test_add() {
        let fe1 = FieldElement::new("2", "7").unwrap();
        let fe2 = FieldElement::new("6", "7").unwrap();
        let fe_res = FieldElement::new("1", "7").unwrap();
        assert_eq!(fe1 + fe2, fe_res);
    }

    #[test]
    #[should_panic]
    fn test_add_fail_different_field() {
        let fe1 = FieldElement::new("2", "7").unwrap();
        let fe2 = FieldElement::new("2", "6").unwrap();
        let _ = fe1.clone() + fe2.clone();
    }

    #[test]
    fn test_sub() {
        let fe1 = FieldElement::new("2", "7").unwrap();
        let fe2 = FieldElement::new("6", "7").unwrap();
        let fe_res = FieldElement::new("3", "7").unwrap();
        assert_eq!(fe1 - fe2, fe_res);
    }

    #[test]
    #[should_panic(expected = "Cannot subtract 2 numbers in different fields")]
    fn test_sub_fail_different_field() {
        let fe1 = FieldElement::new("2", "7").unwrap();
        let fe2 = FieldElement::new("2", "6").unwrap();
        let _ = fe1 - fe2;
    }

    #[test]
    fn test_mul() {
        let fe1 = FieldElement::new("2", "7").unwrap();
        let fe2 = FieldElement::new("4", "7").unwrap();
        let fe_res = FieldElement::new("1", "7").unwrap();
        assert_eq!(fe1 * fe2, fe_res);
    }

    #[test]
    #[should_panic(expected = "Cannot multiple 2 numbers in different fields")]
    fn test_mul_fail_different_field() {
        let fe1 = FieldElement::new("2", "6").unwrap();
        let fe2 = FieldElement::new("2", "7").unwrap();
        let _ = fe1.clone() * fe2.clone();
    }

    #[test]
    fn test_pow() {
        let fe = FieldElement::new("2", "7").unwrap();
        let b = BigInt::from_str_radix("2", 16).unwrap();

        let res = FieldElement::new("4", "7").unwrap();
        assert_eq!(fe.pow(b), res);
    }

    #[test]
    fn test_div() {
        let fe1 = FieldElement::new("2", "7").unwrap();
        let fe2 = FieldElement::new("6", "7").unwrap();
        let res = FieldElement::new("5", "7").unwrap();

        assert_eq!(fe1 / fe2, res);
    }

    #[test]
    #[should_panic(expected = "Cannot divide 2 numbers in different fields")]
    fn test_div_fail_different_field() {
        let fe1 = FieldElement::new("2", "7").unwrap();
        let fe2 = FieldElement::new("2", "13").unwrap();
        let _ = fe1.clone() / fe2.clone();
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero in a finite field")]
    fn test_div_fail_zero() {
        let fe1 = FieldElement::new("2", "7").unwrap();
        let fe2 = FieldElement::new("0", "7").unwrap();
        let _ = fe1.clone() / fe2.clone();
    }
}
