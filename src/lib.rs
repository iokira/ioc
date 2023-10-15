pub mod architecture;
pub mod error;
pub mod generator;
pub mod lexer;
pub mod numtype;
pub mod parser;
pub mod token;
pub mod tree;

use std::{
    fs::File,
    io::{Read, Write},
};

use architecture::myarchitecture::*;
use error::myerror::*;
use generator::mygenerator::*;
use lexer::mylexer::Lexer;
use parser::myparser::*;

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
    let (trees, lexer) = program(lexer);
    let ident_count = lexer.get_ident_count();

    // intel syntaxの宣言
    if cfg!(target_arch = "x86_64") {
        assembly.push_str(&intel_syntax());
    }

    // main関数
    assembly.push_str(&main_func());

    // 変数の領域を確保
    assembly.push_str(&memory_allocate(ident_count * 8));

    // 構文木をアセンブリに変換
    for tree in trees {
        generate_assembly(&mut assembly, tree);
        assembly.push_str("\tpop rax\n");
    }

    // 最後の式の結果がraxに残り、返される
    assembly.push_str("\tmov rsp, rbp\n");
    assembly.push_str("\tpop rbp\n");
    assembly.push_str(&ret());
    Ok(assembly)
}
