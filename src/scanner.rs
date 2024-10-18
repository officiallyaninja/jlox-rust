use std::{clone, thread::current};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.into(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
        }
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: "".into(),
            literal: None,
            line: self.line,
        });
        // TODO: replace with reference?
        self.tokens.clone()
    }
    pub fn scan_token(&self) {
        todo!()
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        todo!();
        //return self.source.charAt(self.current - 1);
    }

    fn add_non_literal(&self, kind: TokenKind) {
        self.addToken(kind, None);
    }

    fn addToken(&self, kind: TokenKind, literal: Option<Literal>) {
        todo!();
        //String text = source.substring(start, current);
        //tokens.add(new Token(type, text, literal, line));
    }
}

#[derive(Clone)]
enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Clone)]
enum Literal {}

#[derive(Clone)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
