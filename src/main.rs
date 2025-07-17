// TODO comment sections to say what is going on

mod environment;
mod expr;
mod interpreter;
mod literals;
mod parser;
mod scanner;
mod stmt;
mod functions;
mod runner;

use crate::runner::*;

use std::env::args;
use std::process::exit;
use std::io::{
    self,
    Write,
    stdout,
};



// use raz::runner::*;

fn main() {
    // might todo later the args here kinda got a nicer method of doing those
    let args: Vec<String> = args().collect();
    // if given more arguments than 2: `raz file1.raz file2.raz`
    // exit out // why thats kinda stupid
    if args.len() > 2 {
        println!("Usage:\n\traz [file]"); // amazing help
    }
    // other wise if its just 2: `raz file.raz` execute mention file
    else if args.len() == 2 {
        // make sure it's a .raz file
        if args[1].ends_with(".raz") {
            match run_file(&args[1]) {
                Ok(_) => exit(0),
                Err(msg) => {
                    eprintln!("ERROR:\n{}", msg);
                }
            }
        } else { eprintln!("Wrong file type disclosed: {}\nHas to be '.raz' file.", &args[1]) }
    }
    // use the interactive mode, similar to one as python
    else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                eprintln!("ERROR:\n{}", msg);
            }
        }
    }
}
