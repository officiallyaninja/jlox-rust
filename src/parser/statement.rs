use crate::environment::Environment;

use super::expression::Expr;

#[derive(Debug)]
pub enum Stmt {
    //Class(Token , Box<Expr>.Variable superclass, Vec<Stmt.Function> methods)  ,
    Block(Vec<Stmt>),
    Expression(Expr),
    // Function definition
    // Function(Token, Vec<Token>, Vec<Stmt>),
    // If(Expr, Box<Stmt>, Box<Stmt>),
    Print(Expr),
    // Return(Token, Expr),
    Var(String, Expr),
    // While(Expr, Box<Stmt>),
}

impl Stmt {
    pub fn execute<W: std::io::Write>(self, environment: &mut Environment, output: &mut W) {
        match self {
            Stmt::Print(expr) => {
                let text = expr.evaluate(environment).to_string();
                output
                    .write(text.as_bytes())
                    .and_then(|_| output.write(b"\n"))
                    .expect("Write Error");
            }
            Stmt::Expression(expr) => {
                expr.evaluate(environment);
            }
            Stmt::Var(name, value) => {
                let value = value.evaluate(environment);
                environment.insert(name, value);
            }
            Stmt::Block(_) => {
                let env = Environment::with_parent(environment);
            }
        };
    }
}
