mod field_element; 
use field_element::FieldElement; 

#[cfg(test)]
mod tests {
    use super::*;

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
        
        let wanted_result = fe1.clone() + fe2.clone();
        let expected_result = FieldElement::new(5, 7).unwrap();
        assert_eq!(&wanted_result, &expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_fail_add_different_field() {  
        let fe1 = FieldElement::new(2, 6).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        fe1.clone() + fe2.clone();
    }
    
    #[test]
    fn test_sub_same_field() {  
        let fe1 = FieldElement::new(2, 7).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        
        let wanted_result = fe1.clone() - fe2.clone();
        let expected_result = FieldElement::new(6, 7).unwrap();
        assert_eq!(&wanted_result, &expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_fail_sub_different_field() {  
        let fe1 = FieldElement::new(2, 6).unwrap();
        let fe2 = FieldElement::new(3, 7).unwrap();
        fe1.clone() - fe2.clone();
    }
}
