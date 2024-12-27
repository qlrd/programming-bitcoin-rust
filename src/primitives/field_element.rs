/*
 * We want to represent each finite field element
 * in a field F_prime
 * See "Constructing a finite field in python"
 */
use num_bigint::{BigInt, BigUint};
use num_traits::{Num, One, Zero};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct FieldElement {
    pub num: BigUint,
    pub prime: BigUint,
}

/// This implementation represents a single finite field element.
impl FieldElement {
    #[allow(dead_code)]
    pub fn new(num: &str, prime: &str) -> Result<Self, String> {
        let bignum = BigUint::from_str_radix(num, 16).expect("Invalid number");

        let bigprime = BigUint::from_str_radix(prime, 16).expect("Invalid prime");

        match bignum.cmp(&bigprime) {
            Ordering::Greater => {
                let minus = bigprime - BigUint::one();
                Err(format!("{} isnt in the field [0..{})", num, minus))
            }
            Ordering::Equal => {
                let minus = bigprime - BigUint::one();
                Err(format!("{} isnt in the field [0..{})", num, minus))
            }
            Ordering::Less => Ok(Self {
                num: bignum,
                prime: bigprime,
            }),
        }
    }

    #[allow(dead_code)]
    fn wrap_exponent(&self, exponent: &BigInt) -> BigUint {
        let zero = BigInt::zero();
        let one = BigUint::one();

        match exponent.cmp(&zero) {
            Ordering::Less => {
                let pos_exp = (-exponent).to_biguint().unwrap();
                &self.prime - &one - &pos_exp
            }
            Ordering::Equal => exponent.to_biguint().unwrap(),
            Ordering::Greater => exponent.to_biguint().unwrap(),
        }
    }

    /// Repeatedly square the base and reduce it modulo prime at each step.
    /// Also multiply by base when the current exponent bit is 1.
    /// This approach works well with arbitrarily large exponents.
    #[allow(dead_code)]
    pub fn pow(&self, exponent: BigInt) -> Self {
        let exp = self.wrap_exponent(&exponent);

        // Continue with exponentiation by squaring
        let mut base = self.num.clone();
        let mut result = BigUint::one();

        let mut exp_copy = exp.clone();
        while exp_copy > BigUint::zero() {
            if &exp_copy % BigUint::from(2u32) == BigUint::one() {
                result = (&result * &base) % &self.prime;
            }
            base = (&base * &base) % &self.prime;
            exp_copy /= BigUint::from(2u32);
        }

        Self {
            num: result.clone(),
            prime: self.prime.clone(),
        }
    }
}

/// Implement Display trait to mimic  __repr__ in python
impl fmt::Display for FieldElement {
    /// When you implement Display, you’re defining how the type
    /// will be printed in a human-readable form.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FiniteElement_{}({})", self.prime, self.num)
    }
}

/// Implement PartialEq trait to mimic __eq__ in python
impl PartialEq for FieldElement {
    /// Check if two implementations of FieldElement are equal.
    /// This is only true when num and prime are equal
    /// In Rust, implementing the != operator directly is not
    /// required because Rust automatically provides !=
    /// when you implement the PartialEq trait
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

/// Implement Add trait to mimic __add__ in python
impl Add for FieldElement {
    type Output = Self;

    /// We have to ensure that the elements are from the same
    /// finite field and define it with the modulo operation,
    /// returning an instance of FiniteElement struct
    fn add(self, other: FieldElement) -> Self {
        match self.prime.cmp(&other.prime) {
            Ordering::Equal => {
                let bignum = (&self.num + &other.num) % &self.prime;
                Self {
                    num: bignum,
                    prime: self.prime.clone(),
                }
            }
            _ => panic!("Cannot add two numbers in different fields"),
        }
    }
}

/// Implement Add trait to mimic __sub__ in python
impl Sub for FieldElement {
    type Output = Self;

    /// We have to ensure that the elements are from the same
    /// finite field and define it with the modulo operation,
    /// returning an instance of FiniteElement struct
    fn sub(self, other: FieldElement) -> Self {
        match self.prime.cmp(&other.prime) {
            Ordering::Equal => {
                // wrap around by adding `self.prime` to avoid negative result
                match self.num.cmp(&other.num) {
                    Ordering::Less => Self {
                        num: &self.num + &self.prime - &other.num,
                        prime: self.prime.clone(),
                    },
                    Ordering::Equal => Self {
                        num: &self.num - &other.num,
                        prime: self.prime.clone(),
                    },
                    Ordering::Greater => Self {
                        num: &self.num - &other.num,
                        prime: self.prime.clone(),
                    },
                }
            }
            _ => panic!("Cannot subtract 2 numbers in different fields"),
        }
    }
}

/// Implement Mul trait to mimic __mul__ in python
impl Mul for FieldElement {
    type Output = Self;

    /// We have to ensure that the elements are from the same
    /// finite field and define it with the modulo operation,
    /// returning an instance of FiniteElement struct
    fn mul(self, other: FieldElement) -> Self {
        match self.prime.cmp(&other.prime) {
            Ordering::Equal => {
                let bignum = (&self.num * &other.num) % &self.prime;
                Self {
                    num: bignum,
                    prime: self.prime.clone(),
                }
            }
            _ => panic!("Cannot multiple 2 numbers in different fields"),
        }
    }
}

/// Implement Div trait to mimic __truediv__ in python
impl Div for FieldElement {
    type Output = Self;

    /// We have to ensure that the elements are from the same
    /// finite field and define it with the modulo operation,
    /// returning an instance of FiniteElement struct
    fn div(self, other: FieldElement) -> Self {
        match self.prime.cmp(&other.prime) {
            Ordering::Equal => {
                match other.num.cmp(&BigUint::zero()) {
                    Ordering::Equal => panic!("Cannot divide by zero in a finite field"),
                    _ => {
                        // Fermat's little theorem
                        let two = BigUint::from_str_radix("2", 16).unwrap();
                        let bigexp = &self.prime - &two;
                        let exp = other.pow(bigexp.into());
                        let num = (&self.num * &exp.num) % &self.prime;
                        Self {
                            num: num.clone(),
                            prime: self.prime.clone(),
                        }
                    }
                }
            }
            _ => panic!("Cannot divide 2 numbers in different fields"),
        }
    }
}

#[cfg(test)]
mod tests {

    use num_bigint::BigInt;
    use num_traits::Num;

    use super::FieldElement;

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

        assert_eq!(fe_1.pow(exponent), fe_expected);
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

        assert_eq!(fe_1.pow(exponent), fe_expected);
    }
}
