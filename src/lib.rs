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
}

#[derive(Debug, PartialEq)]
enum ErrorToken {
    InvaildChar(char),
}

fn tokenize(expr: &str) -> Result<Vec<Token>, ErrorToken> {
    expr.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '+' => Ok(Token::Operator(Operator::Add)),
            '-' => Ok(Token::Operator(Operator::Sub)),
            '*' => Ok(Token::Operator(Operator::Mul)),
            '/' => Ok(Token::Operator(Operator::Div)),
            n => {
                if let Ok(num) = n.to_string().parse::<f64>() {
                    Ok(Token::Operand(num))
                } else {
                    Err(ErrorToken::InvaildChar(n))
                }
            }
        })
        .into_iter()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_test() {
        assert_eq!(tokenize("1"), Ok(vec![Token::Operand(1.0)]));
        assert_eq!(tokenize("2"), Ok(vec![Token::Operand(2.0)]));
        assert_eq!(tokenize("3"), Ok(vec![Token::Operand(3.0)]));
        assert_eq!(tokenize("4"), Ok(vec![Token::Operand(4.0)]));
        assert_eq!(tokenize("5"), Ok(vec![Token::Operand(5.0)]));
        assert_eq!(tokenize("6"), Ok(vec![Token::Operand(6.0)]));
        assert_eq!(tokenize("7"), Ok(vec![Token::Operand(7.0)]));
        assert_eq!(tokenize("8"), Ok(vec![Token::Operand(8.0)]));
        assert_eq!(tokenize("9"), Ok(vec![Token::Operand(9.0)]));
        assert_eq!(tokenize("0"), Ok(vec![Token::Operand(0.0)]));

        assert_eq!(tokenize("+"), Ok(vec![Token::Operator(Operator::Add)]));
        assert_eq!(tokenize("-"), Ok(vec![Token::Operator(Operator::Sub)]));
        assert_eq!(tokenize("*"), Ok(vec![Token::Operator(Operator::Mul)]));
        assert_eq!(tokenize("/"), Ok(vec![Token::Operator(Operator::Div)]));

        assert_eq!(tokenize("a"), Err(ErrorToken::InvaildChar('a')));

        assert_eq!(tokenize(" "), Ok(vec![]));
        assert_eq!(tokenize(" 1"), Ok(vec![Token::Operand(1.0)]));
        assert_eq!(tokenize("1 "), Ok(vec![Token::Operand(1.0)]));
        assert_eq!(
            tokenize("1 1"),
            Ok(vec![Token::Operand(1.0), Token::Operand(1.0)])
        );

        assert_eq!(
            tokenize("123 + 4 * 2 - 20 / 2"),
            Ok(vec![
                Token::Operand(1.0),
                Token::Operand(2.0),
                Token::Operand(3.0),
                Token::Operator(Operator::Add),
                Token::Operand(4.0),
                Token::Operator(Operator::Mul),
                Token::Operand(2.0),
                Token::Operator(Operator::Sub),
                Token::Operand(2.0),
                Token::Operand(0.0),
                Token::Operator(Operator::Div),
                Token::Operand(2.0)
            ])
        )
    }
}
