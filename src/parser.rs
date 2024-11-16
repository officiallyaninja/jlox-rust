mod expression;
mod statement;

use self::{expression::Expr, statement::Stmt};
use crate::{literal::Literal, scanner::Token};
use std::iter::Peekable;

pub struct Parser<T>
where
    T: Iterator<Item = Token>,
{
    tokens: Peekable<T>,
    line: u64,
    //last_processed_stmt - for errors errors or something
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    fn syntax_error(&self, message: &str) -> ! {
        panic!("[line {}] Error: {}", self.line, message);
    }

    fn check_semicolon(&mut self) {
        let Token::SemiColon = self
            .tokens
            .next()
            .expect("every statement should end in a semicolon")
        else {
            self.syntax_error("every statement should end in a semicolon")
        };
    }
    // fn runtime_error(&self, message: &str) -> ! {
    //     panic!("[line {}] Error: {}", self.line, message);
    // }

    pub fn new<U: IntoIterator<IntoIter = T>>(tokens: U) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            line: 1,
        }
    }

    // NOTE: maybe should take owned self?
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while self.tokens.peek() != Some(&Token::EOF) {
            statements.push(self.statement());
        }
        statements
    }

    pub fn statement(&mut self) -> Stmt {
        let stmt = if let Some(Token::Var) = self.tokens.peek() {
            self.declaration_statement()
        } else {
            self.non_declaration_statement()
        };
        match stmt {
            Stmt::Block(_) => {}
            Stmt::Expression(_) | Stmt::Print(_) | Stmt::Var(_, _) => self.check_semicolon(),
        }
        // Removing extraneous semicolons
        while let Some(Token::SemiColon) = self.tokens.peek() {
            self.tokens.next();
        }
        self.line += 1;
        stmt
    }

    fn declaration_statement(&mut self) -> Stmt {
        let Some(Token::Var) = self.tokens.next() else {
            unreachable!("function should only be called if next is Token::Var")
        };

        let Token::Identifier(name) = self
            .tokens
            .next()
            .expect("Var should be followed by identifier")
        else {
            panic!("Var should be followed by identifier");
        };

        if let Token::Equal = self
            .tokens
            .peek()
            .expect("next token should either be semicolon or equal")
        {
            self.tokens
                .next()
                .expect("throwing away the equals we know exists here");
            return Stmt::Var(name, self.expression());
        }

        Stmt::Var(name, Literal::Nil.into())
    }

    /// We use this because some places where we accept statements
    /// we only really allow non declaration statements
    fn non_declaration_statement(&mut self) -> Stmt {
        if let Some(Token::Print) = self.tokens.peek() {
            return self.print_statement();
        }
        if let Some(Token::LeftBrace) = self.tokens.peek() {
            return Stmt::Block(self.block());
        }
        self.expression_statement()
    }

    fn block(&mut self) -> Vec<Stmt> {
        assert_eq!(self.tokens.next(), Some(Token::LeftBrace));
        let mut statements = vec![];
        while self.tokens.peek().is_some() && self.tokens.peek() != Some(&Token::RightBrace) {
            statements.push(self.statement())
        }
        assert_eq!(self.tokens.next(), Some(Token::RightBrace));
        statements
    }

    fn print_statement(&mut self) -> Stmt {
        let Some(Token::Print) = self.tokens.next() else {
            unreachable!("function should only be called if next is Token::Print")
        };
        Stmt::Print(self.expression())
    }

    fn expression_statement(&mut self) -> Stmt {
        Stmt::Expression(self.expression())
    }

    // TODO: Find a way to make this private
    // if that's even desirable
    // (which I think it is but idk)
    pub fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.equality();
        if let Some(_) = self.tokens.next_if_eq(&Token::Equal) {
            let value = self.assignment();
            if let Expr::Variable(name) = expr {
                return Expr::Assign {
                    name,
                    value: Box::new(value),
                };
            } else {
                self.syntax_error("invalid assigment target");
            }
        }
        expr
    }
    fn equality(&mut self) -> Expr {
        use Token::*;
        let mut result = self.comparison();
        while let Some(op) = self
            .tokens
            .next_if(|t| vec![BangEqual, EqualEqual].contains(t))
        {
            let right = self.comparison();
            result = Expr::Binary {
                left: result.into(),
                op,
                right: right.into(),
            };
        }
        result
    }
    fn comparison(&mut self) -> Expr {
        use Token::*;
        let mut result = self.term();
        while let Some(op) = self
            .tokens
            .next_if(|t| vec![Greater, GreaterEqual, Less, LessEqual].contains(t))
        {
            let right = self.term();
            result = Expr::Binary {
                left: result.into(),
                op,
                right: right.into(),
            };
        }
        result
    }

    fn term(&mut self) -> Expr {
        use Token::*;
        let mut result = self.factor();
        while let Some(op) = self.tokens.next_if(|t| vec![Minus, Plus].contains(t)) {
            let right = self.factor();
            result = Expr::Binary {
                left: result.into(),
                op,
                right: right.into(),
            };
        }
        result
    }

    fn factor(&mut self) -> Expr {
        use Token::*;
        let mut result = self.unary();
        while let Some(op) = self.tokens.next_if(|t| vec![Slash, Star].contains(t)) {
            result = Expr::Binary {
                left: result.into(),
                op,
                right: self.unary().into(),
            };
        }
        result
    }

    fn unary(&mut self) -> Expr {
        use Token::*;
        if let Some(op) = self.tokens.next_if(|t| vec![Bang, Minus].contains(t)) {
            let right = self.unary();
            Expr::Unary {
                op,
                expr: right.into(),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        match self.tokens.next().unwrap() {
            Token::True => Literal::True,
            Token::False => Literal::False,
            Token::Nil => Literal::Nil,
            Token::Number(name, value) => Literal::Number(value),
            Token::String(value) => Literal::String(value),
            Token::LeftParen => {
                let expr = self.expression();
                let Token::RightParen = self.tokens.next().unwrap() else {
                    self.syntax_error("missing right paren");
                };
                return Expr::Grouping(expr.into());
            }
            Token::Identifier(name) => {
                return Expr::Variable(name);
            }
            //TODO: better error message
            invalid => self.syntax_error(&format!(
                "invalid primary token found {}",
                invalid.token_type()
            )),
        }
        .into()
    }
}

// TODO : Add tests for when you expect things to panic
#[cfg(test)]
mod test {
    use crate::{
        scanner::{tokenize, Token},
        Context,
    };

    use super::Parser;

    fn get_parser(src: &str) -> Parser<impl Iterator<Item = Token>> {
        let mut context = Context::new();
        let tokens = tokenize(src, &mut context).into_iter().peekable();
        assert!(context.errors.is_empty());
        Parser::new(tokens)
    }

    fn utf8_to_string(buffer: &[u8]) -> &str {
        std::str::from_utf8(&buffer)
            .expect("comes from a valid string, so it should be a valid string")
    }

    mod expressions {
        use crate::parser::test::get_parser;
        #[test]
        fn complex() {
            let expr_text = "(5+2)*-6 == 9";
            assert_eq!(
                get_parser(expr_text).expression().to_string_normal(),
                "(5 + 2) * -6 == 9"
            )
        }

        #[test]
        fn equalities() {
            let expr_text = "(5==2) == -6 != 9";
            assert_eq!(
                get_parser(expr_text).expression().to_string_normal(),
                "(5 == 2) == -6 != 9"
            )
        }

        #[test]
        fn literal() {
            let expr_text = "\"testing\"";
            assert_eq!(
                get_parser(expr_text).expression().to_string_normal(),
                "testing"
            )
        }
    }

    mod evaluate {
        use crate::{environment::Environment, literal::Literal, parser::test::get_parser};

        #[test]
        fn equality() {
            let expr_text = "(5+2)*-6 == 9";
            assert_eq!(
                get_parser(expr_text)
                    .expression()
                    .evaluate(&mut Environment::new()),
                Literal::False
            )
        }
        #[test]
        fn arithemtic() {
            let expr_text = "(5+2)*-6";
            assert_eq!(
                get_parser(expr_text)
                    .expression()
                    .evaluate(&mut Environment::new()),
                Literal::Number(-42.0)
            )
        }

        #[test]
        fn arithemtic2() {
            let expr_text = "2 - 3 + 2";
            assert_eq!(
                get_parser(expr_text)
                    .expression()
                    .evaluate(&mut Environment::new()),
                Literal::Number(1.0)
            )
        }

        #[test]
        fn relational() {
            let expr_text = "2 - 3 + 2 < 2";
            assert_eq!(
                get_parser(expr_text)
                    .expression()
                    .evaluate(&mut Environment::new()),
                Literal::True
            )
        }

        #[test]
        fn concatenation() {
            let expr_text = "\"Hello,\" + \" \" + \"World!\"";
            assert_eq!(
                get_parser(expr_text)
                    .expression()
                    .evaluate(&mut Environment::new()),
                Literal::String("Hello, World!".into())
            )
        }
    }

    mod run {
        use crate::{environment::Environment, parser::test::get_parser};

        #[test]
        fn declare() {
            let code = "var x = 5;
                    var x = x + 2;
                    print x;";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                environment = statement.execute(environment, &mut buffer);
            }

            assert_eq!(buffer, b"7\n");
        }

        #[test]
        fn declare2() {
            let code = "var x = 5;
                    var y = x + 2;
                    print x + y;";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                environment = statement.execute(environment, &mut buffer);
            }

            assert_eq!(buffer, b"12\n")
        }

        #[test]
        fn assign() {
            let code = "var x = 5;
                    x = x + 2;
                    print x;";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                environment = statement.execute(environment, &mut buffer);
            }

            assert_eq!(buffer, b"7\n")
        }

        #[test]
        fn complex_assign() {
            let code = "
                    var x = 5; 
                    x = x + 2; // x is 7
                    print x - 2; // prints 5
                    var y = x - 10; // y is -3
                    print y == -3; 
                    print x - y;
                    ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                environment = statement.execute(environment, &mut buffer);
            }

            assert_eq!(buffer, b"5\ntrue\n10\n")
        }
    }
    mod should_not_work {

        use crate::{environment::Environment, parser::test::get_parser};

        #[test]
        #[should_panic]
        fn basic() {
            let code = "
                var x = 2
                print x
                x = x+1
                ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                environment = statement.execute(environment, &mut buffer);
            }
        }

        #[test]
        #[should_panic]
        fn undefined() {
            let code = "
                print(x);
                ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                environment = statement.execute(environment, &mut buffer);
            }
        }
    }
    mod block {
        use crate::{
            environment::Environment,
            parser::test::{get_parser, utf8_to_string},
        };

        #[test]
        fn basic() {
            let code = "
{
    var x = 2;
    print x;
}
            ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                environment = statement.execute(environment, &mut buffer);
            }

            assert_eq!(utf8_to_string(&buffer), "2\n")
        }

        #[test]
        fn nested() {
            let code = "
            var a = \"global a\";
            var b = \"global b\";
            var c = \"global c\";
            {
                var a = \"outer a\";
                var b = \"outer b\";
                {
                    var a = \"inner a\";
                    print a;
                    print b;
                    print c;
                }
                print a;
                print b;
                print c;
            }
            print a;
            print b;
            print c;
            ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                environment = statement.execute(environment, &mut buffer);
            }
            let output = vec![
                "inner a", "outer b", "global c", //
                "outer a", "outer b", "global c", //
                "global a", "global b", "global c", //
            ];
            let mut output = output.join("\n");
            output.push('\n');
            assert_eq!(utf8_to_string(&buffer), output)
        }
    }
}
