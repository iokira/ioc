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

    writeln!(output_file, ".intel_syntax noprefix")?;
    writeln!(output_file, ".globl main")?;
    writeln!(output_file, "main:")?;
    writeln!(output_file, "\tmov rax, {}", contents)?;
    writeln!(output_file, "\tret")?;
    output_file.flush()?;

    Ok(())
}
