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

#[derive(Debug, PartialEq)]
pub struct Point {
    x: Coordinate,
    y: Coordinate,
    a: f32,
    b: f32,
}

impl Eq for Point {}

impl Point {
    pub fn new<T: Into<Coordinate>, U: Into<f32>>(x: T, y: T, a: U, b: U) -> Result<Self, String> {
        let x: Coordinate = x.into();
        let y: Coordinate = y.into();
        let a: f32 = a.into();
        let b: f32 = b.into();

        if !(x.is_infinity() && y.is_infinity()) && y.pow(2.0) != x.pow(3.0) + x * a + b {
            return Err(format!("({},{}) is not on the curve", x, y));
        }

        Ok(Self { x, y, a, b })
    }

    fn add_point(self, other: Point) -> Point {
        if self.x == other.x && self.y != other.y {
            return Point {
                x: Coordinate::Infinity,
                y: Coordinate::Infinity,
                a: self.a,
                b: self.b,
            };
        }

        let slope = if self == other {
            if self.y == Coordinate::Value(0.0) {
                return Point {
                    x: Coordinate::Infinity,
                    y: Coordinate::Infinity,
                    a: self.a,
                    b: self.b,
                };
            }

            (self.x.pow(2.0) * 3.0 + self.a) / (self.y * 2.0)
        } else {
            (other.y - self.y) / (other.x - self.x)
        };

        let x_res = slope.pow(2.0) - self.x - other.x;
        let y_res = slope * (self.x - x_res) - self.y;

        let res = Point {
            a: self.a,
            b: self.b,
            x: x_res,
            y: y_res,
        };

        return res;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({},{})_{}_{}", self.x, self.y, self.a, self.b)
    }
}

impl Add for Point {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Self::Output {
        if self.a != other.a || self.b != other.b {
            return Err(format!(
                "Points {}, {} are not on the same curve",
                self, other
            ));
        }

        if self.x.is_infinity() {
            return Ok(other);
        }

        if other.x.is_infinity() {
            return Ok(self);
        }

        return Ok(self.add_point(other));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_create_point_that_is_not_in_curve() {
        let point_res = Point::new(-1, -2, 5.0, 7.0);
        assert!(point_res.is_err());
    }

    #[test]
    fn can_create_point_that_is_in_curve() {
        let point_res = Point::new(-1, -1, 5.0, 7.0);
        assert!(point_res.is_ok());
    }

    #[test]
    fn can_create_point_that_is_in_infinity() {
        let point_res = Point::new(Coordinate::Infinity, Coordinate::Infinity, 5.0, 7.0);
        assert!(point_res.is_ok());
    }
}
