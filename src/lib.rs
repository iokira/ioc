pub mod generator;
pub mod lexer;
pub mod myerror;
pub mod parser;
pub mod token;
pub mod tree;

use std::{
    fs::File,
    io::{Read, Write},
};

use generator::generator::*;
use lexer::lexer::Lexer;
use myerror::myerror::*;
use parser::parser::*;
use tree::tree::*;

// 引数解析後に格納する構造体
pub struct Input {
    input_file_name: String,
    output_file_name: String,
}

// 引数解析器
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

// コンパイル処理
pub fn run(input: Input) -> Result<(), MyError> {
    // ソースコードを開く
    let mut input_file = match File::open(input.input_file_name) {
        Ok(it) => it,
        Err(err) => {
            return Err(MyError {
                message: err.to_string(),
                position: 0,
            })
        }
    };

    // 出力するアセンブリファイルの用意
    let mut output_file = match File::create(input.output_file_name) {
        Ok(it) => it,
        Err(err) => {
            return Err(MyError {
                message: err.to_string(),
                position: 0,
            })
        }
    };

    // ソースコードを文字列に格納
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

    // 完成したアセンブリをファイルに書き込む
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

// ソースコードからアセンブリを生成する
fn construct_assembly(contents: &str) -> Result<String, MyError> {
    let mut assembly = String::new();

    // 字句解析
    let lexer = &mut Lexer::new(contents);

    // 構文解析
    let tree: Tree = program(lexer);

    // intel syntaxの序文
    assembly.push_str(".intel_syntax noprefix\n");
    assembly.push_str(".globl main\n");
    assembly.push_str("main:\n");

    // 構文木をアセンブリに変換
    generate_assembly(&mut assembly, tree);

    // スタックの最後に残った値をraxに入れret
    assembly.push_str("\tpop rax\n");
    assembly.push_str("\tret\n");
    Ok(assembly)
}
