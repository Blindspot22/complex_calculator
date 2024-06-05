mod operations;
mod parser;

use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter an expression (or type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        match parser::parse_and_compute(input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}
