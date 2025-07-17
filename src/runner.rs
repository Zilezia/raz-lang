
use std::fs;
use std::io::{self, Write};

use crate::parser::*;
use crate::scanner::*;
use crate::interpreter::*;

// realised that the interpreter is being passed because of mostly `run_prompt` which needs a sort of environment initialised before of it happening
// kinda might not have a cli version i just realised follow some tutorial they did it but now after going consiousness and having my own opinion i dont like the cli
fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    interpreter.interpret(stmts.iter().collect())?;
    return Ok(());
}

pub fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();

    match fs::read_to_string(path) {
        Ok(contents) => run(&mut interpreter, &contents),
        Err(msg) => Err(msg.to_string()),
    }
}

pub fn run_prompt() -> Result<(), String> {
    let mut interpreter = Interpreter::new();

    loop {
        print!("$ => ");
        let mut buffer = String::new();

        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Couldn't flush stdout".to_string())
        }

        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n <= 2 {
                    return Ok(());
                }
            },
            Err(_) => return Err("Couldn't read line".to_string())
        }

        match run(&mut interpreter, &buffer) {
            Ok(_) => (),
            Err(msg) => return Err(msg.to_string())
        }
    }
}
