//! In here is the macro `eval_postfix!` defined. With that, you can evaluate expressions in reverse polish notation at compile time. Handy, isn't it?
/// Evaluate a mathematical expression in postfix notation ("Reverse Polish Notation") at compile time. greater than, less than etc. are also possible.
/// # What is the advantage?
/// The postfix notation does not need parentheses, which means less typing - especially for long, complicated formulas.
/// Implementations of the postfix notation only need a stack - which is commonly associated with faster evaluations.
/// It does need however a bit time to get used to it.
/// 
/// # Returns
/// The result of the calculation.
/// # Supported Operators
/// * `+` - Plus
/// * `-` - Minus
/// * `*` - Multiplication
/// * `/` - Division
/// * `%` - Modulo
/// * `>` - Greater than
/// * `>=` - Greater than or equal to
/// * `<` - Less than
/// * `<=` - Less than or equal to
/// * `==` - Equals
/// * `!=` - Does not equal
/// * `&&` - Logical AND
/// * `||` - Logical OR
/// * `&` - Bitwise AND
/// * `|` - Bitwise OR
/// * `^` - Bitwise XOR
/// * `>>` - Bitshift Right
/// * `<<` - Bitshift Left
/// # Examples
/// ```
/// extern crate lib_rapid;
/// use lib_rapid::math::postfix::eval_postfix;
/// println!("{}", eval_postfix!(1.0 1.0 + 2.0 %)); // Prints "0", because (1 + 1) % 2 = 0.
/// ```
/// ```
/// use lib_rapid::math::postfix::eval_postfix;
/// assert_eq!(0.0, eval_postfix!(1.0 1.0 + 2.0 %));
/// assert_eq!(16, eval_postfix!(1 1 + 2 / 4 <<));
/// assert_eq!(1, eval_postfix!(16 2 * 5 >>));
/// assert_eq!(true, eval_postfix!(1.0 1.0 + 2.0 % 0.0 ==));
/// ```
#[macro_export]
macro_rules! eval_postfix {
    (@operator [$b:expr, $a:expr $(,$call_stack:expr)*] $operator:tt $($leftover:tt)*) => {
        eval_postfix!([$a $operator $b $(,$call_stack)*] $($leftover)*)
    };
    
    // Could not apply operator
    (@operator $call_stack:tt $operator:tt $($leftover:tt)*) => {
        compile_error!(concat!("Could not apply operator `",
                                stringify!($operator),
                                "` to the current call stack: ",
                                stringify!($call_stack)
                            )
                    )
    };

    // Addition
    ($call_stack:tt + $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack + $($leftover)*)
    };
    
    // Subtraction
    ($call_stack:tt - $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack - $($leftover)*)
    };

    // Multiplication
    ($call_stack:tt * $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack * $($leftover)*)
    };
    
    // Division
    ($call_stack:tt / $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack / $($leftover)*)
    };

    // Modulo
    ($call_stack:tt % $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack % $($leftover)*)
    };

    // Greater than
    ($call_stack:tt > $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack > $($leftover)*)
    };

    // Greater than or equal to
    ($call_stack:tt >= $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack >= $($leftover)*)
    };

    // Less than
    ($call_stack:tt < $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack < $($leftover)*)
    };

    // Less than or equal to
    ($call_stack:tt <= $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack <= $($leftover)*)
    };

    // Equals
    ($call_stack:tt == $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack == $($leftover)*)
    };

    // Not equals
    ($call_stack:tt != $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack != $($leftover)*)
    };

    // Logical AND
    ($call_stack:tt && $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack && $($leftover)*)
    };

    // Logical OR
    ($call_stack:tt || $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack || $($leftover)*)
    };

    // XOR
    ($call_stack:tt ^ $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack ^ $($leftover)*)
    };

    // Bitwise AND
    ($call_stack:tt & $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack & $($leftover)*)
    };

    // Bitwise OR
    ($call_stack:tt | $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack | $($leftover)*)
    };

    // BSL
    ($call_stack:tt << $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack << $($leftover)*)
    };

    // BSR
    ($call_stack:tt >> $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack >> $($leftover)*)
    };

    // Recursively call macro with the rest of the expression
    ([$($call_stack:expr),*] $num:tt $($leftover:tt)*) => {
        eval_postfix!([$num $(,$call_stack)*] $($leftover)*)
    };
    
    // Return final value
    ([$res:expr]) => {
        $res
    };
    
    // Missed operator
    ([$($call_stack:expr),*]) => {
        compile_error!(concat!("Error: Could not find final value \
                                for the expression. \
                                Maybe you missed an operator? \
                                Final call stack: ",
                               stringify!([$($call_stack),*])
                            )
                    )
    };
    
    // Catch everything else
    ($($tokens:tt)*) => {
        eval_postfix!([] $($tokens)*)
    };
}

pub use eval_postfix;