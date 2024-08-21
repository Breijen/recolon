use std::cell::RefCell;
use std::rc::Rc;
use colored::Colorize;

use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::literal_value::LiteralValue;
use crate::modules::{rcn_std};
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::types::rcn_struct::StructDefinition;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

pub enum ControlFlow {
    Continue,
    Return(LiteralValue),
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();

        Self::define_std(&mut globals);

        Self {
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

    fn define_std(globals: &mut Environment) {
        globals.define("clock".to_string(), LiteralValue::Callable {
            name: "clock".to_string(),
            arity: 0,
            fun: Rc::new(|_env, _args| rcn_std::clock_impl(_env, _args)),
        }, true);
        globals.define("wait_ms".to_string(), LiteralValue::Callable {
            name: "wait_ms".to_string(),
            arity: 1,
            fun: Rc::new(|_env, _args| rcn_std::wait_ms(_env, _args)),
        }, true);
        globals.define("color_console".to_string(), LiteralValue::Callable {
            name: "color_console".to_string(),
            arity: 3,
            fun: Rc::new(|_env, _args| rcn_std::color_console(_env, _args)),
        }, true);
    }

    fn load_module(&self, module_name: String) -> Result<String, String> {
        let stripped_module_name = module_name.trim_matches('"');
        let module_path = format!("{}.rcn", stripped_module_name);
        std::fs::read_to_string(module_path).map_err(|e| format!("Failed to load module '{}': {}", module_name, e))
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<ControlFlow, String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression} => {
                    let value = expression.evaluate(&self.environment)?;
                    // You can do something with `value` here if needed
                }
                Stmt::Log { expression } => {
                    let value = expression.evaluate(&self.environment)?;
                    println!("{} \"{}\"", "LOG".bright_blue(), value.to_string());
                }
                Stmt::Err { expression } => {
                    let value = expression.evaluate(&self.environment)?;
                    println!("{} \"{}\"", "ERR!".red(), value.to_string());
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate(&self.environment)?;
                    println!("{}", value.to_string());
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(&self.environment)?;
                    self.environment.borrow_mut().define(name.lexeme, value, false);
                }
                Stmt::Const { name, initializer } => {
                    let value = initializer.evaluate(&self.environment)?;

                    if self.environment.borrow().get(&name.lexeme).is_some() {
                        return Err(format!("Constant '{}' is already defined.", name.lexeme));
                    }

                    self.environment.borrow_mut().define(name.lexeme, value, true);
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
                    let truth_value = predicate.evaluate(&self.environment)?;

                    if truth_value.is_truthy() == LiteralValue::True {
                        self.interpret(vec![*then])?;
                    } else {
                        let mut executed = false;

                        // Check elif conditions
                        for (elif_predicate, elif_body) in elifs {
                            let elif_truth_value = elif_predicate.evaluate(&self.environment)?;
                            if elif_truth_value.is_truthy() == LiteralValue::True {
                                self.interpret(vec![*elif_body.clone()])?;
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
                    while condition.evaluate(&self.environment)?.is_truthy() == LiteralValue::True {
                        self.interpret(vec![(*body).clone()])?;
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

                    let defining_env = self.environment.clone();  // Capture the environment where the function is defined

                    let fun_impl = move |call_env, args: &Vec<LiteralValue>| {
                        let mut closure_int = Interpreter::for_closure(defining_env.clone());

                        for (i, arg) in args.iter().enumerate() {
                            // println!("Defining parameter {}: {:?}", params[i].lexeme, arg);
                            closure_int.environment.borrow_mut().define(params[i].lexeme.clone(), (*arg).clone(), false);
                        }

                        // Execute the function body
                        for stmt in body.iter() {
                            match closure_int.interpret(vec![*stmt.clone()]) {
                                Ok(ControlFlow::Return(return_value)) => return return_value,
                                Ok(ControlFlow::Continue) => continue,
                                Err(e) => {
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

                    self.environment.borrow_mut().define(name.clone(), callable, false);

                    // println!("Function {} defined successfully", name);
                }
                Stmt::StructStmt { name, params } => {
                    let struct_def = LiteralValue::StructDef(StructDefinition {
                        name: name.clone(),
                        fields: params.clone(),
                    });

                    self.environment.borrow_mut().define(name, struct_def, false);
                }
                Stmt::Import { module_name, alias_name } => {
                    // Load the module code from the file system
                    let module_code = self.load_module(module_name)?;

                    let mut scanner = Scanner::new(module_code.as_str());
                    let tokens = scanner.scan_tokens()?;

                    let mut parser = Parser::new(tokens);
                    let module_statements = parser.parse()?;

                    // Create a new environment for the module
                    let module_environment = Rc::new(RefCell::new(Environment::new_with_enclosing(self.environment.clone())));

                    // Create an interpreter for the module using the new environment
                    let mut module_interpreter = Interpreter {
                        environment: module_environment.clone(),
                    };

                    // Interpret each statement in the module within its environment
                    module_interpreter.interpret(module_statements)?;

                    // println!("Created module environment: {:?}", &module_environment);
                    // Store the module's environment under the alias in the current environment
                    self.environment.borrow_mut().define(alias_name.clone(), LiteralValue::Namespace(module_environment), false);
                }
                _ => todo!()
            };

        }


        Ok(ControlFlow::Continue)
    }

}
