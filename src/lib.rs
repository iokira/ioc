pub mod lexer;
pub mod token;

use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

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
    assembly.push_str(".intel_syntax noprefix\n");
    assembly.push_str(".globl main\n");
    assembly.push_str("main:\n");
    assembly.push_str("\tmov rax, ");
    assembly.push_str(contents);
    assembly.push_str("\tret\n");
    assembly
}
