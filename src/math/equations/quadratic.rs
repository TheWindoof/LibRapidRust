//! Quadratic functions.
use std::{fmt::Display, convert::*, ops::*};
use crate::math::general::NumTools;
use super::{linear::LinearEquation, polynomial::Polynomial};
/// A struct for storing quadratic equations of the form `f(x) = ax² + bx + c`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadraticEquation<T> {
    pub(crate) a:          T,
    pub(crate) b:          T,
    pub(crate) c:          T,
    vertex:                Option<(T, T)>,
    solutions:             (Option<T>, Option<T>),
    derivative:            Option<LinearEquation<T>>
}

impl<T: Copy +
        Clone +
        From<u8> +
        TryFrom<f64> +
        PartialEq +
        PartialOrd +
        NumTools<T> +
        Mul<Output = T> +
        Add<Output = T> +
        Sub<Output = T> +
        Div<Output = T> +
        Neg<Output = T>> QuadraticEquation<T>
        where <T as TryFrom<f64>>::Error: std::fmt::Debug,
              f64: From<T> {
    /// Create a new `QuadraticEquation` with the values `a = 1, b = 0, c = 0`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, 0.0, 0.0);
    /// f_x.get_vertex();
    /// f_x.get_solutions();
    /// 
    /// assert_eq!(QuadraticEquation::new(), f_x);
    /// ```
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, 0.0, -1.5);
    /// 
    /// assert_eq!("1x^2 + 0x - 1.5", &f_x.to_string());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> QuadraticEquation<T> {
        QuadraticEquation { a:          T::from(1),
                            b:          T::from(0),
                            c:          T::from(0),
                            vertex:     Some((T::from(0), T::from(0))),
                            solutions:  (Some(T::from(0)), None),
                            derivative: None }
    }
    /// Create a new `QuadraticEquation` from coefficients.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, 0.0, 0.0);
    /// f_x.get_vertex();
    /// f_x.get_solutions();
    /// 
    /// assert_eq!(QuadraticEquation::new(), f_x);
    /// ```
    #[inline]
    #[must_use]
    pub fn new_from_coefficients(a: T, b: T, c: T) -> QuadraticEquation<T> {
        if a == T::from(0)
        { panic!("a was zero and is thus not allowed."); }
        QuadraticEquation { a,
                            b,
                            c,
                            vertex:     None,
                            solutions: (None, None),
                            derivative: None }
    }
    /// Get `a` of a `QuadraticEquation`.
    /// # Returns
    /// A `T.`
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(-2.0, f_x.b());
    /// ```
    #[inline]
    #[must_use]
    pub fn a(&self) -> T {
        self.a
    }
    /// Get `b` of a `QuadraticEquation`.
    /// # Returns
    /// A `T.`
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(-2.0, f_x.b());
    /// ```
    #[inline]
    #[must_use]
    pub fn b(&self) -> T {
        self.b
    }
    /// Get `c` of a `QuadraticEquation`.
    /// # Returns
    /// A `T.`
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(-3.0, f_x.c());
    /// ```
    #[inline]
    #[must_use]
    pub fn c(&self) -> T {
        self.c 
    }
    /// Set `c` of a `QuadraticEquation`.
    /// # Panics
    /// Panics if `value` is zero.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(1.0, f_x.a());
    /// 
    /// f_x.set_a(-1.0);
    /// 
    /// assert_eq!(-1.0, f_x.a());
    /// ```
    #[inline]
    pub fn set_a(&mut self, value: T) {
        if value == T::from(0)
        { panic!("a was zero and is thus not allowed."); }
        self.solutions = (None, None);
        self.derivative = None;
        self.a = value;
    }
    /// Set `b` of a `QuadraticEquation`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(-2.0, f_x.b());
    /// 
    /// f_x.set_b(-1.0);
    /// 
    /// assert_eq!(-1.0, f_x.b());
    /// ```
    #[inline] 
    pub fn set_b(&mut self, value: T) {
        self.solutions = (None, None);
        self.derivative = None;
        self.b = value;
    }
    /// Set `c` of a `QuadraticEquation`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!(-3.0, f_x.c());
    /// 
    /// f_x.set_c(-1.0);
    /// 
    /// assert_eq!(-1.0, f_x.c());
    /// ```
    #[inline]
    pub fn set_c(&mut self, value: T) {
        self.solutions = (None, None);
        self.c = value;
    }
    /// Get the solutions of a quadratic equation.
    /// # Returns
    /// A `(Option<T>, Option<T>)`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, -3.0);
    /// 
    /// assert_eq!((Some(3.0), Some(-1.0)), f_x.get_solutions());
    /// ```
    #[inline]
    #[must_use]
    pub fn get_solutions(&mut self) -> (Option<T>, Option<T>) {
        if self.solutions != (None, None)
        { return self.solutions; }
        let discriminant = self.b.square() - T::from(4) * self.a * self.c;
        if discriminant < T::from(0)
        { return (None, None); }
        let x_1 = (- self.b + (f64::from(discriminant).sqrt()).try_into().unwrap()) /
                  (T::from(2) * self.a);

        let x_2 = (- self.b - T::try_from(f64::from(discriminant).sqrt()).unwrap()) /
                  (T::from(2) * self.a);
        
        match x_1 == x_2 {
            true  => { self.solutions = (Some(x_1), None); }
            false => { self.solutions = (Some(x_1), Some(x_2)); }
        }
        self.solutions
    }
    /// Get the intersection point(s) between `self` and `other`.
    /// Returns `(None, None)` if both arguments are equal.
    /// # Arguments
    /// * `self`.
    /// * `other: &QuadraticEquation`.
    /// # Returns
    /// A `(Option<(T, T)>, Option<(T, T)>)` tuple.
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, 2.0, 0.0);
    /// let mut g_x = QuadraticEquation::new_from_coefficients(1.0, 2.0, 0.0);
    /// let mut h_x = QuadraticEquation::new_from_coefficients(0.5, 1.0, 0.0);
    /// 
    /// assert_eq!( (None, None), f_x.intsect_with(&g_x));
    /// assert_eq!( (Some( (0.0, 0.0) ), Some( (-2.0, 0.0) )), g_x.intsect_with(&h_x) );
    /// ```
    #[inline]
    #[must_use]
    pub fn intsect_with(&self, other: &QuadraticEquation<T>)
    -> (Option<(T, T)>, Option<(T, T)>) {
        if self == other
        { return (None, None) }
        let solquad = QuadraticEquation::new_from_coefficients(self.a - other.a,
                                                               self.b - other.b,
                                                               self.c - other.c).get_solutions();
        if solquad == (None, None)
        { return (None, None); }
        let solquad0_unsafe = unsafe { solquad.0.unwrap_unchecked() };
        let mut res = (Some((solquad0_unsafe, self.eval(solquad0_unsafe))), None );
        match solquad {
            (Some(_), None)     => { }
            (Some(_), Some(s1)) => {
                res.1 = Some((s1, self.eval(s1)));
            }
            _                   => { }
        }
        res
    }
    /// Get the intersection point(s) between `self` and a linear equation if there is some.
    /// # Arguments
    /// * `self`.
    /// * `other: &LinearEquation`.
    /// # Returns
    /// A `(Option<(T, T)>, Option<(T, T)>)` tuple.
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = LinearEquation::new(2.0, 2.0);
    /// let mut g_x = QuadraticEquation::new_from_coefficients(1.2, 2.0, -2.0);
    /// 
    /// assert_eq!( ( Some((1.8257418583505536, 5.651483716701107)),
    ///               Some((-1.8257418583505536, -1.6514837167011072)) ),
    ///             g_x.intsect_with_linear(&f_x));
    #[inline]
    #[must_use]
    pub fn intsect_with_linear(&self, other: &LinearEquation<T>)
    -> (Option<(T, T)>, Option<(T, T)>) {
        other.intsect_with_quadratic(self)
    }
    /// Get the vertex (lowest or highest point) of a quadratic equation.
    /// # Returns
    /// A `(T, T)`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, 3.0);
    /// 
    /// assert_eq!((1.0, 2.0), f_x.get_vertex());
    /// ```
    #[inline]
    #[must_use]
    pub fn get_vertex(&mut self) -> (T, T) {
        if self.vertex.is_some()
        { return unsafe { self.vertex.unwrap_unchecked() }; }
        let x = - self. b / (T::from(2) * self.a);
        self.vertex = Some((x, self.a * x.square() + self.b * x + self.c));
        unsafe { self.vertex.unwrap_unchecked() }
    }
    /// Get the value of a value `x` under the function of the `QuadraticEquation`.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, 3.0);
    /// 
    /// assert_eq!(2.0, f_x.eval(1.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn eval(&self, x: T) -> T {
        self.a * x.square() + self.b * x + self.c
    }
    /// Get the derivative of a `QuadraticEquation<T>`. The derivative is the graph of the
    /// development of the slope for a given function `self`.
    /// # Returns
    /// A `LinearEquation<T>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut f_x = QuadraticEquation::new_from_coefficients(1.0, -2.0, 3.0);
    /// 
    /// assert_eq!(LinearEquation::new(2.0, -2.0), f_x.get_derivative());
    /// ```
    #[inline]
    #[must_use = "This returns the result of the operation, without modifying the original."]
    pub fn get_derivative(&mut self) -> LinearEquation<T> {
        match self.derivative {
            Some(d) => { return d; }
            None    => { self.derivative = Some(LinearEquation::new(T::from(2) * self.a, self.b)); }
        }

        unsafe { self.derivative.unwrap_unchecked() }
    }
}

impl<T: Display +
        std::ops::Neg<Output = T> +
        PartialOrd +
        From<u8> +
        Copy> std::fmt::Display for QuadraticEquation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::with_capacity(22);
        res.push_str(&format!("{}x^2", self.a));
        if self.b < T::from(0)
        { res.push_str(&format!(" - {}x", self.b)); }
        else
        { res.push_str(&format!(" + {}x", self.b)); }
        if self.c < T::from(0)
        { res.push_str(&format!(" - {}", - self.c)); }
        else
        { res.push_str(&format!(" + {}", self.c)); }
        write!(f, "{}", res)
    }
}

impl<T: Add<Output = T> +
        Sub<Output = T> +
        Mul<Output = T> +
        Div<Output = T> +
        PartialOrd +
        Neg<Output = T> +
        From<u8> +
        Copy +
        SubAssign +
        AddAssign +
        MulAssign +
        TryFrom<f64> +
        Display, const C: usize> From<Polynomial<C, T>> for QuadraticEquation<T> {
    fn from(val: Polynomial<C, T>) -> Self {
        if C > 3
        { panic!("Could not convert because coefficients were more than 3."); }

        QuadraticEquation { a:          val.get_coefficients()[0],
                            b:          val.get_coefficients()[1],
                            c:          val.get_coefficients()[2],
                            vertex:     None,
                            solutions:  (None, None),
                            derivative: None
        }
    }
}