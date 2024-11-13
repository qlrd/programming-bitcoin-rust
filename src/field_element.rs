/*
 * We want to represent each finite field element
 * in a field F_prime
 * See "Constructing a finite field in python"
 */
use std::fmt;
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div};
use num_bigint::{BigInt, BigUint, ToBigUint};
use num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub struct FieldElement {
    pub num: BigUint,
    pub prime: BigUint
}

/*
 * This implementation represents a single finite field element.
 */
impl FieldElement {
    
    /*
     * We first check that num is between 0 and prime-1 inclusive.
     * If not, we get an invalid FieldElement and we return an Err.
     * (an inapropriate value)
     * 
     * @param num<u64>: the number to be represented
     * @param prime<u64>: the finite field's order
     */
    #[allow(dead_code)]
    pub fn new(num: BigUint, prime: BigUint) -> Result<Self, String> {

        // Since we defined num as a U256 type, it's useless
        // to compare if num < 0
        match num.cmp(&num) {
            Ordering::Less => {
                return Err(format!("{} not in field [0..{})", num, &prime - BigUint::one()))
            },
            Ordering::Equal => return Ok(Self {num, prime}),
            Ordering::Greater => return Ok(Self {num, prime}),
        }
    }

    #[allow(dead_code)]
    fn wrap_exponent(&self, exponent: &BigInt) -> BigUint {
        match exponent.cmp(&BigInt::zero()) {
            Ordering::Less => {
                let pos_exp = (-exponent).to_biguint().unwrap();
                return &self.prime - BigUint::one() - pos_exp
            },
            Ordering::Equal => return exponent.to_biguint().unwrap(),
            Ordering::Greater => return exponent.to_biguint().unwrap(),
        }
    }
    
    /*
     * Repeatedly square the base and reduce it modulo prime at each step.
     * Also multiply by base when the current exponent bit is 1. 
     * This approach works well with arbitrarily large exponents.
     */
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
            exp_copy = exp_copy / BigUint::from(2u32);
        } 
        
        Self { num: result, prime: self.prime.clone() }
    }
}

// Implement Display trait to mimic  __repr__ in python
impl fmt::Display for FieldElement {

    /*
     * When you implement Display, you’re defining how the type 
     * will be printed in a human-readable form.
     * 
     * @param &self: An immutable reference to a FieldElement
     * @param &mut<fmt::Formatter>: A mutable reference to a Formatter
     * @returns fmt::Result<Ok(T), fmt::Error>
     */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FiniteElement_{}({})", self.prime, self.num)
    }
}

// Implement PartialEq trait to mimic __eq__ in python
impl PartialEq for FieldElement {

    /*
     * Check if two implementations of FieldElement are equal.
     * This is only true when num and prime are equal
     * In Rust, implementing the != operator directly is not
     * required because Rust automatically provides !=
     * when you implement the PartialEq trait
     *
     * @param &self: a immutable reference to a FieldElement
     * @param &Self: a immutable reference to another FieldElement
     * @returns bool 
     */
     fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
     }
}

// Implement Add trait to mimic __add__ in python
impl Add for FieldElement {

    type Output = Self;
    
    /* 
     * We have to ensure that the elements are from the same
     * finite field and define it with the modulo operation,
     * returning an instance of FiniteElement struct
     * 
     * @param self: a immutable FiniteElement
     * @param other: another immutable FieldElement
     * @returns FieldElement
     */
    fn add(self, other: FieldElement) -> Self {
        match self.prime.cmp(&other.prime) {
            Ordering::Equal => {
                let num = (self.num.clone() + other.num) % &self.prime;
                return Self { num: num, prime: self.prime.clone() }   
            },
            _ => panic!("Cannot add two numbers in different fields")
        }
    }
}

// Implement Add trait to mimic __sub__ in python
impl Sub for FieldElement {

    type Output = Self;
    
    /* 
     * We have to ensure that the elements are from the same
     * finite field and define it with the modulo operation,
     * returning an instance of FiniteElement struct
     * 
     * @param self: a immutable FiniteElement
     * @param other: another immutable FieldElement
     * @returns FieldElement
     */
    fn sub(self, other: FieldElement) -> Self {
        match self.prime.cmp(&other.prime) {
            Ordering::Equal => {
                // wrap around by adding `self.prime` to avoid negative result
                match self.num.cmp(&other.num) {
                    Ordering::Less => {
                        Self {
                            num: &self.num + &self.prime - &other.num,
                            prime: self.prime.clone()
                        }
                    },
                    Ordering::Equal => {
                        Self {
                            num: &self.num - &other.num,
                            prime: self.prime.clone()
                        }
                    },
                    Ordering::Greater => {
                        Self {
                            num: &self.num - &other.num,
                            prime: self.prime.clone()
                        }
                    }
                }
            },
            _ => panic!("Cannot subtract two numbers in different fields")
        }
    }
}

// Implement Mul trait to mimic __mul__ in python
impl Mul for FieldElement {

    type Output = Self;
    
    /* 
     * We have to ensure that the elements are from the same
     * finite field and define it with the modulo operation,
     * returning an instance of FiniteElement struct
     * 
     * @param self: a immutable FiniteElement
     * @param other: another immutable FieldElement
     * @returns FieldElement
     */
    fn mul(self, other: FieldElement) -> Self {
        match self.prime.cmp(&other.prime) {
            Ordering::Equal => {
                let num = (self.num * other.num) % &self.prime;
                Self { num: num, prime: self.prime.clone() }
            },
            _ => panic!("Cannot mull two numbers in different fields")
        }
    }
}

// Implement Div trait to mimic __truediv__ in python
impl Div for FieldElement {

    type Output = Self;
    
    /* 
     * We have to ensure that the elements are from the same
     * finite field and define it with the modulo operation,
     * returning an instance of FiniteElement struct
     * 
     * @param self: a immutable FiniteElement
     * @param other: another immutable FieldElement
     * @returns FieldElement
     */
    fn div(self, other: FieldElement) -> Self {
        match self.prime.cmp(&other.prime) {
            Ordering::Equal => {
                match other.num.cmp(&BigUint::zero()) {
                    Ordering::Equal => panic!("Cannot divide by zero in a finite field"),
                    _ => {
                        // Fermat's little theorem
                        let exp = other.pow(
                            (self.prime.clone() - 2.to_biguint().unwrap()).into()
                        );
                        let num = (self.num * exp.num) % &self.prime;
                        Self { num: num, prime: self.prime.clone() }
                    }
                }
            },
            _ => panic!("Cannot div two numbers in different fields")
        }
    }
}
