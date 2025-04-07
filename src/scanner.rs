use crate::Context;

#[derive(PartialEq, Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    RightBracket,
    LeftBracket,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    SemiColon,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    EOF,
    Slash,
    String(String),
    /// String is the string from which the number was generated. eg: 42 and 42.0 have same vbalue
    /// but not the same string, this is just for testing and reporting, idk a better way to do
    /// this.
    Number(String, f64),
    Identifier(String),
    And,
    Class,
    Else,
    False,
    For,
    Fun,
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
}
impl Token {
    pub fn token_type(&self) -> String {
        String::from(match self {
            Token::EOF => "EOF",
            Token::LeftParen => "LEFT_PAREN",
            Token::RightParen => "RIGHT_PAREN",
            Token::LeftBrace => "LEFT_BRACE",
            Token::RightBrace => "RIGHT_BRACE",
            Token::RightBracket => "RIGHT_BRACKET",
            Token::LeftBracket => "LEFT_BRACKET",
            Token::Star => "STAR",
            Token::Dot => "DOT",
            Token::Comma => "COMMA",
            Token::Plus => "PLUS",
            Token::Minus => "MINUS",
            Token::SemiColon => "SEMICOLON",
            Token::Equal => "EQUAL",
            Token::EqualEqual => "EQUAL_EQUAL",
            Token::Bang => "BANG",
            Token::BangEqual => "BANG_EQUAL",
            Token::Less => "LESS",
            Token::LessEqual => "LESS_EQUAL",
            Token::Greater => "GREATER",
            Token::GreaterEqual => "GREATER_EQUAL",
            Token::Slash => "SLASH",
            Token::String(_) => "STRING",
            Token::Number(_, _) => "NUMBER",
            Token::Identifier(_) => "IDENTIFIER",
            Token::And => "AND",
            Token::Class => "CLASS",
            Token::Else => "ELSE",
            Token::False => "FALSE",
            Token::For => "FOR",
            Token::Fun => "FUN",
            Token::If => "IF",
            Token::Nil => "NIL",
            Token::Or => "OR",
            Token::Print => "PRINT",
            Token::Return => "RETURN",
            Token::Super => "SUPER",
            Token::This => "THIS",
            Token::True => "TRUE",
            Token::Var => "VAR",
            Token::While => "WHILE",
        })
    }
    pub fn lexeme(&self) -> String {
        String::from(match self {
            Token::EOF => "",
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LeftBrace => "{",
            Token::RightBrace => "}",
            Token::RightBracket => "[",
            Token::LeftBracket => "]",
            Token::Star => "*",
            Token::Dot => ".",
            Token::Comma => ",",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::SemiColon => ";",
            Token::Equal => "=",
            Token::EqualEqual => "==",
            Token::Bang => "!",
            Token::BangEqual => "!=",
            Token::Less => "<",
            Token::LessEqual => "<=",
            Token::Greater => ">",
            Token::GreaterEqual => ">=",
            Token::Slash => "/",
            Token::String(text) => return format!("\"{text}\""),
            Token::Number(num_as_str, _) => return format!("{num_as_str}"),
            Token::Identifier(ident) => return format!("{ident}"),
            Token::And => "and",
            Token::Class => "class",
            Token::Else => "else",
            Token::False => "false",
            Token::For => "for",
            Token::Fun => "fun",
            Token::If => "if",
            Token::Nil => "nil",
            Token::Or => "or",
            Token::Print => "print",
            Token::Return => "return",
            Token::Super => "super",
            Token::This => "this",
            Token::True => "true",
            Token::Var => "var",
            Token::While => "while",
        })
    }

    pub fn literal(&self) -> String {
        match self {
            Token::String(text) => text,
            Token::Number(_, num) => {
                if num.fract() == 0.0 {
                    return format!("{}.0", num);
                }
                return format!("{}", num);
            }
            _ => "null",
        }
        .into()
    }
}

fn is_valid_identifier_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
pub fn tokenize(input: &str, ctx: &mut Context) -> Vec<Token> {
    use Token::*;
    let mut tokens = vec![];
    let mut line: u64 = 1;

    let mut chars = input.chars().peekable();
    'main: while let Some(char) = chars.next() {
        tokens.push(match char {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            '[' => LeftBracket,
            ']' => RightBracket,
            '*' => Star,
            '.' => Dot,
            '+' => Plus,
            '-' => Minus,
            ';' => SemiColon,
            ',' => Comma,
            '=' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    EqualEqual
                } else {
                    Equal
                }
            }
            '!' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    BangEqual
                } else {
                    Bang
                }
            }
            '<' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    LessEqual
                } else {
                    Less
                }
            }
            '>' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    GreaterEqual
                } else {
                    Greater
                }
            }
            '/' => {
                if let Some('/') = chars.peek() {
                    while let Some(character) = chars.next() {
                        if character == '\n' {
                            line += 1;
                            break;
                        }
                    }
                    continue;
                } else {
                    Slash
                }
            }

            '"' => {
                let mut text = "".to_string();
                loop {
                    if let Some(character) = chars.next() {
                        if character == '"' {
                            break;
                        } else {
                            text.push(character);
                        }
                    } else {
                        ctx.error("Unterminated string.", line);
                        continue 'main;
                    }
                }
                String(text)
            }
            '\n' => {
                line += 1;
                continue;
            }
            '\t' | ' ' | '\r' => {
                continue;
            }
            num if num.is_digit(10) => {
                let mut encountered_decimal_point = false;
                let mut as_string = num.to_string();
                while let Some(next) = chars.peek() {
                    if !next.is_digit(10) {
                        if *next != '.' {
                            break;
                        }
                        // next == '.'
                        if encountered_decimal_point {
                            break;
                        }
                        encountered_decimal_point = true;
                    }

                    as_string.push(
                        chars
                            .next()
                            .expect("peeked, this branch should only be run if peek wasn't None"),
                    );
                }

                let num: f64 = as_string
                    .parse()
                    .expect("parsing should guarantee this is valid");
                Number(as_string, num)
            }

            letter if is_valid_identifier_char(letter) => {
                let mut name = letter.to_string();
                while let Some(next) = chars.peek() {
                    if !(next.is_ascii_alphanumeric() || *next == '_') {
                        break;
                    }

                    name.push(
                        chars
                            .next()
                            .expect("peeked, this branch should only be run if peek wasn't None"),
                    );
                }

                match name.as_ref() {
                    "and" => And,
                    "class" => Class,
                    "else" => Else,
                    "false" => False,
                    "for" => For,
                    "fun" => Fun,
                    "if" => If,
                    "nil" => Nil,
                    "or" => Or,
                    "print" => Print,
                    "return" => Return,
                    "super" => Super,
                    "this" => This,
                    "true" => True,
                    "var" => Var,
                    "while" => While,
                    _ => Identifier(name),
                }
            }

            invalid => {
                ctx.error(&format!("Unexpected character: '{invalid}'"), line);
                continue;
            } // Ignore other characters for now
        });
    }
    tokens.push(EOF);
    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    // Mock Context struct for testing purposes

    #[test]
    fn test_single_char_tokens() {
        let input = "+ - * / ( ) { } ; , .";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        let expected_tokens = vec![
            Token::Plus,
            Token::Minus,
            Token::Star,
            Token::Slash,
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::SemiColon,
            Token::Comma,
            Token::Dot,
            Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens);
        assert!(ctx.errors.is_empty());
    }

    #[test]
    fn test_numbers() {
        let input = "42 3.14 0.5";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        let expected_tokens = vec![
            Token::Number("42".to_string(), 42.0),
            Token::Number("3.14".to_string(), 3.14),
            Token::Number("0.5".to_string(), 0.5),
            Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens);
        assert!(ctx.errors.is_empty());
    }

    #[test]
    fn test_strings() {
        let input = "\"hello\" \"world\" \"unterminated string";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        let expected_tokens = vec![
            Token::String("hello".to_string()),
            Token::String("world".to_string()),
            Token::EOF, // Unterminated string is not added to tokens
        ];

        assert_eq!(tokens, expected_tokens);
        // There should be one error for the unterminated string
        assert_eq!(ctx.errors.len(), 1);
        assert_eq!(ctx.errors[0], ("Unterminated string.".to_string(), 1));
    }

    #[test]
    fn test_identifiers_and_keywords() {
        let input = "var x = 42; if (x > 0) { print x; }";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        let expected_tokens = vec![
            Token::Var,
            Token::Identifier("x".to_string()),
            Token::Equal,
            Token::Number("42".to_string(), 42.0),
            Token::SemiColon,
            Token::If,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Greater,
            Token::Number("0".to_string(), 0.0),
            Token::RightParen,
            Token::LeftBrace,
            Token::Print,
            Token::Identifier("x".to_string()),
            Token::SemiColon,
            Token::RightBrace,
            Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens);
        assert!(ctx.errors.is_empty());
    }

    #[test]
    fn test_operators() {
        let input = "== != <= >= = ! < >";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        let expected_tokens = vec![
            Token::EqualEqual,
            Token::BangEqual,
            Token::LessEqual,
            Token::GreaterEqual,
            Token::Equal,
            Token::Bang,
            Token::Less,
            Token::Greater,
            Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens);
        assert!(ctx.errors.is_empty());
    }

    #[test]
    fn test_comments() {
        let input = "// This is a comment\nvar x = 42; // Another comment\nx;";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        let expected_tokens = vec![
            Token::Var,
            Token::Identifier("x".to_string()),
            Token::Equal,
            Token::Number("42".to_string(), 42.0),
            Token::SemiColon,
            Token::Identifier("x".to_string()),
            Token::SemiColon,
            Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens);
        assert!(ctx.errors.is_empty());
    }

    #[test]
    fn test_invalid_characters() {
        let input = "@ # $ % ^ &";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        // Only EOF token should be present
        let expected_tokens = vec![Token::EOF];

        assert_eq!(tokens, expected_tokens);
        // Errors should be reported for each invalid character
        assert_eq!(ctx.errors.len(), 6);
        let expected_errors = vec![
            ("Unexpected character: '@'".to_string(), 1),
            ("Unexpected character: '#'".to_string(), 1),
            ("Unexpected character: '$'".to_string(), 1),
            ("Unexpected character: '%'".to_string(), 1),
            ("Unexpected character: '^'".to_string(), 1),
            ("Unexpected character: '&'".to_string(), 1),
        ];
        assert_eq!(ctx.errors, expected_errors);
    }

    #[test]
    fn test_number_followed_by_identifier() {
        let input = "42abc";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        let expected_tokens = vec![
            Token::Number("42".to_string(), 42.0),
            Token::Identifier("abc".to_string()),
            Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens);
        assert!(ctx.errors.is_empty());
    }

    #[test]
    fn test_invalid_numbers() {
        let input = "1..2";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        let expected_tokens = vec![
            Token::Number("1.".to_string(), 1.0),
            Token::Dot,
            Token::Number("2".to_string(), 2.0),
            Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens);
        assert!(ctx.errors.is_empty());
    }

    #[test]
    fn test_identifiers_with_underscores_and_digits() {
        let input = "_var var1 var_name";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        let expected_tokens = vec![
            Token::Identifier("_var".to_string()),
            Token::Identifier("var1".to_string()),
            Token::Identifier("var_name".to_string()),
            Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens);
        assert!(ctx.errors.is_empty());
    }

    #[test]
    fn test_unterminated_string_with_newline() {
        let input = "\"This is a\nstring with a newline\"";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        let expected_tokens = vec![
            Token::String("This is a\nstring with a newline".to_owned()),
            Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_multiline_comments() {
        let input = "// Comment line 1\n// Comment line 2\nvar x = 10;";
        let mut ctx = Context::new();
        let tokens = tokenize(input, &mut ctx);

        let expected_tokens = vec![
            Token::Var,
            Token::Identifier("x".to_string()),
            Token::Equal,
            Token::Number("10".to_string(), 10.0),
            Token::SemiColon,
            Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens);
        assert!(ctx.errors.is_empty());
    }
}
