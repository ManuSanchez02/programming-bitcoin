use std::{
    fmt::{format, write, Display},
    iter::Map,
    ops::{Add, Mul, Sub, Div},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Coordinate {
    Value(i32),
    Infinity,
}

impl From<i32> for Coordinate {
    fn from(value: i32) -> Self {
        Coordinate::Value(value)
    }
}

impl Coordinate {
    pub fn pow(&self, exp: u32) -> Self {
        if self.is_infinity() {
            return Coordinate::Infinity;
        }

        return self.map(|x: i32| x.pow(exp));
    }

    pub fn is_infinity(&self) -> bool {
        *self == Coordinate::Infinity
    }

    pub fn map<F>(self, f: F) -> Coordinate
    where
        F: FnOnce(i32) -> i32,
    {
        match self {
            Coordinate::Value(x) => Coordinate::Value(f(x)),
            Coordinate::Infinity => Coordinate::Infinity,
        }
    }
}

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

impl Mul<i32> for Coordinate {
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        if let Coordinate::Value(x) = self {
            return Coordinate::Value(x * other);
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

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: Coordinate,
    y: Coordinate,
    a: Coordinate,
    b: Coordinate,
}

impl Point {
    pub fn new<T: Into<Coordinate>, U: Into<Coordinate>>(
        x: T,
        y: T,
        a: U,
        b: U,
    ) -> Result<Self, String> {
        let x: Coordinate = x.into();
        let y: Coordinate = y.into();
        let a: Coordinate = a.into();
        let b: Coordinate = b.into();

        if !(x.is_infinity() && y.is_infinity()) && y.pow(2) != x.pow(3) + a * x + b {
            return Err(format!("({},{}) is not on the curve", x, y));
        }

        Ok(Self { x, y, a, b })
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

        if self.x == other.x && self.y != other.y {
            return Ok(Point {
                x: Coordinate::Infinity,
                y: Coordinate::Infinity,
                a: self.a,
                b: self.b,
            });
        }

        let slope = (other.y - self.y) / (other.x - self.x);
        let x_res = slope.pow(2) - self.x - other.x;
        let y_res = slope * (self.x - x_res) - self.y;

        let res = Point {
            a: self.a,
            b: self.b,
            x: x_res,
            y: y_res
        };

        return Ok(res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_create_point_that_is_not_in_curve() {
        let point_res = Point::new(-1, -2, 5, 7);
        assert!(point_res.is_err());
    }

    #[test]
    fn can_create_point_that_is_in_curve() {
        let point_res = Point::new(-1, -1, 5, 7);
        assert!(point_res.is_ok());
    }

    #[test]
    fn can_create_point_that_is_in_infinity() {
        let point_res = Point::new(Coordinate::Infinity, Coordinate::Infinity, 5, 7);
        assert!(point_res.is_ok());
    }
}
