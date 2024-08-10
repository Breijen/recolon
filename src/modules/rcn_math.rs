use crate::expr::{Expr, LiteralValue};

pub fn check_type(identifier: String) -> Result<Expr, String>{
    match identifier.as_str() {
        "pi" => Ok(Expr::Literal {
            value: LiteralValue::Number(get_pi()), // Call the function to get PI
        }),
        "e" => Ok(Expr::Literal {
            value: LiteralValue::Number(get_e()), // Call the function to get PI
        }),
        "tau" => Ok(Expr::Literal {
            value: LiteralValue::Number(get_tau()), // Call the function to get PI
        }),
        "nan" => Ok(Expr::Literal {
            value: LiteralValue::Nil, // Call the function to get PI
        }),
        _ => Err(format!("Unknown identifier '{}'.", identifier)),
    }
}

pub fn get_pi() -> f32 {
    std::f32::consts::PI
}

pub fn get_e() -> f32 {
    std::f32::consts::E
}

pub fn get_tau() -> f32 {
    std::f32::consts::TAU
}

pub(crate) fn get_floor() -> Expr {
    todo!()
}