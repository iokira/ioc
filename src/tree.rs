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
        Num(f64),
        Val(f64),
        Node(NodeKind, Box<Tree>, Box<Tree>),
    }

    impl Tree {
        pub fn new_tree(kind: NodeKind, lhs: Tree, rhs: Tree) -> Tree {
            Tree::Node(kind, Box::new(lhs), Box::new(rhs))
        }

        pub fn new_leaf(num: f64) -> Tree {
            Tree::Num(num)
        }
    }
}
