mod field_element; 
mod point;

#[cfg(test)]
mod field_element_test {
    use super::*;
    use field_element::FieldElement;
    use num_bigint::{BigInt, BigUint};
    use num_traits::{Num, Zero};

    #[test]
    fn test_000_print() {
        let num = BigUint::from_str_radix("2", 10).unwrap();
        let prime = BigUint::from_str_radix("7", 10).unwrap();
        let fe = FieldElement::new(num, prime);
        
         match fe {
            Ok(f) => println!("{}", f), // Prints "FieldElement_7(2)"
            Err(e) => panic!("Error: {}", e),
        }
    }
    
    #[test]
    fn test_001_equality_same_field(){
        let num = BigUint::from_str_radix("2", 10).unwrap();
        let prime = BigUint::from_str_radix("7", 10).unwrap();
        let fe1 = FieldElement::new(num.clone(), prime.clone()).unwrap();
        let fe2 = FieldElement::new(num.clone(), prime.clone()).unwrap();

        assert_eq!(&fe1 == &fe2, true);
    }
    
    #[test]
    fn test_002_inequality_same_field(){
        let num1 = BigUint::from_str_radix("2", 10).unwrap();
        let num2 = BigUint::from_str_radix("3", 10).unwrap();
        let prime = BigUint::from_str_radix("7", 10).unwrap();
        
        let fe1 = FieldElement::new(num1.clone(), prime.clone()).unwrap();
        let fe2 = FieldElement::new(num2.clone(), prime.clone()).unwrap();

        assert_eq!(&fe1 != &fe2, true);
    }
    
    #[test]
    fn test_003_inequality_different_field(){
        let num = BigUint::from_str_radix("2", 10).unwrap();
        let prime1 = BigUint::from_str_radix("7", 10).unwrap();
        let prime2 = BigUint::from_str_radix("13", 10).unwrap();

        let fe1 = FieldElement::new(num.clone(), prime1).unwrap();
        let fe2 = FieldElement::new(num.clone(), prime2).unwrap();
        assert_eq!(&fe1 == &fe2, false);
    }

    #[test]
    fn test_004_add_same_field() {  
        let n1 = BigUint::from_str_radix("2", 10).unwrap();
        let n2 = BigUint::from_str_radix("3", 10).unwrap();
        let res = BigUint::from_str_radix("5", 10).unwrap();
        let prime = BigUint::from_str_radix("7", 10).unwrap();
        
        let fe1 = FieldElement::new(n1, prime.clone()).unwrap();
        let fe2 = FieldElement::new(n2, prime.clone()).unwrap();
        
        let expected_result = FieldElement::new(res.clone(), prime).unwrap();
        assert_eq!(fe1 + fe2, expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_005_fail_add_different_field() {  
        let n1 = BigUint::from_str_radix("2", 10).unwrap();
        let n2 = BigUint::from_str_radix("3", 10).unwrap();
        let prime1 = BigUint::from_str_radix("6", 10).unwrap();
        let prime2 = BigUint::from_str_radix("7", 10).unwrap();
        
        let fe1 = FieldElement::new(n1, prime1).unwrap();
        let fe2 = FieldElement::new(n2, prime2).unwrap();
        let _ = fe1.clone() + fe2.clone();
    }
    
    #[test]
    fn test_006_sub_same_field() {  
        let n1 = BigUint::from_str_radix("2", 10).unwrap();
        let n2 = BigUint::from_str_radix("3", 10).unwrap();
        let res = BigUint::from_str_radix("6", 10).unwrap();
        let prime = BigUint::from_str_radix("7", 10).unwrap();
        
        let fe1 = FieldElement::new(n1, prime.clone()).unwrap();
        let fe2 = FieldElement::new(n2, prime.clone()).unwrap();
        
        let expected_result = FieldElement::new(res.clone(), prime).unwrap();
        assert_eq!(fe1 - fe2, expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_007_fail_sub_different_field() {  
        let n1 = BigUint::from_str_radix("2", 10).unwrap();
        let n2 = BigUint::from_str_radix("3", 10).unwrap();
        let prime1 = BigUint::from_str_radix("6", 10).unwrap();
        let prime2 = BigUint::from_str_radix("7", 10).unwrap();
        
        let fe1 = FieldElement::new(n1, prime1).unwrap();
        let fe2 = FieldElement::new(n2, prime2).unwrap();
        let _ = fe1.clone() - fe2.clone();
    }
    
    #[test]
    fn test_008_mul_same_field() {  
        let n1 = BigUint::from_str_radix("2", 10).unwrap();
        let n2 = BigUint::from_str_radix("3", 10).unwrap();
        let res = BigUint::from_str_radix("6", 10).unwrap();
        let prime = BigUint::from_str_radix("7", 10).unwrap();
        
        let fe1 = FieldElement::new(n1, prime.clone()).unwrap();
        let fe2 = FieldElement::new(n2, prime.clone()).unwrap();
        
        let expected_result = FieldElement::new(res.clone(), prime).unwrap();
        assert_eq!(fe1 * fe2, expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_009_fail_mul_different_field() {  
        let n1 = BigUint::from_str_radix("2", 10).unwrap();
        let n2 = BigUint::from_str_radix("4", 10).unwrap();
        let prime1 = BigUint::from_str_radix("6", 10).unwrap();
        let prime2 = BigUint::from_str_radix("7", 10).unwrap();
        
        let fe1 = FieldElement::new(n1, prime1).unwrap();
        let fe2 = FieldElement::new(n2, prime2).unwrap();
        let _ = fe1.clone() * fe2.clone();
    }

    #[test]
    fn test_010_pow() {
        let n1 = BigUint::from_str_radix("2", 10).unwrap();
        let exp = BigUint::from_str_radix("4", 10).unwrap();
        let prime = BigUint::from_str_radix("7", 10).unwrap();
        
        let fe1 = FieldElement::new(n1.clone(), prime.clone()).unwrap();
        let expected_result = FieldElement::new(n1.clone(), prime.clone()).unwrap();
        assert_eq!(fe1.pow(exp.clone().into()), expected_result);
    }
    
    #[test]
    fn test_011_negative_pow() {
        let n1 = BigUint::from_str_radix("17", 10).unwrap();
        let exp = BigInt::from(-3); // Using BigInt to handle negative exponent
        let res = BigUint::from_str_radix("29", 10).unwrap();
        let prime = BigUint::from_str_radix("31", 10).unwrap();
    
        let fe1 = FieldElement::new(n1, prime.clone()).unwrap();
        let expected_result = FieldElement::new(res, prime.clone()).unwrap();
        assert_eq!(fe1.pow(exp), expected_result);
    }

    
    #[test]
    fn test_012_div() {
        let n1 = BigUint::from_str_radix("2", 10).unwrap();
        let n2 = BigUint::from_str_radix("7", 10).unwrap();
        let prime = BigUint::from_str_radix("19", 10).unwrap();
        
        let fe1 = FieldElement::new(n1, prime.clone()).unwrap();
        let fe2 = FieldElement::new(n2, prime.clone()).unwrap();
        
        let res = BigUint::from_str_radix("3", 10).unwrap();
        let expected_result = FieldElement::new(res.clone(), prime).unwrap();
        
        assert_eq!(fe1 / fe2, expected_result);
    }
    
    #[test]
    #[should_panic]
    fn test_013_fail_div_different_field() {
        let n1 = BigUint::from_str_radix("2", 10).unwrap();
        let n2 = BigUint::from_str_radix("0", 10).unwrap();
        let prime = BigUint::from_str_radix("19", 10).unwrap();
        let fe1 = FieldElement::new(n1, prime.clone()).unwrap();
        let fe2 = FieldElement::new(n2, prime.clone()).unwrap();
        let _ = fe1.clone() / fe2.clone();
    }
    
    #[test]
    #[should_panic]
    fn test_014_fail_div_zero() {
        let n1 = BigUint::from_str_radix("2", 10).unwrap();
        let prime = BigUint::from_str_radix("19", 10).unwrap();
        let fe1 = FieldElement::new(n1, prime.clone()).unwrap();
        let fe2 = FieldElement::new(BigUint::zero(), prime.clone()).unwrap();
        let _ = fe1.clone() / fe2.clone();
    }
}

#[cfg(test)]
mod point_test {  
    use super::*;
    use point::Point;
    use field_element::FieldElement;
    use num_bigint::BigUint;
    use num_traits::Num;
    
    #[test]
    fn test_015_print() {
        let x = BigUint::from_str_radix("1", 10).unwrap();
        let y = BigUint::from_str_radix("3", 10).unwrap();
        let a = BigUint::from_str_radix("2", 10).unwrap();
        let b = BigUint::from_str_radix("6", 10).unwrap();
        let p = BigUint::from_str_radix("13", 10).unwrap();

        
        let fe_x = FieldElement::new(x, p.clone()).unwrap();
        let fe_y = FieldElement::new(y, p.clone()).unwrap();
        let fe_a = FieldElement::new(a, p.clone()).unwrap();
        let fe_b = FieldElement::new(b, p.clone()).unwrap();

        let point = Point::new(fe_x, fe_y, fe_a, fe_b);
        
         match point {
            Ok(f) => println!("{}", f),
            Err(e) => panic!("Error: {}", e),
        }
    }
    
    #[test]
    fn test_016_equality_same_point(){
        let x = BigUint::from_str_radix("1", 10).unwrap();
        let y = BigUint::from_str_radix("3", 10).unwrap();
        let a = BigUint::from_str_radix("2", 10).unwrap();
        let b = BigUint::from_str_radix("6", 10).unwrap();
        let p = BigUint::from_str_radix("13", 10).unwrap();

        
        let fe_x = FieldElement::new(x, p.clone()).unwrap();
        let fe_y = FieldElement::new(y, p.clone()).unwrap();
        let fe_a = FieldElement::new(a, p.clone()).unwrap();
        let fe_b = FieldElement::new(b, p.clone()).unwrap();

        let p1 = Point::new(fe_x.clone(), fe_y.clone(), fe_a.clone(), fe_b.clone());
        let p2 = Point::new(fe_x.clone(), fe_y.clone(), fe_a.clone(), fe_b.clone());

        assert_eq!(p1, p2)
    }
    
    
}
