pub mod token {
    use core::fmt;

    #[derive(Debug, PartialEq)]
    pub enum Token {
        Operator(OperatorKind),
        EOF,
    }

    #[derive(Debug, PartialEq)]
    pub enum ErrorToken {
        InvaildChar(char),
    }

    #[derive(Debug, PartialEq)]
    pub enum OperatorKind {
        Ident(char),
        Semi,
        Operand(f64),
        Equality,
        Nonequality,
        Less,
        LessOrEqual,
        Greater,
        GreaterOrEqual,
        Add,
        Sub,
        Mul,
        Div,
        LParen,
        RParen,
    }

    impl fmt::Display for OperatorKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                OperatorKind::Ident(c) => write!(f, "{}", c),
                OperatorKind::Semi => write!(f, ";"),
                OperatorKind::Operand(n) => write!(f, "{}", n),
                OperatorKind::Equality => write!(f, "=="),
                OperatorKind::Nonequality => write!(f, "!="),
                OperatorKind::Less => write!(f, "<"),
                OperatorKind::LessOrEqual => write!(f, "<="),
                OperatorKind::Greater => write!(f, ">"),
                OperatorKind::GreaterOrEqual => write!(f, ">="),
                OperatorKind::Add => write!(f, "+"),
                OperatorKind::Sub => write!(f, "-"),
                OperatorKind::Mul => write!(f, "*"),
                OperatorKind::Div => write!(f, "/"),
                OperatorKind::LParen => write!(f, "("),
                OperatorKind::RParen => write!(f, ")"),
            }
        }
    }
}
