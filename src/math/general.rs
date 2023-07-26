//! Traits and functions for general purpose, everyday mathematics.
//! Everything you need.
use std::{convert::{TryInto, TryFrom}, ops::*, cmp::*};
use super::{complex::ComplexNumber, constants::{E, EULERMASCHERONI}};
use crate::eval_postfix;

use super::constants::{SQRT5, GOLDENRATIO};
pub mod avg_impl;
pub mod avg_macros;
/// Trait for several kinds of averages.
pub trait Averages<T> {
    type Output;
    /// Calculate the arithmetic mean.
    /// # Returns
    /// A `f32` by default, except for the `&[f64]` input type.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Averages;
    /// 
    /// let v = vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    /// 
    /// assert_eq!(3.125, v.arithmetic_mean());
    /// ```
    #[must_use]
    fn arithmetic_mean(&self) -> Self::Output;
    /// Calculate the harmonic mean.
    /// # Returns
    /// A `f32` by default, except for the `&[f64]` input type.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Averages;
    /// 
    /// let v = vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    /// 
    /// assert_eq!(2.318840579710145, v.harmonic_mean());
    /// ```
    #[must_use]
    fn harmonic_mean(&self) -> Self::Output;
    /// Calculate the median.
    /// # Returns
    /// A `f32` by default, except for the `&[f64]` input type.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Averages;
    /// 
    /// let v = vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    /// 
    /// assert_eq!(2.5, v.median());
    /// ```
    #[must_use]
    fn median(&self) -> Self::Output;
    /// Calculate the mode.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Averages;
    /// 
    /// let v = vec![1, 2, 2, 2, 3, 4, 5, 6];
    /// 
    /// assert_eq!(2, v.mode());
    /// ```
    #[must_use]
    fn mode(&self) -> T;
    /// Calculate the mid range.
    /// # Returns
    /// A `f32` by default, except for the `&[f64]` input type.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::Averages;
    /// 
    /// let v = vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    /// 
    /// assert_eq!(3.5, v.mid_range());
    /// ```
    #[must_use]
    fn mid_range(&self) -> Self::Output;
}
/// Trait for the Digits of a given number.
pub trait NumDigits {
    /// Calculates the cross sum of a number.
    /// # Returns
    /// A `Self`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumDigits;
    /// 
    /// assert_eq!(3u8, 12.cross_sum());
    /// assert_eq!(9u16, 342.cross_sum());
    /// assert_eq!(52u64, 4928947234u64.cross_sum());
    /// ```
    #[must_use = "This returns the result of the operation, without modifying the original."]
    fn cross_sum(&self) -> Self;
    /// Gets the digits as a number with the 1s place at index 0.
    /// # Returns
    /// A `Vec<u8>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumDigits;
    /// 
    /// assert_eq!(vec![4,3,2,1], 1234u16.digits());
    /// assert_eq!(vec![0], 0u8.digits());
    /// ```
    #[must_use]
    fn digits(&self) -> Vec<u8>;
}
/// Trait for several useful generic functions.
pub trait NumTools<T> {
    /// Determine whether `self` is in the interval `[start; end]`.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// 
    /// assert_eq!(true, 5.is_in_range(0, 10));
    /// assert_eq!(true, 5.is_in_range(0, 5));
    /// assert_eq!(false, 3.14.is_in_range(5.0, 10.0));
    /// ```
    #[must_use]
    fn is_in_range(&self, start: Self, end: Self) -> bool;
    /// Determine whether `self` is in the interval `(start; end)`.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// 
    /// assert_eq!(true, 5.is_in_range_exclusive(0, 10));
    /// assert_eq!(false, 5.is_in_range_exclusive(0, 5));
    /// ```
    #[must_use]
    fn is_in_range_exclusive(&self, start: Self, end: Self) -> bool;
    /// Maps a given number of a range onto another range.
    /// # Arguments
    /// * `self` - The value which is to be mapped.
    /// * `start1` - The original start value of the number range.
    /// * `end1` - The original end value of the number range.
    /// * `start2` - The new start value of the number range.
    /// * `end2` - The new start value of the number range.
    /// # Returns
    /// A number containing the new mapped value.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// let result: f32 = 5.0.map_to(0., 10., 0., 1.); // Original value 5 in the range from 0-10
    /// assert_eq!(result, 0.5);
    /// ```
    #[must_use = "This returns the result of the operation, without modifying the original."]
    fn map_to(&self, start1: T, end1: T, start2: T, end2: T) -> T;
    /// Increment a number by one.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// let mut five: i32 = 5;
    /// five.inc();
    /// assert_eq!(five, 6);
    /// ```
    fn inc(&mut self);
    /// Increment a number by a specified value.
    /// # Arguments
    /// * `n` - The value to be incremented by.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// let mut five: i32 = 5;
    /// five.inc_by(2);
    /// assert_eq!(five, 7);
    /// ```
    fn inc_by(&mut self, n: Self);
    /// Decrement a number by one.
    /// # Warning
    /// Does not check for underflow.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// let mut five: i32 = 5;
    /// five.dec();
    /// assert_eq!(five, 4);
    /// ```
    fn dec(&mut self);
    /// Decrement a number by a specified value.
    /// # Arguments
    /// * `n` - The value to be decremented by.
    /// # Returns
    /// Nothing.
    /// # Warning
    /// Does not check for underflow.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// let mut five: i32 = 5;
    /// five.dec_by(2);
    /// assert_eq!(five, 3);
    /// ```
    fn dec_by(&mut self, n: Self);
    /// Square a number.
    /// # Returns
    /// A `Self`.
    /// # Caution
    /// This function does not check if overflow occurs.
    /// # Examples
    /// ```
    ///use lib_rapid::math::general::NumTools;
    ///let i = 12;
    ///let res = i.square();
    ///assert_eq!(144, res);
    /// ```
    #[must_use = "This returns the result of the operation, without modifying the original."]
    fn square(&self) -> Self;

    /// Cube a number.
    /// # Returns
    /// A `Self`.
    /// # Caution
    /// This function does not check if overflow occurs.
    /// # Examples
    /// ```
    ///use lib_rapid::math::general::NumTools;
    ///let i = 12;
    ///let res = i.cube();
    ///assert_eq!(1728, res);
    /// ```
    #[must_use = "This returns the result of the operation, without modifying the original."]
    fn cube(&self) -> Self;
    /// Calculates the reciprocal ( 1 ÷ number ) of a given number.
    /// # Returns
    /// A `Self`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// 
    /// assert_eq!(5.recip(), 0);
    /// assert_eq!(NumTools::recip(&0.2), 0.2.recip());
    /// ```
    #[must_use = "This returns the result of the operation, without modifying the original."]
    fn recip(&self) -> Self;
    /// Calculates the nth power of a given number.
    /// # Returns
    /// A `Self`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// 
    /// assert_eq!(5.pow(4), 625);
    /// assert_eq!(5.pow(0), 1);
    /// ```
    #[must_use = "This returns the result of the operation, without modifying the original."]
    fn pow(&self, power: usize) -> Self;
    /// Determines whether a number is negative. Only implemented for readability.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::general::NumTools;
    /// 
    /// assert_eq!(5u8.is_negative(), false);
    /// assert_eq!((-5.0).is_negative(), true);
    /// ```
    #[must_use]
    fn is_negative(&self) -> bool;
}

impl<T: PartialOrd +
        Sub<Output = T> +
        Add<Output = T> +
        Div<Output = T> +
        Mul<Output = T> +
        From<u8> +
        Copy +
        SubAssign +
        AddAssign +
        MulAssign> NumTools<T> for T {
    #[inline]
    fn is_in_range(&self, start: Self, end: Self) -> bool {
        self >= &start &&
        self <= &end
    }

    #[inline]
    fn is_in_range_exclusive(&self, start: Self, end: Self) -> bool {
        self > &start &&
        self < &end
    }

    #[inline]
    fn map_to(&self, start1: T, end1: T, start2: T, end2: T) -> T {
        eval_postfix!(start2 end2 start2 - + (*self) start1 - end1 / start1 - *)
    }

    #[inline]
    fn inc(&mut self) {
        *self += 1u8.into();
    }

    #[inline]
    fn inc_by(&mut self, n: Self) {
        *self += n;
    }

    #[inline]
    fn dec(&mut self) {
        *self -= 1u8.into();
    }
    #[inline]
    fn dec_by(&mut self, n: Self) {
        *self -= n;
    }
    #[inline]
    fn square(&self) -> Self {
        *self * *self
    }
    #[inline]
    fn cube(&self) -> Self {
        *self * *self * *self
    }

    #[inline]
    fn recip(&self) -> Self {
        Self::from(1u8) / *self
    }

    #[inline]
    fn pow(&self, power: usize) -> Self {
        let self_is_zero  = self  == &0u8.into();
        let power_is_zero = power == 0;

        if self_is_zero && !power_is_zero
        { return 0u8.into(); }
        if !self_is_zero && power_is_zero
        { return 1u8.into(); }
        if self_is_zero && power_is_zero
        { panic!("0^0 is undefined."); }
        
        let mut res = *self;
        for _ in 2..(power + 1) {
            res *= *self;
        }
        res
    }

    #[inline]
    fn is_negative(&self) -> bool {
        *self < T::from(0)
    }
}

impl<'a, T: PartialEq +
        DivAssign + PartialOrd +
        Rem<Output = T> + Copy +
        AddAssign +
        Div<Output = T> +
        Sub<Output = T> +
        Add<Output = T> +
        Mul<Output = T> +
        AddAssign +
        SubAssign +
        MulAssign>
    NumDigits
    for T
    where
        u8: TryFrom<T>,
        T: From<u8> {
    fn cross_sum(&self) -> Self {
        let     d:   Vec<u8> = self.digits();
        let mut res: Self    = 0.into();
        for i in d {
            res.inc_by(i.into());
        }
        res
    }

    fn digits(&self) -> Vec<u8> {
        if self == &0u8.into() { return vec![0]; }
        let mut v: Self = *self;
        let mut digits: Vec<u8> = Vec::with_capacity(39);
        while v > 0.into() {
            let n: u8 = unsafe { (v % 10.into()).try_into().unwrap_unchecked() };
            if n == 255
            { panic!("Oops! Something went wrong. please contact the developers using the error code 0x00001.")}
            v /= 10.into();
            digits.push(n);
        }
        digits
    }
}
/// Compute the nth-root of a number.
/// # Arguments
/// * `degree` - the `f64` specifying the `n`.
/// * `radicand` - the `f64` of which the root should be computed.
/// # Returns
/// A `f64`.
/// # Examples
/// ```
/// use lib_rapid::math::general::nth_root;
/// use lib_rapid::math::constants;
/// assert_eq!(constants::CUBEROOT3, nth_root(3.0, 3.0));
/// ```
/// ```
/// use lib_rapid::math::general::nth_root;
/// use lib_rapid::math::constants;
/// assert_eq!(constants::SQRT2, nth_root(2.0, 2.0));
/// ```
#[must_use = "This returns the result of the operation, without modifying the original."]
#[inline]
pub fn nth_root(degree: f64, radicand: f64) -> f64 {
    radicand.powf(degree.recip())
}
/// Computes th nth fibonacci number (up to 186th) accurately using the fastest available computing method.
/// # Arguments
/// * `n` - the nth-fibonacci number to be computed.
/// # How does it work?
/// This function uses the function `φⁿ ÷ √5` for all numbers `n < 76`, as this is the maximum precision for that formula. For every number above that, it uses the iterative approach.
/// # Examples
/// ```
/// use lib_rapid::math::general::nth_fibonacci;
/// 
/// assert_eq!(1304969544928657, nth_fibonacci(74));
/// assert_eq!(332825110087067562321196029789634457848, nth_fibonacci(186));
/// ```
#[must_use]
pub fn nth_fibonacci(n: u128) -> u128 {
    if n > 186
    { panic!("Error: The 187th Fibonacci number and all above are not allowed, as they would cause a overflow in the u128 type."); }
    if n < 76 {
        return ( GOLDENRATIO.powi(n as i32) / SQRT5 ).round() as u128;
    }

    let mut x: u128;
    let mut y: u128 = 1304969544928657;
    let mut z: u128 = 2111485077978050;
    let mut i: u128 = 75;

    while i < n {
        x = y;
        y = z;
        z = x + y;
        i.inc();
    }

    z
}

/// Compute the absolute difference ("Delta") between two values.
/// # But Rust already has a abs_diff() function!
/// Yes, it does. But it does not have support for floating point numbers.
/// That's why we implemented a generic function.
/// # Arguments
/// - `a: T` - The first value.
/// - `b: T` - The second value.
/// # Returns
/// A `T`.
/// # Examples
/// ```
/// use lib_rapid::math::general::delta;
/// 
/// assert_eq!(4, delta(4, 8));
/// assert_eq!(3.14, delta(6.28, 3.14));
/// assert_eq!(4.905, delta(4.905, 9.81));
/// ```
#[must_use]
#[inline]
pub fn delta<T: Sub<Output = T> +
                PartialOrd>(a: T, b: T) -> T {
    match a > b {
        true  => { return a - b; }
        false => { return b - a; }
    }
}

/// Panics if a given value `x` is odd.
/// # Arguments
/// - `x: T` - The number to be checked.
/// # Examples
/// ```should_panic
/// use lib_rapid::math::general::better_be_even;
/// 
/// better_be_even(-3); // Panics, because -3 is odd.
/// ```
#[inline]
pub fn better_be_even<T: From<bool> +
                         BitAnd<Output = T> +
                         PartialEq +
                         std::fmt::Display +
                         Copy>(x: T) {
    if x & true.into() != false.into()
    { core::panic!("Oops! {} was not even.", x); }
}

/// Allows for square roots of negative numbers.
/// # Arguments
/// - `x: f64` - The number of which the square root needs to be drawn.
/// # Returns
/// A `ComplexNumber<f64>`.
/// # Examples
/// ```
/// use lib_rapid::math::{general::sqrt_f64, complex::ComplexNumber, constants::SQRT2};
/// 
/// let c = ComplexNumber::new(0.0f64, SQRT2);
/// 
/// assert_eq!(sqrt_f64(-2.0), c);
/// ```
#[inline]
#[must_use]
pub fn sqrt_f64(x: f64) -> ComplexNumber<f64> {
    if x < 0.0
    { return ComplexNumber { real: 0.0, complex: (-x).sqrt() }; }

    ComplexNumber { real: x.sqrt(), complex: 0.0 }
}
/// Allows for square roots of negative numbers.
/// # Arguments
/// - `x: f64` - The number of which the square root needs to be drawn.
/// # Returns
/// A `ComplexNumber<f64>`.
/// # Examples
/// ```
/// use lib_rapid::math::{general::sqrt_f32, complex::ComplexNumber, constants::SQRT2};
/// 
/// let c = ComplexNumber::new(0.0f32, SQRT2 as f32);
/// 
/// assert_eq!(sqrt_f32(-2.0), c);
/// ```
#[inline]
#[must_use]
pub fn sqrt_f32(x: f32) -> ComplexNumber<f32> {
    if x < 0.0
    { return ComplexNumber { real: 0.0, complex: (-x).sqrt() }; }

    ComplexNumber { real: x.sqrt(), complex: 0.0 }
}

/// The gamma function `Γ(z)` for all `z ∈ ℝ \ {-1}`.
/// # Arguments
/// * `z: T` - The function value to be computed.
/// * `precision: f64` - The precision of the computation. Numbers of the form 10^n with `n ∈ ℤ⁻` roughly give a precision of `10^((n / 2) - 1)`.
/// # Examples
/// ```
/// use lib_rapid::math::general::{euler_gamma, delta};
/// 
/// assert!(delta(euler_gamma( 1.6, 1e-12), 0.8935153492876903) < 1e-5);
/// assert!(delta(euler_gamma(-1.6, 1e-12), 2.3105828580809252) < 1e-5);
/// ```
pub fn euler_gamma<T: Into<f64>>(z: T, precision: f64) -> f64 {
    let     _z:       f64 = z.into();
    let     pre:      f64 = E.powf(-_z * EULERMASCHERONI) / _z;
    let mut term_inf: f64 = E.powf(_z) /
                            (1.0 + _z);
    let mut res:      f64 = 1.0;
    let mut n:        f64 = 1.0;
    let mut zdivn:    f64;

    if delta(term_inf, 1.0) < precision
    { return res; }

    // Infinite Series _Gamma:
    // \prod_{n = 1}^{\infty} \frac{e^{z \div n}}{1 + z \div n}.
    loop {
        res *= term_inf;
        
        if delta(term_inf, 1.0) < precision
        { break; }
        
        n.inc();
        zdivn = _z / n;
        term_inf  = E.powf(zdivn) /
                    (1.0 + zdivn);
    }
    pre * res
}

/// Find the greatest common divisor between two values.
/// # Arguments
/// * `a: T` - The first value.
/// * `b: T` - The second value.
/// # Returns
/// A `T`.
/// # Examples
/// ```
/// use lib_rapid::math::general::gcd;
/// 
/// assert_eq!(gcd(3, 6), 3);
/// assert_eq!(gcd(52, 345), 1);
/// ```
#[must_use]
pub fn gcd<T: From<u8> +
              PartialEq +
              PartialOrd +
              Shl<Output = T> +
              NumTools<T> +
              Copy +
              ShrAssign>(a: T, b: T) -> T
              where
              i128: From<T> {
    
    let mut _a = a;
    let mut _b = b;
    if _b == 0.into()
    { return _a; }

    if _a == 0.into()
    { return _b; }

    let a_two_factor:   u8 =
    unsafe { i128::from(_a).trailing_zeros().try_into().unwrap_unchecked() };
    let b_two_factor:   u8 =
    unsafe { i128::from(_b).trailing_zeros().try_into().unwrap_unchecked() };
    let min_two_factor: u8 = std::cmp::min(a_two_factor, b_two_factor);
    _a >>= a_two_factor.into();
	_b >>= b_two_factor.into();
    loop {
         
        if _b > _a
        { std::mem::swap(&mut _b, &mut _a); }

        _a.dec_by(_b);

        if _a == 0.into()
        { return _b << min_two_factor.into(); }
        _a >>= unsafe { u8::try_from(i128::from(_a).trailing_zeros()).unwrap_unchecked() }.into();
    }
}
/// Calculates the factorial of a number.
/// # Arguments
/// * `n: u8` - The number of which the factorial should be calculated.
/// # Returns
/// A `u128`.
/// # Examples
/// ```
/// use lib_rapid::math::general::factorial;
/// 
/// assert_eq!(factorial(7), 7 * 6 * 5 * 4 * 3 * 2);
/// ```
pub const fn factorial(n: u8) -> u128 {
    let mut x    = n as u128;
    let mut _res = 1;
    while x > 1 {
        _res *= x;
        x    -= 1;
    }

    _res
}
/// Calculates the division of two factorials of the form `n! ÷ (n - k)!`.
/// # Arguments
/// * `x: usize` - `n`.
/// * `y: usize` - `n - k`.
/// # Returns
/// A `usize`.
/// # Examples
/// ```
/// use lib_rapid::math::general::div_factorials;
/// 
/// assert_eq!(div_factorials(245, 242), 14526540); // 245! ÷ 242! = 245 · 244 · 243.
/// assert_eq!(div_factorials(1, 0), 1); // 1! ÷ 0! = 1 ÷ 1.
/// ```
pub const fn div_factorials(x: usize, y: usize) -> usize {
    let k        = x - y;
    let mut _res = 1;
    let mut i    = 0;
    while i != k {
        _res *= x - i;
        i    += 1;
    }
    _res
}
/// Rounds to the nearest multiple of `n`.
/// # Arguments
/// * `x: T` - The number which shall be rounded.
/// * `n: T` - The multiplicant.
/// # Returns
/// A `T`.
/// # Examples
/// ```
/// use lib_rapid::math::general::round_to_n_mult;
/// 
/// assert_eq!(round_to_n_mult(14, 5), 15);
/// assert_eq!(round_to_n_mult(1932, 2), 1932);
/// // This function rounds up if in doubt.
/// assert_eq!(round_to_n_mult(1945, 2), 1946);
/// ```
pub fn round_to_n_mult<T: Div<Output = T> +
                          Mul<Output = T> +
                          Sub<Output = T> +
                          Add<Output = T> +
                          PartialOrd +
                          From<bool> +
                          Copy>(x: T, n: T) -> T {
    let x_over_n = x / n;
    let f1 = (x + true.into()) / n * n;

    match delta(x_over_n, x) <= delta(x_over_n, f1) {
        true  => f1,
        false => x
    }
}