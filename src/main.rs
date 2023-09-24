use std::{env, process};

use ioc::Input;

fn main() {
    // コマンドライン引数を読み取る
    let args: Vec<String> = env::args().collect();

    // コマンドライン引数を解釈
    let input = Input::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // エラー処理はここで一元管理したい
    if let Err(e) = ioc::run(input) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
