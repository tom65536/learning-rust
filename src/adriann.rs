//! # Some Code for solving Adrian Neumann's Programming Problems
//! From [Adrian Neumann's programming Problems](https://adriann.github.io/programming_problems.html)

//! This module contains functions required for some of the problems.

/// Elementary Problem 5:
/// Sum up all numbers that are divisible by 3 or 5 up to a number `n`
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