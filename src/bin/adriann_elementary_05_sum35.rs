//! # Adrian Neumann's Programming Problems - Elementary 05
//! From [Adrian Neumann's programming Problems](https://adriann.github.io/programming_problems.html)
//! ## Problem
//! > 5. Modify the previous program such that only multiples of three or five are considered in the sum,
//! > e.g. 3, 5, 6, 9, 10, 12, 15 for n=17
//!
//! ## Lessons Learned
//! * This time I added a separate module for the function summing the numbers.
//! * Getting the path to the module right.
//! * It would be nice to have `fn sum35<T>(n:T)` instead of `fn sum35(n:u32)`.

use learning_rust::adriann::sum35;
use std::io;

fn main() {
    println!("Please, enter a positive number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read your input.");

    let maybe_n = input.lines().next().unwrap().parse::<u32>();

    match maybe_n {
        Ok(n) => println!("Result: {}", sum35(n)),
        Err(err) => println!("No result: {}", err),
    }
}
