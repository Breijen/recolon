use std::rc::Rc;
use colored::Colorize;

use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::expr::LiteralValue;

pub struct Interpreter {
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(Environment::new()),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression} => {
                    expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get a mutable reference to environment"),
                    )?;
                }
                Stmt::Log { expression } => {
                    let value = expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get a mutable reference to environment"),
                    )?;
                    println!("{} \"{}\"", "LOG".bright_blue(), value.to_string());
                }
                Stmt::Err { expression } => {
                    let value = expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get a mutable reference to environment"),
                    )?;
                    println!("{} \"{}\"", "ERR!".red(), value.to_string());
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get a mutable reference to environment"),
                    )?;

                    Rc::get_mut(&mut self.environment)
                        .expect("Could not get a mutable reference to environment")
                        .define(name.lexeme, value)
                }
                Stmt::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(new_environment);
                    let block_result = self.interpret(statements);
                    self.environment = old_environment;

                    block_result?;
                }
                Stmt::IfStmt { predicate, then, elifs, els } => {
                    let truth_value = predicate.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get a mutable reference to environment"))?;

                    if truth_value.is_truthy() == LiteralValue::True {
                        self.interpret(vec![*then])?;
                    } else {
                        let mut executed = false;

                        // Check elif conditions
                        for (elif_predicate, elif_body) in elifs {
                            let elif_truth_value = elif_predicate.evaluate(
                                Rc::get_mut(&mut self.environment)
                                    .expect("Could not get a mutable reference to environment"))?;
                            if elif_truth_value.is_truthy() == LiteralValue::True {
                                self.interpret(vec![*elif_body.clone()])?; // Interpret the elif block
                                executed = true;
                                break;
                            }
                        }

                        // If no elif was executed, check else
                        if !executed {
                            if let Some(els_stmt) = els {
                                self.interpret(vec![*els_stmt])?;
                            }
                        }
                    }
                }
                Stmt::WhileStmt { condition, body } => {
                    while {
                        let flag = condition.evaluate(
                            Rc::get_mut(&mut self.environment)
                                .expect("Could not get a mutable reference to environment"),
                        )?;
                        flag.is_truthy() == LiteralValue::True
                    } {
                        self.interpret(vec![(*body).clone()])?; // Dereference the Box to clone the Stmt
                    }
                }
            };
        }

        Ok(())
    }
}