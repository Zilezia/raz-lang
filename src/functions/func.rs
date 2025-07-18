
use std::rc::Rc;
use std::cell::RefCell;
use std::time::SystemTime;

use crate::digit::*;
use crate::literals::LiteralValue;
use crate::environment::Environment;

// use raz::{
//     digit::*,
//     literals::LiteralValue,
//     environment::Environment,
// };

pub fn clock_impl(_env: Rc<RefCell<Environment>>, _args: &Vec<LiteralValue>) -> LiteralValue {
    let now = SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time.")
        .as_secs_f64();
    // TODO give option to get time in milli, micro and nano
    LiteralValue::NumberValue(DigitType::f64(now))
}
