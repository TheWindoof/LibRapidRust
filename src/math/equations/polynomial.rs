//! Polynomials in Rust.
// * TODO: Finding roots.
// * TODO: Summation/Difference of Polynomials with arbitrary length.
// * TODO: Macro for fast creation.

use std::{ops::{Add, Sub, SubAssign, AddAssign, Neg}, fmt::Display, fmt::Debug, convert::TryInto, convert::TryFrom};

use crate::math::general::NumTools;
/// The struct for storing and evaluating polynomials.
/// # Generics
/// * `const C: usize` - The number of coefficients (degree = C - 1).
/// * `T` - The type of the coefficients.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Polynomial<const C: usize, T> {
    coefficients: [T; C],
    solutions:    Option<[Option<T>; C]>
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
                        std::ops::Add + 
                        std::fmt::Display> Polynomial<C, T> {
    /// Create a new standard polynomial with *C* coefficients set to `1` and of *C - 1*th degree.
    /// # Returns
    /// A new `Polynomial<C, T>`
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::polynomial::Polynomial;
    /// 
    /// let p: Polynomial<4, f32> = Polynomial::new();
    /// assert!(p.get_degree() == 3);
    /// ```
    pub fn new() -> Polynomial<C, T> {
        Polynomial {
            coefficients: [1u8.into(); C],
            solutions:    None
        }
    }
    /// Create a new polynomial with *C* custom coefficients.
    /// # Arguments
    /// * `coefficients: [T; C]` - The coefficients of the polynomial.
    /// # Returns
    /// A new `Polynomial<C, T>`
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::polynomial::Polynomial;
    /// 
    /// let p: Polynomial<4, usize> = Polynomial::new_from_coefficients([1, 3, 5, 2]);
    /// assert!(p.get_degree() == 3);
    /// ```
    pub fn new_from_coefficients(coefficients: [T; C]) -> Polynomial<C, T> {
        Polynomial {
            coefficients,
            solutions: None
        }
    }
    /// Gets the coefficients of a `Polynomial`.
    /// # Returns
    /// A `[T; C]`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::polynomial::Polynomial;
    /// 
    /// let p: Polynomial<4, usize> = Polynomial::new_from_coefficients([1, 3, 5, 2]);
    /// assert_eq!(p.get_coefficients(), [1, 3, 5, 2]);
    /// ```
    pub fn get_coefficients(&self) -> [T; C] {
        self.coefficients
    }
    /// Gets the degree of a `Polynomial`.
    /// # Returns
    /// A `[T; C]`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::polynomial::Polynomial;
    /// 
    /// let p: Polynomial<4, usize> = Polynomial::new_from_coefficients([1, 3, 5, 2]);
    /// assert_eq!(p.get_degree(), 3);
    /// ```
    pub fn get_degree(&self) -> usize {
        self.coefficients.len() - 1
    }
    /// Evaluates a `Polynomial` for a given input `x`.
    /// # Arguments
    /// * `x: T` the input for the polynomial.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::polynomial::Polynomial;
    /// 
    /// let p: Polynomial<4, usize> = Polynomial::new_from_coefficients([1, 3, 5, 2]);
    /// assert_eq!(p.eval(4), 134);
    /// ```
    pub fn eval(&self, x: T) -> T {
        let mut result: T = 0u8.into();
        let mut exponent: usize = self.get_degree();

        for coefficient in self.coefficients {
            
            result.inc_by(coefficient * x.pow(exponent));
            if exponent > 0
            { exponent.dec(); }
        }

        result
    }

    pub fn get_solutions(&mut self) -> Option<[Option<T>; C]>
    where
    f64: From<T> + TryInto<T>,
    T:   Neg<Output = T> + TryFrom<f64>,
    <f64 as std::convert::TryInto<T>>::Error: Debug,
    <T as std::convert::TryFrom<f64>>::Error: Debug {
        if self.solutions.is_some()
        { return self.solutions; }

        let mut res = [None; C];

        if self.get_degree() == 2 {
            let discriminant = self.coefficients[1].square() -
                               T::from(4) * self.coefficients[0] * self.coefficients[2];
            if discriminant < T::from(0)
            { return None; }
            let x_1 = (- self.coefficients[1] +
                      (f64::from(discriminant).sqrt()).try_into().unwrap()) /
                      (T::from(2) * self.coefficients[0]);
    
            let x_2 = (- self.coefficients[1] -
                      (f64::from(discriminant).sqrt()).try_into().unwrap()) /
                      (T::from(2) * self.coefficients[0]);

            match x_1 == x_2 {
                true  => { res[0] = Some(x_1); self.solutions = Some(res); }
                false => { res[0] = Some(x_1); res[1] = Some(x_2);
                           self.solutions = Some(res) }
            }
            return self.solutions.clone();
        }
        if self.get_degree() == 1 {
            res[0] = Some(- self.coefficients[1] / self.coefficients[0]);
            return self.solutions;
        }
        None
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
            },
            solutions: None
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
            coefficients: res_coeff,
            solutions:    None
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
/// Converts a polynomial into a `String.`
/// # Examples
/// ```
/// use lib_rapid::math::equations::polynomial::Polynomial;
/// 
/// let p: Polynomial<4, isize> = Polynomial::new_from_coefficients([1, -3, 5, 2]);
/// 
/// assert_eq!(&p.to_string(), "1x^3 - 3x^2 + 5x + 2");
/// ```
impl<const C: usize, T: Display +
        PartialOrd +
        From<u8> +
        Copy> std::fmt::Display for Polynomial<C, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::with_capacity(C * 4);
        let mut current_exponent: usize;
        for (exp, coeff) in self.coefficients.iter().enumerate() {
            current_exponent = self.coefficients.len() - exp - 1;
            if current_exponent == 0
            { break; }
            if coeff < &0u8.into()
            { res.push_str(&format!(" - {}x", &coeff.to_string()[1..coeff.to_string().len()])); }
            else
            { res.push_str(&format!(" + {}x", coeff)); }

            if current_exponent != 1
            { res.push_str(&format!("^{}", current_exponent)); }
        }
        if self.coefficients[self.coefficients.len() - 1] < 0u8.into()
        { res.push_str(&format!(" - {}", &self.coefficients[self.coefficients.len() - 1].to_string()[1..self.coefficients[self.coefficients.len() - 1].to_string().len()])); }
        else
        { res.push_str(&format!(" + {}", self.coefficients[self.coefficients.len() - 1])); }

        
        write!(f, "{}", &res[3..res.len()])
    }
}