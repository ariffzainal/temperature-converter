# Temperature Converter with Rust

A command-line temperature converter built in Rust.

Converts between Celsius and Fahrenheit with full input validation.

## Features

- Converts Celsius to Fahrenheit and Fahrenheit to Celsius
- Validates unit input, rejects anything that is not C or F
- Validates temperature input, rejects non-numeric values
- Displays results to 2 decimal places
- Handles edge cases without crashing

## How to run

Make sure you have Rust installed. Then:

```bash
git clone git@github.com:yourusername/temperature-converter.git
cd temperature-converter
cargo run
```

## What I learned building this

- Reading and validating user input in Rust
- String manipulation with trim() and to_uppercase()
- Type conversion from String to f64 using parse()
- Input validation loops using loop and break
- Error handling with match, Ok() and Err()
- Variable scope and shadowing
- Why read_line() appends and how to handle it with clear()

## Built as part of learning Rust

Following The Rust Book (https://doc.rust-lang.org/book/).
Chapter 3 practice project.