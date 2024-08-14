use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use crate::scanner::{Token, TokenType};
use crate::scanner;
use crate::environment::Environment;

use LiteralValue::*;
use crate::literal_value::LiteralValue;
use crate::modules::{rcn_math};
use crate::types::rcn_struct::{StructDefinition, StructInstance};

#[derive(Clone)]
pub enum Expr {
    Assign { name: Token, value: Box<Expr>, },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Call { callee: Box<Expr>, paren: Token, arguments: Vec<Expr>,  },
    FieldAccess { object: Box<Expr>, field: Token },
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Logical { left: Box<Expr>, operator: Token, right: Box<Expr> },
    PreFunction { module: String, name: String, args: Vec<Expr> },
    Unary { operator: Token, right: Box<Expr> },
    Variable { name: Token, },
    StructInst {
        name: String,
        fields: HashMap<String, Expr>, // Field names and their values (expressions)
    },
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Assign {
                name,
                value
            } => format!("({name:?} = {}", value.to_string()),
            Expr::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expr::Call { callee, paren: _, arguments } => format!("({} {:?}", (*callee).to_string(), arguments),
            Expr::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
            }
            Expr::Variable { name } => format!("(var {})", name.lexeme),
            Expr::Logical { left, operator, right } => format!("({} {} {})", operator.to_string(), left.to_string(), right.to_string()),
            _ => todo!()
        }
    }

    pub fn evaluate(&self, environment: &RefCell<Environment>) -> Result<LiteralValue, String> {
        match self {
            Expr::Assign { name, value } => {
                let new_value = value.evaluate(environment)?; // Evaluate the assigned value

                // Check if the value is a struct, and if so, create a new instance
                let new_value = match new_value {
                    StructInst(ref struct_obj) => {
                        // Create a new struct instance with the same fields
                        let mut new_fields = HashMap::new();
                        for (field_name, field_value) in &struct_obj.fields {
                            new_fields.insert(field_name.clone(), field_value.clone());
                        }

                        LiteralValue::StructInst(StructInstance {
                            name: struct_obj.name.clone(),
                            fields: new_fields,
                        })
                    }
                    _ => new_value,
                };

                // Assign the new value to the variable in the environment
                let assign_success = environment.borrow_mut().assign(&name.lexeme, new_value.clone());

                if assign_success {
                    Ok(new_value)
                } else {
                    Err(format!("Variable {} has not been declared.", name.lexeme))
                }
            },
            Expr::FieldAccess { object, field } => {
                let struct_instance = match object.evaluate(environment)? {
                    LiteralValue::StructInst(instance) => instance,
                    _ => return Err("Expected a struct instance in field access".to_string()),
                };

                match struct_instance.get_field(&field.lexeme) {
                    Some(value) => Ok(value.clone()),
                    None => Err(format!("Field '{}' not found in struct '{}'", field.lexeme, struct_instance.name)),
                }
            },
            Expr::Variable { name } => match environment.borrow_mut().get(&name.lexeme) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Undefined variable '{}'.", name.lexeme.to_string())),
            },
            Expr::Logical {
                left,
                operator,
                right,
            } => match operator.token_type {
                TokenType::Or => {
                    let lhs_true = left.evaluate(environment)?.is_truthy();
                    let rhs_true = right.evaluate(environment)?.is_truthy();
                    if lhs_true == True {
                        Ok(True)
                    } else {
                        if rhs_true == True {
                            Ok(True)
                        } else {
                            Ok(False)
                        }
                    }
                }
                TokenType::And => {
                    let lhs_true = left.evaluate(environment)?.is_truthy();
                    let rhs_true = right.evaluate(environment)?.is_truthy();
                    if lhs_true == False {
                        Ok(False)
                    } else {
                        if rhs_true == True {
                            Ok(True)
                        } else {
                            Ok(False)
                        }
                    }
                }
                t_type => Err(format!("Invalid token in logical expression: {}", t_type))
            },
            Expr::Literal { value } => Ok((*value).clone()),
            Expr::Grouping { expression } => expression.evaluate(environment),
            Expr::Unary { operator, right } => {
                let right = right.evaluate(environment)?;

                match (&right, operator.token_type) {
                    (Number(x), TokenType::Minus) => Ok(Number(-x)),
                    (_, TokenType::Minus) => Err(format!("Cannot use - for {:?}", right.to_type())),

                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (_, t_type) => Err(format!("{} is not a valid operator.", t_type.to_string()))
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(environment)?;
                let right = right.evaluate(environment)?;

                match (&left, operator.token_type, &right) {

                    //PLUS
                    (Number(x), TokenType::Plus, Number(y)) => Ok(Number(x + y)),
                    (StringValue(s1), TokenType::Plus, StringValue(s2)) => { Ok(StringValue(format!("{}{}", s1, s2))) }
                    (StringValue(s1), TokenType::Plus, Number(x)) => Ok(StringValue(format!("{}{}", s1, x.to_string()))),
                    (Number(x), TokenType::Plus, StringValue(s1)) => Ok(StringValue(format!("{}{}", x.to_string(), s1))),

                    (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),
                    (StringValue(_s1), TokenType::Minus, StringValue(_s2)) => Err("NaN".to_string()),
                    (StringValue(_s1), TokenType::Minus, Number(_x)) => Err("NaN".to_string()),
                    (Number(_x), TokenType::Minus, StringValue(_s1)) => Err("NaN".to_string()),

                    (Number(x), TokenType::Slash, Number(y)) => Ok(Number(x / y)),
                    (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),

                    (Number(x), TokenType::Greater, Number(y)) => Ok(LiteralValue::check_bool(x > y)),
                    (StringValue(s1), TokenType::Greater, StringValue(s2)) => Ok(LiteralValue::check_bool(s1 > s2)),
                    (Number(x), TokenType::GreaterEqual, Number(y)) => Ok(LiteralValue::check_bool(x >= y)),
                    (StringValue(s1), TokenType::GreaterEqual, StringValue(s2)) => Ok(LiteralValue::check_bool(s1 >= s2)),

                    (Number(x), TokenType::Less, Number(y)) => Ok(LiteralValue::check_bool(x < y)),
                    (StringValue(s1), TokenType::Less, StringValue(s2)) => Ok(LiteralValue::check_bool(s1 < s2)),
                    (Number(x), TokenType::LessEqual, Number(y)) => Ok(LiteralValue::check_bool(x <= y)),
                    (StringValue(s1), TokenType::LessEqual, StringValue(s2)) => Ok(LiteralValue::check_bool(s1 <= s2)),

                    (x, TokenType::BangEqual, y) => Ok(LiteralValue::check_bool(x != y)),
                    (x, TokenType::EqualEqual, y) => Ok(LiteralValue::check_bool(x == y)),
                    (_x, t_type, _y) => Err(format!("{} has not been implemented", t_type.to_string()))
                }
            }
            Expr::PreFunction { module, name, args } => {
                let function = name;

                // Evaluate arguments
                let evaluated_args: Result<Vec<_>, _> = args.iter().map(|arg| arg.evaluate(environment)).collect();
                let evaluated_args = evaluated_args?;

                // Handle the "math" module functions
                if module == "math" {
                    match function.as_str() {
                        "floor" => rcn_math::floor(evaluated_args),
                        "ceil" => rcn_math::ceil(evaluated_args),
                        "round" => rcn_math::round(evaluated_args),
                        "sqrt" => rcn_math::sqrt(evaluated_args),
                        "abs" => rcn_math::abs(evaluated_args),
                        "max" => rcn_math::max(evaluated_args),
                        "min" => rcn_math::min(evaluated_args),
                        "random" => rcn_math::random(evaluated_args),
                        "pow" => rcn_math::pow(evaluated_args),
                        "lgm" => rcn_math::lgm(evaluated_args),
                        "cos" => rcn_math::cos(evaluated_args),
                        "sin" => rcn_math::sin(evaluated_args),
                        "tan" => rcn_math::tan(evaluated_args),
                        "degrees" => rcn_math::tan(evaluated_args),
                        "radians" => rcn_math::tan(evaluated_args),
                        // Add more math functions here
                        _ => Err(format!("Function '{}.{}' not implemented.", module, function)),
                    }
                } else {
                    Err(format!("Module '{}' not found.", module))
                }
            }
            Expr::Call { callee, paren: _, arguments} => {
                let callable = callee.evaluate(environment)?;
                match callable {
                    Callable { name, arity, fun } => {
                        if arguments.len() != arity.try_into().unwrap() {
                            return Err(format!("Callable {} expected {} arguments but got {}", name, arity, arguments.len()));
                        }
                        let mut arg_vals = vec![];
                        for arg in arguments {
                            let val = arg.evaluate(&mut environment.clone())?;
                            arg_vals.push(val);
                        }

                        Ok(fun(Rc::new(environment.clone()), &arg_vals ))
                    }
                    _ => Err(format!("'{}' is not callable", callee.to_string())),
                }
            }
            Expr::StructInst { name, fields } => {
                // Retrieve the struct definition
                let struct_def = match environment.borrow().get(name) {
                    Some(StructDef(def)) => def.clone(),
                    _ => return Err(format!("Struct definition '{}' not found", name)),
                };

                // Create a new struct instance with evaluated fields
                let mut evaluated_fields = HashMap::new();

                for (field_name, expr) in fields {
                    // Ensure the field exists in the struct definition
                    if let Some(expected_expr) = struct_def.fields.get(field_name) {
                        let value = expr.evaluate(environment)?;

                        // Optionally: Check if the type of the evaluated value matches the expected type.
                        // This assumes that the expected type can be derived from the definition. You might need to add logic here.
                        let expected_value = expected_expr.evaluate(environment)?;

                        if value.to_type() != expected_value.to_type() {
                            return Err(format!(
                                "Type mismatch for field '{}': expected {:?}, got {:?}",
                                field_name,
                                expected_value.to_type(),
                                value.to_type()
                            ));
                        }

                        evaluated_fields.insert(field_name.clone(), value);
                    } else {
                        return Err(format!(
                            "Field '{}' does not exist in struct definition '{}'",
                            field_name, struct_def.name
                        ));
                    }
                }

                // Ensure all fields in the definition are accounted for
                for field_name in struct_def.fields.keys() {
                    if !evaluated_fields.contains_key(field_name) {
                        return Err(format!(
                            "Missing field '{}' in struct instantiation '{}'",
                            field_name, struct_def.name
                        ));
                    }
                }

                Ok(StructInst(StructInstance {
                    name: struct_def.name.clone(),
                    fields: evaluated_fields,
                }))
            }

            _ => todo!()
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::TokenType;

    #[test]
    fn print_ast() {
        let minus_token = Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 0,
        };

        let new_number = Expr::Literal {
            value: LiteralValue::Number(123.0),
        };

        let group = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        };

        let multi_token = Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line_number: 0,
        };

        let ast = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: minus_token,
                right: Box::new(new_number),
            }),
            operator: multi_token,
            right: Box::new(group),
        };

        // Print the abstract syntax tree.
        let result = ast.to_string();
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}