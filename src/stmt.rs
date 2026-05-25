use std::collections::HashMap;
use crate::expr::{Expr};
use crate::scanner::Token;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression { expression: Expr },
    Log { expression: Expr },
    Err { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
    Const { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt>},
    IfStmt {
        predicate: Expr,
        then: Box<Stmt>,
        elifs: Vec<(Expr, Box<Stmt>)>,
        els: Option<Box<Stmt>>,
    },
    Import {
        module_name: String,
        alias_name: String
    },
    WhileStmt {
        condition: Expr,
        body: Box<Stmt>,
    },
    ReturnStmt {
        keyword: Token,
        value: Option<Expr>
    },
    LoopStmt {
        body: Box<Stmt>
    },
    FuncStmt {
        name: String,
        parameters: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    StructStmt {
        name: String,
        params: HashMap<String, Expr>
    }
}

impl Stmt {
    pub fn to_string(&self) -> String {
        use Stmt::*;
        match self {
            Expression { expression } => expression.to_string(),
            Log { expression } => format!("(log {})", expression.to_string()),
            Err { expression } => format!("(err {})", expression.to_string()),
            Print { expression } => format!("(log {})", expression.to_string()),
            Var { name, initializer: _ } => format!("(var {})", name.lexeme),
            Block { statements } => format!(
                "(block {}",
                statements.into_iter().map(|stmt| stmt.to_string())
                    .collect::<String>()
            ),
            ReturnStmt { keyword: _, value } => {
                let value_str = match value {
                    Some(expr) => expr.to_string(),
                    None => "None".to_string(),
                };
                format!("(return {})", value_str)
            }
            Const { name, initializer: _ } => format!("(const {})", name.lexeme),
            IfStmt { predicate, then, elifs, els } => {
                let mut s = format!("(if {} {})", predicate.to_string(), then.to_string());
                for (cond, body) in elifs {
                    s.push_str(&format!(" (elif {} {})", cond.to_string(), body.to_string()));
                }
                if let Some(else_body) = els {
                    s.push_str(&format!(" (else {})", else_body.to_string()));
                }
                s
            }
            WhileStmt { condition, body } => format!("(while {} {})", condition.to_string(), body.to_string()),
            LoopStmt { body } => format!("(loop {})", body.to_string()),
            FuncStmt { name, parameters, body: _ } => {
                let params: Vec<String> = parameters.iter().map(|p| p.lexeme.clone()).collect();
                format!("(fn {}({}))", name, params.join(", "))
            }
            StructStmt { name, params } => {
                let fields: Vec<String> = params.keys().cloned().collect();
                format!("(struct {} {{ {} }})", name, fields.join(", "))
            }
            Import { module_name, alias_name } => format!("(import {} as {})", module_name, alias_name),
        }
    }
}

