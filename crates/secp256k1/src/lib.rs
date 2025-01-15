use field_element::FieldElement;
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{Num, One, Zero};
use std::io::{Cursor, Read};
use std::{
    array::TryFromSliceError,
    ops::{Add, Mul},
};

pub const PRIME: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
pub const ORDER: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

#[derive(Debug, Clone)]
pub struct Secp256k1Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
}

pub enum Secp256k1 {
    Generator,
    Infinity,
    Prime,
    Order,
}

/// This represent secp256k1 group elements (curve points or infinity) - doc copied from Floresta
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
                    "Invalid secp256k1 point:x = {:?}, y = {:?}",
                    &x, &y
                ))
            }
        }
    }

    /// Binary version of uncompressed SEC format
    pub fn to_uncompressed_sec(&self) -> Result<[u8; 65], TryFromSliceError> {
        let mut serialized = vec![4u8];
        serialized.extend(self.x.as_ref().unwrap().num.to_bytes_be());
        serialized.extend(self.y.as_ref().unwrap().num.to_bytes_be());
        <[u8; 65]>::try_from(serialized.as_slice())
    }

    /// Binary version of compressed SEC format
    pub fn to_compressed_sec(&self) -> Result<[u8; 33], TryFromSliceError> {
        let y = &self.y.as_ref().unwrap().num;
        let two = BigUint::from(2u32);
        let zero = BigUint::from(0u32);

        let mut serialized = if y % two == zero {
            vec![2u8]
        } else {
            vec![3u8]
        };

        serialized.extend(self.x.as_ref().unwrap().num.to_bytes_be());
        <[u8; 33]>::try_from(serialized.as_slice())
    }

    /// Desserialize a vector of bytes to a point
    pub fn deserialize(sec: Vec<u8>) -> Result<Secp256k1Point, String> {
        let mut cursor = Cursor::new(sec);
        let mut sec_type = [0u8; 1];
        let mut x = [0u8; 32];

        cursor.read_exact(&mut sec_type).unwrap();
        cursor.read_exact(&mut x).unwrap();

        let fe_x = FieldElement {
            num: BigUint::from_bytes_be(x.as_slice()),
            prime: Secp256k1::Prime.as_biguint(),
        };

        // Deserialize a uncompressed SEC formated point
        if sec_type[0] == 4u8 {
            let mut y = [0u8; 32];
            cursor.read_exact(&mut y).unwrap();

            let fe_y = FieldElement {
                num: BigUint::from_bytes_be(y.as_slice()),
                prime: Secp256k1::Prime.as_biguint(),
            };

            return Ok(Secp256k1Point {
                x: Some(fe_x),
                y: Some(fe_y),
            });
        }

        // Deserialize a compressed SEC formated point
        let is_even = sec_type[0] == 2u8;
        let fe_7 = FieldElement {
            num: BigUint::from(7u8),
            prime: Secp256k1::Prime.as_biguint(),
        };

        let alpha_fe = fe_x.pow(&BigInt::from(3u8)) + fe_7;
        let beta_fe = alpha_fe.sqrt();

        let prime = Secp256k1::Prime.as_biguint();

        if is_even == beta_fe.num.is_even() {
            Ok(Secp256k1Point {
                x: Some(fe_x),
                y: Some(beta_fe),
            })
        } else {
            let odd = FieldElement {
                num: &prime - &beta_fe.num,
                prime: prime.clone(),
            };
            Ok(Secp256k1Point {
                x: Some(fe_x),
                y: Some(odd),
            })
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

impl Mul<&Secp256k1Point> for &BigUint {
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
