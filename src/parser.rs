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
        // TODO: print the line
        panic!("[line {}] Error: {}", self.line, message);
    }

    fn assert_next_token(&mut self, token: Token) {
        let next_token = self.tokens.next();
        match next_token {
            Some(found) if found == token => {}
            Some(found) if found != token => self.syntax_error(&format!(
                "expected '{}' ({}), found '{}' ({})",
                token.lexeme(),
                token.token_type(),
                found.lexeme(),
                found.token_type()
            )),
            None => self.syntax_error(&format!(
                "expected '{}' ({}), found EOF",
                token.lexeme(),
                token.token_type(),
            )),
            _ => unreachable!(),
        }
    }
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

        self.line += 1;
        stmt
    }

    fn declaration_statement(&mut self) -> Stmt {
        self.assert_next_token(Token::Var);

        let Some(Token::Identifier(name)) = self.tokens.next() else {
            panic!("Var should be followed by an identifier");
        };

        let initializer = if self.tokens.next_if_eq(&Token::Equal).is_some() {
            self.expression()
        } else {
            Literal::Nil.into()
        };

        self.assert_next_token(Token::SemiColon);
        return Stmt::Var(name, initializer);
    }

    /// We use this because some places where we accept statements
    /// we only really allow non declaration statements
    fn non_declaration_statement(&mut self) -> Stmt {
        match self
            .tokens
            .peek()
            .expect("statement function should not be called on empty tokenstream")
        {
            Token::If => self.if_statement(),
            Token::Print => self.print_statement(),
            Token::While => self.while_statement(),
            Token::LeftBrace => self.block(),
            _ => self.expression_statement(),
        }
    }

    fn while_statement(&mut self) -> Stmt {
        self.assert_next_token(Token::While);
        self.assert_next_token(Token::LeftParen);
        let condition = self.expression();
        self.assert_next_token(Token::RightParen);

        let body = self.statement();
        Stmt::While(condition, body.into())
    }

    fn if_statement(&mut self) -> Stmt {
        self.assert_next_token(Token::If);
        self.assert_next_token(Token::LeftParen);
        let condition = self.expression();
        self.assert_next_token(Token::RightParen);

        let then_stmt = self.statement().into();
        let else_stmt = if self.tokens.next_if_eq(&Token::Else).is_some() {
            Some(self.statement().into())
        } else {
            None
        };
        Stmt::If {
            condition,
            then_stmt,
            else_stmt,
        }
    }
    fn block(&mut self) -> Stmt {
        self.assert_next_token(Token::LeftBrace);
        let mut statements = vec![];
        while self.tokens.peek().is_some() && self.tokens.peek() != Some(&Token::RightBrace) {
            statements.push(self.statement())
        }
        self.assert_next_token(Token::RightBrace);
        Stmt::Block(statements)
    }

    fn print_statement(&mut self) -> Stmt {
        self.assert_next_token(Token::Print);
        let stmt = Stmt::Print(self.expression());
        self.assert_next_token(Token::SemiColon);
        stmt
    }

    fn expression_statement(&mut self) -> Stmt {
        let stmt = Stmt::Expression(self.expression());
        self.assert_next_token(Token::SemiColon);
        stmt
    }

    pub fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.or();
        if self.tokens.next_if_eq(&Token::Equal).is_some() {
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

    fn or(&mut self) -> Expr {
        let mut expr = self.and();
        while let Some(op) = self.tokens.next_if(|t| t == &Token::Or) {
            let right = self.and();
            expr = Expr::Logical {
                left: expr.into(),
                op,
                right: right.into(),
            };
        }
        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();
        while let Some(op) = self.tokens.next_if(|t| t == &Token::And) {
            let right = self.equality();
            expr = Expr::Logical {
                left: expr.into(),
                op,
                right: right.into(),
            };
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
            Token::Number(_, value) => Literal::Number(value),
            Token::String(value) => Literal::String(value),
            Token::LeftParen => {
                let expr = self.expression();
                self.assert_next_token(Token::RightParen);
                return Expr::Grouping(expr.into());
            }
            Token::Identifier(name) => {
                return Expr::Variable(name);
            }
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

    fn utf8_to_string(buffer: &[u8]) -> Vec<&str> {
        std::str::from_utf8(&buffer)
            .expect("comes from a valid string, so it should be a valid string")
            .split('\n')
            .collect()
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
                statement.execute(&mut environment, &mut buffer);
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
                statement.execute(&mut environment, &mut buffer);
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
                statement.execute(&mut environment, &mut buffer);
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
                statement.execute(&mut environment, &mut buffer);
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
                statement.execute(&mut environment, &mut buffer);
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
                statement.execute(&mut environment, &mut buffer);
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
                statement.execute(&mut environment, &mut buffer);
            }

            assert_eq!(utf8_to_string(&buffer), vec!["2", ""])
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
                statement.execute(&mut environment, &mut buffer);
            }
            let output = vec![
                "inner a", "outer b", "global c", //
                "outer a", "outer b", "global c", //
                "global a", "global b", "global c", //
                "",
            ];
            assert_eq!(utf8_to_string(&buffer), output)
        }
    }

    mod control_flow {
        use crate::{
            environment::Environment,
            parser::test::{get_parser, utf8_to_string},
        };

        #[test]
        fn outputs() {
            let code = "
                print true or true;
                print true or false;
                print false or true;
                print false or false;
                print true and true;
                print true and false;
                print false and true;
                print false and false;
            ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                statement.execute(&mut environment, &mut buffer);
            }

            let output = vec![
                "true", "true", "true", "false", "true", "false", "false", "false", "",
            ];
            assert_eq!(utf8_to_string(&buffer), output)
        }

        #[test]
        fn short_circuiting() {
            let code = "
                print \"hi\" or 2; 
                print nil or \"yes\"; 
            ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                statement.execute(&mut environment, &mut buffer);
            }

            let output = vec!["hi", "yes", ""];
            assert_eq!(utf8_to_string(&buffer), output)
        }

        #[test]
        fn just_if() {
            let code = "
                if (true) {
                print \"true\";
                }
            ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                statement.execute(&mut environment, &mut buffer);
            }

            let output = vec!["true", ""];
            assert_eq!(utf8_to_string(&buffer), output)
        }
        #[test]
        fn if_then_else() {
            let code = "
                if (true) {
                print \"true\";
                } else {
                print \"false\"; 
                }
            ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                statement.execute(&mut environment, &mut buffer);
            }

            let output = vec!["true", ""];
            assert_eq!(utf8_to_string(&buffer), output)
        }

        #[test]
        fn if_then_else_alt() {
            let code = "
                if (false) {
                print \"true\";
                } else {
                print \"false\"; 
                }
            ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                statement.execute(&mut environment, &mut buffer);
            }

            let output = vec!["false", ""];
            assert_eq!(utf8_to_string(&buffer), output)
        }

        #[test]
        fn if_then_else_no_brackets() {
            let code = "
                if (true) 
                print \"true\";
                 else 
                print \"false\"; 
                
            ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                statement.execute(&mut environment, &mut buffer);
            }

            let output = vec!["true", ""];
            assert_eq!(utf8_to_string(&buffer), output)
        }

        #[test]
        fn nested() {
            let code = "
                if (true) 
                    if (false)
                        print \"true then false\";
                     else 
                        print \"true then true\";
                else
                    print \"unreachable\"; 
                
            ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                statement.execute(&mut environment, &mut buffer);
            }

            let output = vec!["true then true", ""];
            assert_eq!(utf8_to_string(&buffer), output)
        }

        #[test]
        fn while_loop() {
            let code = "
                  var i = 0;
                  while (i < 10) {
                    print i;
                    i = i + 1;
                  }
                
            ";
            let mut environment = Environment::new();

            let program = get_parser(code).parse();
            let mut buffer = Vec::<u8>::new();
            for statement in program {
                statement.execute(&mut environment, &mut buffer);
            }

            let output = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ""];
            assert_eq!(utf8_to_string(&buffer), output)
        }
    }
}
