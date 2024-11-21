/*
 * We're going to define a struct/impl Point
 * to be a point on the general curve
 * y^2 = x^3 + ax + b
 */

use crate::primitives::field_element::FieldElement;
use num_bigint::BigInt;
use num_traits::Num;
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
    pub a: FieldElement,
    pub b: FieldElement,
}

/*
 * This implementation represents a point on elliptic curve
 */
impl Point {
    /*
     * In elliptic curve arithmetic, the Point at Infinity acts as the identity element
     * for the addition operation on points. When implementing an elliptic curve point struct,
     * we need a way to handle this special case since the Point at Infinity does not have
     * x or y coordinates.
     * To incorporate this in your Rust Point struct, one common approach
     * (and what Jimmy Song suggests in his book Programming Bitcoin) is to use an Option
     * for the coordinates. This allows the Point struct to handle both regular points
     * (with x and y values) and the Point at Infinity (without x and y values).
     */
    #[allow(dead_code)]
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self, String> {
        // Point at inifinity

        if x.is_none() && y.is_none() {
            Ok(Self {
                x: None,
                y: None,
                a,
                b,
            })
        } else {
            let two = BigInt::from_str_radix("2", 10).unwrap();
            let three = BigInt::from_str_radix("3", 10).unwrap();
            let fe_x = x.clone().unwrap();
            let fe_y = y.clone().unwrap();
            let fe_a = a.clone();
            let fe_b = b.clone();
            let in_curve_y = fe_y.pow(two);
            let in_curve_x = fe_x.pow(three) + (fe_a * fe_x) + fe_b;

            if in_curve_y != in_curve_x {
                Err(format!(
                    "Point({}, {})_<a={}, b={}> is not on the curve",
                    x.unwrap(),
                    y.unwrap(),
                    a,
                    b
                ))
            } else {
                Ok(Self { x, y, a, b })
            }
        }
    }
}

// Implement Display trait to mimic  __repr__ in python
impl fmt::Display for Point {
    /*
     * When you implement Display, you’re defining how the type
     * will be printed in a human-readable form.
     *
     * @param &self: An immutable reference to a FieldElement
     * @param &mut<fmt::Formatter>: A mutable reference to a Formatter
     * @returns fmt::Result<Ok(T), fmt::Error>
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fe_x = self.x.clone().unwrap();
        let fe_y = self.y.clone().unwrap();
        write!(f, "Point({}, {})_{}_{}", &fe_x, &fe_y, self.a, self.b)
    }
}

// Implement PartialEq trait to mimic __eq__ in python
impl PartialEq for Point {
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
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

// Implement Add trait to mimic __add__ in python
impl Add for Point {
    type Output = Self;

    /*
     * We have to ensure that the points are in the same
     * curve
     *
     * @param self: a immutable FiniteElement
     * @param other: another immutable FieldElement
     * @returns FieldElement
     */
    fn add(self, other: Point) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points {} and {} aren't on the same curve", self, other)
        } else if self.x.is_none() {
            other
        } else if other.x.is_none() {
            self
        } else if self.x == other.x && self.y != other.y {
            Self {
                x: None,
                y: other.y,
                a: self.a,
                b: self.b,
            }
        } else if self.x != other.x {
            let delta_y = other.y.clone().unwrap() - self.y.clone().unwrap();
            let delta_x = other.x.clone().unwrap() - self.x.clone().unwrap();
            let slope = delta_y / delta_x;
            let two = BigInt::from_str_radix("2", 10).unwrap();
            let x_minus = self.x.unwrap() - other.x.unwrap();
            let x3 = slope.pow(two) - x_minus.clone();
            let y3 = (slope * x_minus) - self.y.unwrap();
            Self {
                x: Some(x3),
                y: Some(y3),
                a: self.a,
                b: self.b,
            }
        } else {
            panic!("Unknow error for {} and {}", self, other)
        }
    }
}
