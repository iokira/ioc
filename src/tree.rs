pub mod tree {
    #[derive(Debug, PartialEq)]
    pub enum NodeKind {
        ADD,
        SUB,
        MUL,
        DIV,
    }

    #[derive(Debug, PartialEq)]
    pub enum Tree {
        Leaf(f64),
        Node(NodeKind, Box<Tree>, Box<Tree>),
    }

    impl Tree {
        pub fn new_tree(kind: NodeKind, lhs: Tree, rhs: Tree) -> Tree {
            Tree::Node(kind, Box::new(lhs), Box::new(rhs))
        }

        pub fn new_leaf(num: f64) -> Tree {
            Tree::Leaf(num)
        }
    }
}
