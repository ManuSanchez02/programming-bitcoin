use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

use crate::{field_element::FieldElement, is_zero::IsZero, pow::Pow, real_value::RealValue};

pub trait GraphPoint:
    Display
    + Debug
    + PartialEq
    + Add<Output = Self>
    + Add<i32, Output = Self>
    + Mul<Output = Self>
    + Mul<i32, Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Pow
    + Sized
    + Clone
    + Copy
    + IsZero
{
}

impl<
        T: Display
            + Debug
            + PartialEq
            + Add<Output = Self>
            + Add<i32, Output = Self>
            + Mul<Output = Self>
            + Mul<i32, Output = Self>
            + Sub<Output = Self>
            + Div<Output = Self>
            + Pow
            + Sized
            + Clone
            + Copy
            + IsZero,
    > GraphPoint for T
{
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coordinate<G: GraphPoint> {
    Value(G),
    Infinity,
}

impl From<i32> for Coordinate<RealValue> {
    fn from(value: i32) -> Self {
        Coordinate::Value(RealValue::new(value as f32))
    }
}

impl From<u32> for Coordinate<RealValue> {
    fn from(value: u32) -> Self {
        Coordinate::Value(RealValue::new(value as f32))
    }
}

impl From<f32> for Coordinate<RealValue> {
    fn from(value: f32) -> Self {
        Coordinate::Value(RealValue::new(value))
    }
}

impl From<RealValue> for Coordinate<RealValue> {
    fn from(value: RealValue) -> Self {
        Coordinate::Value(value)
    }
}

impl From<FieldElement> for Coordinate<FieldElement> {
    fn from(value: FieldElement) -> Self {
        Coordinate::Value(value)
    }
}

impl<T: GraphPoint> Coordinate<T> {
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(T) -> T,
    {
        match self {
            Coordinate::Value(x) => Coordinate::Value(f(x)),
            Coordinate::Infinity => Coordinate::Infinity,
        }
    }

    pub fn is_infinity(&self) -> bool {
        *self == Coordinate::Infinity
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Coordinate::Value(x) => x.is_zero(),
            Coordinate::Infinity => false,
        }
    }
}

impl<T: GraphPoint> Pow for Coordinate<T> {
    fn pow(&self, exp: i32) -> Self {
        if self.is_infinity() {
            return Coordinate::Infinity;
        }

        return self.map(|x| x.pow(exp));
    }
}

impl<T: GraphPoint> Eq for Coordinate<T> {}

impl<T: GraphPoint> Add for Coordinate<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Coordinate::Value(x), Coordinate::Value(y)) =>Coordinate::Value(x + y),
            (Coordinate::Value(_), Coordinate::Infinity) => self,
            (Coordinate::Infinity, Coordinate::Value(_)) => other,
            _ => Coordinate::Infinity
        }
    }
}

impl<T: GraphPoint> Add<i32> for Coordinate<T> {
    type Output = Self;

    fn add(self, other: i32) -> Self::Output {
        if let Coordinate::Value(x) = self {
            return Coordinate::Value(x + other);
        } else {
            return Coordinate::Infinity;
        }
    }
}

impl<T: GraphPoint> Sub for Coordinate<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if let (Coordinate::Value(x), Coordinate::Value(y)) = (self, other) {
            return Coordinate::Value(x - y);
        } else {
            return Coordinate::Infinity;
        }
    }
}

impl<T: GraphPoint> Mul for Coordinate<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if let (Coordinate::Value(x), Coordinate::Value(y)) = (self, other) {
            return Coordinate::Value(x * y);
        } else {
            return Coordinate::Infinity;
        }
    }
}

impl<T: GraphPoint> Mul<i32> for Coordinate<T> {
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        if let Coordinate::Value(x) = self {
            return Coordinate::Value(x * other);
        } else {
            return Coordinate::Infinity;
        }
    }
}

impl<T: GraphPoint> Div for Coordinate<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if let (Coordinate::Value(x), Coordinate::Value(y)) = (self, other) {
            return Coordinate::Value(x / y);
        } else {
            return Coordinate::Infinity;
        }
    }
}

// impl Mul<f32> for Coordinate {
//     type Output = Self;

//     fn mul(self, other: f32) -> Self::Output {
//         if let Coordinate::Value(x) = self {
//             return Coordinate::Value(x * other);
//         } else {
//             return Coordinate::Infinity;
//         }
//     }
// }

// impl Add<f32> for Coordinate {
//     type Output = Self;

//     fn add(self, other: f32) -> Self::Output {
//         if let Coordinate::Value(x) = self {
//             return Coordinate::Value(x + other);
//         } else {
//             return Coordinate::Infinity;
//         }
//     }
// }

impl<T: GraphPoint> Display for Coordinate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Coordinate::Value(v) = self {
            write!(f, "{}", v)
        } else {
            write!(f, "Inf")
        }
    }
}
