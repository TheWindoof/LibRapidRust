//! Certain functions for working with Strings.

use crate::math::general::NumTools;

/// Useful String related functions.
pub trait StringUtils {
    /// Get the `char` at a given index from a `String` or `&str`.
    /// If only dealing with ASCII-characters, `byte_at` is recommended.
    /// 
    /// # Arguments
    /// * `index` - The index of the character
    /// # Returns
    /// A `Option<char>`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = String::from("Hello");
    /// assert_eq!('H', s.char_at(0).unwrap());
    /// assert_eq!('e', s.char_at(1).unwrap());
    /// assert_eq!('l', s.char_at(2).unwrap());
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
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
    /// # Returns
    /// A `u8`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = String::from("Hello");
    /// assert_eq!('H', s.byte_at(0) as char);
    /// assert_eq!('e', s.byte_at(1) as char);
    /// assert_eq!('l', s.byte_at(2) as char);
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
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
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = "([{Some text won't throw it off}])";
    /// 
    /// assert_eq!(Ok(true), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = String::from("([{}])");
    /// 
    /// assert_eq!(Ok(true), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = "([{}]";
    /// 
    /// assert_eq!(Err(5), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = "([{Some text won't throw it off}]))";
    /// 
    /// assert_eq!(Err(34), s.validate_brackets());
    /// ```
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// let s = "([{]}))";
    /// 
    /// assert_eq!(Err(3), s.validate_brackets());
    /// ```
    fn validate_brackets(&self) -> Result<bool, usize>;
    /// Checks if a String is alphanumeric.
    /// # Returns
    /// A `Result<bool, usize>`: `true` if it is alphanumeric, else the index of the non alpha numeric character.
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// assert_eq!(true, "h3ll0".is_alphanumeric().is_ok());
    /// assert_eq!(true, "30".is_alphanumeric().is_ok());
    /// assert_eq!(false, "äÄ".is_alphanumeric().is_ok());
    /// assert_eq!(2, "Heä10!".is_alphanumeric().err().unwrap());
    /// ```
    fn is_alphanumeric(&self) -> Result<bool, usize>;
    /// Checks if a String is numeric.
    /// # Returns
    /// A `Result<bool, usize>`: `true` if it is alphanumeric, else the index of the non alpha numeric character.
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// assert_eq!(1, "3ll0".is_numeric().err().unwrap());
    /// assert_eq!(true, "30".is_numeric().is_ok());
    /// ```
    fn is_numeric(&self) -> Result<bool, usize>;
    /// Gets the similarity between two Strings ( correct characters divided by length). In other words, this similarity measures the character changes needed for the two Strings to match.
    /// # Arguments
    /// * `other: &str` - The other string to be compared to.
    /// # Returns
    /// A `f32`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// assert_eq!("Hello!".similarity_with("hello."), 4.0 / 6.0);
    /// assert_eq!("ABCD".similarity_with("ABCDEFGH"), 4.0 / 8.0);
    /// assert_eq!("abcdEfGh".similarity_with("abcD"), 3.0 / 8.0);
    /// assert_eq!("my_test_var".similarity_with("ym_test_var"), 9.0 / 11.0);
    /// ```
    #[must_use]
    fn similarity_with(&self, other: &str) -> f32;
    /// Calculate the Levenshtein-distance between two Strings.
    /// # Arguments
    /// * `a: &str`
    /// * `b: &str`
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// assert_eq!("Japan".levenshtein_dist_with("Canada"), 4);
    /// assert_eq!("kitten".levenshtein_dist_with("sitting"), 3);
    /// ```
    #[must_use]
    fn levenshtein_dist_with(&self, other: &str) -> usize;
    /// Converts a given String into an `isize`. This function
    /// ignores whitespaces, commas and full stops.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::stringhelpers::StringUtils;
    /// 
    /// assert_eq!("  -  5 52 3 ".to_isize().unwrap(), -5523);
    /// assert_eq!("256".to_isize().unwrap(), 256);
    /// ```
    #[must_use]
    fn to_isize(&self) -> Option<isize>;
}

impl StringUtils for String {
    #[inline]
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
    #[inline]
    fn byte_at(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }
    #[inline]
    fn validate_brackets(&self) -> Result<bool, usize> {
        backend_val_brackets(self)
    }
    #[inline]
    fn is_alphanumeric(&self) -> Result<bool, usize> {
        backend_alphanumeric(self)
    }
    #[inline]
    fn is_numeric(&self) -> Result<bool, usize> {
        backend_numeric(self)
    }
    #[inline]
    fn similarity_with(&self, other: &str) -> f32 {
        backend_sim(self, other)
    }
    #[inline]
    fn levenshtein_dist_with(&self, other: &str) -> usize {
        backend_levenshtein(self, other)
    }
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        backend_to_isize(self)
    }
}

impl StringUtils for str {
    #[inline]
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
    #[inline]
    fn byte_at(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }
    #[inline]
    fn validate_brackets(&self) -> Result<bool, usize> {
        backend_val_brackets(self)
    }
    #[inline]
    fn is_alphanumeric(&self) -> Result<bool, usize> {
        backend_alphanumeric(self)
    }
    #[inline]
    fn is_numeric(&self) -> Result<bool, usize> {
        backend_numeric(self)
    }
    #[inline]
    fn similarity_with(&self, other: &str) -> f32 {
        backend_sim(self, other)
    }
    #[inline]
    fn levenshtein_dist_with(&self, other: &str) -> usize {
        backend_levenshtein(self, other)
    }
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        backend_to_isize(self)
    }
}

fn backend_val_brackets(s: &str) -> Result<bool, usize> {
    let mut res_vec: Vec<char> = Vec::with_capacity(s.len());
    let mut i: usize = 0;
    for c in s.chars() {
        match c {
            '[' | '{' | '(' => { res_vec.push(c); },
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

fn backend_numeric(s: &str) -> Result<bool, usize> {
    for c in s.chars().enumerate() {
        if !(c.1 as u8).is_in_range(48, 57)
        { return Err(c.0); }
    }
    Ok(true)
}

fn backend_alphanumeric(s: &str) -> Result<bool, usize> {
    for c in s.chars().enumerate() {
        if !((c.1 >= '0' && c.1 <= '9') ||
             (c.1 >= 'a' && c.1 <= 'z') || 
             (c.1 >= 'A' && c.1 <= 'Z'))
        { return Err(c.0); }
    }
    Ok(true)
}

fn backend_sim(s1: &str, s2: &str) -> f32 {
    let mut sum = 0;

    match s1.len() > s2.len() {
        true => { 
            for i in s2.chars().enumerate() {
                if unsafe { s1.char_at(i.0).unwrap_unchecked() } == i.1
                { sum.inc(); }
            }
        }
        false => { 
            for i in s1.chars().enumerate() {
                if unsafe { s2.char_at(i.0).unwrap_unchecked() } == i.1
                { sum.inc(); }
            }
        }
    }

    sum as f32 / std::cmp::max(s1.len(), s2.len()) as f32
}

// *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
// *   Original Implementation due to Titus Wormer (github.com/wooorm).  *
// *              https://github.com/wooorm/levenshtein-rs               *
// *                   Licensed under the MIT License.                   *
// *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
fn backend_levenshtein(str1: &str, str2: &str) -> usize {
    if str1 == str2
    { return 0; }
    
    let mut res  = 0;
    let str1_len = str1.chars().count();
    let str2_len = str2.chars().count();

    if str1_len == 0
    { return str2_len; }
    if str2_len == 0
    { return str1_len; }

    let mut buffer: Vec<usize> = (1..).take(str1_len).collect();
    let mut str1_dist:  usize;
    let mut str2_dist:  usize;

    for (index_str2, char_str2) in str2.chars().enumerate() {
        res       = index_str2;
        str1_dist = index_str2;

        for (index_str1, char_str1) in str1.chars().enumerate() {

            str2_dist =
            match char_str1 == char_str2 {
                true  => { str1_dist }
                false => { str1_dist + 1 }
            };

            str1_dist = buffer[index_str1];

            res =
            if str1_dist > res {
                match str2_dist > res {
                    true  => { res + 1 }
                    false => { str2_dist }
                }
            }
            else if str1_dist < str2_dist
            { str1_dist + 1 }
            else
            { str2_dist };

            buffer[index_str1] = res;
        }
    }

    res
}

fn backend_to_isize(s: &str) -> Option<isize> {
    let     _s           = s.trim();
    let mut res          = 0;
    let     flag         = _s.starts_with('-');
    let mut power_of_ten = 1;
    for (i, character) in _s.chars().rev().enumerate() {
        if character == '-' && i != _s.len() - 1
        { return None; }
        match character {
            '1' => res.inc_by(    power_of_ten),
            '2' => res.inc_by(2 * power_of_ten),
            '3' => res.inc_by(3 * power_of_ten),
            '4' => res.inc_by(4 * power_of_ten),
            '5' => res.inc_by(5 * power_of_ten),
            '6' => res.inc_by(6 * power_of_ten),
            '7' => res.inc_by(7 * power_of_ten),
            '8' => res.inc_by(8 * power_of_ten),
            '9' => res.inc_by(9 * power_of_ten),
            _   => { }
        }
        if !character.is_whitespace()
        { power_of_ten *= 10; }
    }
    if flag
    { res *= -1; }

    Some(res)
}
/// A Rust implementation of C's `strcmp()` function.
/// # Arguments
/// * `s1` - The String reference to be compared to.
/// * `s2` - The String which is compared with `s1`.
/// # Returns
/// It returns:
/// * `0`, if `s1 == s2`.
/// * `s2[i] - s1[i]`, if `s1[i] < s2[i]` in ASCII.
/// * `s1[i] - s2[i]`, if `s1[i] > s2[i]` in ASCII.
/// # Examples
/// ```
/// use lib_rapid::compsci::stringhelpers::strcmp;
/// let a = "hello";
/// let b = "hEllo";
/// assert!(strcmp(a, b) == 101 - 69);
/// // 101 == 'e' in ASCII.
/// // 69 == `E` in ASCII.
/// ```
/// ```
/// use lib_rapid::compsci::stringhelpers::strcmp;
/// let a = "hello";
/// let b = "hello";
/// assert!(strcmp(a, b) == 0);
/// ```
/// ```
/// use lib_rapid::compsci::stringhelpers::strcmp;
/// let a = "Hello";
/// let b = "hello";
/// assert!(strcmp(a, b) == 72 - 104);
/// // 72 == 'H' in ASCII.
/// // 104 == `h` in ASCII.
/// ```
#[must_use]
pub const fn strcmp(s1: &str, s2: &str) -> i16 {
    let mut i:    usize = 0;
    let mut flag: i16   = 0;
    while flag == 0 {
        flag = s1.as_bytes()[i] as i16 - s2.as_bytes()[i] as i16;
        i += 1;
        if i == s1.len()
        { break; }
    }
    flag
}