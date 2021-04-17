//! # Some Code for solving Adrian Neumann's Programming Problems
//! From [Adrian Neumann's programming Problems](https://adriann.github.io/programming_problems.html)

//! This module contains functions required for some of the problems.

use core::ops::{Rem, Add};

/// Sums up all numbers that are divisible by 3 or 5 up to a number `n`.
/// 
/// This function sums all positive integer numbers divisible by 3 or 5
/// up to and including an upper bound `n`.
/// 
/// # Parameter
/// 
/// * `n`   - the upper bound (included)
/// 
/// # Examples
/// 
/// ```rust
/// use learning_rust::adriann::sum35;
/// 
/// let s6 = sum35(6);
/// assert_eq!(3 + 5 + 6, s6);
/// 
/// let s19 = sum35(19);
/// assert_eq!(3 + 5 + 6 + 9 + 10 + 12 + 15 + 18, s19);
/// ```
pub fn sum35(n : u32) -> u32 {
    let zero  = 0u32;
    let one = 1u32;
    let range = one..=n;
    let pred = |k| k % 3 == zero || k % 5 == zero;

    let mut result = zero;
    for i in range {
        if pred(i) {
            result += i;
        }
    }

    result
}

/// Sums up all numbers that are divisible by 3 or 5 up to a number `n`.
/// 
/// This function sums all positive integer numbers divisible by 3 or 5
/// up to and including an upper bound `n`.
/// 
/// # Parameter
/// 
/// * `n`   - the upper bound (included)
/// 
/// # Examples
/// 
/// ```rust
/// use learning_rust::adriann::sum35_generic;
/// 
/// let s6 = sum35_generic(6u32);
/// assert_eq!(3 + 5 + 6, s6);
/// 
/// let s19 = sum35_generic(19u32);
/// assert_eq!(3 + 5 + 6 + 9 + 10 + 12 + 15 + 18, s19);
/// ```
pub fn sum35_generic<T: Sized + Rem<T, Output = T> + Add<T, Output = T> + PartialOrd + Ord + Eq  + From<u8> + std::ops::AddAssign + Copy>(n : T) -> T {
    let zero  = T::from(0u8);
    let one   = T::from(1u8);
    let three = T::from(3u8);
    let five  = T::from(5u8);
    let pred = |k : &T| (*k % three) == zero || (*k % five) == zero;

    let mut i = one;
    let mut result = zero;
    while i <= n {
        if pred(&i) {
            result += i;
        }
        i += one;
    }

    result
}