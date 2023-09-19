use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

pub struct Input {
    inputfilename: String,
    outputfilename: String,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let inputfilename = args[1].clone();
        let outputfilename = args[2].clone();

        Ok(Input {
            inputfilename,
            outputfilename,
        })
    }
}

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let mut inputfile = File::open(input.inputfilename)?;
    let mut outputfile = File::create(input.outputfilename)?;

    let mut contents = String::new();
    inputfile.read_to_string(&mut contents)?;

    writeln!(outputfile, ".intel_syntax noprefix")?;
    writeln!(outputfile, ".globl main")?;
    writeln!(outputfile, "main:")?;
    writeln!(outputfile, "\tmov rax, {}", contents)?;
    writeln!(outputfile, "\tret")?;
    outputfile.flush()?;

    Ok(())
}
