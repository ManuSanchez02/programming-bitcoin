use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coordinate {
    Value(f32),
    Infinity,
}

impl From<f32> for Coordinate {
    fn from(value: f32) -> Self {
        Coordinate::Value(value)
    }
}

impl From<i32> for Coordinate {
    fn from(value: i32) -> Self {
        Coordinate::Value(value as f32)
    }
}

impl Coordinate {
    pub fn pow(&self, exp: f32) -> Self {
        if self.is_infinity() {
            return Coordinate::Infinity;
        }

        return self.map(|x: f32| x.powf(exp));
    }

    pub fn is_infinity(&self) -> bool {
        *self == Coordinate::Infinity
    }

    pub fn map<F>(self, f: F) -> Coordinate
    where
        F: FnOnce(f32) -> f32,
    {
        match self {
            Coordinate::Value(x) => Coordinate::Value(f(x)),
            Coordinate::Infinity => Coordinate::Infinity,
        }
    }
}

impl Eq for Coordinate {}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if let (Coordinate::Value(x), Coordinate::Value(y)) = (self, other) {
            return Coordinate::Value(x + y);
        } else {
            return Coordinate::Infinity;
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if let (Coordinate::Value(x), Coordinate::Value(y)) = (self, other) {
            return Coordinate::Value(x - y);
        } else {
            return Coordinate::Infinity;
        }
    }
}

impl Mul for Coordinate {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if let (Coordinate::Value(x), Coordinate::Value(y)) = (self, other) {
            return Coordinate::Value(x * y);
        } else {
            return Coordinate::Infinity;
        }
    }
}

impl Div for Coordinate {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if let (Coordinate::Value(x), Coordinate::Value(y)) = (self, other) {
            return Coordinate::Value(x / y);
        } else {
            return Coordinate::Infinity;
        }
    }
}

impl Mul<f32> for Coordinate {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        if let Coordinate::Value(x) = self {
            return Coordinate::Value(x * other);
        } else {
            return Coordinate::Infinity;
        }
    }
}

impl Add<f32> for Coordinate {
    type Output = Self;

    fn add(self, other: f32) -> Self::Output {
        if let Coordinate::Value(x) = self {
            return Coordinate::Value(x + other);
        } else {
            return Coordinate::Infinity;
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Coordinate::Value(v) = self {
            write!(f, "{}", v)
        } else {
            write!(f, "Inf")
        }
    }
}