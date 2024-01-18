use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::{is_zero::IsZero, pow::Pow};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RealValue(f32);

impl Eq for RealValue {}

impl RealValue {
    pub fn new(value: f32) -> Self {
        RealValue(value)
    }
}

impl Add for RealValue {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        RealValue(self.0 + other.0)
    }
}

impl Add<i32> for RealValue {
    type Output = Self;

    fn add(self, other: i32) -> Self::Output {
        RealValue(self.0 + other as f32)
    }
}

impl Sub for RealValue {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        RealValue(self.0 - other.0)
    }
}

impl Mul for RealValue {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        RealValue(self.0 * other.0)
    }
}

impl Mul<i32> for RealValue {
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        RealValue(self.0 * other as f32)
    }
}

impl Div for RealValue {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        RealValue(self.0 / other.0)
    }
}

impl Pow for RealValue {
    fn pow(&self, exp: i32) -> Self {
        RealValue(self.0.powi(exp))
    }
}

impl IsZero for RealValue {
    fn is_zero(&self) -> bool {
        self.0 == 0.0
    }
}

impl From<i32> for RealValue {
    fn from(value: i32) -> Self {
        RealValue(value as f32)
    }
}

impl From<f32> for RealValue {
    fn from(value: f32) -> Self {
        RealValue(value)
    }
}

impl Display for RealValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
