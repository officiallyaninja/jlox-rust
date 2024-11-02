mod environment;
mod literal;
mod parser;
mod scanner;
use std::env;
use std::fs;

use environment::Environment;

// TODO: use clap

struct Context {
    errors: Vec<(String, u64)>,
}

impl Context {
    fn new() -> Self {
        Context { errors: vec![] }
    }

    fn error(&mut self, message: &str, line: u64) {
        self.errors.push((message.to_string(), line));
    }

    fn print_errors(&self) {
        for (message, line) in &self.errors {
            eprintln!("[line {line}] Error: {}", message);
        }
    }
}

fn main() {
    let mut context = Context::new();
    let mut environment = Environment::new();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <command> <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });
    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.

            let tokens = scanner::tokenize(&file_contents, &mut context);
            context.print_errors();
            for token in tokens {
                println!(
                    "{} {} {}",
                    token.token_type(),
                    token.lexeme(),
                    token.literal()
                )
            }
        }
        "parse" => {
            let tokens = scanner::tokenize(&file_contents, &mut context);
            context.print_errors();
            let mut parser = parser::Parser::new(tokens);
            let parsed = parser.expression();
            println!("{}", parsed.pretty_string());
        }
        "evaluate" => {
            let tokens = scanner::tokenize(&file_contents, &mut context);
            context.print_errors();
            let mut parser = parser::Parser::new(tokens);
            let parsed = parser.expression();
            println!("{}", parsed.evaluate(&mut environment))
        }

        "run" => {
            let tokens = scanner::tokenize(&file_contents, &mut context);
            context.print_errors();
            let mut parser = parser::Parser::new(tokens);
            let program = parser.parse();
            for statement in dbg!(program) {
                statement.execute(&mut environment);
            }
        }

        _ => {
            eprintln!("Unknown command: {}", command);
            return;
        }
    }
    if !context.errors.is_empty() {
        std::process::exit(65);
    }
}
