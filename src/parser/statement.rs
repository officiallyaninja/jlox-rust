use crate::environment::Environment;

use super::expression::Expr;

#[derive(Debug)]
pub enum Stmt {
    //Class(Token , Box<Expr>.Variable superclass, Vec<Stmt.Function> methods)  ,
    // Block(Vec<Stmt>),
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
    pub fn execute(self, environment: &mut Environment) {
        match self {
            Stmt::Print(expr) => {
                let text = expr.evaluate(environment).to_string();
                environment.output.push(text.clone());
                println!("{}", text)
            }
            Stmt::Expression(expr) => {
                expr.evaluate(environment);
            }
            Stmt::Var(name, value) => {
                let value = value.evaluate(environment);
                environment.variables.insert(name, value);
            }
        };
    }
}
