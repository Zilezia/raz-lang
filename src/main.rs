use std::env::args;
use std::process::exit;

use raz::runner::*;

#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
fn main() {
    // might todo later the args here kinda got a nicer method of doing those
    let args: Vec<String> = args().collect();
    // if given more arguments than 2: `raz file1.raz file2.raz`
    // exit out // why thats kinda stupid
    if args.len() > 2 {
        println!("Usage:\n\traz [file]"); // amazing help
    }
    // other wise if its just 2: `raz file.rz` execute mention file
    else if args.len() == 2 {
        // make sure it's a .raz file
        if args[1].ends_with(".rz") || args[1].ends_with(".raz")  { // atm the file extension has no difference
            match run_file(&args[1]) {
                Ok(_) => exit(0),
                Err(msg) => eprintln!("ERROR:\n\t{msg}")
            }
        } else { eprintln!("Wrong file type disclosed: {}\nHas to be '.rz' or '.raz' file.", &args[1]) }
    }
    // use the interactive mode, similar to one as python
    else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                eprintln!("ERROR:\n\t{msg}");
            }
        }
    }
}

#[cfg(all(not(feature = "interpreter"), feature = "compiler"))]
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 || args.len() > 2 {
        println!("Usage:\n\traz [file]"); // amazing help
    } else if args.len() == 2 {
        if args[1].ends_with(".raz")  {
            let dir_path = "./out/";
            let file_name = "print";
            match run_compile(&args[1]) {
                Ok(_) => exit(0),
                Err(msg) => eprintln!("ERROR:\n\t{msg}")
            }
        } else {
            eprintln!("Wrong file type disclosed: {}\nHas to be '.rz' or '.raz' file.", &args[1])
        }
    }
}
#[cfg(any(
    all(feature = "interpreter", feature = "compiler"),
    all(not(feature = "interpreter"), not(feature = "compiler"))
))]
fn main() {
    eprintln!("Either both features are enabled or both are disabled.");
    eprintln!("Please have only one enabled (only \"interpreter\" is valid atm).");
}
