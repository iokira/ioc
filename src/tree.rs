pub mod tree {
    use crate::numtype::numtype::NumType;

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
        Val(NumType),
        Node(NodeKind, Box<Tree>, Box<Tree>),
    }

    impl Tree {
        pub fn new_tree(kind: NodeKind, lhs: Tree, rhs: Tree) -> Tree {
            Tree::Node(kind, Box::new(lhs), Box::new(rhs))
        }

        pub fn new_num(num: NumType) -> Tree {
            Tree::Num(num)
        }

        pub fn new_val(c: char) -> Tree {
            Tree::Val((c as NumType - 'a' as NumType + 1) * 8)
        }
    }
}
