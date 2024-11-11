mod field_element; 

#[cfg(test)]
mod field_element_test {
    use super::*;
    use field_element::FieldElement; 

    #[test]
    fn test_000_print() {
        match FieldElement::new(2, 7) {
            Ok(fe) => println!("{}", fe), // Prints "FieldElement_7(2)"
            Err(e) => panic!("Error: {}", e),
        }
    }
    
    #[test]
    fn test_001_equality_same_field(){
        let fe1 = FieldElement::new(2, 7).unwrap();
        let fe2 = FieldElement::new(2, 7).unwrap();

        assert_eq!(&fe1 == &fe2, true);
    }
    
    #[test]
    fn test_002_inequality_same_field(){
        let fe1 = FieldElement::new(2, 7).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();

        assert_eq!(&fe1 != &fe2, true);
    }
    
    #[test]
    fn test_003_inequality_different_field(){
        let fe1 = FieldElement::new(2, 6).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        assert_eq!(&fe1 == &fe2, false);
    }

    #[test]
    fn test_004_add_same_field() {  
        let fe1 = FieldElement::new(2, 7).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        
        let expected_result = FieldElement::new(5, 7).unwrap();
        assert_eq!(fe1 + fe2, expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_005_fail_add_different_field() {  
        let fe1 = FieldElement::new(2, 6).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        let _ = fe1.clone() + fe2.clone();
    }
    
    #[test]
    fn test_006_sub_same_field() {  
        let fe1 = FieldElement::new(2, 7).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        
        let expected_result = FieldElement::new(6, 7).unwrap();
        assert_eq!(fe1 - fe2, expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_007_fail_sub_different_field() {  
        let fe1 = FieldElement::new(2, 6).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        let _ = fe1.clone() - fe2.clone();
    }
    
    #[test]
    fn test_008_mul_same_field() {  
        let fe1 = FieldElement::new(2, 7).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        
        let expected_result = FieldElement::new(6, 7).unwrap();
        assert_eq!(fe1 * fe2, expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_009_fail_mul_different_field() {  
        let fe1 = FieldElement::new(2, 6).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        let _ = fe1.clone() * fe2.clone();
    }

    #[test]
    fn test_010_pow() {
        let fe1 = FieldElement::new(2, 7).unwrap();
        let expected_result = FieldElement::new(2, 7).unwrap();
        assert_eq!(fe1.pow(4), expected_result);
    }
    
    #[test]
    fn test_011_negative_pow() {
        let fe1 = FieldElement::new(17, 31).unwrap();
        let expected_result = FieldElement::new(29, 31).unwrap();
        assert_eq!(fe1.pow(-3), expected_result);
    }

    
    #[test]
    fn test_012_div() {
        let fe1 = FieldElement::new(2, 19).unwrap();
        let fe2 = FieldElement::new(7, 19).unwrap();
        let expected_result = FieldElement::new(3, 19).unwrap();
        assert_eq!(fe1 / fe2, expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_013_fail_div_different_field() {
        let fe1 = FieldElement::new(2, 18).unwrap();
        let fe2 = FieldElement::new(7, 19).unwrap();
        let _ = fe1.clone() / fe2.clone();
    }
    
    #[test]
    #[should_panic]
    fn test_014_fail_div_zero() {
        let fe1 = FieldElement::new(2, 18).unwrap();
        let fe2 = FieldElement::new(0, 19).unwrap();
        let _ = fe1.clone() / fe2.clone();
    }
}
