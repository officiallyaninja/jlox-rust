use crate::{environment::Environment, literal::Literal, scanner::Token};

#[derive(Debug)]
pub enum Expr {
    Grouping(Box<Expr>),
    Literal(Literal),

    Unary {
        // Change to type Unary Operator
        op: Token,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        // Change to type Binary operator
        op: Token,
        right: Box<Expr>,
    },
    // This(Token),
    Variable(String),
    Assign {
        name: String,
        value: Box<Expr>,
    },
    // Call {
    //     callee: Box<Expr>,
    //     paren: Token,
    //     arguments: Vec<Box<Expr>>,
    // },
    // Get {
    //     object: Box<Expr>,
    //     name: Token,
    // },
    // Set {
    //     object: Box<Expr>,
    //     name: Token,
    //     value: Box<Expr>,
    // },
    // Super {
    //     keyword: Token,
    //     method: Token,
    // },
}

impl From<Literal> for Expr {
    fn from(value: Literal) -> Self {
        Expr::Literal(value)
    }
}

impl From<Literal> for Option<Expr> {
    fn from(value: Literal) -> Self {
        Some(Expr::Literal(value))
    }
}

impl Expr {
    pub fn to_string_normal(&self) -> String {
        match self {
            Expr::Grouping(expr) => format!("({})", expr.to_string_normal()),
            Expr::Literal(literal) => format!("{literal}"),
            Expr::Unary { op, expr } => format!("{}{}", op.lexeme(), expr.to_string_normal()),
            Expr::Binary { left, op, right } => format!(
                "{} {} {}",
                left.to_string_normal(),
                op.lexeme(),
                right.to_string_normal()
            ),
            Expr::Variable(name) => name.clone(),
            Expr::Assign { name, value } => format!("{name} = {}", value.to_string_normal()),
        }
    }
    pub fn pretty_string(&self) -> String {
        match self {
            Expr::Grouping(expr) => format!("(group {})", expr.pretty_string()),
            Expr::Literal(literal) => format!("{literal}"),
            Expr::Unary { op, expr } => format!("({} {})", op.lexeme(), expr.pretty_string()),
            Expr::Binary { left, op, right } => format!(
                "({} {} {})",
                op.lexeme(),
                left.pretty_string(),
                right.pretty_string()
            ),
            Expr::Variable(_) => todo!(),
            Expr::Assign { name, value } => todo!(),
        }
    }
    pub fn evaluate(self, environment: &mut Environment) -> Literal {
        match self {
            Expr::Grouping(expr) => expr.evaluate(environment),
            Expr::Literal(literal) => literal,
            Expr::Unary { op, expr } => match op {
                Token::Bang => (!expr.evaluate(environment).truthy()).into(),
                Token::Minus => {
                    let Literal::Number(num) = expr.evaluate(environment) else {
                        panic!("cannot take negative of non number");
                    };
                    (-num).into()
                }
                t => unreachable!(
                    "invalid unary operator, should be unreachable (it was {}: {})",
                    t.lexeme(),
                    t.token_type()
                ),
            },
            Expr::Binary { left, op, right } => {
                match (left.evaluate(environment), right.evaluate(environment)) {
                    (Literal::Number(left), Literal::Number(right)) => match op {
                        Token::Plus => (left + right).into(),
                        Token::Minus => (left - right).into(),
                        Token::Star => (left * right).into(),
                        Token::Slash => (left / right).into(),
                        // relational
                        Token::Less => (left < right).into(),
                        Token::LessEqual => (left <= right).into(),
                        Token::Greater => (left > right).into(),
                        Token::GreaterEqual => (left >= right).into(),
                        // Equality
                        Token::EqualEqual => (left == right).into(),
                        Token::BangEqual => (left != right).into(),
                        op => {
                            panic!("invalid operation {} on numbers", op.token_type())
                        }
                    },
                    (Literal::String(left), Literal::String(right)) => match op {
                        Token::Plus => format!("{left}{right}").into(),
                        Token::EqualEqual => (left == right).into(),
                        Token::BangEqual => (left != right).into(),
                        op => {
                            panic!("invalid operation {} on strings", op.token_type())
                        }
                    },
                    (left, right) => match op {
                        Token::EqualEqual => left == right,
                        Token::BangEqual => left != right,
                        op => {
                            panic!(
                                "invalid operation {} on {} and {}",
                                left.to_string(),
                                op.token_type(),
                                right.to_string()
                            )
                        }
                    }
                    .into(),
                }
            }
            // TODO: turn into a runtime error
            Expr::Variable(name) => environment
                .get(&name)
                .expect("Variable not defined")
                .clone(),
            Expr::Assign { name, value } => {
                let value = value.evaluate(environment);
                let Some(_) = environment.insert(name.clone(), value.clone()) else {
                    panic!("undefined variable \"{}\"", name);
                };
                value
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::{Expr::*, Literal};

    #[test]
    fn basic() {
        let expr_text = "(5 + 2) * -6 == 9";
        let expr = Binary {
            left: Binary {
                left: Grouping(
                    Binary {
                        left: Literal(Literal::Number(5.0)).into(),
                        op: crate::scanner::Token::Plus,
                        right: Literal(Literal::Number(2.0)).into(),
                    }
                    .into(),
                )
                .into(),
                op: crate::scanner::Token::Star,
                right: Unary {
                    op: crate::scanner::Token::Minus,
                    expr: Literal(Literal::Number(6.0)).into(),
                }
                .into(),
            }
            .into(),
            op: crate::scanner::Token::EqualEqual,
            right: Literal(Literal::Number(9.0)).into(),
        };

        assert_eq!(expr.to_string_normal(), expr_text);
    }
}
