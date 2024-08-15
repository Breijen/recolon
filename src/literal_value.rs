use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;
use crate::scanner;
use crate::scanner::{Token, TokenType};
use crate::types::rcn_struct::{StructDefinition, StructInstance};

#[derive(Clone)]
pub enum LiteralValue {
    Array(Vec<LiteralValue>),
    Callable { name: String, arity: i32, fun: Rc<dyn Fn(Rc<RefCell<Environment>>, &Vec<LiteralValue>) -> LiteralValue> },
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
    StructDef(StructDefinition),
    StructInst(StructInstance),
}


impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LiteralValue::Number(x), LiteralValue::Number(y)) => x == y,
            (
                LiteralValue::Callable {
                    name,
                    arity,
                    fun: _,
                },
                LiteralValue::Callable {
                    name: name2,
                    arity: arity2,
                    fun: _,
                },
            ) => name == name2 && arity == arity2,
            (LiteralValue::StringValue(x), LiteralValue::StringValue(y)) => x == y,
            (LiteralValue::True, LiteralValue::True) => true,
            (LiteralValue::False, LiteralValue::False) => true,
            (LiteralValue::Nil, LiteralValue::Nil) => true,
            _ => false,
        }
    }
}

impl std::fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn unwrap_as_f32(literal: Option<scanner::LiteralValue>) -> f32 {
    match literal {
        Some(scanner::LiteralValue::IntValue(x)) => x as f32,
        Some(scanner::LiteralValue::FloatValue(x)) => x as f32,
        _ => panic!("Could not unwrap as f32"),
    }
}

fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        Some(scanner::LiteralValue::StringValue(s)) => s.clone(),
        Some(scanner::LiteralValue::IdentifierValue(s)) => s.clone(),
        _ => panic!("Could not unwrap as string"),
    }
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(x) => x.clone(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Callable { name, arity, fun: _ } => format!("{name}/{arity}"),
            LiteralValue::StructDef(struct_value) =>  {
                format!("{} {:?}", struct_value.name, struct_value.fields)
            },
            LiteralValue::StructInst(struct_value) => format!("{{ name: \"{}\", fields: {:?} }}", struct_value.name, struct_value.fields),
            LiteralValue::Array(elements) => format!("{elements:?}"),
            _ => todo!()
        }
    }

    pub fn to_type(&self) -> String {
        match self {
            LiteralValue::Number(_) => "Number".to_string(),
            LiteralValue::StringValue(_) => "String".to_string(),
            LiteralValue::True => "Bool".to_string(),
            LiteralValue::False => "Bool".to_string(),
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::StructDef(_) => "Struct".to_string(),
            _ => todo!()
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => LiteralValue::Number(unwrap_as_f32(token.literal)),
            TokenType::String => LiteralValue::StringValue(unwrap_as_string(token.literal)),
            TokenType::False => LiteralValue::False,
            TokenType::True => LiteralValue::True,
            TokenType::Nil => LiteralValue::Nil,
            _ => panic!("Could not create LiteralValue from {:?}", token)
        }
    }

    pub fn check_bool(b: bool) -> Self {
        if b {
            LiteralValue::True
        } else {
            LiteralValue::False
        }
    }

    pub fn is_falsy(&self) -> LiteralValue {
        match self {
            LiteralValue::Number(x) => {
                if *x == 0.0f32 {
                    LiteralValue::True
                } else {
                    LiteralValue::False
                }
            }
            LiteralValue::StringValue(s) => {
                if s.len() == 0 {
                    LiteralValue::True
                } else {
                    LiteralValue::False
                }
            }
            LiteralValue::True => LiteralValue::False,
            LiteralValue::False => LiteralValue::True,
            LiteralValue::Nil => LiteralValue::False,
            LiteralValue::Callable{ name: _, arity: _, fun: _ } => panic!("Can not use callable as falsy value"),
            _ => todo!()
        }
    }

    pub fn is_truthy(&self) -> LiteralValue {
        match self {
            LiteralValue::Number(x) => {
                if *x == 0.0f32 {
                    LiteralValue::False
                } else {
                    LiteralValue::True
                }
            }
            LiteralValue::StringValue(s) => {
                if s.len() == 0 {
                    LiteralValue::False
                } else {
                    LiteralValue::True
                }
            }
            LiteralValue::True => LiteralValue::True,
            LiteralValue::False => LiteralValue::False,
            LiteralValue::Nil => LiteralValue::False,
            LiteralValue::Callable{ name: _, arity: _, fun: _ } => panic!("Can not use callable as truthy value"),
            _ => todo!()
        }
    }

    pub fn call_method(&mut self, method_name: &str, args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
        match self {
            LiteralValue::Array(ref mut vec) => {
                match method_name {
                    "pop" => {
                        if args.len() == 0 {
                            // Remove and return the last element
                            vec.pop().ok_or_else(|| "Array is empty".to_string())
                        } else if args.len() == 1 {
                            // Remove and return the element at the specified index
                            if let LiteralValue::Number(idx) = args[0] {
                                let idx = idx as usize;
                                if idx < vec.len() {
                                    Ok(vec.remove(idx))
                                } else {
                                    Err("Index out of bounds".to_string())
                                }
                            } else {
                                Err("Index must be a number.".to_string())
                            }
                        } else {
                            Err("pop method takes 0 or 1 arguments".to_string())
                        }
                    }
                    "push" => {
                        if args.len() != 1 {
                            Err("push method takes exactly one argument.".to_string())
                        } else {
                            vec.push(args[0].clone());
                            Ok(LiteralValue::Nil) // You might return Nil or the array itself depending on your language's convention
                        }
                    }
                    "length" => {
                        if args.len() != 0 {
                            Err("length method takes no arguments.".to_string())
                        } else {
                            Ok(LiteralValue::Number(vec.len() as f32))
                        }
                    }
                    // Handle other array methods like push, etc.
                    _ => Err(format!("Unknown method '{}' for arrays", method_name)),
                }
            }
            // Handle method calls for other LiteralValue types if needed
            _ => Err(format!("'{}' method not available on this type", method_name)),
        }
    }
}
