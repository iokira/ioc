pub mod lexer {
    use crate::{numtype::numtype::NumType, token::token::*};

    pub struct Lexer {
        input: Vec<char>,
        position: usize,
        idents: Vec<Ident>,
    }

    impl Lexer {
        pub fn new(input: &str) -> Lexer {
            Lexer {
                input: input.chars().collect(),
                position: 0,
                idents: vec![],
            }
        }

        pub fn next_token(&mut self) -> Result<Token, ErrorToken> {
            // 空白スキップ
            while self.current_char().is_whitespace() {
                self.proceed_char(1);
            }

            let curr = self.current_char();

            // 数字をまとめる
            let token = if Self::is_number(&curr) {
                let mut number = vec![curr];
                while Self::is_number(&self.peek_char(1)) {
                    self.proceed_char(1);
                    number.push(self.current_char());
                }
                let s: String = number.iter().collect();
                Ok(Token::Operator(OperatorKind::Operand(
                    s.parse::<NumType>().unwrap(),
                )))
            } else if Self::is_ident_char(&curr) {
                let mut ident = vec![curr];
                while Self::is_ident_char(&self.peek_char(1)) {
                    self.proceed_char(1);
                    ident.push(self.current_char());
                }
                Ok(Token::Operator(OperatorKind::Ident(Ident::new(
                    &ident.iter().collect::<String>(),
                ))))
            } else {
                // 演算子を変換
                match curr {
                    '=' if (self.peek_char(1) == '=') => {
                        self.proceed_char(1);
                        Ok(Token::Operator(OperatorKind::Equality))
                    }
                    '=' => Ok(Token::Operator(OperatorKind::Equal)),
                    '!' if (self.peek_char(1) == '=') => {
                        self.proceed_char(1);
                        Ok(Token::Operator(OperatorKind::Nonequality))
                    }
                    '<' if (self.peek_char(1) == '=') => {
                        self.proceed_char(1);
                        Ok(Token::Operator(OperatorKind::LessOrEqual))
                    }
                    '<' => Ok(Token::Operator(OperatorKind::Less)),
                    '>' if (self.peek_char(1) == '=') => {
                        self.proceed_char(1);
                        Ok(Token::Operator(OperatorKind::GreaterOrEqual))
                    }
                    '>' => Ok(Token::Operator(OperatorKind::Greater)),
                    '+' => Ok(Token::Operator(OperatorKind::Add)),
                    '-' => Ok(Token::Operator(OperatorKind::Sub)),
                    '*' => Ok(Token::Operator(OperatorKind::Mul)),
                    '/' => Ok(Token::Operator(OperatorKind::Div)),
                    '(' => Ok(Token::Operator(OperatorKind::LParen)),
                    ')' => Ok(Token::Operator(OperatorKind::RParen)),
                    ';' => Ok(Token::Operator(OperatorKind::Semi)),
                    '\0' => Ok(Token::EOF),
                    _ => Err(ErrorToken::InvaildChar(curr)),
                }
            };
            self.proceed_char(1);
            token
        }

        // 次のトークンが期待している演算子のときはトークンを一つ読み進める
        // それ以外はErrorTokenで包んで返す
        pub fn consume(&mut self, token: Token) -> Result<Token, ErrorToken> {
            while self.current_char().is_whitespace() {
                self.proceed_char(1);
            }

            let op = match token {
                Token::Operator(o) => o,
                Token::EOF => {
                    let curr = self.current_char();
                    if curr == '\0' {
                        return Ok(Token::EOF);
                    } else {
                        return Err(ErrorToken::InvaildChar(curr));
                    }
                }
            };

            let op_chars = format!("{}", op);
            let op_chars_len = op_chars.len();

            let mut curr = String::new();
            let c = self.current_char();

            for i in 0..op_chars_len {
                curr.push_str(&self.peek_char(i).to_string()[..]);
            }

            if curr == op_chars {
                self.proceed_char(op_chars_len);
                Ok(Token::Operator(op))
            } else {
                Err(ErrorToken::InvaildChar(c))
            }
        }

        pub fn consume_ident(&mut self) -> Result<Token, ErrorToken> {
            while self.current_char().is_whitespace() {
                self.proceed_char(1);
            }

            let curr = self.current_char();

            let token = if Self::is_ident_char(&curr) {
                let mut ident = vec![curr];
                while Self::is_ident_char(&self.peek_char(1)) {
                    self.proceed_char(1);
                    ident.push(self.current_char());
                }
                Ok(Token::Operator(OperatorKind::Ident(Ident::new(
                    &ident.iter().collect::<String>(),
                ))))
            } else {
                return Err(ErrorToken::InvaildChar(curr));
            };

            token
        }

        // 次のトークンが期待している演算子のときはトークンを一つ読み進める
        pub fn expect(&mut self, token: Token) -> bool {
            while self.current_char().is_whitespace() {
                self.proceed_char(1);
            }

            let op = match token {
                Token::Operator(o) => o,
                Token::EOF => {
                    let curr = self.current_char();
                    return curr == '\0';
                }
            };

            let op_chars = format!("{}", op);
            let op_chars_len = op_chars.len();

            let mut curr = String::new();

            for i in 0..op_chars_len {
                curr.push_str(&self.peek_char(i).to_string()[..]);
            }

            curr == op_chars
        }

        pub fn expect_ident(&mut self) -> bool {
            while self.current_char().is_whitespace() {
                self.proceed_char(1);
            }

            let curr = self.current_char();
            Self::is_ident_char(&curr)
        }

        // 入力n分だけ読み進める
        fn proceed_char(&mut self, n: usize) {
            self.position += n;
        }

        // 現在の文字を返す
        // ファイル末尾の場合\0を返す
        fn current_char(&mut self) -> char {
            match self.input.get(self.position) {
                Some(c) => *c,
                None => '\0',
            }
        }

        // 入力n分だけ先の文字を取得
        fn peek_char(&mut self, n: usize) -> char {
            match self.input.get(self.position + n) {
                Some(c) => *c,
                None => '\0',
            }
        }

        // 数字の判定
        // 1.1.1のような文字に対応できるよう修正が必要
        fn is_number(c: &char) -> bool {
            c.is_ascii_digit() || c == &'.'
        }

        fn is_ident_char(c: &char) -> bool {
            c.is_alphabetic() || c == &'_'
        }

        fn push_ident(&mut self, ident: Ident) {
            self.idents.push(ident);
        }

        pub fn calc_offset(&mut self, ident: Ident) -> usize {
            match self.idents.iter().position(|i| *i == ident) {
                Some(index) => (index + 1) * 8,
                None => {
                    self.push_ident(ident);
                    self.idents.len() * 8
                }
            }
        }

        pub fn get_ident_count(&mut self) -> usize {
            self.idents.len()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::lexer::Lexer, token::token::*};

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("1 +10 - 2*3 + 6/2a == < >= != $;");
        assert_eq!(
            lexer.next_token(),
            Ok(Token::Operator(OperatorKind::Operand(1)))
        );
        assert_eq!(lexer.next_token(), Ok(Token::Operator(OperatorKind::Add)));
        assert_eq!(
            lexer.next_token(),
            Ok(Token::Operator(OperatorKind::Operand(10)))
        );
        assert_eq!(lexer.next_token(), Ok(Token::Operator(OperatorKind::Sub)));
        assert_eq!(
            lexer.next_token(),
            Ok(Token::Operator(OperatorKind::Operand(2)))
        );
        assert_eq!(lexer.next_token(), Ok(Token::Operator(OperatorKind::Mul)));
        assert_eq!(
            lexer.next_token(),
            Ok(Token::Operator(OperatorKind::Operand(3)))
        );
        assert_eq!(lexer.next_token(), Ok(Token::Operator(OperatorKind::Add)));
        assert_eq!(
            lexer.next_token(),
            Ok(Token::Operator(OperatorKind::Operand(6)))
        );
        assert_eq!(lexer.next_token(), Ok(Token::Operator(OperatorKind::Div)));
        assert_eq!(
            lexer.next_token(),
            Ok(Token::Operator(OperatorKind::Operand(2)))
        );
        assert_eq!(
            lexer.next_token(),
            Ok(Token::Operator(OperatorKind::Ident(Ident::new("a"))))
        );
        assert_eq!(
            lexer.next_token(),
            Ok(Token::Operator(OperatorKind::Equality))
        );
        assert_eq!(lexer.next_token(), Ok(Token::Operator(OperatorKind::Less)));
        assert_eq!(
            lexer.next_token(),
            Ok(Token::Operator(OperatorKind::GreaterOrEqual))
        );
        assert_eq!(
            lexer.next_token(),
            Ok(Token::Operator(OperatorKind::Nonequality))
        );
        assert_eq!(lexer.next_token(), Err(ErrorToken::InvaildChar('$')));
        assert_eq!(lexer.next_token(), Ok(Token::Operator(OperatorKind::Semi)));
        assert_eq!(lexer.next_token(), Ok(Token::EOF));
    }
}
