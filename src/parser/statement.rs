use super::expression::Expr;
use crate::environment::Environment;

#[derive(Debug)]
pub enum Stmt {
    //Class(Token , Box<Expr>.Variable superclass, Vec<Stmt.Function> methods)  ,
    Block(Vec<Stmt>),
    Expression(Expr),
    // Function definition
    // Function(Token, Vec<Token>, Vec<Stmt>),
    If {
        condition: Expr,
        then_stmt: Box<Stmt>,
        else_stmt: Option<Box<Stmt>>,
    },
    Print(Expr),
    // Return(Token, Expr),
    Var(String, Expr),
    While(Expr, Box<Stmt>),
}

impl Stmt {
    pub fn execute<W: std::io::Write>(self, env: &mut Environment<'_>, output: &mut W) {
        match self {
            Stmt::Print(expr) => {
                let text = expr.evaluate(env).to_string();
                output
                    .write(text.as_bytes())
                    .and_then(|_| output.write(b"\n"))
                    .expect("Write Error");
            }
            Stmt::Expression(expr) => {
                expr.evaluate(env);
            }
            Stmt::Var(name, value) => {
                let value = value.evaluate(env);
                // idk if we need to do anything on redefinition
                env.insert(name, value);
            }
            Stmt::Block(statements) => {
                let mut env = Environment::with_parent(env);
                for statement in statements {
                    statement.execute(&mut env, output);
                }
            }
            Stmt::If {
                condition,
                then_stmt,
                else_stmt,
            } => match (condition.evaluate(env).truthy(), else_stmt) {
                (true, _) => then_stmt.execute(env, output),
                (false, Some(else_stmt)) => else_stmt.execute(env, output),
                (false, None) => {}
            },
            Stmt::While(condition, body) => {
                while condition.evaluate(env).truthy() {
                    //body.execute(env, output)
                }
            }
        };
    }
}
