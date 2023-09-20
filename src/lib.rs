pub mod lexer;
pub mod token;

use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use lexer::lexer::Lexer;
use token::token::*;

pub struct Input {
    input_file_name: String,
    output_file_name: String,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let input_file_name = args[1].clone();
        let output_file_name = args[2].clone();

        Ok(Input {
            input_file_name,
            output_file_name,
        })
    }
}

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open(input.input_file_name)?;
    let mut output_file = File::create(input.output_file_name)?;

    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let assembly = construct_assembly(&contents);

    write!(output_file, "{}", assembly)?;
    output_file.flush()?;

    Ok(())
}

fn construct_assembly(contents: &str) -> String {
    let mut assembly = String::new();
    let mut lexer = Lexer::new(contents);
    assembly.push_str(".intel_syntax noprefix\n");
    assembly.push_str(".globl main\n");
    assembly.push_str("main:\n");
    loop {
        match lexer.next_token() {
            Ok(Token::Operand(n)) => {
                let str = format!("\tmov rax, {}\n", n);
                assembly.push_str(&str);
            }
            Ok(Token::Operator(o)) => match lexer.next_token() {
                Ok(Token::Operand(n)) => match o {
                    Operator::Add => {
                        let str = format!("\tadd rax, {}\n", n);
                        assembly.push_str(&str);
                    }
                    Operator::Sub => {
                        let str = format!("\tsub rax, {}\n", n);
                        assembly.push_str(&str);
                    }
                    _ => panic!("unimplemented"),
                },
                _ => panic!("syntax error"),
            },
            _ => break,
        }
    }
    assembly.push_str("\tret\n");
    assembly
}
