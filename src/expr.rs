use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use crate::scanner::{Token, TokenType};
use crate::environment::Environment;

use LiteralValue::*;
use crate::literal_value::LiteralValue;
use crate::error::RecolonError;
use crate::types::r#struct::StructInstance;

#[derive(Clone)]
pub enum Expr {
    Array { elements: Vec<Expr> },
    Assign { name: Token, value: Box<Expr>, },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Call { callee: Box<Expr>, paren: Token, arguments: Vec<Expr>,  }, // Function calls
    Dictionary { pairs: Vec<(Expr, Expr)> }, // Dictionary literal
    FieldAccess { object: Box<Expr>, field: Token }, // Access to fields in struct instance
    FieldAssign { object: Box<Expr>, field: Token, value: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Index { array: Box<Expr>, index: Box<Expr> }, // Array indexing
    IndexAssign { object: Box<Expr>, index: Box<Expr>, value: Box<Expr> }, // Dictionary/Array assignment
    Literal { value: LiteralValue },
    Logical { left: Box<Expr>, operator: Token, right: Box<Expr> },
    MethodCall { object: Box<Expr>, method_name: String, arguments: Vec<Expr> },
    PreFunction { module: String, name: String, args: Vec<Expr> }, // Pre-built functions
    StructInst {
        name: String,
        fields: HashMap<String, Expr>,
    }, // Struct Instance
    Unary { operator: Token, right: Box<Expr> },
    Variable { name: Token, },
    Const { name: String, value: Box<Expr> },
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Array { elements} => format!("({elements:?}"),
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
            Expr::Dictionary { pairs } => {
                let pairs_str: Vec<String> = pairs.iter()
                    .map(|(k, v)| format!("{}: {}", k.to_string(), v.to_string()))
                    .collect();
                format!("(dict {{{}}})", pairs_str.join(", "))
            },
            Expr::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expr::IndexAssign { object, index, value } => {
                format!("({}[{}] = {})", object.to_string(), index.to_string(), value.to_string())
            },
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
            }
            Expr::Variable { name } => format!("(var {})", name.lexeme),
            Expr::Const { name, value: _ } => format!("(const {})", name),
            Expr::Logical { left, operator, right } => format!("({} {} {})", operator.to_string(), left.to_string(), right.to_string()),
            _ => todo!()
        }
    }

    pub fn evaluate(&self, environment: &RefCell<Environment>) -> Result<LiteralValue, String> {
        match self {
            Expr::Array { elements } => {
                let mut evaluated_elements = Vec::new();
                for element in elements {
                    evaluated_elements.push(element.evaluate(environment)?);
                }

                Ok(Array(evaluated_elements))

            },
            Expr::Dictionary { pairs } => {
                let mut evaluated_map = HashMap::new();
                for (key_expr, value_expr) in pairs {
                    let key = key_expr.evaluate(environment)?;
                    let value = value_expr.evaluate(environment)?;
                    
                    // Keys must be strings
                    if let LiteralValue::StringValue(key_str) = key {
                        evaluated_map.insert(key_str, value);
                    } else {
                        return Err("Dictionary keys must be strings".to_string());
                    }
                }

                Ok(LiteralValue::Dictionary(evaluated_map))
            },
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

                // Check if the variable is a constant
                if environment.borrow().constants.contains_key(&name.lexeme) {
                    return Err(format!("Cannot reassign to constant '{}'.", name.lexeme));
                }

                // Assign the new value to the variable in the environment
                let assign_success = environment.borrow_mut().assign(&name.lexeme, new_value.clone());

                if assign_success {
                    Ok(new_value)
                } else {
                    Err(format!("Variable {} has not been declared.", name.lexeme))
                }
            },
            Expr::FieldAccess { object, field } => {
                let object_value = object.evaluate(environment)?;

                match object_value {
                    StructInst(struct_instance) => {
                        if let Some(value) = struct_instance.get_field(&field.lexeme) {
                            Ok(value.clone())
                        } else {
                            Err(RecolonError::runtime(
                                format!("Field '{}' not found in struct '{}'", field.lexeme, struct_instance.name),
                                field.line_number
                            ).with_suggestion("Check if the field exists in the struct definition".to_string()).to_string())
                        }
                    }
                    Namespace(namespace_env) => {
                        // Check if the field is a variable or a function in the namespace
                        if let Some(value) = namespace_env.borrow().get(&field.lexeme) {
                            match value {
                                Callable { .. } => Ok(value.clone()), // Function call
                                _ => Ok(value.clone()), // Variable
                            }
                        } else {
                            Err(format!("Variable or function '{}' not found in namespace.", field.lexeme))
                        }
                    }

                    LiteralValue::Dictionary(_) => {
                        // For dictionaries, treat field access as a method call with no arguments
                        let method_name = field.lexeme.clone();
                        let mut dict_value = object_value.clone();
                        match dict_value.call_method(&method_name, vec![]) {
                            Ok(result) => Ok(result),
                            Err(e) => Err(e)
                        }
                    }
                    _ =>  {
                        Err(format!("Expected a struct or namespace for field access, but got '{}'.", object_value.to_type()))
                    }
                }
            },
            Expr::FieldAssign { object, field, value } => {
                let mut struct_instance_value = object.evaluate(environment)?;

                let evaluated_value = value.evaluate(environment)?;

                match struct_instance_value.update_struct_field(field.lexeme.clone(), evaluated_value.clone()) {
                    Ok(_) => {
                        if let Expr::Variable { name } = &**object {
                            environment.borrow_mut().assign(&name.lexeme, struct_instance_value.clone());
                        }
                        Ok(struct_instance_value)
                    },
                    Err(e) => Err(e)
                }
            }
            Expr::Variable { name } => {
                match environment.borrow().get(&name.lexeme) {
                    Some(value) => {
                        match value {
                            StructInst(_) => {
                                // Handle as a struct instance
                                Ok(value.clone())
                            },
                            _ => {
                                // Handle as a regular variable or other type
                                Ok(value.clone())
                            }
                        }
                    },
                    None => {
                        Err(RecolonError::runtime(
                            format!("Undefined variable '{}'", name.lexeme),
                            name.line_number
                        ).with_suggestion("Check if the variable is declared before use".to_string()).to_string())
                    },
                }
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
                t_type => {
                    Err(format!("Invalid token in logical expression: {}", t_type))
                }
            },
            Expr::Literal { value } => Ok((*value).clone()),
            Expr::Grouping { expression } => expression.evaluate(environment),
            Expr::Unary { operator, right } => {
                let right = right.evaluate(environment)?;

                match (&right, operator.token_type) {
                    (Number(x), TokenType::Minus) => Ok(Number(-x)),
                    (_, TokenType::Minus) => {
                        Err(RecolonError::type_error(
                            "Invalid unary operation".to_string(),
                            1, // TODO: Get actual line number
                            "Number".to_string(),
                            right.to_type()
                        ).with_suggestion("The minus operator can only be used with numbers".to_string()).to_string())
                    },

                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (_, t_type) => {
                        Err(format!("{} is not a valid operator.", t_type.to_string()))
                    }
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
                    (StringValue(s1), TokenType::Plus, LiteralValue::Dictionary(_)) => Ok(StringValue(format!("{}{}", s1, right.to_string()))),
                    (LiteralValue::Dictionary(_), TokenType::Plus, StringValue(s2)) => Ok(StringValue(format!("{}{}", left.to_string(), s2))),

                    (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),
                    (StringValue(_s1), TokenType::Minus, StringValue(_s2)) => Err("NaN".to_string()),
                    (StringValue(_s1), TokenType::Minus, Number(_x)) => Err("NaN".to_string()),
                    (Number(_x), TokenType::Minus, StringValue(_s1)) => Err("NaN".to_string()),

                    (Number(x), TokenType::Slash, Number(y)) => {
                        if *y == 0.0 {
                            Err(RecolonError::math("Division by zero".to_string(), 1, "division".to_string()).to_string())
                        } else {
                            Ok(Number(x / y))
                        }
                    },
                    (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),
                    (Number(x), TokenType::Percent, Number(y)) => {
                        if *y == 0.0 {
                            Err(RecolonError::math("Modulo by zero".to_string(), 1, "modulo".to_string()).to_string())
                        } else {
                            Ok(Number(x % y))
                        }
                    },

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
                    (_x, t_type, _y) => {
                        match t_type {
                            TokenType::Plus => {
                                Err(format!("Cannot concatenate {} with {} using +. Consider checking for nil values.", left.to_type(), right.to_type()))
                            }
                            _ => {
                                Err(format!("{} has not been implemented", t_type.to_string()))
                            }
                        }
                    }
                }
            }
            Expr::PreFunction { module, name, args } => {
                // Evaluate arguments
                let evaluated_args: Result<Vec<_>, _> = args.iter().map(|arg| arg.evaluate(environment)).collect();
                let evaluated_args = evaluated_args?;

                // Look up the namespace (module) in the environment
                let namespace = environment.borrow().get(module).ok_or_else(|| {
                    format!("Module '{}' not found.", module)
                })?;

                // Get the function from the namespace
                match namespace {
                    LiteralValue::Namespace(namespace_env) => {
                        let function = namespace_env.borrow().get(name).ok_or_else(|| {
                            format!("Function '{}.{}' not found.", module, name)
                        })?;

                        // Call the function
                        match function {
                            LiteralValue::Callable { fun, arity, .. } => {
                                if evaluated_args.len() != arity as usize {
                                    return Err(format!("Function '{}.{}' expects {} arguments, got {}", 
                                        module, name, arity, evaluated_args.len()));
                                }
                                Ok(fun(environment.clone().into(), &evaluated_args))
                            }
                            _ => Err(format!("'{}.{}' is not a function.", module, name))
                        }
                    }
                    _ => Err(format!("'{}' is not a module.", module))
                }
            }
            Expr::Call { callee, paren: _, arguments} => {
                let callable = callee.evaluate(environment)?;
                match callable {
                    Callable { name, arity, fun } => {
                        if arguments.len() != arity.try_into().unwrap() {
                            // Remove duplicate print statement
                            return Err(format!("Callable {} expected {} arguments but got {}", name, arity, arguments.len()));
                        }

                        let mut arg_vals = vec![];
                        for arg in arguments {
                            let val = arg.evaluate(environment)?;
                            arg_vals.push(val);
                        }

                        let result = fun(Rc::from(environment.clone()), &arg_vals);
                        Ok(result)
                    }
                    _ => {
                        // Remove duplicate print statement
                        Err(format!("'{}' is not callable", callee.to_string()))
                    },
                }
            }
            Expr::MethodCall { object, method_name, arguments } => {
                let mut obj_value = object.evaluate(environment)?;
                let evaluated_args: Vec<LiteralValue> = arguments.iter()
                    .map(|arg| arg.evaluate(environment))
                    .collect::<Result<Vec<_>, _>>()?;

                // Namespace: look up the function and call it directly
                if let Namespace(ref namespace_env) = obj_value {
                    let func_opt = namespace_env.borrow().get(&method_name);
                    if let Some(LiteralValue::Callable { arity, fun, .. }) = func_opt {
                        if evaluated_args.len() as i32 != arity {
                            return Err(format!("Function '{}' expected {} arguments but got {}", method_name, arity, evaluated_args.len()));
                        }
                        return Ok(fun(Rc::from(environment.clone()), &evaluated_args));
                    }
                    return Err(format!("Function '{}' not found in namespace", method_name));
                }

                // StructInst: look up field as callable
                if let StructInst(ref struct_instance) = obj_value {
                    let field_opt = struct_instance.get_field(&method_name).cloned();
                    if let Some(LiteralValue::Callable { arity, fun, .. }) = field_opt {
                        if evaluated_args.len() as i32 != arity {
                            return Err(format!("Method '{}' expected {} arguments but got {}", method_name, arity, evaluated_args.len()));
                        }
                        return Ok(fun(Rc::from(environment.clone()), &evaluated_args));
                    }
                    return Err(format!("Method '{}' not found on struct '{}'", method_name, struct_instance.name));
                }

                // Arrays, Dictionaries, and other value types
                let result = obj_value.call_method(&method_name, evaluated_args)?;
                if let Expr::Variable { name } = &**object {
                    environment.borrow_mut().assign(&name.lexeme, obj_value.clone());
                }
                Ok(result)
            }
            Expr::StructInst { name, fields } => {
                // Retrieve the struct definition
                let struct_def = match environment.borrow().get(name) {
                    Some(LiteralValue::StructDef(def)) => def.clone(),
                    _ => {
                        return Err(format!("Struct definition '{}' not found", name));
                    },
                };

                // Create a new struct instance with evaluated fields
                let mut evaluated_fields = HashMap::new();

                // Evaluate provided fields and check against the struct definition
                for (field_name, expr) in fields {
                    // Ensure the field exists in the struct definition
                    if let Some(expected_expr) = struct_def.fields.get(field_name) {
                        let value = expr.evaluate(environment)?;

                        // Optionally: Check if the type of the evaluated value matches the expected type.
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
                for (field_name, default_expr) in struct_def.fields.iter() {
                    // If the field wasn't provided during instantiation, use the default value
                    if !evaluated_fields.contains_key(field_name) {
                        let default_value = default_expr.evaluate(environment)?;
                        evaluated_fields.insert(field_name.clone(), default_value);
                    }
                }

                Ok(LiteralValue::StructInst(StructInstance {
                    name: struct_def.name.clone(),
                    fields: evaluated_fields,
                }))
            }
            Expr::Index { array, index } => {
                let array_value = array.evaluate(environment)?;
                let index_value = index.evaluate(environment)?;

                match array_value {
                    Array(arr) => {
                        if let Number(idx) = index_value {
                            let idx = idx as usize;
                            if idx < arr.len() {
                                Ok(arr[idx].clone())
                            } else {
                                Err("Array index out of bounds".to_string())
                            }
                        } else {
                            Err("Array index must be a number".to_string())
                        }
                    }
                    LiteralValue::Dictionary(dict) => {
                        if let LiteralValue::StringValue(key) = index_value {
                            Ok(dict.get(&key).cloned().unwrap_or(LiteralValue::Nil))
                        } else {
                            Err("Dictionary key must be a string".to_string())
                        }
                    }
                    _ => Err("Attempt to index a non-indexable value".to_string())
                }
            }
            Expr::IndexAssign { object, index, value } => {
                let mut object_value = object.evaluate(environment)?;
                let index_value = index.evaluate(environment)?;
                let new_value = value.evaluate(environment)?;

                match object_value {
                    Array(ref mut arr) => {
                        if let Number(idx) = index_value {
                            let idx = idx as usize;
                            if idx < arr.len() {
                                arr[idx] = new_value.clone();
                                if let Expr::Variable { name } = &**object {
                                    environment.borrow_mut().assign(&name.lexeme, object_value.clone());
                                }
                                Ok(new_value)
                            } else {
                                Err("Array index out of bounds".to_string())
                            }
                        } else {
                            Err("Array index must be a number".to_string())
                        }
                    }
                    LiteralValue::Dictionary(ref mut dict) => {
                        if let LiteralValue::StringValue(key) = index_value {
                            dict.insert(key, new_value.clone());
                            if let Expr::Variable { name } = &**object {
                                environment.borrow_mut().assign(&name.lexeme, object_value.clone());
                            }
                            Ok(new_value)
                        } else {
                            Err("Dictionary key must be a string".to_string())
                        }
                    }
                    _ => Err("Index assignment only supported for arrays and dictionaries".to_string())
                }
            }
            Expr::Const { name, value } => {
                let evaluated_value = value.evaluate(environment)?;

                // Attempt to assign this value as a constant in the environment
                if environment.borrow().get(name).is_none() {
                    environment.borrow_mut().define(name.clone(), evaluated_value.clone(), true);
                    Ok(evaluated_value)
                } else {
                    Err(format!("Constant '{}' is already defined.", name))
                }
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