
use std::fs;
use std::io::{self, Write};

use crate::parser::*;
use crate::scanner::*;

#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
use crate::interpreter::*;
#[cfg(all(feature = "compiler", not(feature = "interpreter")))]
use crate::compiler::*;
// use raz::{
//     parser::*,
//     scanner::*,
//     interpreter::*
// };

// this does the interpreter way how do i compile way this?
#[cfg(all(feature = "compiler", not(feature = "interpreter")))]
pub fn run_compile(path: &str) -> Result<(), String>{
    let mut compiler = Compiler::new(); // the compiler environment

    match fs::read_to_string(path) {
        Ok(contents) => runc(&mut compiler, path, &contents),
        Err(err) => Err(err.to_string())
    }
}

#[cfg(all(feature = "compiler", not(feature = "interpreter")))]
fn runc(compiler: &mut Compiler, path: &str, contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    compiler.compile(path, stmts.iter().collect())?; // so i guess this will be similar to as the Interpreter `interpret` but in there I just write to an .asm file?

    Ok(())
}

// realised that the interpreter is being passed because of mostly `run_prompt` which needs a sort of environment initialised before of it happening
// kinda might not have a cli version i just realised follow some tutorial they did it but now after going consiousness and having my own opinion i dont like the cli
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    interpreter.interpret(stmts.iter().collect())?;

    Ok(())
}

// this simple
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();

    match fs::read_to_string(path) {
        Ok(contents) => run(&mut interpreter, &contents),
        Err(err) => Err(err.to_string()),
    }
}

// this might be just changed to some cli thingy maybe local raz package manager and other stuff
// i dont like the interpreter being there like that
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
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
