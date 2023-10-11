pub mod mytoken {
    use core::fmt;

    use crate::numtype::mynumtype::NumType;

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
        Ident(Ident),
        Semi,
        Operand(NumType),
        Equal,
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

    #[derive(Debug, PartialEq)]
    pub struct Ident {
        pub name: String,
    }

    impl Ident {
        pub fn new(name: &str) -> Ident {
            Ident {
                name: name.to_string(),
            }
        }
    }

    impl fmt::Display for OperatorKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                OperatorKind::Ident(c) => write!(f, "{}", c.name),
                OperatorKind::Semi => write!(f, ";"),
                OperatorKind::Operand(n) => write!(f, "{}", n),
                OperatorKind::Equal => write!(f, "="),
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
