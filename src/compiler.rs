
// this entire thing is similar to the interpreter.rs, but changed the `interpret` to `compile` :p

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

use std::path::Path;

use crate::stmt::Stmt;
use crate::scanner::Token;
use crate::literals::LiteralValue;
use crate::environment::Environment;
use crate::functions::func::clock_impl;

pub struct Compiler {
    specials: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

const ASM_DB: &str = "section .data";
const ASM_TEXT_SECTION: &str =
"section .text
    strsz:
        xor rcx, rcx
        not rcx
        xor al, al
        cld
        repne scasb
        not rcx
        dec rcx
        mov rax, rcx
        ret
    strprn:
        push rdi
        call strsz
        pop rsi
        mov rdx, rax
        mov rax, 1
        mov rdi, 1
        syscall
        ret
global _start
_start:";
const ASM_EXIT: &str =
"    xor rdi,rdi
    mov rax, 60
    mov rdi, 0
    syscall";

impl Compiler {
    pub fn new() -> Self {
        let mut natives = Environment::new();

        natives.define(
            "clock".to_string(),
            LiteralValue::Callable {
                name: "clock".to_string(),
                arity: 0,
                func: Rc::new(clock_impl)
            });

        Self {
            specials: Rc::new(RefCell::new(Environment::new())),
            environment: Rc::new(RefCell::new(natives))
        }
    }

    pub fn compile(&mut self, path: &str, stmts: Vec<&Stmt>) -> Result<(), String> {
        let mut data_section = ASM_DB.to_string();
        let mut text_section = ASM_TEXT_SECTION.to_string();
        let mut string_counter = 0;

        for stmt in stmts {
            let env = self.environment.clone();
            // only does like print statements I will do the rest tomorrow or later
            match stmt {
                Stmt::Print { expression } => {
                    let print_contents = expression.evaluate(env).unwrap();

                    let string_label = format!("s{string_counter}");
                    data_section.push_str(&format!("\n    {} db \"{}\", 10, 0", string_label, print_contents.to_string()));

                    text_section.push_str(&format!("\n    mov rdi, {string_label}\n    call strprn",));

                    string_counter += 1;
                },
                _ => todo!()
            }
        }

        let asm_str = format!("{}\n\n{}\n{}", data_section, text_section, ASM_EXIT);

        if !Path::new("./out").exists() {
            fs::create_dir("./out").expect("Failed to create out directory with binaries");
        }

        let file_name = path.rsplit_once("/")
            .unwrap()
            .1
            .trim_end_matches(".raz");
        let file_path = format!("./out/{file_name}.asm");
        fs::write(file_path, asm_str).unwrap();
        assembly("./out/", file_name);
        Ok(())
    }
}

fn assembly(dir: &str, file: &str) {
    let file_path = &format!("{dir}{file}");
    //let nasm = format!("nasm -felf64 {file_path}.asm");
    // let runasm = format!("nasm -felf64 {file_path}.asm && ld {file_path}.o -o {file_path} && {file_path}");
    // assembler
    let _ = std::process::Command::new("nasm")
        .arg("-felf64")
        .arg(format!("{file_path}.asm"))
        .output()
        .expect("Failed to run nasm assembly.");
    // linker
    let _ = std::process::Command::new("ld")
        .arg(&format!("{file_path}.o"))
        .arg("-o")
        .arg(file_path)
        .output()
        .expect("Failed to run ld linker.");
    // executable
    let output = std::process::Command::new(file_path)
        .output()
        .expect("Failed to run the executable");

    use std::io::Write;
    std::io::stdout()
        .write_all(&output.stdout)
        .expect("Failed to write out the executable stdout");
}


