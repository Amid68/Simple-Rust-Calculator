# Simple Rust Calculator

A simple command-line calculator built with Rust that performs basic arithmetic operations such as addition, subtraction, multiplication, and division.

## Project Structure

```
calculator/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── operations.rs
    └── utils/
        ├── input.rs
        └── mod.rs
```

- **Cargo.toml**: Configuration file for the Rust project.
- **main.rs**: Entry point of the application.
- **operations.rs**: Contains functions for arithmetic operations.
- **utils/input.rs**: Handles user input and validation.

## Getting Started

### Prerequisites

- **Rust and Cargo**: Ensure you have Rust and Cargo installed. You can install them from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/Amid68/Simple-Rust-Calculator
   cd calculator
   ```

2. **Run the Calculator**

   ```bash
   cargo run
   ```

## Usage

When you run the application, follow the prompts to perform calculations:

```
Simple Rust Calculator
Enter the first number:
> 10
Enter an operator (+, -, *, /):
> *
Enter the second number:
> 5
Result: 50
```

