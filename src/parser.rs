mod expression;
mod statement;

use self::{expression::Expr, statement::Stmt};
use crate::{literal::Literal, scanner::Token};
use std::iter::Peekable;

//TODO: implement anyhow
pub struct Parser<T>
where
    T: Iterator<Item = Token>,
{
    tokens: Peekable<T>,
    line: u64,
    //last_processed_stmt - for errors errors or something
}

macro_rules! syntax_error {
    ($self: expr, $message: expr) => {
        panic!("[line {}] Error: {}", $self.line, $message)
    };
}
macro_rules! assert_next_token {
    ($self: expr, $expected: expr) => {
        match $self.tokens.next() {
            Some(found) if found == $expected => {}
            Some(found) if found != $expected => syntax_error!(
                $self,
                &format!(
                    "$expected '{}' ({}), found '{}' ({})",
                    $expected.lexeme(),
                    $expected.token_type(),
                    found.lexeme(),
                    found.token_type()
                )
            ),
            None => syntax_error!(
                $self,
                &format!(
                    "$expected '{}' ({}), found EOF",
                    $expected.lexeme(),
                    $expected.token_type(),
                )
            ),
            _ => unreachable!(),
        }
    };
}
impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
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
        assert_next_token!(self, Token::Var);

        let Some(Token::Identifier(name)) = self.tokens.next() else {
            panic!("Var should be followed by an identifier");
        };

        let initializer = if self.tokens.next_if_eq(&Token::Equal).is_some() {
            self.expression()
        } else {
            Literal::Nil.into()
        };

        assert_next_token!(self, Token::SemiColon);
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
            Token::For => self.for_statement(),
            Token::LeftBrace => self.block(),
            _ => self.expression_statement(),
        }
    }

    fn for_statement(&mut self) -> Stmt {
        assert_next_token!(self, Token::For);
        assert_next_token!(self, Token::LeftParen);
        let initializer = match self
            .tokens
            .peek()
            .expect("for statement must contain something after left paren")
        {
            Token::SemiColon => {
                assert_next_token!(self, Token::SemiColon);
                None
            }
            Token::Var => Some(self.declaration_statement()),
            _ => Some(self.expression_statement()),
        };
        let condition = if let &Token::SemiColon = self
            .tokens
            .peek()
            .expect("for statement must contain something after initializer")
        {
            assert_next_token!(self, Token::SemiColon);
            None
        } else {
            Some(self.expression_statement())
        };
        let increment = if let &Token::RightParen = self
            .tokens
            .peek()
            .expect("for statement must contain something after condition")
        {
            assert_next_token!(self, Token::RightParen);
            None
        } else {
            let inc = Some(self.expression());
            assert_next_token!(self, Token::RightParen);
            inc
        };

        let mut body = self.statement();

        if let Some(increment) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(increment)]);
        }
        if let Some(condition) = condition {
            let Stmt::Expression(condition) = condition else {
                unreachable!(
                    "impossible to reach as we create condition as an expression_statement"
                );
            };
            body = Stmt::While(condition, Box::new(body))
        }

        if let Some(initializer) = initializer {
            body = Stmt::Block(vec![initializer, body]);
        }
        body
    }
    fn while_statement(&mut self) -> Stmt {
        assert_next_token!(self, Token::While);
        assert_next_token!(self, Token::LeftParen);
        let condition = self.expression();
        assert_next_token!(self, Token::RightParen);

        let body = self.statement();
        Stmt::While(condition, body.into())
    }

    fn if_statement(&mut self) -> Stmt {
        assert_next_token!(self, Token::If);
        assert_next_token!(self, Token::LeftParen);
        let condition = self.expression();
        assert_next_token!(self, Token::RightParen);

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
        assert_next_token!(self, Token::LeftBrace);
        let mut statements = vec![];
        while self.tokens.peek().is_some() && self.tokens.peek() != Some(&Token::RightBrace) {
            statements.push(self.statement())
        }
        assert_next_token!(self, Token::RightBrace);
        Stmt::Block(statements)
    }

    fn print_statement(&mut self) -> Stmt {
        assert_next_token!(self, Token::Print);
        let stmt = Stmt::Print(self.expression());
        assert_next_token!(self, Token::SemiColon);
        stmt
    }

    fn expression_statement(&mut self) -> Stmt {
        let stmt = Stmt::Expression(self.expression());
        assert_next_token!(self, Token::SemiColon);
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
                syntax_error!(self, "invalid assigment target");
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
                assert_next_token!(self, Token::RightParen);
                return Expr::Grouping(expr.into());
            }
            Token::Identifier(name) => {
                return Expr::Variable(name);
            }
            invalid => syntax_error!(
                self,
                &format!("invalid primary token found {}", invalid.token_type())
            ),
        }
        .into()
    }
}

// TODO : Add tests for when you expect things to panic
// TODO : move to separate files
