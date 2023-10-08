pub mod tree {
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
        Num(usize),
        Val(usize),
        Node(NodeKind, Box<Tree>, Box<Tree>),
    }

    impl Tree {
        pub fn new_tree(kind: NodeKind, lhs: Tree, rhs: Tree) -> Tree {
            Tree::Node(kind, Box::new(lhs), Box::new(rhs))
        }

        pub fn new_num(num: usize) -> Tree {
            Tree::Num(num)
        }

        pub fn new_val(c: char) -> Tree {
            Tree::Val((c as usize - 'a' as usize + 1) * 8)
        }
    }
}
