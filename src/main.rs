mod scanner;
use std::{
    env, fs,
    io::{self, Write},
    process::exit,
    sync::atomic::AtomicBool,
};

use crate::scanner::Scanner;

static HAD_ERROR: AtomicBool = AtomicBool::new(false);
fn had_error() -> bool {
    HAD_ERROR.load(std::sync::atomic::Ordering::SeqCst)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
    if args.len() > 2 {
        panic!("Error: too many arguments");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_prompt() {
    loop {
        let mut line = String::new();
        print!("> ");
        io::stdout().flush().expect("flush error");
        io::stdin()
            .read_line(&mut line)
            .expect("Error reading input");
        run(&line);
    }
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Error Reading file");
    run(&contents);
    if had_error() {
        exit(65)
    }
}

fn run(source: &str) {
    println!("{source}");
    let mut scanner: Scanner = Scanner::new(source);
    let tokens: Vec<scanner::Token> = scanner.scan_tokens();

    // For now, just print the tokens.
    for token in tokens {
        println!("{token}");
    }
}

fn error(line: u32, message: &str) {
    report(line, "", message);
}

fn report(line: u32, location: &str, message: &str) {
    eprintln!("[line {line}] Error {location}: {message}");
    HAD_ERROR.store(true, std::sync::atomic::Ordering::SeqCst)
}

#[cfg(test)]
mod test {}
