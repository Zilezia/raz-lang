

fn clock_impl(_env: Rc<RefCell<Environment>>, _args: &Vec<LiteralValue>) -> LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time.")
        .as_secs(); // output maybe as mili or nano? // TODO give option to output as those
    LiteralValue::NumberValue(now as f64)
}