use crate::expr::Expr;
use crate::scanner::Token;

#[derive(Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Log { expression: Expr },
    Err { expression: Expr },
    Var { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt>},
    IfStmt {
        predicate: Expr,
        then: Box<Stmt>,
        els: Option<Box<Stmt>>,
    },
    WhileStmt {
        condition: Expr,
        body: Box<Stmt>,
    }
}

impl Stmt {
    pub fn to_string(&self) -> String {
        use Stmt::*;
        match self {
            Expression { expression } => expression.to_string(),
            Log { expression } => format!("(log {})", expression.to_string()),
            Err { expression } => format!("(err {})", expression.to_string()),
            Var { name, initializer } => format!("(var {})", name.lexeme),
            Block { statements } => format!(
                "(block {}",
                statements.into_iter().map(|stmt| stmt.to_string())
                    .collect::<String>()
            ),
            _ => todo!(),
        }
    }
}

