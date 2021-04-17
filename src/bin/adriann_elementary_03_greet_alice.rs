//! # Adrian Neumann's Programming Problems - Elementary 03
//! From [Adrian Neumann's programming Problems](https://adriann.github.io/programming_problems.html)
//! ## Problem
//! > 3. Modify the previous program such that only the users Alice and Bob are greeted with their names.
//!
//! ## Lessons Learned
//! * Inside `match` you can use `|` to match one out of multiple values.

use std::io;

fn main() {
    println!("Please, enter your name: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read your name.");

    let name = input.lines().next().expect("Could not read your name.");

    match &name as &str {
        "" => println!("Poor thing! Don't even have a name!"),
        "name" => println!("Funny, your name is name!"),
        "Alice" | "Bob" => println!("Hello, {}!", name),
        _ => println!("\u{1F636}"),
    }
}
