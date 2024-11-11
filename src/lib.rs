mod field_element; 

#[cfg(test)]
mod tests {
    use super::*;
    use field_element::FieldElement; 

    #[test]
    fn test_print() {
        match FieldElement::new(2, 7) {
            Ok(fe) => println!("{}", fe), // Prints "FieldElement_7(2)"
            Err(e) => panic!("Error: {}", e),
        }
    }
    
    #[test]
    fn test_equality_same_field(){
        let fe1 = FieldElement::new(2, 7).unwrap();
        let fe2 = FieldElement::new(2, 7).unwrap();

        assert_eq!(&fe1 == &fe2, true);
    }
    
    #[test]
    fn test_inequality_same_field(){
        let fe1 = FieldElement::new(2, 7).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();

        assert_eq!(&fe1 != &fe2, true);
    }
    
    #[test]
    fn test_inequality_different_field(){
        let fe1 = FieldElement::new(2, 6).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        assert_eq!(&fe1 == &fe2, false);
    }

    #[test]
    fn test_add_same_field() {  
        let fe1 = FieldElement::new(2, 7).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        
        let expected_result = FieldElement::new(5, 7).unwrap();
        assert_eq!(fe1 + fe2, expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_fail_add_different_field() {  
        let fe1 = FieldElement::new(2, 6).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        let _ = fe1.clone() + fe2.clone();
    }
    
    #[test]
    fn test_sub_same_field() {  
        let fe1 = FieldElement::new(2, 7).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        
        let expected_result = FieldElement::new(6, 7).unwrap();
        assert_eq!(fe1 - fe2, expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_fail_sub_different_field() {  
        let fe1 = FieldElement::new(2, 6).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        let _ = fe1.clone() - fe2.clone();
    }
    
    #[test]
    fn test_mul_same_field() {  
        let fe1 = FieldElement::new(2, 7).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        
        let expected_result = FieldElement::new(6, 7).unwrap();
        assert_eq!(fe1 * fe2, expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_fail_mul_different_field() {  
        let fe1 = FieldElement::new(2, 6).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        let _ = fe1.clone() * fe2.clone();
    }

    #[test]
    fn test_pow() {
        let fe1 = FieldElement::new(2, 7).unwrap();
        let expected_result = FieldElement::new(2, 7).unwrap();
        assert_eq!(fe1.pow(4), expected_result);
    }
}
