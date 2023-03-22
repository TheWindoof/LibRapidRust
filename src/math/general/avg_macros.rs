//! Macros for calculating different kinds of averages of values.
/// Calculate the arithmetic mean.
/// # Examples
/// ```
/// use lib_rapid::math::general::avg_macros::arithmetic_mean;
/// 
/// let a = arithmetic_mean![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
/// 
/// assert_eq!(3.125, a);
/// ```
#[macro_export]
macro_rules! arithmetic_mean {
    ( $( $a:expr ),* ) => {
        {
            use lib_rapid::math::general::Averages;
            let mut res_vec = Vec::with_capacity(20);
            $(
                res_vec.push($a);
            )*
            res_vec.arithmetic_mean()
        }
    };
}
pub use arithmetic_mean;
/// Calculate the harmonic mean.
/// # Examples
/// ```
/// use lib_rapid::math::general::avg_macros::harmonic_mean;
/// 
/// let v = harmonic_mean![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
/// 
/// assert_eq!(2.318840579710145, v);
/// ```
#[macro_export]
macro_rules! harmonic_mean {
    ( $( $a:expr ),* ) => {
        {
            use lib_rapid::math::general::Averages;
            let mut res_vec = Vec::with_capacity(20);
            $(
                res_vec.push($a);
            )*
            res_vec.harmonic_mean()
        }
    };
}
pub use harmonic_mean;
/// Calculate the median.
/// # Examples
/// ```
/// use lib_rapid::math::general::avg_macros::median;
/// 
/// let v = median![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
/// 
/// assert_eq!(2.5, v);
/// ```
#[macro_export]
macro_rules! median {
    ( $( $a:expr ),* ) => {
        {
            use lib_rapid::math::general::Averages;
            let mut res_vec = Vec::with_capacity(20);
            $(
                res_vec.push($a);
            )*
            res_vec.median()
        }
    };
}
pub use median;
/// Calculate the mode.
/// # Examples
/// ```
/// use lib_rapid::math::general::avg_macros::mode;
/// 
/// let v = mode![1, 2, 2, 2, 3, 4, 5, 6];
/// 
/// assert_eq!(2, v);
/// ```
#[macro_export]
macro_rules! mode {
    ( $( $a:expr ),* ) => {
        {
            use lib_rapid::math::general::Averages;
            let mut res_vec = Vec::with_capacity(20);
            $(
                res_vec.push($a);
            )*
            res_vec.mode()
        }
    };
}
pub use mode;
/// Calculate the mid range.
/// # Examples
/// ```
/// use lib_rapid::math::general::avg_macros::mid_range;
/// 
/// let v = mid_range![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
/// 
/// assert_eq!(3.5, v);
/// ```
#[macro_export]
macro_rules! mid_range {
    ( $( $a:expr ),* ) => {
        {
            use lib_rapid::math::general::Averages;
            let mut res_vec = Vec::with_capacity(20);
            $(
                res_vec.push($a);
            )*
            res_vec.mid_range()
        }
    };
}
pub use mid_range;