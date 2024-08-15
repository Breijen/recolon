use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;
use crate::literal_value::LiteralValue;

pub(crate) fn clock_impl(_env: Rc<RefCell<Environment>>, _args: &Vec<LiteralValue>) -> LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();

    LiteralValue::Number(now as f32 / 1000.0)
}