/*
 * We're going to define a struct/impl Point
 * to be a point on the general curve
 * y^2 = x^3 + ax + b
 */

use crate::primitives::field_element::FieldElement;
use core::panic;
use num_bigint::{BigInt, BigUint};
use num_traits::{Num, One, Zero};
use std::ops::{Add, Mul, Shl, Shr};

pub const PRIME: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
pub const ORDER: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

#[derive(Debug, Clone)]
pub struct Secp256k1Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

pub enum Secp256k1 {
    Generator,
    Infinity,
    Prime,
    Order,
}

/// This represent secp256k1 group elements (curve points or infinity)
///
/// Normal points on the curve have fields:
///    * x: the x coordinate (a field element)
///    * y: the y coordinate (a field element, satisfying y^2 = x^3 + 7)
///
/// In elliptic curve arithmetic, the Point at Infinity acts as the identity element
/// for the addition operation on points. When implementing an elliptic curve point struct,
/// we need a way to handle this special case since the Point at Infinity does not have
/// x or y coordinates.
///
/// To incorporate this in your Rust Point struct, one common approach
/// (and what Jimmy Song suggests in his book Programming Bitcoin) is to use an Option
/// for the coordinates. This allows the Point struct to handle both regular points
/// (with x and y values) and the Point at Infinity (without x and y values).
impl Secp256k1Point {
    #[allow(dead_code)]
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>) -> Result<Self, String> {
        // Point at inifinity
        if x.is_none() && y.is_none() {
            Ok(Self { x: None, y: None })
        } else if x.is_none() || y.is_none() {
            return Err("Both x and y must be provided, or none for point at infinity".to_string());
        } else {
            // check for y**2 == x**3 + 7
            let two = BigInt::from(2u32);
            let three = BigInt::from(3u32);
            let seven = FieldElement::new("7", PRIME).unwrap();

            let _x = x.as_ref().unwrap();
            let _y = y.as_ref().unwrap();

            let lhs = _y.pow(&two); // y**2
            let rhs = _x.pow(&three) + seven; // x**3 + 7

            if lhs == rhs {
                Ok(Self { x, y })
            } else {
                Err(format!(
                    "Invalid secp256k1 point:x = {:#?}, y = {:#?}",
                    &x, &y
                ))
            }
        }
    }
}

impl Secp256k1 {
    pub fn as_point(&self) -> Secp256k1Point {
        match self {
            Secp256k1::Infinity => Secp256k1Point::new(None, None).unwrap(),
            Secp256k1::Generator => {
                let x = FieldElement::new(
                    "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
                    PRIME,
                )
                .unwrap();
                let y = FieldElement::new(
                    "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
                    PRIME,
                )
                .unwrap();
                Secp256k1Point::new(Some(x), Some(y)).unwrap()
            }
            _ => panic!("Invalid enum as_point"),
        }
    }

    pub fn as_biguint(&self) -> BigUint {
        match self {
            Secp256k1::Prime => BigUint::from_str_radix(PRIME, 16).unwrap(),
            Secp256k1::Order => BigUint::from_str_radix(ORDER, 16).unwrap(),
            _ => panic!("Invalid enum as biguint"),
        }
    }
}

// Implement PartialEq trait to mimic __eq__ in python
impl PartialEq for Secp256k1Point {
    /*
     * Check if two implementations of Point are equal.
     * In Rust, implementing the != operator directly is not
     * required because Rust automatically provides !=
     * when you implement the PartialEq trait
     *
     * @param &self: a immutable reference to a FieldElement
     * @param &Self: a immutable reference to another FieldElement
     * @returns bool
     */
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// Implement Add trait to mimic __add__ in python
impl Add for Secp256k1Point {
    type Output = Self;

    fn add(self, other: Secp256k1Point) -> Self {
        // This is like P1 + 0 = P1
        if self.x.is_none() {
            return other;
        }

        // This is like 0 + P2 = P2
        if other.x.is_none() {
            return self;
        }

        // Tangent at y == 0 is Point at infinity
        let zero = FieldElement::new("0", PRIME).unwrap();
        if self == other && self.y.as_ref().unwrap() == &zero {
            return Self { x: None, y: None };
        }

        if self.x == other.x {
            if self.y != other.y {
                // A point added to its negation is the point at infinity
                return Self { x: None, y: None };
            } else {
                // Doubling algorithm
                // Extract FieldElement references
                let x1 = self.x.as_ref().unwrap();
                let y1 = self.y.as_ref().unwrap();

                // Compute slope: s = (y2 - y1) / (x2 - x1)
                let two = FieldElement::new("2", PRIME).unwrap();
                let three = FieldElement::new("3", PRIME).unwrap();
                let numerator = &three * &x1.pow(&BigInt::from(2u32));
                let denominator = &two * y1;
                let s = &numerator / &denominator;

                // Compute x3: x3 = s^2 - x1 - x2
                let s2 = s.pow(&BigInt::from(2u32));
                let x3 = &s2 - &(&two * x1);

                // Compute y3: y3 = s * (x1 - x3) - y1
                let x1_minus_x3 = x1 - &x3;
                let y3 = &(&s * &x1_minus_x3) - y1;

                return Self {
                    x: Some(x3),
                    y: Some(y3),
                };
            }
        }

        // Adding algorithm
        // Extract FieldElement references
        let x1 = self.x.as_ref().unwrap();
        let y1 = self.y.as_ref().unwrap();
        let x2 = other.x.as_ref().unwrap();
        let y2 = other.y.as_ref().unwrap();

        // Compute slope: s = (y2 - y1) / (x2 - x1)
        let numerator = y2 - y1;
        let denominator = x2 - x1;
        let s = &numerator / &denominator;

        // Compute x3: x3 = s^2 - x1 - x2
        let s2 = s.pow(&BigInt::from(2u32));
        let x3 = &(&s2 - x1) - x2;

        // Compute y3: y3 = s * (x1 - x3) - y1
        let x1_minus_x3 = x1 - &x3;
        let y3 = &(&s * &x1_minus_x3) - y1;

        // Return new point
        Self {
            x: Some(x3),
            y: Some(y3),
        }
    }
}

impl<'b> Add<&'b Secp256k1Point> for &Secp256k1Point {
    type Output = Secp256k1Point;

    fn add(self, other: &'b Secp256k1Point) -> Secp256k1Point {
        // This is like P1 + 0 = P1
        if self.x.is_none() {
            return other.clone();
        }

        // This is like 0 + P2 = P2
        if other.x.is_none() {
            return self.clone();
        }

        // Tangent at y == 0 is Point at infinity
        let zero = FieldElement::new("0", PRIME).unwrap();
        if self == other && self.y.as_ref().unwrap() == &zero {
            return Secp256k1Point { x: None, y: None };
        }

        if self.x == other.x {
            if self.y != other.y {
                // A point added to its negation is the point at infinity
                return Secp256k1Point { x: None, y: None };
            } else {
                // Doubling algorithm
                // Extract FieldElement references
                let x1 = self.x.as_ref().unwrap();
                let y1 = self.y.as_ref().unwrap();

                // Compute slope: s = (3 * x1^2) / (2 * y1)
                let two = FieldElement::new("2", PRIME).unwrap();
                let three = FieldElement::new("3", PRIME).unwrap();
                let numerator = &three * &x1.pow(&BigInt::from(2u32));
                let denominator = &two * y1;
                let s = &numerator / &denominator;

                // Compute x3: x3 = s^2 - 2 * x1
                let s2 = s.pow(&BigInt::from(2u32));
                let x3 = &s2 - &(&two * x1);

                // Compute y3: y3 = s * (x1 - x3) - y1
                let x1_minus_x3 = x1 - &x3;
                let y3 = &(&s * &x1_minus_x3) - y1;

                return Secp256k1Point {
                    x: Some(x3),
                    y: Some(y3),
                };
            }
        }

        // Adding algorithm
        // Extract FieldElement references
        let x1 = self.x.as_ref().unwrap();
        let y1 = self.y.as_ref().unwrap();
        let x2 = other.x.as_ref().unwrap();
        let y2 = other.y.as_ref().unwrap();

        // Compute slope: s = (y2 - y1) / (x2 - x1)
        let numerator = y2 - y1;
        let denominator = x2 - x1;
        let s = &numerator / &denominator;

        // Compute x3: x3 = s^2 - x1 - x2
        let s2 = s.pow(&BigInt::from(2u32));
        let x3 = &(&s2 - x1) - x2;

        // Compute y3: y3 = s * (x1 - x3) - y1
        let x1_minus_x3 = x1 - &x3;
        let y3 = &(&s * &x1_minus_x3) - y1;

        // Return new point
        Secp256k1Point {
            x: Some(x3),
            y: Some(y3),
        }
    }
}

impl Mul<BigUint> for Secp256k1Point {
    type Output = Secp256k1Point;

    fn mul(self, other: BigUint) -> Secp256k1Point {
        let mut coef = other.clone();
        let mut current = self.clone();
        let mut result = Secp256k1Point::new(None, None).unwrap();

        while coef > BigUint::zero() {
            if &coef & BigUint::one() == BigUint::one() {
                result = &result + &current;
            }
            current = &current + &current;
            coef >>= 1;
        }
        result.clone()
    }
}

impl Mul<Secp256k1Point> for BigUint {
    type Output = Secp256k1Point;

    fn mul(self, other: Secp256k1Point) -> Secp256k1Point {
        let mut coef = self.clone();
        let mut current = other.clone();
        let mut result = Secp256k1Point::new(None, None).unwrap();

        while coef > BigUint::zero() {
            if &coef & BigUint::one() == BigUint::one() {
                result = &result + &current;
            }
            current = &current + &current;
            coef >>= 1;
        }
        result.clone()
    }
}

impl Mul<&BigUint> for &Secp256k1Point {
    type Output = Secp256k1Point;

    fn mul(self, coefficient: &BigUint) -> Secp256k1Point {
        let mut coef = coefficient.clone();
        let mut current = self.clone();
        let mut result = Secp256k1Point::new(None, None).unwrap();

        while coef > BigUint::zero() {
            if &coef & BigUint::one() == BigUint::one() {
                result = &result + &current;
            }
            current = &current + &current;
            coef >>= 1;
        }
        result.clone()
    }
}

impl Mul<&Secp256k1Point> for BigUint {
    type Output = Secp256k1Point;

    fn mul(self, other: &Secp256k1Point) -> Secp256k1Point {
        let mut coef = self.clone();
        let mut current = other.clone();
        let mut result = Secp256k1Point::new(None, None).unwrap();

        while coef > BigUint::zero() {
            if &coef & BigUint::one() == BigUint::one() {
                result = &result + &current;
            }
            current = &current + &current;
            coef >>= 1;
        }
        result.clone()
    }
}
