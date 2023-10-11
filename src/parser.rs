pub mod myparser {
    use crate::lexer::mylexer::Lexer;
    use crate::token::mytoken::*;
    use crate::tree::mytree::*;

    // プログラム
    pub fn program(lexer: &mut Lexer) -> (Vec<Tree>, &mut Lexer) {
        let mut trees = Vec::new();
        while !lexer.expect(Token::EOF) {
            trees.push(stmt(lexer));
        }
        (trees, lexer)
    }

    // 命令
    fn stmt(lexer: &mut Lexer) -> Tree {
        let tree = expr(lexer);
        if lexer.consume(Token::Operator(OperatorKind::Semi)).is_err() {
            panic!("expected semi");
        }
        tree
    }

    // 式
    fn expr(lexer: &mut Lexer) -> Tree {
        assign(lexer)
    }

    // 代入式
    fn assign(lexer: &mut Lexer) -> Tree {
        let mut tree = equality(lexer);
        if lexer.consume(Token::Operator(OperatorKind::Equal)).is_ok() {
            tree = Tree::new_tree(NodeKind::Assign, tree, assign(lexer));
        }
        tree
    }

    // 比較 ==, !=
    fn equality(lexer: &mut Lexer) -> Tree {
        let mut tree = relational(lexer);
        while lexer.expect(Token::Operator(OperatorKind::Equality))
            || lexer.expect(Token::Operator(OperatorKind::Nonequality))
        {
            if lexer
                .consume(Token::Operator(OperatorKind::Equality))
                .is_ok()
            {
                tree = Tree::new_tree(NodeKind::Equality, tree, relational(lexer));
            }
            if lexer
                .consume(Token::Operator(OperatorKind::Nonequality))
                .is_ok()
            {
                tree = Tree::new_tree(NodeKind::Nonequality, tree, relational(lexer));
            }
        }
        tree
    }

    // 比較 <, <=, >, >=
    fn relational(lexer: &mut Lexer) -> Tree {
        let mut tree = add(lexer);
        while lexer.expect(Token::Operator(OperatorKind::Less))
            || lexer.expect(Token::Operator(OperatorKind::LessOrEqual))
            || lexer.expect(Token::Operator(OperatorKind::Greater))
            || lexer.expect(Token::Operator(OperatorKind::GreaterOrEqual))
        {
            if lexer.consume(Token::Operator(OperatorKind::Less)).is_ok() {
                tree = Tree::new_tree(NodeKind::Less, tree, add(lexer));
            }
            if lexer
                .consume(Token::Operator(OperatorKind::LessOrEqual))
                .is_ok()
            {
                tree = Tree::new_tree(NodeKind::LessOrEqual, tree, add(lexer));
            }
            if lexer
                .consume(Token::Operator(OperatorKind::Greater))
                .is_ok()
            {
                tree = Tree::new_tree(NodeKind::Less, add(lexer), tree);
            }
            if lexer
                .consume(Token::Operator(OperatorKind::GreaterOrEqual))
                .is_ok()
            {
                tree = Tree::new_tree(NodeKind::LessOrEqual, add(lexer), tree);
            }
        }
        tree
    }

    // 加減算 +, -
    fn add(lexer: &mut Lexer) -> Tree {
        let mut tree = mul(lexer);
        while lexer.expect(Token::Operator(OperatorKind::Add))
            || lexer.expect(Token::Operator(OperatorKind::Sub))
        {
            if lexer.consume(Token::Operator(OperatorKind::Add)).is_ok() {
                tree = Tree::new_tree(NodeKind::Add, tree, mul(lexer));
            }
            if lexer.consume(Token::Operator(OperatorKind::Sub)).is_ok() {
                tree = Tree::new_tree(NodeKind::Sub, tree, mul(lexer));
            }
        }
        tree
    }

    // 乗除算 *, /
    fn mul(lexer: &mut Lexer) -> Tree {
        let mut tree = unary(lexer);
        while lexer.expect(Token::Operator(OperatorKind::Mul))
            || lexer.expect(Token::Operator(OperatorKind::Div))
        {
            if lexer.consume(Token::Operator(OperatorKind::Mul)).is_ok() {
                tree = Tree::new_tree(NodeKind::Mul, tree, unary(lexer));
            }
            if lexer.consume(Token::Operator(OperatorKind::Div)).is_ok() {
                tree = Tree::new_tree(NodeKind::Div, tree, unary(lexer));
            }
        }
        tree
    }

    // 単行演算子 +, -
    fn unary(lexer: &mut Lexer) -> Tree {
        if lexer.consume(Token::Operator(OperatorKind::Add)).is_ok() {
            return primary(lexer);
        }
        if lexer.consume(Token::Operator(OperatorKind::Sub)).is_ok() {
            return Tree::new_tree(NodeKind::Sub, Tree::Num(0), primary(lexer));
        }
        primary(lexer)
    }

    // 数字
    fn primary(lexer: &mut Lexer) -> Tree {
        match lexer.next_token() {
            Ok(Token::Operator(OperatorKind::LParen)) => {
                let tree = expr(lexer);
                match lexer.consume(Token::Operator(OperatorKind::RParen)) {
                    Ok(_) => tree,
                    _ => panic!("expect ')' but disappear"),
                }
            }
            Ok(Token::Operator(OperatorKind::Operand(n))) => Tree::new_num(n),
            Ok(Token::Operator(OperatorKind::Ident(i))) => Tree::new_val(i, lexer),
            _ => panic!("expect number or block but disappear"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::mylexer::Lexer, parser::myparser::*, tree::mytree::*};

    #[test]
    fn test_parser() {
        let lexer1 = &mut Lexer::new("1+1;");
        let (lexer1, _) = program(lexer1);
        assert_eq!(
            lexer1,
            [Tree::Node(
                NodeKind::Add,
                Box::new(Tree::Num(1)),
                Box::new(Tree::Num(1))
            )]
        );

        let lexer2 = &mut Lexer::new("1+1*2;");
        let (lexer2, _) = program(lexer2);
        assert_eq!(
            lexer2,
            [Tree::Node(
                NodeKind::Add,
                Box::new(Tree::Num(1)),
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Num(1)),
                    Box::new(Tree::Num(2))
                ))
            )]
        );

        let lexer3 = &mut Lexer::new("3 * (2 + 3) - (6 / 2 + 2);");
        let (lexer3, _) = program(lexer3);
        assert_eq!(
            lexer3,
            [Tree::Node(
                NodeKind::Sub,
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Num(3)),
                    Box::new(Tree::Node(
                        NodeKind::Add,
                        Box::new(Tree::Num(2)),
                        Box::new(Tree::Num(3))
                    ))
                )),
                Box::new(Tree::Node(
                    NodeKind::Add,
                    Box::new(Tree::Node(
                        NodeKind::Div,
                        Box::new(Tree::Num(6)),
                        Box::new(Tree::Num(2))
                    )),
                    Box::new(Tree::Num(2))
                ))
            )]
        );

        let lexer4 = &mut Lexer::new("5 + 6 * 7;");
        let (lexer4, _) = program(lexer4);
        assert_eq!(
            lexer4,
            [Tree::Node(
                NodeKind::Add,
                Box::new(Tree::Num(5)),
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Num(6)),
                    Box::new(Tree::Num(7))
                ))
            )]
        );

        let lexer5 = &mut Lexer::new("2 * 3 == 3 + 1;");
        let (lexer5, _) = program(lexer5);
        assert_eq!(
            lexer5,
            [Tree::Node(
                NodeKind::Equality,
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Num(2)),
                    Box::new(Tree::Num(3)),
                )),
                Box::new(Tree::Node(
                    NodeKind::Add,
                    Box::new(Tree::Num(3)),
                    Box::new(Tree::Num(1)),
                ))
            )]
        );
        let lexer6 = &mut Lexer::new("a;");
        let (lexer6, _) = program(lexer6);
        assert_eq!(lexer6, [Tree::Val(8)]);
    }
}
