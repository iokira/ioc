pub mod lexer;
pub mod myerror;
pub mod token;

use std::{
    fs::File,
    io::{Read, Write},
};

use lexer::lexer::Lexer;
use myerror::myerror::*;
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

pub fn run(input: Input) -> Result<(), MyError> {
    let mut input_file = match File::open(input.input_file_name) {
        Ok(it) => it,
        Err(err) => {
            return Err(MyError {
                message: err.to_string(),
            })
        }
    };
    let mut output_file = match File::create(input.output_file_name) {
        Ok(it) => it,
        Err(err) => {
            return Err(MyError {
                message: err.to_string(),
            })
        }
    };

    let mut contents = String::new();
    match input_file.read_to_string(&mut contents) {
        Ok(it) => it,
        Err(err) => {
            return Err(MyError {
                message: err.to_string(),
            })
        }
    };

    match construct_assembly(&contents) {
        Ok(a) => {
            let assembly = a;
            match write!(output_file, "{}", assembly) {
                Ok(it) => it,
                Err(err) => {
                    return Err(MyError {
                        message: err.to_string(),
                    })
                }
            };
            match output_file.flush() {
                Ok(it) => it,
                Err(err) => {
                    return Err(MyError {
                        message: err.to_string(),
                    })
                }
            };
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

fn construct_assembly(contents: &str) -> Result<String, MyError> {
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
                    _ => {
                        return Err(MyError {
                            message: "unimplemented".to_string(),
                        })
                    }
                },
                _ => {
                    return Err(MyError {
                        message: "syntax error".to_string(),
                    })
                }
            },
            Ok(Token::EOF) => break,
            _ => {
                return Err(MyError {
                    message: "syntax error".to_string(),
                })
            }
        }
    }
    assembly.push_str("\tret\n");
    Ok(assembly)
}
