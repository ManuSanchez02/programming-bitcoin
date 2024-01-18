use std::{
    fmt::Display,
    ops::{Add, Mul},
};

use crate::{
    coordinate::{Coordinate, GraphPoint},
    field_element::FieldElement,
    pow::Pow,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point<G: GraphPoint> {
    pub x: Coordinate<G>,
    pub y: Coordinate<G>,
    a: Coordinate<G>,
    b: Coordinate<G>,
}

impl Point<FieldElement> {
    pub fn from_finite_field(x: u32, y: u32, a: u32, b: u32, prime: u32) -> Result<Self, String> {
        let x = Coordinate::Value(FieldElement::new(x, prime)?);
        let y = Coordinate::Value(FieldElement::new(y, prime)?);
        let a = Coordinate::Value(FieldElement::new(a, prime)?);
        let b = Coordinate::Value(FieldElement::new(b, prime)?);
        Self::new(x, y, a, b)
    }
}

impl<G: GraphPoint> Point<G> {
    pub fn new<A: Into<Coordinate<G>>, B: Into<Coordinate<G>>>(
        x: A,
        y: A,
        a: B,
        b: B,
    ) -> Result<Self, String> {
        let x: Coordinate<G> = x.into();
        let y: Coordinate<G> = y.into();
        let a: Coordinate<G> = a.into();
        let b: Coordinate<G> = b.into();

        if !(x.is_infinity() && y.is_infinity()) && y.pow(2) != x.pow(3) + x * a + b {
            return Err(format!("({},{}) is not on the curve", x, y));
        }

        Ok(Self { x, y, a, b })
    }

    fn add_point(self, other: Self) -> Self {
        if self.x == other.x && self.y != other.y {
            return Point {
                x: Coordinate::Infinity,
                y: Coordinate::Infinity,
                a: self.a,
                b: self.b,
            };
        }

        let slope = if self == other {
            if self.y.is_zero() {
                return Point {
                    x: Coordinate::Infinity,
                    y: Coordinate::Infinity,
                    a: self.a,
                    b: self.b,
                };
            }

            (self.x.pow(2) * 3 + self.a) / (self.y * 2)
        } else {
            (other.y - self.y) / (other.x - self.x)
        };

        let x_res = slope.pow(2) - self.x - other.x;
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

impl<G: GraphPoint> Display for Point<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({},{})_{}_{}", self.x, self.y, self.a, self.b)
    }
}

impl<G: GraphPoint> Add for Point<G> {
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

fn binary_expansion<T: GraphPoint>(point: Point<T>, coefficient: u32) -> Point<T> {
    let mut coef = coefficient;
    let mut current = point;
    let mut result =
        Point::new(Coordinate::Infinity, Coordinate::Infinity, point.a, point.b).unwrap();

    while coef > 0 {
        if coef & 1 == 1 {
            result = (result + current).unwrap();
        }

        current = (current + current).unwrap();
        coef >>= 1;
    }

    return result;
}

impl<T: GraphPoint> Mul<Point<T>> for u32 {
    type Output = Point<T>;

    fn mul(self, other: Point<T>) -> Self::Output {
        // let mut result =
        //     Point::new(Coordinate::Infinity, Coordinate::Infinity, other.a, other.b).unwrap();

        // for _ in 0..self {
        //     result = (result + other).unwrap();
        // }
        // result

        binary_expansion(other, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::real_value::RealValue;

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
        let point_res = Point::new::<Coordinate<RealValue>, i32>(
            Coordinate::Infinity,
            Coordinate::Infinity,
            5,
            7,
        );
        assert!(point_res.is_ok());
    }

    #[test]
    fn cannot_add_points_in_different_curves() {
        let p1 = Point::new(-1, 1, 5, 7).unwrap();
        let p2 = Point::new(0, 1, 1, 1).unwrap();
        let res = p1 + p2;
        assert!(res.is_err());
    }

    #[test]
    fn adding_points_in_vertical_line_returns_infinity_point() {
        let p1 = Point::new(-1, 1, 5, 7).unwrap();
        let p2 = Point::new(-1, -1, 5, 7).unwrap();
        let res = p1 + p2;
        let expected = Point::new(Coordinate::Infinity, Coordinate::Infinity, 5, 7).unwrap();
        assert!(res.is_ok_and(|x| x == expected));
    }

    #[test]
    fn adding_point_with_identity_point_returns_same_point() {
        let p1 = Point::new(-1, -1, 5, 7).unwrap();
        let p2 = Point::new(Coordinate::Infinity, Coordinate::Infinity, 5, 7).unwrap();
        let res = p1 + p2;
        assert!(res.is_ok_and(|x| x == p1));
    }

    #[test]
    fn adding_point_with_other_point_returns_correct_result() {
        let p1 = Point::new(2, 5, 5, 7).unwrap();
        let p2 = Point::new(-1, -1, 5, 7).unwrap();
        let expected = Point::new(3, -7, 5, 7).unwrap();
        let res = p1 + p2;
        assert!(res.is_ok_and(|x| x == expected));
    }

    #[test]
    fn adding_point_with_same_point_on_y_different_to_0_returns_correct_result() {
        let p1 = Point::new(-1, -1, 5, 7).unwrap();
        let p2 = Point::new(-1, -1, 5, 7).unwrap();
        let expected = Point::new(18, 77, 5, 7).unwrap();
        let res = p1 + p2;
        assert!(res.is_ok_and(|x| x == expected));
    }

    #[test]
    fn adding_point_with_same_point_on_y_equal_to_0_returns_infinity_point() {
        let p1 = Point::new(-1, 0, 5, 6).unwrap();
        let p2 = Point::new(-1, 0, 5, 6).unwrap();
        let expected = Point::new(Coordinate::Infinity, Coordinate::Infinity, 5, 6).unwrap();
        let res = p1 + p2;
        assert!(res.is_ok_and(|x| x == expected));
    }

    #[test]
    fn can_create_point_from_finite_field() {
        let p1 = Point::from_finite_field(1, 3, 10, 9, 11);
        assert!(p1.is_ok());
    }

    #[test]
    fn test_valid_points_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        let valid_points = [(192, 105), (17, 56), (1, 193)];

        for (x_raw, y_raw) in valid_points {
            let x = FieldElement::new(x_raw, prime).unwrap();
            let y = FieldElement::new(y_raw, prime).unwrap();
            let p = Point::new(x, y, a, b);
            assert!(p.is_ok());
        }
    }
    #[test]
    fn test_invalid_points_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        let invalid_points = [(200, 119), (42, 99)];

        for (x_raw, y_raw) in invalid_points {
            let x = FieldElement::new(x_raw, prime).unwrap();
            let y = FieldElement::new(y_raw, prime).unwrap();
            let p = Point::new(x, y, a, b);
            assert!(p.is_err());
        }
    }

    #[test]
    fn test_add_1() {
        let prime = 223;
        let a = 0;
        let b = 7;
        let p1 = Point::from_finite_field(170, 142, a, b, prime).unwrap();
        let p2 = Point::from_finite_field(60, 139, a, b, prime).unwrap();
        let res = p1 + p2;
        assert!(res.is_ok());
    }

    #[test]
    fn test_add_2() {
        let prime = 223;
        let a = 0;
        let b = 7;
        let p1 = Point::from_finite_field(47, 71, a, b, prime).unwrap();
        let p2 = Point::from_finite_field(17, 56, a, b, prime).unwrap();
        let res = p1 + p2;
        assert!(res.is_ok());
    }

    #[test]
    fn test_add_3() {
        let prime = 223;
        let a = 0;
        let b = 7;
        let p1 = Point::from_finite_field(143, 98, a, b, prime).unwrap();
        let p2 = Point::from_finite_field(76, 66, a, b, prime).unwrap();
        let res = p1 + p2;
        assert!(res.is_ok());
    }

    #[test]
    fn scalar_multiplication_with_zero_is_point_at_infinity() {
        let prime = 223;
        let x = FieldElement::new(15, prime).unwrap();
        let y = FieldElement::new(86, prime).unwrap();
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let point = Point::new(x, y, a, b).unwrap();
        let expected = Point::new(Coordinate::Infinity, Coordinate::Infinity, a, b).unwrap();
        let res = 0 * point;

        assert_eq!(res, expected);
    }

    #[test]
    fn scalar_multiplication_with_non_zero_is_correct() {
        let prime = 223;
        let x = FieldElement::new(47, prime).unwrap();
        let y = FieldElement::new(71, prime).unwrap();
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let point = Point::new(x, y, a, b).unwrap();
        let res = 2 * point;
        let x_expected = FieldElement::new(36, prime).unwrap();
        let y_expected = FieldElement::new(111, prime).unwrap();
        let expected = Point::new(x_expected, y_expected, a, b).unwrap();

        assert_eq!(res, expected);
    }

    #[test]
    fn scalar_multiplication_with_group_order_is_point_at_infinity() {
        let prime = 223;
        let x = FieldElement::new(47, prime).unwrap();
        let y = FieldElement::new(71, prime).unwrap();
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let point = Point::new(x, y, a, b).unwrap();
        let expected = Point::new(Coordinate::Infinity, Coordinate::Infinity, a, b).unwrap();
        let res = 21 * point;

        assert_eq!(res, expected);
    }
}
