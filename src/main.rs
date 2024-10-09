use std::{
    cell::OnceCell,
    env, fs,
    io::{self, Write},
    process::{exit, Stdio},
};

static HAD_ERROR: OnceCell<()> = OnceCell::new();

pub fn had_error() -> bool {
    HAD_ERROR.get().is_some()
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
        io::stdout().flush();
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

pub struct Scanner {}

impl Scanner {
    pub fn new(source: &str) -> Self {
        unimplemented!()
    }

    fn scanTokens(&self) -> Vec<Token> {
        todo!()
    }
}
pub struct Token {}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn run(source: &str) {
    println!("{source}");
    let scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scanTokens();

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
    HAD_ERROR.get_or_init(|| ());
}

#[cfg(test)]
mod test {}
