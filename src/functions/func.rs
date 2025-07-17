use crate::environment::Environment;
use crate::literals::LiteralValue;

use std::cell::RefCell;
use std::rc::Rc;

pub fn clock_impl(_env: Rc<RefCell<Environment>>, _args: &Vec<LiteralValue>) -> LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time.")
        .as_secs(); 
    // TODO give option to get time in milli, micro and nano
    LiteralValue::NumberValue(now as f64)
}