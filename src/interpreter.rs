use std::cell::RefCell;
use std::rc::Rc;
use colored::Colorize;

use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::expr::{LiteralValue, StructDefinition};

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

pub enum ControlFlow {
    Continue,
    Return(LiteralValue),
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
            globals: Rc::new(RefCell::from(Environment::new())),
            environment: Rc::new(RefCell::from(globals)),
        }
    }

    fn for_closure(parent: Rc<RefCell<Environment>>) -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));
        environment.borrow_mut().enclosing = Some(parent);

        Self {
            globals: Rc::new(RefCell::from(Environment::new())),
            environment
        }
    }
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(ControlFlow), String> {
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
                    // Create a new environment for the block
                    let old_env = self.environment.clone();
                    self.environment = Rc::new(RefCell::new(Environment::new()));
                    self.environment.borrow_mut().enclosing = Some(old_env.clone());

                    // Interpret the block
                    let block_result = self.interpret(statements.clone());
                    self.environment = old_env; // Restore the old environment

                    if let Ok(ControlFlow::Return(value)) = block_result {
                        return Ok(ControlFlow::Return(value));
                    }
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
                Stmt::ReturnStmt { keyword: _, value } => {

                    let eval_val = if let Some(expr) = value {
                        expr.evaluate(&self.environment.clone())?
                    } else {
                        LiteralValue::Nil
                    };

                    return Ok(ControlFlow::Return(eval_val));
                }
                Stmt::FuncStmt { name, parameters, body } => {
                    let arity = parameters.len() as i32;

                    let params = parameters.clone();
                    let body = body.clone();

                    let fun_impl = move |parent_env, args: &Vec<LiteralValue>| {
                        let mut closure_int = Interpreter::for_closure(parent_env);

                        for (i, arg) in args.iter().enumerate() {
                            // println!("Defining parameter {}: {:?}", params[i].lexeme, arg);
                            closure_int.environment.borrow_mut().define(params[i].lexeme.clone(), (*arg).clone());
                        }

                        for stmt in body.iter() {
                            match closure_int.interpret(vec![*stmt.clone()]) {
                                Ok(ControlFlow::Return(return_value)) => {
                                    // If a return statement is encountered, return the value
                                    return return_value;
                                }
                                Ok(ControlFlow::Continue) => {
                                    // Continue execution if no return statement is encountered
                                    continue;
                                }
                                Err(e) => {
                                    // Handle any interpretation errors
                                    eprintln!("Error executing statement: {:?}", e);
                                    return LiteralValue::Nil;
                                }
                            }
                        }

                        LiteralValue::Nil
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
                Stmt::StructStmt { name, params } => {
                    let struct_def = LiteralValue::StructDef(StructDefinition {
                        name: name.clone(),
                        fields: params.clone(),
                    });

                    self.environment.borrow_mut().define(name, struct_def);
                }
            };
        }
        Ok(ControlFlow::Continue)
    }
}
