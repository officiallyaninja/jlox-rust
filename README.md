# jlox-rust

A Rust interpreter for the Lox programming language, inspired by the [jlox interpreter](http://craftinginterpreters.com/) by Robert Nystrom.

## Overview

**jlox-rust** is an implementation of the Lox programming language written in Rust. It follows the principles outlined in the book *Crafting Interpreters*.


## Features
- **Script Execution**: Execute Lox scripts from files.
- **Partial Language Support**: Printing, expression evaluation, variable declaration and assignment implemented
- **Interactive REPL**: Run Lox interactively through a Read-Eval-Print Loop (REPL).
## TODO
- **Complete Lox Language Support**: Covers all the language features including variables, functions, classes, inheritance, and more.
- **Error Handling**: Descriptive error messages to make debugging easier.
- **Extensible Design**: Code structured for ease of extension and learning.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Installation

Clone the repository:

```bash
git clone https://github.com/officiallyaninja/jlox-rust.git
cd jlox-rust
```

Build the project:

```bash
cargo build --release
```

### Running the Interpreter

To start the interpreter in REPL mode:

```bash
cargo run
```

To run a Lox script:

```bash
cargo run [command] path/to/your_script.lox
```

## Usage
### Running Scripts

Create a file named `example.lox` with the following content:

```lox
// example.lox
var x = 2
var y = 3

print (x + y) * 10 + 5; // Outputs 55
```

Run the script:

```bash
cargo run run example.lox
```
### Other Commands
#### Tokenize
Tokenizes file and prints tokens to Stdout
#### Parse
Parses first line of file assuming it to be a bare expression and print out AST
#### Evaluate
Evaluates first line of file assuming it to be a bare expressiona and prints out the value
#### Run
Runs the Lox program
#### [No command]
Starts REPL
## Project Structure

- `src/`: Source code of the interpreter.
  - `main.rs`: Entry point of the application.
  - `scanner.rs`: Tokenizes the source code into lexemes.
  - `parser.rs`: Parses tokens into an abstract syntax tree (AST).
  - `interpreter.rs`: Evaluates the AST and executes Lox code.
  - `environment.rs`: Manages scopes and variable bindings.

## Acknowledgments

- **Robert Nystrom** for creating *Crafting Interpreters*, which served as the basis for this project.
- The Rust community for their excellent tools and resources

