//! Polynomials in Rust.
// * TODO: Documentation.
// * TODO: Finding roots.
// * TODO: Summation/Difference of Polynomial&s with arbitrary length.
// * TODO: Macro for fast creation.
// * TODO: Interoperation with QuadraticEquation and LinearEquation.

use std::{ops::{Add, Sub, SubAssign, AddAssign}, fmt::Display};

use crate::math::general::NumTools;

use super::{quadratic::QuadraticEquation, linear::LinearEquation};
/// The struct for storing and evaluating polynomials.
/// # Generics
/// * `const C: usize` - The number of coefficients (degree = C - 1).
/// * `T` - The type of the coefficients.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Polynomial<const C: usize, T> {
    coefficients: [T; C]
}
#[test]
fn test() {
    let p: Polynomial<5, usize> = Polynomial::new();
    assert_eq!(p.coefficients.len(), 5);
    assert_eq!(p.get_degree(), 4);
    assert_eq!(p.coefficients, [1,1,1,1,1]);
    assert_eq!(&p.to_string(), "1x^4 + 1x^3 + 1x^2 + 1x^1 + 1");
    assert_eq!(p.eval(2), 31);
}

impl<const C: usize, T: From<u8> +
                        Copy +
                        std::ops::Sub<Output = T> +
                        std::ops::Add<Output = T> +
                        std::ops::Div<Output = T> +
                        std::ops::Mul<Output = T> +
                        std::ops::Div +
                        std::ops::Mul + 
                        std::ops::SubAssign +
                        std::ops::AddAssign +
                        std::ops::MulAssign +
                        std::cmp::PartialOrd +
                        std::ops::Sub +
                        std::ops::Add> Polynomial<C, T> {
    /// Create a new standard polynomial with *C* coefficients set to `1` and of *C - 1*th degree.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::polynomial::Polynomial;
    /// 
    /// let p: Polynomial<4, T> = Polynomial::new();
    /// assert!(p.get_degree() == 3);
    /// ```
    pub fn new() -> Polynomial<C, T> {
        Polynomial {
            coefficients: [1u8.into(); C]
        }
    }

    pub fn new_from_coefficients(coefficients: [T; C]) -> Polynomial<C, T> {
        Polynomial {
            coefficients
        }
    }

    pub fn get_coefficients(&self) -> [T; C] {
        self.coefficients
    }

    pub fn get_degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn eval(&self, x: T) -> T {
        let mut result: T = 0u8.into();
        let mut exponent: usize = self.get_degree();

        for coefficient in self.coefficients {
            result.inc_by(coefficient * x.pow(exponent));
            if exponent == 0
            { break; }
            exponent.dec();
        }

        result
    }

    pub fn from_quadratic(q: QuadraticEquation<T>) -> Polynomial<3, T> {
        let mut coeff = [0u8.into(); 3];
        coeff[0] = q.a;
        coeff[1] = q.b;
        coeff[2] = q.c;
        Polynomial { coefficients: coeff }
    }

    pub fn from_linear(l: LinearEquation<T>) -> Polynomial<2, T> {
        let mut coeff = [0u8.into(); 2];
        coeff[0] = l.m;
        coeff[1] = l.c;
        Polynomial { coefficients: coeff }
    }
}

impl<const C: usize, T: std::ops::AddAssign + Copy> Add for Polynomial<C, T> {
    type Output = Polynomial<C, T>;

    fn add(self, rhs: Self) -> Self::Output {

        Polynomial {
            coefficients: {
                let mut res_coeff = self.coefficients;
                for (index, i) in rhs.coefficients.iter().enumerate() {
                    res_coeff[index] += *i;
                }
                res_coeff
            }
        }
    }
}

impl<const C: usize, T: std::ops::SubAssign +
                        Copy +
                        PartialEq +
                        From<u8>> Sub for Polynomial<C, T> {
    type Output = Polynomial<C, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res_coeff = self.coefficients;
        for (index, i) in rhs.coefficients.iter().enumerate() {
            res_coeff[index] -= *i;
        }
        let mut _deg = C;
        for i in res_coeff {
            if i == 0u8.into() {
                _deg.dec();
                continue;
            }
            break;
        }
        Polynomial {
            coefficients: res_coeff
        }
    }
}

impl<const C: usize, T: Copy +
                        Clone +
                        PartialEq +
                        Sub<Output = T> +
                        SubAssign +
                        From<u8>> SubAssign for Polynomial<C, T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const C: usize, T: Copy +
                        Clone +
                        PartialEq +
                        Add<Output = T> +
                        AddAssign +
                        From<u8>> AddAssign for Polynomial<C, T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const C: usize, T: Display +
        PartialOrd +
        From<u8> +
        Copy> std::fmt::Display for Polynomial<C, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::with_capacity(C * 4);
        let mut current_exponent: usize;
        for (exp, coeff) in self.coefficients.iter().enumerate() {
            current_exponent = self.coefficients.len() - exp - 1;
            if current_exponent != 0
            { res.push_str(&format!("{}x^{} + ", coeff, current_exponent)); }
        }
        res.push_str(&format!("{}", self.coefficients[self.coefficients.len() - 1]));
        write!(f, "{}", res)
    }
}