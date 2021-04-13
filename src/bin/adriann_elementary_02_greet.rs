//! # Adrian Neumann's Programming Problems - Elementary 02
//! From [Adrian Neumann's programming Problems](https://adriann.github.io/programming_problems.html)
//! ## Problem
//! > 2. Write a program that asks the user for their name and greets them with their name.
//!
//! ## Lessons Learned
//! * When working with `std::io` directly, you need to
//!   consume `std::io::Result` values.
use std::io;

fn main() {
    println!("Please, enter your name: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Coud not read your name.");
    
    let name = input.lines().next()
        .expect("Could not read your name.");

    match &name as &str {
        ""     => println!("Poor thing! Don't even have a name!"),
        "name" => println!("Funny, your name is name!"),
        _      => println!("Hello, {}!", name)
    }
}
