pub mod tree {
    use crate::{lexer::lexer::Lexer, numtype::numtype::NumType, token::token::Ident};

    #[derive(Debug, PartialEq)]
    pub enum NodeKind {
        Assign,      // =
        VAL,         // value
        Equality,    // ==
        Nonequality, // !=
        Less,        // <
        LessOrEqual, // <=
        Add,         // +
        Sub,         // -
        Mul,         // *
        Div,         // /
    }

    #[derive(Debug, PartialEq)]
    pub enum Tree {
        Num(NumType),
        Val(usize),
        Node(NodeKind, Box<Tree>, Box<Tree>),
    }

    impl Tree {
        pub fn new_tree(kind: NodeKind, lhs: Tree, rhs: Tree) -> Tree {
            Tree::Node(kind, Box::new(lhs), Box::new(rhs))
        }

        pub fn new_num(num: NumType) -> Tree {
            Tree::Num(num)
        }

        pub fn new_val(i: Ident, lexer: &mut Lexer) -> Tree {
            Tree::Val(lexer.calc_offset(i))
        }
    }
}
