pub mod lexer;
pub mod myerror;
pub mod parser;
pub mod token;
pub mod tree;

use std::{
    fs::File,
    io::{Read, Write},
};

use lexer::lexer::Lexer;
use myerror::myerror::*;
use parser::parser::*;
use tree::tree::*;

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
                position: 0,
            })
        }
    };
    let mut output_file = match File::create(input.output_file_name) {
        Ok(it) => it,
        Err(err) => {
            return Err(MyError {
                message: err.to_string(),
                position: 0,
            })
        }
    };

    let mut contents = String::new();
    match input_file.read_to_string(&mut contents) {
        Ok(it) => it,
        Err(err) => {
            return Err(MyError {
                message: err.to_string(),
                position: 0,
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
                        position: 0,
                    })
                }
            };
            match output_file.flush() {
                Ok(it) => it,
                Err(err) => {
                    return Err(MyError {
                        message: err.to_string(),
                        position: 0,
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
    let lexer = &mut Lexer::new(contents);
    let tree: Tree = expr(lexer);

    assembly.push_str(".intel_syntax noprefix\n");
    assembly.push_str(".globl main\n");
    assembly.push_str("main:\n");

    generate_assembly(&mut assembly, tree);

    assembly.push_str("\tpop rax\n");
    assembly.push_str("\tret\n");
    Ok(assembly)
}

fn generate_assembly(assembly: &mut String, tree: Tree) {
    if let Tree::Leaf(n) = tree {
        let str = format!("\tpush {}\n", n);
        assembly.push_str(&str);
        return;
    }

    if let Tree::Node(kind, lhs, rhs) = tree {
        generate_assembly(assembly, *lhs);
        generate_assembly(assembly, *rhs);

        assembly.push_str("\tpop rdi\n");
        assembly.push_str("\tpop rax\n");

        match kind {
            NodeKind::Equality => assembly.push_str("\tcmp rax, rdi\n\tsete al\n\tmovzb rax, al\n"),
            NodeKind::Nonequality => {
                assembly.push_str("\tcmp rax, rdi\n\tsetne al\n\tmovzb rax, al\n")
            }
            NodeKind::Less => assembly.push_str("\tcmp rax, rdi\n\tsetl al\n\tmovzb rax, al\n"),
            NodeKind::LessOrEqual => {
                assembly.push_str("\tcmp rax, rdi\n\tsetle al\n\tmovzb rax, al\n")
            }
            NodeKind::Add => assembly.push_str("\tadd rax, rdi\n"),
            NodeKind::Sub => assembly.push_str("\tsub rax, rdi\n"),
            NodeKind::Mul => assembly.push_str("\timul rax, rdi\n"),
            NodeKind::Div => assembly.push_str("\tcqo\n\tidiv rdi\n"),
        }
        assembly.push_str("\tpush rax\n");
    }
}
