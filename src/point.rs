/*
 * We're going to define a struct/impl Point
 * to be a point on the general curve
 * y^2 = x^3 + ax + b
 */

use std::fmt;
use num_bigint::{BigUint};
use crate::field_element::FieldElement;

#[derive(Debug, Clone)]
pub struct Point {
    pub a: FieldElement,
    pub b: FieldElement,
    pub x: FieldElement,
    pub y: FieldElement
}

/* 
 * This implementation represents a point on elliptic curve
 */
impl Point {

    #[allow(dead_code)]
    pub fn new(
        x: FieldElement,
        y: FieldElement,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self, String> {
        let two = BigUint::from(2u32).into();
        let three = BigUint::from(3u32).into();
        
        if y.pow(two) != x.pow(three) + (a.clone() * x.clone()) + b.clone() {
            return Err(
                format!(
                    "Point({}, {}) is not on the curve",
                    x,
                    y
                )
            
            )
        }
        Ok(
            Self {
                x: x.clone(),
                y: y.clone(),
                a: a.clone(),
                b: b.clone()
            })
    }
}

// Implement Display trait to mimic  __repr__ in python
impl fmt::Display for Point {

    /*
     * When you implement Display, you’re defining how the type 
     * will be printed in a human-readable form.
     * 
     * @param &self: An immutable reference to a Point
     * @param &mut<fmt::Formatter>: A mutable reference to a Formatter
     * @returns fmt::Result<Ok(T), fmt::Error>
     */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

// Implement PartialEq trait to mimic __eq__ in python
impl PartialEq for Point {

    /*
     * Check if two implementations of Point are equal.
     * This is only true when both x, y, a, b are equal
     *
     * @param &self: a immutable reference to a Point
     * @param &Self: a immutable reference to another Point
     * @returns bool 
     */
     fn eq(&self, other: &Self) -> bool {
        self.x == other.x && 
            self.y == other.y &&
            self.a == other.a &&
            self.b == other.b
     }
}
