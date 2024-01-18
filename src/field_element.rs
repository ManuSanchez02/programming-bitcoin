use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{pow::Pow, is_zero::IsZero};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct FieldElement {
    pub number: u32,
    pub prime: u32,
}

impl FieldElement {
    pub fn new(number: u32, prime: u32) -> Result<Self, String> {
        if number >= prime {
            return Err(format!(
                "Number {} not in field range 0 to {}",
                number,
                prime - 1
            ));
        }

        Ok(FieldElement { number, prime })
    }

    fn positive_pow(&self, power: i32) -> Self {
        let mut number = 1;
        for _ in 0..power {
            number *= self.number;
            number = number.rem_euclid(self.prime);
        }

        Self {
            number,
            prime: self.prime,
        }
    }

    fn negative_pow(&self, power: i32) -> Self {
        let equivalent_power = power.rem_euclid(self.prime as i32 - 1);
        dbg!(equivalent_power);
        return self.positive_pow(equivalent_power);
    }

    pub fn inverse(&self) -> Self {
        self.pow(self.prime as i32 - 2)
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "FieldElement_{}({})", self.number, self.prime)
    }
}

impl Pow for FieldElement {
    fn pow(&self, exp: i32) -> Self {
        if exp.is_positive() {
            self.positive_pow(exp)
        } else {
            self.negative_pow(exp)
        }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!(
                "Elements {} and {} have different prime fields",
                self, other
            );
        }

        let number = (self.number + other.number) % self.prime;

        Self {
            number,
            prime: self.prime,
        }
    }
}

impl Add<i32> for FieldElement {
    type Output = Self;

    fn add(self, other: i32) -> Self::Output {
        let number = (self.number as i32 + other).rem_euclid(self.prime as i32);
        let number = number as u32;

        Self {
            number,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!(
                "Elements {} and {} have different prime fields",
                self, other
            );
        }

        let number = (self.number * other.number) % self.prime;

        Self {
            number,
            prime: self.prime,
        }
    }
}

impl Mul<i32> for FieldElement {
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        let number = (self.number as i32 * other).rem_euclid(self.prime as i32);
        let number = number as u32;

        Self {
            number,
            prime: self.prime,
        }
    }
}

impl Neg for FieldElement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let opposite = self.number as i32 * -1;
        let number = opposite.rem_euclid(self.prime as i32);

        Self {
            number: number as u32,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self * other.inverse()
    }
}

impl IsZero for FieldElement {
    fn is_zero(&self) -> bool {
        self.number == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_create_element_with_number_higher_than_prime() {
        assert!(FieldElement::new(2, 1).is_err());
    }

    #[test]
    fn can_create_element() {
        assert!(FieldElement::new(1, 2).is_ok());
    }

    #[test]
    fn elements_with_same_number_but_different_prime_are_different() {
        let element1 = FieldElement::new(1, 2).unwrap();
        let element2 = FieldElement::new(1, 3).unwrap();
        assert_ne!(element1, element2);
    }

    #[test]
    fn elements_with_same_prime_but_different_number_are_different() {
        let element1 = FieldElement::new(1, 2).unwrap();
        let element2 = FieldElement::new(1, 3).unwrap();
        assert_ne!(element1, element2);
    }

    #[test]
    fn elements_with_same_prime_and_number_are_equal() {
        let element1 = FieldElement::new(1, 3).unwrap();
        let element2 = FieldElement::new(1, 3).unwrap();
        assert_eq!(element1, element2);
    }

    #[test]
    #[should_panic]
    fn cannot_add_elements_with_different_prime() {
        let element1 = FieldElement::new(1, 2).unwrap();
        let element2 = FieldElement::new(1, 3).unwrap();
        let _ = element1 + element2;
    }

    #[test]
    fn can_add_elements_with_same_prime() {
        let element1 = FieldElement::new(1, 3).unwrap();
        let element2 = FieldElement::new(1, 3).unwrap();
        let expected = FieldElement::new(2, 3).unwrap();
        let result = element1 + element2;
        assert_eq!(result, expected);
    }

    #[test]
    fn can_add_elements_with_same_prime_that_overflow() {
        let element1 = FieldElement::new(2, 3).unwrap();
        let element2 = FieldElement::new(2, 3).unwrap();
        let expected = FieldElement::new(1, 3).unwrap();
        let result = element1 + element2;
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn cannot_substract_elements_with_different_prime() {
        let element1 = FieldElement::new(1, 2).unwrap();
        let element2 = FieldElement::new(1, 3).unwrap();
        let _ = element1 - element2;
    }

    #[test]
    fn can_substract_elements_with_same_prime() {
        let element1 = FieldElement::new(1, 3).unwrap();
        let element2 = FieldElement::new(1, 3).unwrap();
        let expected = FieldElement::new(0, 3).unwrap();
        let result = element1 - element2;
        assert_eq!(result, expected);
    }

    #[test]
    fn can_substract_elements_with_same_prime_that_underflow() {
        let element1 = FieldElement::new(1, 3).unwrap();
        let element2 = FieldElement::new(2, 3).unwrap();
        let expected = FieldElement::new(2, 3).unwrap();
        let result = element1 - element2;
        assert_eq!(result, expected);
    }

    #[test]
    fn adding_element_and_additive_inverse_is_0() {
        let element1 = FieldElement::new(1, 3).unwrap();
        let element2 = -element1;
        let expected = FieldElement::new(0, 3).unwrap();
        let result = element1 + element2;
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn cannot_multiply_elements_with_different_prime() {
        let element1 = FieldElement::new(1, 2).unwrap();
        let element2 = FieldElement::new(1, 3).unwrap();
        let _ = element1 * element2;
    }

    #[test]
    fn can_multiply_elements_with_same_prime() {
        let element1 = FieldElement::new(1, 3).unwrap();
        let element2 = FieldElement::new(2, 3).unwrap();
        let expected = FieldElement::new(2, 3).unwrap();
        let result = element1 * element2;
        assert_eq!(result, expected);
    }

    #[test]
    fn can_multiply_elements_with_same_prime_that_overflow() {
        let element1 = FieldElement::new(2, 3).unwrap();
        let element2 = FieldElement::new(2, 3).unwrap();
        let expected = FieldElement::new(1, 3).unwrap();
        let result = element1 * element2;
        assert_eq!(result, expected);
    }

    #[test]
    fn multiplying_by_0_results_in_0() {
        let element1 = FieldElement::new(2, 3).unwrap();
        let element2 = FieldElement::new(0, 3).unwrap();
        let expected = FieldElement::new(0, 3).unwrap();
        let result = element1 * element2;
        assert_eq!(result, expected);
    }

    #[test]
    fn can_raise_element_to_power_of() {
        let element = FieldElement::new(3, 13).unwrap();
        let expected = FieldElement::new(1, 13).unwrap();
        assert_eq!(element.pow(3), expected);
    }

    #[test]
    fn fermat_theorem() {
        let prime = 31;
        let set_res: Result<Vec<FieldElement>, String> =
            (0..prime).map(|x| FieldElement::new(x, prime)).collect();
        let set = set_res.unwrap();
        let mut set_power = set.iter().map(|x| x.pow(prime as i32 - 1));

        set_power.next();
        for elem in set_power {
            assert_eq!(elem.number, 1);
        }
    }

    #[test]
    #[should_panic]
    fn cannot_divide_elements_with_different_prime() {
        let element1 = FieldElement::new(1, 2).unwrap();
        let element2 = FieldElement::new(1, 3).unwrap();
        let _ = element1 / element2;
    }

    #[test]
    fn can_divide_elements_with_denominator_bigger_than_numerator() {
        let element1 = FieldElement::new(2, 19).unwrap();
        let element2 = FieldElement::new(7, 19).unwrap();
        let expected = FieldElement::new(3, 19).unwrap();
        let result = element1 / element2;
        assert_eq!(result, expected);
    }

    #[test]
    fn can_divide_elements_with_numerator_bigger_than_denominator() {
        let element1 = FieldElement::new(7, 19).unwrap();
        let element2 = FieldElement::new(5, 19).unwrap();
        let expected = FieldElement::new(9, 19).unwrap();
        let result = element1 / element2;
        assert_eq!(result, expected);
    }

    #[test]
    fn can_raise_element_to_power_of_negative_exponent() {
        let element = FieldElement::new(7, 13).unwrap();
        let expected = FieldElement::new(8, 13).unwrap();
        assert_eq!(element.pow(-3), expected);
    }
}
