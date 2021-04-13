//! # Adrian Neumann's Programming Problems - Elementary 01
//! From [Adrian Neumann's programming Problems](https://adriann.github.io/programming_problems.html)
//! ## Problem
//! > 1. Write a program that prints ‘Hello World’ to the screen.

use std::io;
use std::io::Write;

/// This solution uses the `println!` macro.
fn use_println() {
    println!("Hello, world!");
}

/// This solution writes to `io:stdout` directly.
fn use_stdout() -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    out.write_all(b"Hello, world!\n")?;
    Ok(())
}

fn main() -> io::Result<()> {
    use_println();
    use_stdout()?;

    Ok(())
}
