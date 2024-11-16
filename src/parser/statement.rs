use super::expression::Expr;
use crate::environment::Environment;

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
    pub fn execute<W: std::io::Write>(self, mut env: Environment, output: &mut W) -> Environment {
        match self {
            Stmt::Print(expr) => {
                let text = expr.evaluate(&mut env).to_string();
                output
                    .write(text.as_bytes())
                    .and_then(|_| output.write(b"\n"))
                    .expect("Write Error");
            }
            Stmt::Expression(expr) => {
                expr.evaluate(&mut env);
            }
            Stmt::Var(name, value) => {
                let value = value.evaluate(&mut env);
                // idk if we need to do anything on redefinition
                &mut env.insert(name, value);
            }
            Stmt::Block(statements) => {
                let mut env = Environment::with_parent(env);
                for statement in statements {
                    env = statement.execute(env, output);
                }
                return env
                    .parent()
                    .expect("we gave it a parent, so we know it must have one");
            }
        };
        env
    }
}
