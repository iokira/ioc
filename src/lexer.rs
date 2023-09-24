pub mod lexer {
    use crate::token::token::*;

    pub struct Lexer {
        input: Vec<char>,
        position: usize,
    }

    impl Lexer {
        pub fn new(input: &str) -> Lexer {
            Lexer {
                input: input.chars().collect(),
                position: 0,
            }
        }

        pub fn next_token(&mut self) -> Result<Token, ErrorToken> {
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
                    &'(' => Ok(Token::Operator(Operator::LParen)),
                    &')' => Ok(Token::Operator(Operator::RParen)),
                    &'\0' => Ok(Token::EOF),
                    _ => Err(ErrorToken::InvaildChar(*curr)),
                }
            };
            self.next_char();
            return token;
        }

        pub fn consume(&mut self, op: Operator) -> Result<Token, ErrorToken> {
            while self.current_char().is_whitespace() {
                self.next_char();
            }
            let curr = self.current_char();
            let op_chars: Vec<char> = format!("{}", op).chars().collect();
            let op_char = op_chars[0];
            if curr == &op_char {
                self.next_char();
                Ok(Token::Operator(op))
            } else {
                Err(ErrorToken::InvaildChar(*curr))
            }
        }

        pub fn expect(&mut self, op: Operator) -> bool {
            while self.current_char().is_whitespace() {
                self.next_char();
            }
            let curr = self.current_char();
            let op_chars: Vec<char> = format!("{}", op).chars().collect();
            let op_char = op_chars[0];
            curr == &op_char
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

        pub fn get_positoin(&mut self) -> usize {
            self.position
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::lexer::Lexer, token::token::*};

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("1 +10 - 2*3 + 6/2a");
        assert_eq!(lexer.next_token(), Ok(Token::Operand(1.0)));
        assert_eq!(lexer.next_token(), Ok(Token::Operator(Operator::Add)));
        assert_eq!(lexer.next_token(), Ok(Token::Operand(10.0)));
        assert_eq!(lexer.next_token(), Ok(Token::Operator(Operator::Sub)));
        assert_eq!(lexer.next_token(), Ok(Token::Operand(2.0)));
        assert_eq!(lexer.next_token(), Ok(Token::Operator(Operator::Mul)));
        assert_eq!(lexer.next_token(), Ok(Token::Operand(3.0)));
        assert_eq!(lexer.next_token(), Ok(Token::Operator(Operator::Add)));
        assert_eq!(lexer.next_token(), Ok(Token::Operand(6.0)));
        assert_eq!(lexer.next_token(), Ok(Token::Operator(Operator::Div)));
        assert_eq!(lexer.next_token(), Ok(Token::Operand(2.0)));
        assert_eq!(lexer.next_token(), Err(ErrorToken::InvaildChar('a')));
        assert_eq!(lexer.next_token(), Ok(Token::EOF));
    }
}
