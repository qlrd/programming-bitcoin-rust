/*
 * We want to represent each finite field element
 * in a field F_prime
 * See "Constructing a finite field in python"
 */
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone)]
pub struct FieldElement {
    pub num: u64,
    pub prime: u64
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
    pub fn new(num: u64, prime: u64) -> Result<Self, String> {

        // Since we defined num as a u64 type, it's useless
        // to compare if num < 0
        if num >= prime {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1))
        }

        // The rest assigns the initialization values
        Ok(Self { num, prime })
    }

    /*
     * Repeatedly square the base and reduce it modulo prime at each step.
     * Also multiply by base when the current exponent bit is 1. 
     * This approach works well with arbitrarily large exponents.
     */
    #[allow(dead_code)]
    pub fn pow(&self, exponent: i64) -> Self {
        let mut base = self.num;
        let mut exp = if exponent < 0 {
            (self.prime - 2) as u64 * (- exponent) as u64
        } else {
            exponent as u64
        };
        
        let mut result = 1;

        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % self.prime;
            }
            base = (base * base) % self.prime;
            exp /= 2;
        } 
        
        Self { num: result, prime: self.prime }
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
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different fields");
        }
        let num = (self.num + other.num) % self.prime;
        Self { num: num, prime: self.prime }   
    }
}

// Implement Add trait to mimic __add__ in python
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
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different fields");
        }
        // i64 (to handle potentially negative results).
        // rem_euclid to ensure the result is positive within the field.
        let num = (self.num as i64 - other.num as i64).rem_euclid(self.prime as i64) as u64;
        Self { num: num, prime: self.prime }
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
        if self.prime != other.prime {
            panic!("Cannot mull two numbers in different fields");
        }
        let num = (self.num * other.num) % self.prime;
        Self { num: num, prime: self.prime}
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
        if self.prime != other.prime {
            panic!("Cannot div two numbers in different fields");
        }
        if other.num == 0 {
            panic!("Cannot divide by zero in a finite field");
        }

        // Fermat's little theorem
        let exp = other.pow((self.prime - 2).try_into().unwrap());
        let num = (self.num * exp.num) % self.prime;
        Self { num: num, prime: self.prime}
    }
} 
