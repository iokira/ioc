pub mod token {
    use core::fmt;

    #[derive(Debug, PartialEq)]
    pub enum Operator {
        Add,
        Sub,
        Mul,
        Div,
        LParen,
        RParen,
    }

    #[derive(Debug, PartialEq)]
    pub enum Token {
        Operator(Operator),
        Operand(f64),
        EOF,
    }

    #[derive(Debug, PartialEq)]
    pub enum ErrorToken {
        InvaildChar(char),
    }

    impl fmt::Display for Operator {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Operator::Add => write!(f, "+"),
                Operator::Sub => write!(f, "-"),
                Operator::Mul => write!(f, "*"),
                Operator::Div => write!(f, "/"),
                Operator::LParen => write!(f, "("),
                Operator::RParen => write!(f, ")"),
            }
        }
    }
}
