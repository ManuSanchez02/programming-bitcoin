use std::{fmt::Display, ops::Add};

use crate::coordinate::Coordinate;

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
