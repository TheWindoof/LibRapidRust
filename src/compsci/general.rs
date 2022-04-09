//! General purpose functionalities for computer science. Got any wishes? Tell us on GitHub or our Discord.
use crate::math::general::NumTools;
use std::{intrinsics::transmute, fmt::Debug, ops::{BitAndAssign, Shl, BitOrAssign, BitXorAssign}, convert::TryFrom};
use core::mem::{size_of_val, size_of};

const BITWISE_ERR1: &str = "Arguments were not the same size in memory.";
const BITWISE_ERR2: &str = "U is bigger than T. Consider reversing the arguments.";
const MANTISSA_MASK_64:      u64 = 0b0000000000001111111111111111111111111111111111111111111111111111;
const EXPONENT_MASK_64:      u64 = 0b0111111111110000000000000000000000000000000000000000000000000000;
const SET_EXP_ZERO_MASK_64:  u64 = 0b0011111111110000000000000000000000000000000000000000000000000000;
const GET_SMANTISSA_MASK_64: u64 = 0b1000000000001111111111111111111111111111111111111111111111111111;
const MANTISSA_MASK_32:      u32 = 0b00000000011111111111111111111111;
const EXPONENT_MASK_32:      u32 = 0b01111111100000000000000000000000;
const SET_EXP_ZERO_MASK_32:  u32 = 0b00111111100000000000000000000000;
const GET_SMANTISSA_MASK_32: u32 = 0b10000000011111111111111111111111;

/// Trait for several functions that extend the functionality of slices.
pub trait SliceOps<T> {
    /// Look at the top of a slice `[T]`. Returns the top value but does not pop it.
    /// # Returns
    /// A `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::SliceOps;
    /// 
    /// let v = vec![0.2, 3.2];
    /// assert_eq!(3.2, v.peek());
    /// assert_eq!(vec![0.2, 3.2], v);
    #[must_use]
    fn peek(&self) -> T;
    /// Swap the top two elements of a slice `[T]`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::SliceOps;
    /// 
    /// let mut v = vec![0.2, 3.2];
    /// v.swap_top();
    /// assert_eq!(vec![3.2, 0.2], v);
    fn swap_top(&mut self);
    /// Find the number of times a item is contained in a slice `[T]`.
    /// # Returns
    /// A `usize`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::SliceOps;
    /// 
    /// let mut v = vec![4, 0, 0, 2, 3, 4, 4, 4];
    /// assert_eq!(4, v.count_of(4));
    #[must_use]
    fn count_of(&self, item: T) -> usize;
    /// Get the indexes at which a item resides in a slice `[T]`.
    /// # Returns
    /// A `Vec<usize>`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::SliceOps;
    /// 
    /// let mut v = vec![4, 0, 0, 2, 3, 4, 4, 4];
    /// assert_eq!(vec![0, 5, 6, 7], v.positions_of(4));
    #[must_use]
    fn positions_of(&self, item: T) -> Vec<usize>;
}

/// Bitwise operations on slices of arbitrary (numeric) types.
pub trait BitwiseSlice<T, U> {
    /// XORs a slice of type `[T]` with a slice of type `[U]`.
    /// # Returns
    /// A `Vec<T>`.
    /// # Panics
    /// Panics if:
    /// * The length of `self` and `other` in bytes differ or
    /// * the size of `U` is bigger than the size of `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::BitwiseSlice;
    /// let fs: Vec<u16> = vec!(0b10000000____00000001, 0b10000000____00000010);
    /// let sn: Vec<u8>  = vec!(0b10000000, 0b00000001, 0b10000000, 0b00000010);
    /// 
    /// let fs2: Vec<u16> = vec!(0b00000000____00000000);
    /// let sn2: Vec<u8>  = vec!(0b00000001, 0b00000011);
    /// assert_eq!(vec!(0, 0), fs.xor_with(&sn));
    /// assert_eq!(vec!(259), fs2.xor_with(&sn2));
    /// ```
    #[must_use]
    fn xor_with(&self, other: &[U]) -> Vec<T>;
    /// ORs a slice of type `[T]` with a slice of type `[U]`.
    /// # Returns
    /// A `Vec<T>`.
    /// # Panics
    /// Panics if:
    /// * The length of `self` and `other` in bytes differ or
    /// * the size of `U` is bigger than the size of `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::BitwiseSlice;
    /// let fs: Vec<u16> = vec!(0b00000000____00000001, 0b00000000____00000010);
    /// let sn: Vec<u8>  = vec!(0b00000000, 0b10000001, 0b00000000, 0b00000010);
    /// 
    /// let fs2: Vec<u16> = vec!(0b00000000____00000100);
    /// let sn2: Vec<u8>  = vec!(0b00000000, 0b00000011);
    /// assert_eq!(vec!(129, 2), fs.or_with(&sn));
    /// assert_eq!(vec!(7), fs2.or_with(&sn2));
    /// ```
    #[must_use]
    fn or_with(&self, other: &[U]) -> Vec<T>;
    /// ANDs a slice of type `[T]` with a slice of type `[U]`.
    /// # Returns
    /// A `Vec<T>`.
    /// # Panics
    /// Panics if:
    /// * The length of `self` and `other` in bytes differ or
    /// * the size of `U` is bigger than the size of `T`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::BitwiseSlice;
    /// let fs: Vec<u16> = vec!(0b00100000____00000001, 0b00000000____00000011);
    /// let sn: Vec<u8>  = vec!(0b00000000, 0b00000001, 0b00000000, 0b00000010);
    /// 
    /// let fs2: Vec<u16> = vec!(0b00000000____00000101);
    /// let sn2: Vec<u8>  = vec!(0b00000000, 0b00000011);
    /// assert_eq!(vec!(1, 2), fs.and_with(&sn));
    /// assert_eq!(vec!(1), fs2.and_with(&sn2));
    /// ```
    #[must_use]
    fn and_with(&self, other: &[U]) -> Vec<T>;
}
/// Trait for `binary_insert`.
pub trait BinaryInsert<T> {
    /// Insert an element into a ***sorted*** `vec` whilst maintaining the order, consuming `other`.
    ///
    /// # Arguments
    /// * `value` - The value which needs to be inserted.
    ///
    /// # Returns
    /// Nothing.
    ///
    /// # Warning
    /// This function ***will not*** check if the parsed `vec` is sorted.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::BinaryInsert;
    /// 
    /// let mut v = vec![0, 1, 2, 3, 4, 5];
    /// v.binary_insert(3);
    /// 
    /// assert_eq!(vec![0, 1, 2, 3, 3, 4, 5], v);
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::BinaryInsert;
    /// 
    /// let mut v = vec![0, 2, 3, 4, 5];
    /// v.binary_insert(1);
    /// 
    /// assert_eq!(vec![0, 1, 2, 3, 4, 5], v);
    /// ```
    fn binary_insert(&mut self, value: T);
    /// The same as `binary_insert`, but doesn't insert a value that is already present.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::BinaryInsert;
    /// 
    /// let mut v = vec![0, 1, 2, 3, 4, 5];
    /// v.binary_insert_no_dup(3);
    /// 
    /// assert_eq!(vec![0, 1, 2, 3, 4, 5], v);
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::BinaryInsert;
    /// 
    /// let mut v = vec![0, 1, 2, 3, 4, 5];
    /// v.binary_insert(6);
    /// 
    /// assert_eq!(vec![0, 1, 2, 3, 4, 5, 6], v);
    /// ```
    fn binary_insert_no_dup(&mut self, value: T);
}
/// Useful String related functions.
pub trait StringUtils {
        /// Get the `char` at a given index from a `String` or `&str`.
    /// If only dealing with ASCII-characters, `byte_at` is recommended.
    /// 
    /// # Arguments
    /// * `index` - The index of the character
    ///
    /// # Returns
    /// A `Option<char>`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::StringIndex;
    /// 
    /// let s = String::from("Hello");
    /// assert_eq!('H', s.char_at(0).unwrap());
    /// assert_eq!('e', s.char_at(1).unwrap());
    /// assert_eq!('l', s.char_at(2).unwrap());
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::StringIndex;
    /// 
    /// let s = "Hello";
    /// assert_eq!('H', s.char_at(0).unwrap());
    /// assert_eq!('e', s.char_at(1).unwrap());
    /// assert_eq!('l', s.char_at(2).unwrap());
    /// ```
    #[must_use]
    fn char_at(&self, index: usize) -> Option<char>;
    /// Get the byte at a given index from a `String` or `&str`.
    /// If dealing with Unicode characters, this function is *not* recommended, as it only returns one byte even though Unicode can be a multi byte encoding.
    /// 
    /// # Arguments
    /// * `index` - The index of the byte
    ///
    /// # Returns
    /// A `u8`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::StringIndex;
    /// 
    /// let s = String::from("Hello");
    /// assert_eq!('H', s.byte_at(0) as char);
    /// assert_eq!('e', s.byte_at(1) as char);
    /// assert_eq!('l', s.byte_at(2) as char);
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::StringIndex;
    /// 
    /// let s = "Hello";
    /// assert_eq!('H', s.byte_at(0) as char);
    /// assert_eq!('e', s.byte_at(1) as char);
    /// assert_eq!('l', s.byte_at(2) as char);
    /// ```
    #[must_use]
    fn byte_at(&self, index: usize) -> u8;
    /// Determines whether the given `&str` or `String` has valid brackets.
    /// # Returns
    /// `Ok(true)` if all brackets were closed, otherwise `Err(usize)`, where `usize` is the index of the String at which the error occured.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::Brackets;
    /// 
    /// let s = "([{Some text won't throw it off}])";
    /// 
    /// assert_eq!(Ok(true), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::Brackets;
    /// 
    /// let s = String::from("([{}])");
    /// 
    /// assert_eq!(Ok(true), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::Brackets;
    /// 
    /// let s = "([{}]";
    /// 
    /// assert_eq!(Err(5), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::Brackets;
    /// 
    /// let s = "([{Some text won't throw it off}]))";
    /// 
    /// assert_eq!(Err(34), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::general::Brackets;
    /// 
    /// let s = "([{]}))";
    /// 
    /// assert_eq!(Err(3), s.validate_brackets());
    /// ```
    fn validate_brackets(&self) -> Result<bool, usize>;
}
/// Simple methods for characters.
pub trait CharTools {
    /// Determines whether a character is a opening bracket.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::CharTools;
    /// 
    /// assert_eq!(true, '('.is_open_bracket());
    /// assert_eq!(true, '['.is_open_bracket());
    /// assert_eq!(false, '}'.is_open_bracket());
    /// ```
    #[must_use]
    fn is_open_bracket(&self) -> bool;
    /// Determines whether a character is a closing bracket.
    /// # Returns
    /// A `bool`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::CharTools;
    /// 
    /// assert_eq!(true, ')'.is_closed_bracket());
    /// assert_eq!(true, ']'.is_closed_bracket());
    /// assert_eq!(false, '{'.is_closed_bracket());
    /// ```
    #[must_use]
    fn is_closed_bracket(&self) -> bool;
    /// Gets the corresponding opening bracket if there is one..
    /// # Returns
    /// A `Option<char>`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::CharTools;
    /// 
    /// assert_eq!(None, '('.get_opening_bracket());
    /// assert_eq!(Some('('), ')'.get_opening_bracket());
    /// assert_eq!(Some('{'), '}'.get_opening_bracket());
    #[must_use]
    fn get_opening_bracket(&self) -> Option<char>;
    /// Gets the corresponding closing bracket if there is one..
    /// # Returns
    /// A `Option<char>`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::CharTools;
    /// 
    /// assert_eq!(Some(')'), '('.get_closing_bracket());
    /// assert_eq!(None, ')'.get_closing_bracket());
    /// assert_eq!(Some('}'), '{'.get_closing_bracket());
    /// ```
    #[must_use]
    fn get_closing_bracket(&self) -> Option<char>;
}
/// Get bits and bytes from a floating point number.
pub trait FloatMagic {
    type Mantissa;
    type Exponent;
    type RealExponent;
    /// Get the raw binary mantissa (fractional part) data.
    /// # Returns
    /// A `u32` if the input is a `f32`, otherwise a `u64`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::FloatMagic;
    /// 
    /// assert_eq!(2570638229164439, 3.1415927_f64.raw_mantissa());
    /// ```
    #[must_use]
    fn raw_mantissa(&self) -> Self::Mantissa;
    /// Get the raw binary exponent data.
    /// # Returns
    /// A `u8` if the input is a `f32`, otherwise a `u16`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::FloatMagic;
    /// 
    /// assert_eq!(1024, 3.1415927_f64.raw_exponent());
    /// assert_eq!(1, 1.1754943508e-38f32.raw_exponent());
    /// ```
    #[must_use]
    fn raw_exponent(&self) -> Self::Exponent;
    /// Get the actual mantissa part.
    /// # Returns
    /// A `Self`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::FloatMagic;
    /// 
    /// assert_eq!(1.57079635, 3.1415927_f64.real_mantissa());
    /// assert_eq!(1.0, 1.1754943508e-38f32.real_mantissa());
    /// ```
    #[must_use]
    fn real_mantissa(&self) -> Self;
    /// Get the actual exponent part.
    /// # Returns
    /// A `i32`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::FloatMagic;
    /// 
    /// assert_eq!(1, 3.1415927_f64.real_exponent());
    /// assert_eq!(-126, 1.1754943508e-38f32.real_exponent());
    /// assert_eq!(-1022, 2.2250738585072014e-308.real_exponent()); // Minimal f64 value.
    /// ```
    #[must_use]
    fn real_exponent(&self) -> Self::RealExponent;
    /// Decompose a floating point number into it's core components.
    /// # Returns
    /// a tuple with this order:
    /// * `u8` - The sign.
    /// * `u8` or `u16` - The raw exponent.
    /// * `u32` or `u64` - The raw mantissa.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::FloatMagic;
    /// 
    /// assert_eq!((0, 1024, 2570638229164439), 3.1415927_f64.raw_decompose());
    /// ```
    #[must_use]
    fn raw_decompose(&self) -> (u8, Self::Exponent, Self::Mantissa);
    /// Decompose a floating point number into it's core components.
    /// # Returns
    /// A tuple with this order:
    /// * `u8` - The sign.
    /// * `i16` or `i32` - The real exponent.
    /// * `f32` or `f64` - The real mantissa.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::general::FloatMagic;
    /// 
    /// assert_eq!((0, 1, 1.5707964), 3.1415927_f32.real_decompose());
    /// assert_eq!((0, 1, 1.57079635), 3.1415927_f64.real_decompose());
    /// ```
    #[must_use]
    fn real_decompose(&self) -> (u8, Self::RealExponent, Self);
    /// Compose a floating point value out of the raw parts.
    /// # Returns
    /// A `Self`.
    /// # Panics
    /// Panics if one of the parts is out of range for `Self`.
    /// # Examples
    /// ```    fn raw_exponent(&self) -> Self::Exponent {

    /// use lib_rapid::compsci::general::FloatMagic;
    /// 
    /// let my_float: f32 = 3.1415927;
    /// let sign          = my_float.is_sign_negative() as u8;
    /// let exponent      = my_float.raw_exponent();
    /// let mantissa      = my_float.raw_mantissa();
    /// 
    /// let my_double       = my_float as f64;
    /// let sign_1          = my_double.is_sign_negative() as u8;
    /// let exponent_1      = my_double.raw_exponent();
    /// let mantissa_1      = my_double.raw_mantissa();
    /// 
    /// assert_eq!(3.1415927, f32::raw_compose(sign, exponent, mantissa));
    /// assert_eq!(3.1415927410125732, f64::raw_compose(sign, exponent_1, mantissa_1));
    /// ```
    #[must_use]
    fn raw_compose(sign: u8, exponent: Self::Exponent, mantissa: Self::Mantissa) -> Self;
}

impl CharTools for char {
    fn is_open_bracket(&self) -> bool {
        match self {
            '(' => { true }
            '{' => { true }
            '[' => { true }
            _   => { false }
        }
    }

    fn is_closed_bracket(&self) -> bool {
        match self {
            ')' => { true }
            '}' => { true }
            ']' => { true }
            _   => { false }
        }
    }

    fn get_closing_bracket(&self) -> Option<char> {
        match self {
            '(' => { Some(')') }
            '{' => { Some('}') }
            '[' => { Some(']') }
            _   => { None }
        }
    }

    fn get_opening_bracket(&self) -> Option<char> {
        match self {
            ')' => { Some('(') }
            ']' => { Some('[') }
            '}' => { Some('{') }
            _   => { None }
        }
    }
}

impl FloatMagic for f32 {
    type Mantissa = u32;
    type Exponent = u8;
    type RealExponent = i16;

    #[inline]
    fn raw_mantissa(&self) -> Self::Mantissa {
        (unsafe { transmute::<f32, u32>(*self) }) & MANTISSA_MASK_32
    }
    #[inline]
    fn raw_exponent(&self) -> Self::Exponent {
        ((unsafe { transmute::<f32, u32>(*self) } &
                   EXPONENT_MASK_32) >> 23) as u8
    }
    #[inline]
    fn real_mantissa(&self) -> Self {
        unsafe { transmute(transmute::<f32, u32>(*self) &
                           GET_SMANTISSA_MASK_32 |
                           SET_EXP_ZERO_MASK_32)
        }
    }
    #[inline]
    fn real_exponent(&self) -> Self::RealExponent {
        self.raw_exponent() as i16 - 127
    }

    #[inline]
    fn raw_decompose(&self) -> (u8, Self::Exponent, Self::Mantissa) {
        (self.is_sign_negative() as u8, self.raw_exponent(), self.raw_mantissa())
    }
    #[inline]
    fn real_decompose(&self) -> (u8, Self::RealExponent, Self) {
        (self.is_sign_negative() as u8, self.real_exponent(), self.real_mantissa())
    }
    #[inline]
    fn raw_compose(sign: u8, exponent: Self::Exponent, mantissa: Self::Mantissa) -> Self {
        if sign     > 1       { panic!("A sign bigger than 1 is not allowed."); }
        if mantissa > 8388607 { panic!("A mantissa bigger than 8388607 is not allowed."); }
        
        unsafe { transmute((mantissa |
               ((exponent as u32) << 23)) |
                (sign     as u32) << 31) }
    }
}

impl FloatMagic for f64 {
    type Mantissa = u64;
    type Exponent = u16;
    type RealExponent = i32;

    #[inline]
    fn raw_mantissa(&self) -> Self::Mantissa {
        (unsafe { transmute::<f64, u64>(*self) } &
                  MANTISSA_MASK_64)
    }
    #[inline]
    fn raw_exponent(&self) -> Self::Exponent {
        ((unsafe { transmute::<f64, u64>(*self) } &
                   EXPONENT_MASK_64) >> 52) as u16
    }
    #[inline]
    fn real_mantissa(&self) -> Self {
        unsafe { transmute(transmute::<f64, u64>(*self) &
                           GET_SMANTISSA_MASK_64 |
                           SET_EXP_ZERO_MASK_64)
        }
    }
    #[inline]
    fn real_exponent(&self) -> Self::RealExponent {
       self.raw_exponent() as i32 - 1023
    }
    #[inline]
    fn raw_decompose(&self) -> (u8, Self::Exponent, Self::Mantissa) {
        (self.is_sign_negative() as u8, self.raw_exponent(), self.raw_mantissa())
    }
    #[inline]
    fn real_decompose(&self) -> (u8, Self::RealExponent, Self) {
        (self.is_sign_negative() as u8, self.real_exponent(), self.real_mantissa())
    }
    #[inline]
    fn raw_compose(sign: u8, exponent: Self::Exponent, mantissa: Self::Mantissa) -> Self {
        if sign     > 1                { panic!("A sign bigger than 1 is not allowed."); }
        if exponent > 2047             { panic!("An exponent bigger than 2047 is not allowed."); }
        if mantissa > 4503599627370000 { panic!("A mantissa bigger than 8388607 is not allowed."); }

        unsafe { transmute((mantissa |
                          ((exponent as u64) << 23)) |
                           (sign     as u64) << 31) }
    }
}

impl StringUtils for String {
    fn validate_brackets(&self) -> Result<bool, usize> {
        backend_val_brackets(self)
    }
    #[inline]
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
    #[inline]
    fn byte_at(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }
}

impl StringUtils for &str {
    #[inline]
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
    #[inline]
    fn byte_at(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }
    fn validate_brackets(&self) -> Result<bool, usize> {
        backend_val_brackets(self)
    }
}

impl<T: Ord + Copy> BinaryInsert<T> for Vec<T> {
    fn binary_insert(&mut self, value: T) {
        match self.binary_search(&value) {
            Ok(pos)  => self.insert(pos + 1, value),
            Err(pos) => self.insert(pos, value),
        }
    }
    fn binary_insert_no_dup(&mut self, value: T) {
        match self.binary_search(&value) {
            Ok(_)    => { },
            Err(pos) => self.insert(pos, value),
        }
    }
}

impl<T: BitXorAssign +
        From<u8> +
        From<U> +
        Clone +
        BitOrAssign +
        BitAndAssign +
        Shl<Output = T> +
        Debug,
        U: Clone +
        Copy +
        Debug +
        Shl<Output = U> +
        From<u8>>
        
    BitwiseSlice<T, U> for [T]
    where
    <T as TryFrom<U>>::Error: Debug {
    fn xor_with(&self, other: &[U]) -> Vec<T> {
        if size_of_val(self) != size_of_val(other)
        { panic!("{}", BITWISE_ERR1); }

        let t_size: usize = size_of::<T>();
        let u_size: usize = size_of::<U>();
        if t_size < u_size
        { panic!("{}", BITWISE_ERR2); }
        
        let mut _res:   Vec<T> = self.to_vec();
        let multiplier: usize  = t_size / u_size;

        for (index, slice) in other.chunks(multiplier).enumerate() {
            _res[index] ^= bitwise_from(slice);
        }

        _res
    }

    fn or_with(&self, other: &[U]) -> Vec<T> {
        if size_of_val(self) != size_of_val(other)
        { panic!("{}", BITWISE_ERR1); }

        let t_size: usize = size_of::<T>();
        let u_size: usize = size_of::<U>();
        if t_size < u_size
        { panic!("{}", BITWISE_ERR2); }

        let mut _res:   Vec<T> = self.to_vec();
        let multiplier: usize  = t_size / u_size;
        for (index, slice) in other.chunks(multiplier).enumerate() {
            _res[index] |= bitwise_from(slice);
        }

        _res
    }

    fn and_with(&self, other: &[U]) -> Vec<T> {
        if size_of_val(self) != size_of_val(other)
        { panic!("{}", BITWISE_ERR1); }

        let t_size: usize = size_of::<T>();
        let u_size: usize = size_of::<U>();
        if t_size < u_size
        { panic!("{}", BITWISE_ERR2); }
        
        let mut _res:   Vec<T> = self.to_vec();
        let multiplier: usize  = t_size / u_size;

        for (index, slice) in other.chunks(multiplier).enumerate() {
            _res[index] &= bitwise_from(slice);
        }

        _res
    }
}

impl<T: Clone + std::cmp::PartialEq> SliceOps<T> for [T] {
    #[inline]
    fn peek(&self) -> T {
        self[self.len() - 1].clone()
    }
    #[inline]
    fn swap_top(&mut self) {
        let a: T = self[self.len() - 1].clone();
        let b: T = self[self.len() - 2].clone();

        self[self.len() - 1] = b;
        self[self.len() - 2] = a;
    }

    fn count_of(&self, item: T) -> usize {
        let mut res: usize = 0;
        for i in self {
            if i == &item
            { res.inc(); }
        }
        res
    }

    fn positions_of(&self, item: T) -> Vec<usize> {
        let mut res = Vec::with_capacity(self.len());
        for i in self.iter().enumerate() {
            if i.1 == &item
            { res.push(i.0); }
        }
        res
    }
}

#[must_use]
fn bitwise_from<T: From<u8> +
                   From<U> +
                   Shl<Output = T> +
                   BitOrAssign,
                   U: Copy>
    (slice: &[U]) -> T {
    let mut end_prod: T = T::from(0);

    for (inner_idx, inner_ref) in slice.iter().rev().enumerate() {
        end_prod |= T::from(*inner_ref) << ((inner_idx as u8) << 3u8).into();
    }

    end_prod
}

#[must_use]
fn backend_val_brackets(s: &str) -> Result<bool, usize> {
    let mut res_vec: Vec<char> = Vec::with_capacity(s.len());
    let mut i: usize = 0;
    for c in s.chars() {
        match c {
            '[' => { res_vec.push(c); },
            '{' => { res_vec.push(c); },
            '(' => { res_vec.push(c); },
            ']' if res_vec.pop() != Some('[') => { return Err(i); }
            '}' if res_vec.pop() != Some('{') => { return Err(i); }
            ')' if res_vec.pop() != Some('(') => { return Err(i); }
            _   => { }
        }
        i.inc();
    }

    if !res_vec.is_empty()
    { return Err(i); }
    Ok(true)
}

/// A Rust implementation of C's `strcmp()` function.
/// # Arguments
/// * `s1` - The String reference to be compared to.
/// * `s2` - The String which is compared with `s1`.
/// # Returns
/// It returns:
/// * `0`, if `s1 == s2`.
/// * `1`, if `s1[i] < s2[i]` in ASCII.
/// * `-1`, if `s1[i] > s2[i]` in ASCII.
/// # Examples
/// ```
/// use lib_rapid::compsci::general::strcmp;
/// let a = "hello";
/// let b = "hEllo";
/// assert!(strcmp(a, b) > 0);
/// ```
/// ```
/// use lib_rapid::compsci::general::strcmp;
/// let a = "hello";
/// let b = "hello";
/// assert!(strcmp(a, b) == 0);
/// ```
/// ```
/// use lib_rapid::compsci::general::strcmp;
/// let a = "Hello";
/// let b = "hello";
/// assert!(strcmp(a, b) < 0);
/// ```
pub fn strcmp(s1: &str, s2: &str) -> i8 {
    let mut i: usize = 0;
    let mut flag: i8 = 0;
    while flag == 0 {
        if s1.byte_at(i) > s2.byte_at(i)
        { flag = 1; }
        else if s1.byte_at(i) < s2.byte_at(i)
        { flag = -1; }
        if i + 1 == s1.len()
        { break; }

        i.inc();
    }
    flag
}