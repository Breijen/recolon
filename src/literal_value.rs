use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;
use crate::literal_value::LiteralValue::{Callable, Number, StringValue, StructDef, StructInst};
use crate::scanner;
use crate::scanner::{Token, TokenType};
use crate::types::rcn_struct::{StructDefinition, StructInstance};

#[derive(Clone)]
pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
    Callable { name: String, arity: i32, fun: Rc<dyn Fn(Rc<RefCell<Environment>>, &Vec<LiteralValue>) -> LiteralValue> },
    StructDef(StructDefinition),
    StructInst(StructInstance),
}


impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number(x), Number(y)) => x == y,
            (
                Callable {
                    name,
                    arity,
                    fun: _,
                },
                Callable {
                    name: name2,
                    arity: arity2,
                    fun: _,
                },
            ) => name == name2 && arity == arity2,
            (StringValue(x), StringValue(y)) => x == y,
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
            LiteralValue::StructDef(struct_value) => format!("{:?}", struct_value),
            LiteralValue::StructInst(struct_value) => format!("{{ name: \"{}\", fields: {:?} }}", struct_value.name, struct_value.fields),
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
            TokenType::Number => Number(unwrap_as_f32(token.literal)),
            TokenType::String => StringValue(unwrap_as_string(token.literal)),
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
            Number(x) => {
                if *x == 0.0f32 {
                    LiteralValue::True
                } else {
                    LiteralValue::False
                }
            }
            StringValue(s) => {
                if s.len() == 0 {
                    LiteralValue::True
                } else {
                    LiteralValue::False
                }
            }
            LiteralValue::True => LiteralValue::False,
            LiteralValue::False => LiteralValue::True,
            LiteralValue::Nil => LiteralValue::False,
            Callable{ name: _, arity: _, fun: _ } => panic!("Can not use callable as falsy value"),
            _ => todo!()
        }
    }

    pub fn is_truthy(&self) -> LiteralValue {
        match self {
            Number(x) => {
                if *x == 0.0f32 {
                    LiteralValue::False
                } else {
                    LiteralValue::True
                }
            }
            StringValue(s) => {
                if s.len() == 0 {
                    LiteralValue::False
                } else {
                    LiteralValue::True
                }
            }
            LiteralValue::True => LiteralValue::True,
            LiteralValue::False => LiteralValue::False,
            LiteralValue::Nil => LiteralValue::False,
            Callable{ name: _, arity: _, fun: _ } => panic!("Can not use callable as truthy value"),
            _ => todo!()
        }
    }
}