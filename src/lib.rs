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

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
enum Token {
    Operator(Operator),
    Operand(f64),
    EOF,
}

#[derive(Debug, PartialEq)]
enum ErrorToken {
    InvaildChar(char),
}

struct Lexer {
    input: Vec<char>,
    position: usize,
    length: usize,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            length: input.len(),
        }
    }

    fn next_token(&mut self) -> Result<Token, ErrorToken> {
        while self.current_char().is_whitespace() {
            self.next_char();
        }

        let curr = self.current_char();
        let token = if Self::is_number(curr) {
            let mut number = vec![*curr];
            while Self::is_number(self.peek_char()) {
                self.next_char();
                number.push(*self.current_char());
            }
            let s: String = number.iter().collect();
            Ok(Token::Operand(s.parse::<f64>().unwrap()))
        } else {
            match curr {
                &'+' => Ok(Token::Operator(Operator::Add)),
                &'-' => Ok(Token::Operator(Operator::Sub)),
                &'*' => Ok(Token::Operator(Operator::Mul)),
                &'/' => Ok(Token::Operator(Operator::Div)),
                &'\0' => Ok(Token::EOF),
                _ => Err(ErrorToken::InvaildChar(*curr)),
            }
        };
        self.next_char();
        return token;
    }

    fn next_char(&mut self) {
        self.position += 1;
    }

    fn current_char(&mut self) -> &char {
        match self.input.get(self.position) {
            Some(c) => c,
            None => &'\0',
        }
    }

    fn peek_char(&mut self) -> &char {
        match self.input.get(self.position + 1) {
            Some(c) => c,
            None => &'\0',
        }
    }

    fn is_number(c: &char) -> bool {
        c.is_ascii_digit() || c == &'.'
    }
}
