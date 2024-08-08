use crate::environment::Environment;
use crate::expr::{Expr, LiteralValue};
use crate::stmt::Stmt;
use crate::expr::Expr::Literal;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression} => {
                    expression.evaluate(&self.environment)?;
                }
                Stmt::Log { expression } => {
                    let value = expression.evaluate(&self.environment)?;
                    println!("{value:?}");
                }
                Stmt::Err { expression } => {
                    let value = expression.evaluate(&self.environment)?;
                    eprintln!("{value:?}");
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(&self.environment)?;

                    self.environment.define(name.lexeme, value);
                }
            };
        }

        Ok(())
    }
}