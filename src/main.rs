// TODO comment sections to say what is going on

mod environment;
mod expr;
mod interpreter;
mod literals;
mod parser;
mod scanner;
mod stmt;
mod functions;
use crate::scanner::*;
use crate::parser::*;
use crate::interpreter::*;

use std::env;
use std::fs;
use std::io::Write;
use std::process::exit;
use std::io;

fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();    
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&mut interpreter, &contents),
    }
}

fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    interpreter.interpret(stmts.iter().collect())?;
    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    loop {
        print!("$ => ");
        let mut buffer = String::new();
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Couldn't flush stdout".to_string())
        }
        let stdin = io::stdin();
        match stdin.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 2 {
                    return Ok(());
                }
            },
            Err(_) => return Err("Couldn't read line".to_string())
        }
        // println!("ECHO: {}", buffer); // debug
        match run(&mut interpreter, &buffer) {
            Ok(_) => (),
            Err(msg) => return Err(msg.to_string()),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // if given more arguments than 2: `raz file1.raz file2.raz
    // exit out
    if args.len() > 2 {
        println!("Usage raz [script]");
        exit(64);
    } 
    // other wise if its just 2: `raz file.raz` execute mention file
    else if args.len() == 2 {
        // make sure it's a .raz file
        if args[1].ends_with(".raz") {
            match run_file(&args[1]) {
                Ok(_) => exit(0),
                Err(msg) => {
                    println!("ERROR:\n{}", msg);
                    exit(1);
                }
            }
        } else { println!("Wrong file type disclosed: {}\nHas to be '.raz' file.", &args[1])}
    } 
    // use the interactive mode, similar to one as python
    else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
        }
    }
}