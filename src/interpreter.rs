use std::cell::RefCell;
use std::rc::Rc;
use colored::Colorize;

use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::expr::LiteralValue;

pub struct Interpreter {
    //globals: Environment,
    environment: Rc<RefCell<Environment>>,
}

fn clock_impl(_env: Rc<RefCell<Environment>>, _args: &Vec<LiteralValue>) -> LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();

    LiteralValue::Number(now as f32 / 1000.0)
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();
        globals.define( "clock".to_string(), LiteralValue::Callable {
            name: "clock".to_string(),
            arity: 0,
            fun: Rc::new(|_env, _args| clock_impl(_env, _args)),
        },);
        Self {
            //globals: Environment::new(),
            environment: Rc::new(RefCell::from(globals)),
        }
    }

    fn for_closure(parent: Rc<RefCell<Environment>>) -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));
        environment.borrow_mut().enclosing = Some(parent);

        Self {
            environment
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
                        .borrow_mut().define(name.lexeme, value)
                }
                Stmt::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(RefCell::from(new_environment));
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
                Stmt::LoopStmt { body } => {
                    loop {
                        self.interpret(vec![(*body).clone()])?; // Dereference the Box to clone the Stmt
                    }
                }
                Stmt::ReturnStmt { value, .. } => {
                    let return_value = match value {
                        Some(expr) => expr.evaluate(
                            Rc::get_mut(&mut self.environment)
                                .expect("Could not get a mutable reference to environment"),
                        )?,
                        None => LiteralValue::Nil, // Default return value if none specified
                    };
                }
                Stmt::FuncStmt { name, parameters, body } => {
                    let arity = parameters.len() as i32;

                    let params = parameters.clone();
                    let body = body.clone();

                    let fun_impl = move |parent_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>| {
                        let mut closure_int = Interpreter::for_closure(parent_env);

                        for (i, arg) in args.iter().enumerate() {
                            // println!("Defining parameter {}: {:?}", params[i].lexeme, arg);
                            closure_int.environment.borrow_mut().define(params[i].lexeme.clone(), arg.clone());
                        }

                        for stmt in &body {
                            // println!("Executing function body statement: {:?}", stmt);
                            if let Err(e) = closure_int.interpret(vec![*stmt.clone()]) {
                                // eprintln!("Error executing statement in function body: {}", e);
                                return LiteralValue::Nil;
                            }
                        }

                        let last_value = match &*body[body.len() - 1] {
                            Stmt::Expression { expression } => {
                                expression.evaluate(&closure_int.environment).unwrap()
                            }
                            _ => LiteralValue::Nil,
                        };

                        last_value
                    };

                    let callable = LiteralValue::Callable {
                        name: name.clone(),
                        arity,
                        fun: Rc::new(fun_impl),
                    };

                    // println!("Assigning function {} to environment", name);

                    self.environment.borrow_mut().define(name.clone(), callable);

                    // println!("Function {} defined successfully", name);
                }
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Expr, LiteralValue};
    use crate::stmt::Stmt;
    use crate::scanner::Token;
    use crate::scanner::TokenType;

    #[test]
    fn test_function_declaration_and_call() {
        let mut interpreter = Interpreter::new();

        // Define a function
        let func_stmt = Stmt::FuncStmt {
            name: "add".to_string(),
            parameters: vec![
                Token {
                    token_type: TokenType::Identifier,
                    lexeme: "a".to_string(),
                    line_number: 1,
                    literal: None,
                },
                Token {
                    token_type: TokenType::Identifier,
                    lexeme: "b".to_string(),
                    line_number: 1,
                    literal: None,
                },
            ],
            body: vec![
                Box::new(Stmt::ReturnStmt {
                    value: Some(*Box::new(Expr::Binary {
                        left: Box::new(Expr::Variable {
                            name: Token {
                                token_type: TokenType::Identifier,
                                lexeme: "a".to_string(),
                                line_number: 1,
                                literal: None,
                            },
                        }),
                        operator: Token {
                            token_type: TokenType::Plus,
                            lexeme: "+".to_string(),
                            line_number: 1,
                            literal: None,
                        },
                        right: Box::new(Expr::Variable {
                            name: Token {
                                token_type: TokenType::Identifier,
                                lexeme: "b".to_string(),
                                line_number: 1,
                                literal: None,
                            },
                        }),
                    })),
                    keyword: Token {
                        token_type: TokenType::Return,
                        lexeme: "return".to_string(),
                        line_number: 1,
                        literal: None,
                    },
                }),
            ],
        };

        // Add function to environment
        let result = interpreter.interpret(vec![func_stmt]);
        assert!(result.is_ok());

        // Call the function
        let call_expr = Expr::Call {
            callee: Box::new(Expr::Variable {
                name: Token {
                    token_type: TokenType::Identifier,
                    lexeme: "add".to_string(),
                    line_number: 1,
                    literal: None,
                },
            }),
            paren: Token {
                token_type: TokenType::LeftParen,
                lexeme: "(".to_string(),
                line_number: 1,
                literal: None,
            },
            arguments: vec![
                Expr::Literal {
                    value: LiteralValue::Number(5.0),
                },
                Expr::Literal {
                    value: LiteralValue::Number(3.0),
                },
            ],
        };

        let call_result = call_expr.evaluate(&interpreter.environment).unwrap();
        assert_eq!(call_result, LiteralValue::Number(8.0));
    }

    #[test]
    fn test_invalid_function_call_error() {
        let mut interpreter = Interpreter::new();

        // Call a non-existent function
        let call_expr = Expr::Call {
            callee: Box::new(Expr::Variable {
                name: Token {
                    token_type: TokenType::Identifier,
                    lexeme: "nonexistent_function".to_string(),
                    line_number: 1,
                    literal: None,
                },
            }),
            paren: Token {
                token_type: TokenType::LeftParen,
                lexeme: "(".to_string(),
                line_number: 1,
                literal: None,
            },
            arguments: vec![],
        };

        let result = call_expr.evaluate(&interpreter.environment);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "'nonexistent_function' is not callable");
    }
}