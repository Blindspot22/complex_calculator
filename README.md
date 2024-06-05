# Complex Calculator

A Rust-based command-line calculator capable of performing basic arithmetic operations. The project demonstrates how to structure a Rust application with modules for operations and parsing.

## Features

- Addition, subtraction, multiplication, and division
- Command-line interface
- Error handling for invalid input and division by zero

## Getting Started

### Prerequisites

- Rust (installation instructions available at [rust-lang.org](https://www.rust-lang.org/))

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/Blindspot22/complex_calculator.git
   cd complex_calculator
2. Build the project:
   ```sh
   cargo build
3. Run the project:
   ```sh
   cargo run
## Usage

The calculator accepts simple mathematical expressions in the format `a operator b` (e.g.,`3 + 4`). Supported operators are `+`, `-`, `*``

### Examples
. Addition: `3 + 4`

. Subtraction: `10 - 2`

. Multiplication: `6 * 7`

. Division: `8 / 2`

## Sample Session
   ```sh
   Enter an expression (or type 'exit' to quit): 3 + 4
   Result: 7

   Enter an expression (or type 'exit' to quit): 10 / 0
   Error: Cannot divide by zero

   Enter an expression (or type 'exit' to quit): exit
  ```
## Project Structure
   ```css
   complex_calculator
├── Cargo.toml
└── src
    ├── main.rs
    ├── operations.rs
    └── parser.rs
  ```
. main.rs: Entry point of the application. Handles user input and orchestrates the parsing and computation.

. operations.rs: Contains functions for basic arithmetic operations.

. parser.rs: Responsible for parsing the input expression and calling the appropriate arithmetic function.

## Future Enhancements
. Advanced parsing to support parentheses and operator precedence

. Support for trigonometric functions (e.g., sine, cosine)

. Exponential and logarithmic functions

. Complex number operations

## Contributing
Contributions are welcome! Please fork the repository and create a pull request with your changes.

## License
This project is licensed under the MIT License. See the [LICENSE](https://github.com/Blindspot22/complex_calculator/blob/main/LICENSE) file for details.

## Acknowledgements
This project was created as a learning exercise for Rust programming language.
  ```arduino
  
Repository link `https://github.com/Blindspot22/complex_calculator.git` on GitHub.

This `README.md` provides a clear overview of the project, installation instructions, usage examples, and future enhancement ideas, making it easy for others to understand and contribute to your project.

