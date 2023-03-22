//! Linear functions.

use std::{fmt::Display, convert::TryFrom, ops::*, cmp::*};
use crate::math::general::NumTools;
use super::{quadratic::QuadraticEquation, polynomial::Polynomial};

#[derive(Clone, Copy, Debug, PartialEq)]
/// The struct for storing linear functions of the form `f(x) = mx + c`.
pub struct LinearEquation<T> {
    pub(crate) m:    T,
    pub(crate) c:    T,
    root:            Option<T>
}

impl<T: Copy +
        Clone +
        From<u8> +
        TryFrom<f64> +
        Div<Output = T> +
        Sub<Output = T> +
        Neg<Output = T> +
        Mul<Output = T> +
        Add<Output = T> +
        PartialEq +
        PartialOrd +
        NumTools<T>> LinearEquation<T>
        where
        <T as TryFrom<f64>>::Error: std::fmt::Debug {
    /// Create a new `LinearEquation`.
    /// # Arguments
    /// * `m: T` - The slope of the function.
    /// * `c: T` - The y-intercept of the function.
    /// # Returns
    /// A new `LinearEquation`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let f_x = LinearEquation::new(1.5, -2.2);
    /// 
    /// assert_eq!("1.5x - 2.2".to_owned(), f_x.to_string());
    /// ```
    #[inline]
    #[must_use]
    pub fn new(m: T, c: T) -> LinearEquation<T> {
        LinearEquation { m, c, root: None }
    }
    /// Get the slope of a `LinearEquation`.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let f_x = LinearEquation::new(1.5, -2.2);
    /// 
    /// assert_eq!(1.5, f_x.m());
    /// ```
    #[inline]
    #[must_use]
    pub fn m(&self) -> T {
        self.m
    }
    /// Get the y-intercept of a `LinearEquation`.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let f_x = LinearEquation::new(1.5, -2.2);
    /// 
    /// assert_eq!(-2.2, f_x.c());
    /// ```
    #[inline]
    #[must_use]
    pub fn c(&self) -> T {
        self.c
    }
    /// set the slope of a `LinearEquation`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut f_x = LinearEquation::new(1.5, -2.2);
    /// 
    /// assert_eq!(1.5, f_x.m());
    /// 
    /// f_x.set_m(1.1);
    /// 
    /// assert_eq!(1.1, f_x.m());
    /// ```
    #[inline]
    pub fn set_m(&mut self, value: T) {
        self.root = None;
        self.m = value
    }
    /// set the y-intercept of a `LinearEquation`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut f_x = LinearEquation::new(1.5, -2.2);
    /// 
    /// assert_eq!(-2.2, f_x.c());
    /// 
    /// f_x.set_c(3.14);
    /// 
    /// assert_eq!(3.14, f_x.c());
    /// ```
    #[inline]
    pub fn set_c(&mut self, value: T) {
        self.root = None;
        self.c = value;
    }
    /// Get the x-coordinate of the root of a `LinearEquation`.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut f_x = LinearEquation::new(1.0, -2.0);
    /// 
    /// assert_eq!(2.0, f_x.get_root());
    /// ```
    #[inline]
    pub fn get_root(&mut self) -> T {
        if self.root.is_some()
        { return unsafe { self.root.unwrap_unchecked() }; }

        self.root = Some(- self.c / self.m);
        unsafe { self.root.unwrap_unchecked() }
    }
    /// Get the value of a value `x` under the function of the `LinearEquation`.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut f_x = LinearEquation::new(1.0, -2.0);
    /// 
    /// assert_eq!(-1.0, f_x.eval(1.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn eval(&self, x: T) -> T {
        self.m * x + self.c
    }
    /// Get the intersection point between `self` and `other` if there is some.
    /// # Arguments
    /// * `self`.
    /// * `other: &LinearEquation`.
    /// # Returns
    /// A `Option<(T, T)>` tuple.
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut f_x = LinearEquation::new(1.5, -2.0);
    /// let mut g_x = LinearEquation::new(-2.0, 5.0);
    /// 
    /// assert_eq!(Some((2.0, 1.0)), f_x.intsect_with(&g_x));
    #[inline]
    #[must_use]
    pub fn intsect_with(&self, other: &LinearEquation<T>) -> Option<(T, T)> {
        if self.m == other.m
        { return None; }
        let x: T = (other.c - self.c) / (self.m - other.m);
        let y: T = self.m * x + self.c;
        Some((x, y))
    }

    /// Get the intersection point(s) between `self` and a quadratic equation if there is some.
    /// # Arguments
    /// * `self`.
    /// * `other: &QuadraticEquation`.
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
    ///             f_x.intsect_with_quadratic(&g_x));
    /// ```
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// use lib_rapid::math::equations::quadratic::QuadraticEquation;
    /// 
    /// let mut f_x = LinearEquation::new(0.0, 1.0);
    /// let mut g_x = QuadraticEquation::new_from_coefficients(1.0, 0.0, 1.0);
    /// 
    /// assert_eq!( ( Some((0.0, 1.0)),
    ///               None ),
    ///             f_x.intsect_with_quadratic(&g_x));
    /// ```
    #[inline]
    #[must_use]
    pub fn intsect_with_quadratic(&self, other: &QuadraticEquation<T>)
    -> (Option<(T, T)>, Option<(T, T)>) where f64: From<T> {
        let mut q: QuadraticEquation<T> =
        QuadraticEquation::new_from_coefficients(other.a(),
                                                 other.b() - self.m,
                                                 other.c() - self.c);

        if q.get_solutions() == (None, None)
        { return (None, None); }

        let q0: T = unsafe { q.get_solutions().0.unwrap_unchecked() };
        let q1: Option<T> = q.get_solutions().1;
        let mut res: (Option<(T, T)>,
                      Option<(T, T)>) =
        ( Some((q0, self.eval(q0))),
          None );

        if q1.is_some() { 
            let uq: T = unsafe { q1.unwrap_unchecked() };
            res.1 = Some((uq, self.eval(uq)));
        }
        res
    }
    /// Constructs the inverse function of `self`.
    /// # Returns
    /// A new `LinearEquation<T>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::equations::linear::LinearEquation;
    /// 
    /// let mut eq     = LinearEquation::new(5.0,  1.0); // 5x + 1.
    /// let mut inv_eq = LinearEquation::new(0.2, -0.2); // ⅕x - ⅕.
    /// inv_eq.get_root();
    ///     eq.get_root();
    /// 
    /// assert_eq!(eq.get_inv(), inv_eq);
    /// assert_eq!(inv_eq.get_inv(), eq);
    /// ```
    #[must_use]
    #[inline]
    pub fn get_inv(&self) -> LinearEquation<T> {
        LinearEquation { m: self.m.recip(), c: -self.c / self.m, root: Some(self.c) }
    }
}

impl<T: From<u8> +
        Display +
        Neg<Output = T> +
        PartialOrd +
        Copy> std::fmt::Display for LinearEquation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.c > T::from(0) {
            true  => { write!(f, "{}x + {}", self.m, self.c) }
            false => { write!(f, "{}x - {}", self.m, - self.c) }
        }
    }
}

impl<const C: usize, T: Add<Output = T> +
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
                        TryFrom<f64> +
                        Display> From<Polynomial<C, T>> for LinearEquation<T>
                        where f64: From<T>,
                        <T as std::convert::TryFrom<f64>>::Error: std::fmt::Debug {
    fn from(val: Polynomial<C, T>) -> Self {
        if C > 2
        { panic!("Could not convert because coefficients were more than 2."); }

        LinearEquation { m:    val.get_coefficients()[0],
                         c:    val.get_coefficients()[1],
                         root: None
        }
    }
}