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
/// # Arithmetic operations
/// ```
/// use lib_rapid::math::equations::polynomial::Polynomial;
/// 
/// let p: Polynomial<usize> = Polynomial::new_from_coefficients(vec![1, 3, 5, 2]);
/// let q: Polynomial<usize> = Polynomial::new_from_coefficients(vec![0, 2, 1, 3]);
/// 
/// let commutative_coefficients = &vec![1, 5, 6, 5];
/// 
/// // Commutativity is given:
/// assert_eq!((p.clone() + q.clone()).get_coefficients(), commutative_coefficients);
/// assert_eq!((q         + p        ).get_coefficients(), commutative_coefficients);
/// 
/// // Arbitrary length arithmetic is given:
/// let a: Polynomial<usize> = Polynomial::new_from_coefficients(vec![1, 3, 5, 2]);
/// let b: Polynomial<usize> = Polynomial::new_from_coefficients(vec![0, 2, 1, 3, 5]);
/// 
/// let arbitrary_len_coefficients = &vec![1, 5, 6, 5, 5];
/// 
/// assert_eq!((a.clone() + b.clone()).get_coefficients(), arbitrary_len_coefficients);
/// assert_eq!((b         + a        ).get_coefficients(), arbitrary_len_coefficients);
/// ```
/// ```
/// use lib_rapid::math::equations::polynomial::Polynomial;
/// 
/// let p: Polynomial<isize> = Polynomial::new_from_coefficients(vec![1, -3, 5, 2]);
/// 
/// assert_eq!(&p.to_string(), "1 - 3x + 5x^2 + 2x^3");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial<T> {
    coefficients: Vec<T>, // I have to use `Vec` since I have not yet found a way to add 2 polynomials with arbitrary length.
    solutions:    Option<Vec<Option<T>>>
}

impl<T> Polynomial<T> {
    /// Gets the degree of a `Polynomial`.
    /// # Returns
    /// A `usize` equivalent to the degree.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::polynomial::Polynomial;
    /// 
    /// let p: Polynomial<usize> = Polynomial::new_from_coefficients(vec![1, 3, 5, 2]);
    /// assert_eq!(p.get_degree(), 3);
    /// ```
    pub fn get_degree(&self) -> usize {
        self.coefficients.len() - 1
    }
    /// Gets the coefficients of a `Polynomial`.
    /// # Returns
    /// A `Vec`-Slice over all coefficients.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::polynomial::Polynomial;
    /// 
    /// let coefficients         = vec![1, 3, 5, 2];
    /// let p: Polynomial<usize> = Polynomial::new_from_coefficients(coefficients.clone());
    /// assert_eq!(p.get_coefficients(), &coefficients);
    /// ```
    pub fn get_coefficients(&self) -> &Vec<T> {
        &self.coefficients
    }
}

impl<T: From<u8> +
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
        std::fmt::Display> Polynomial<T> {
    /// Create a new standard polynomial with *c* coefficients set to `1` and of *c - 1*th degree.
    /// # Arguments
    /// `c: usize` - The number of coefficients.
    /// # Returns
    /// A new `Polynomial<T>`
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::polynomial::Polynomial;
    /// 
    /// let p: Polynomial<f32> = Polynomial::new(4);
    /// assert!(p.get_degree() == 3);
    /// ```
    pub fn new(c: usize) -> Polynomial<T> {
        Polynomial {
            coefficients: vec![1.into(); c],
            solutions:    None
        }
    }
    /// Create a new polynomial with custom coefficients.
    /// # Arguments
    /// * `coefficients: vec<T>` - The coefficients of the polynomial.
    /// # Returns
    /// A new `Polynomial<T>`
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::polynomial::Polynomial;
    /// 
    /// let p: Polynomial<usize> = Polynomial::new_from_coefficients(vec![1, 3, 5, 2]);
    /// assert!(p.get_degree() == 3);
    /// ```
    pub fn new_from_coefficients(coefficients: Vec<T>) -> Polynomial<T> {
        Polynomial {
            coefficients,
            solutions: None
        }
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
    /// let p: Polynomial<usize> = Polynomial::new_from_coefficients(vec![1, 3, 5, 2]); // Define p(x) as 1 + 3x + 5x² + 2x³
    /// assert_eq!(p.eval(4), 221); // Because p(4) = 1 + 3×4 + 5×4² + 2×4³ = 221
    /// ```
    pub fn eval(&self, x: T) -> T {
        let mut result: T = 0u8.into();

        for (exponent, coefficient) in self.coefficients.iter().enumerate() {
            
            result.inc_by(*coefficient * x.pow(exponent));
        }
        result
    }

    pub fn get_solutions(&mut self) -> &Option<Vec<Option<T>>>
    where
    f64: From<T> + TryInto<T>,
    T:   Neg<Output = T> + TryFrom<f64>,
    <f64 as std::convert::TryInto<T>>::Error: Debug,
    <T as std::convert::TryFrom<f64>>::Error: Debug {
        if self.solutions.is_some()
        { return &self.solutions; }

        let mut res = Vec::with_capacity(self.get_degree());

        if self.get_degree() == 2 {
            let discriminant = self.coefficients[1].square() -
                               T::from(4) * self.coefficients[0] * self.coefficients[2];
            if discriminant.is_negative()
            { return &None; }
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
            return &self.solutions;
        }
        if self.get_degree() == 1 {
            res[0] = Some(- self.coefficients[1] / self.coefficients[0]);
            return &self.solutions;
        }
        &None
    }
}

impl<T: std::ops::AddAssign + Copy> Add for Polynomial<T> {
    type Output = Polynomial<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Polynomial {
            coefficients: {
                let degree_self_geq_rhs = self.get_degree() >= rhs.get_degree();
                let mut res_coeff = if degree_self_geq_rhs { self.coefficients.clone() } else { rhs.coefficients.clone() };
                for (index, i) in (if degree_self_geq_rhs {rhs} else {self}).coefficients.iter().enumerate() {
                    res_coeff[index] += *i;
                }
                res_coeff
            },
            solutions: None
        }
    }
}

impl<T: std::ops::Sub<Output = T> +
        Copy +
        PartialEq +
        From<u8>> Sub for Polynomial<T> {
    type Output = Polynomial<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res_coeff: Vec<T> = Vec::with_capacity(self.get_degree() + 1);
        for (index, i) in rhs.coefficients.iter().enumerate() {
            res_coeff.push(self.coefficients[index] - *i)
        }
        let mut _deg = self.get_degree();
        for i in &res_coeff {
            if i == &0u8.into() {
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

impl<T: Copy +
        Clone +
        PartialEq +
        Sub<Output = T> +
        SubAssign +
        From<u8>> SubAssign for Polynomial<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl<T: Copy +
        Clone +
        PartialEq +
        Add<Output = T> +
        AddAssign +
        From<u8>> AddAssign for Polynomial<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}
/// Converts a polynomial into a `String.`
/// # Examples
/// ```
/// use lib_rapid::math::equations::polynomial::Polynomial;
/// 
/// let p: Polynomial<isize> = Polynomial::new_from_coefficients(vec![1, -3, 5, 2]);
/// 
/// assert_eq!(&p.to_string(), "1 - 3x + 5x^2 + 2x^3");
/// ```
impl<T: Display +
        PartialOrd +
        From<u8> +
        Copy> std::fmt::Display for Polynomial<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::with_capacity(self.get_degree() * 4);

        for (exp, coeff) in self.coefficients.iter().enumerate() {
            if coeff == &0.into()
            { continue; }
            if exp == 0
            { res.push_str(&format!("{}", &coeff.to_string())); continue; }

            if coeff < &0u8.into()
            { res.push_str(&format!(" - {}x", &coeff.to_string()[1..coeff.to_string().len()])); }
            else
            { res.push_str(&format!(" + {}x", coeff)); }

            if exp != 1
            { res.push_str(&format!("^{}", exp)); }
        }
        
        write!(f, "{}", res)
    }
}