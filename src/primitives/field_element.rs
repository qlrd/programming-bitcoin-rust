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
    pub fn pow(&self, exponent: &BigInt) -> Self {
        let exp = self.wrap_exponent(exponent);

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

    pub fn sqrt(&self) -> Self {
        let one = BigUint::one();
        let four = BigUint::from(4u32);
        let exp = (&self.prime + &one) / &four;
        let res = self.num.modpow(&exp, &self.prime);
        FieldElement {
            num: res,
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

/// Implement Add trait to mimic __add__ in python (for references)
impl<'b> Add<&'b FieldElement> for &FieldElement {
    type Output = FieldElement;

    /// Modular addition for references
    fn add(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add elements from different fields");
        }

        let result = (&self.num + &other.num) % &self.prime;

        FieldElement {
            num: result,
            prime: self.prime.clone(),
        }
    }
}

/// Implement Sub trait to mimic __sub__ in python
impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: FieldElement) -> Self {
        if self.prime != other.prime {
            panic!("Cannot subtract numbers from different fields");
        }

        let result = if self.num < other.num {
            // Wrap around if b > a
            (&self.num + &self.prime - &other.num) % &self.prime
        } else {
            (&self.num - &other.num) % &self.prime
        };

        Self {
            num: result,
            prime: self.prime.clone(),
        }
    }
}

/// Implement Sub trait to mimic __sub__ in python (for references)
impl<'b> Sub<&'b FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn sub(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot subtract elements from different fields");
        }

        let result = if self.num < other.num {
            (&self.num + &self.prime - &other.num) % &self.prime
        } else {
            (&self.num - &other.num) % &self.prime
        };

        FieldElement {
            num: result,
            prime: self.prime.clone(),
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

/// Implement Mul trait to mimic __mul__ in python (for references)
impl<'b> Mul<&'b FieldElement> for &FieldElement {
    type Output = FieldElement;

    /// Modular multiplication for references
    fn mul(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot multiply elements from different fields");
        }

        let bignum = (&self.num * &other.num) % &self.prime;

        FieldElement {
            num: bignum,
            prime: self.prime.clone(),
        }
    }
}

/// Implement Div trait to mimic __truediv__ in python
impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: FieldElement) -> Self {
        if self.prime != other.prime {
            panic!("Cannot divide numbers from different fields");
        }
        if other.num.is_zero() {
            panic!("Cannot divide by zero in a finite field");
        }

        // Compute modular inverse of `other.num` using Extended Euclidean Algorithm
        let inv = other
            .num
            .modpow(&(self.prime.clone() - BigUint::from(2u32)), &self.prime);
        let result = (&self.num * inv) % &self.prime;

        Self {
            num: result,
            prime: self.prime.clone(),
        }
    }
}

/// Implement Div trait to mimic __truediv__ in python (for references)
impl<'b> Div<&'b FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn div(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot divide elements from different fields");
        }

        if other.num.is_zero() {
            panic!("Cannot divide by zero in a finite field");
        }

        // Compute modular inverse of `other.num`
        let inv = other
            .num
            .modpow(&(self.prime.clone() - BigUint::from(2u32)), &self.prime);

        // Perform modular multiplication
        let result = (&self.num * &inv) % &self.prime;

        FieldElement {
            num: result,
            prime: self.prime.clone(),
        }
    }
}
