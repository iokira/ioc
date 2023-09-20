pub mod token {
    #[derive(Debug, PartialEq)]
    pub enum Operator {
        Add,
        Sub,
        Mul,
        Div,
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
}
