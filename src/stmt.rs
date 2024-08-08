use crate::expr::Expr;
use crate::scanner::Token;

pub enum Stmt {
    Expression { expression: Expr },
    Log { expression: Expr },
    Err { expression: Expr },
    Var { name: Token, initializer: Expr },
}



