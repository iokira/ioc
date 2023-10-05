pub mod parser {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::*;
    use crate::tree::tree::*;

    // プログラム
    pub fn program(lexer: &mut Lexer) -> Tree {
        // while !lexer.expect(Token::EOF) {
        //     stmt(lexer)
        // }
        stmt(lexer)
    }

    // 命令
    fn stmt(lexer: &mut Lexer) -> Tree {
        let tree = expr(lexer);
        if let Err(_) = lexer.consume(Token::Operator(OperatorKind::Semi)) {
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
        if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::Equal)) {
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
            if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::Equality)) {
                tree = Tree::new_tree(NodeKind::Equality, tree, relational(lexer));
            }
            if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::Nonequality)) {
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
            if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::Less)) {
                tree = Tree::new_tree(NodeKind::Less, tree, add(lexer));
            }
            if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::LessOrEqual)) {
                tree = Tree::new_tree(NodeKind::LessOrEqual, tree, add(lexer));
            }
            if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::Greater)) {
                tree = Tree::new_tree(NodeKind::Less, add(lexer), tree);
            }
            if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::GreaterOrEqual)) {
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
            if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::Add)) {
                tree = Tree::new_tree(NodeKind::Add, tree, mul(lexer));
            }
            if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::Sub)) {
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
            if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::Mul)) {
                tree = Tree::new_tree(NodeKind::Mul, tree, unary(lexer));
            }
            if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::Div)) {
                tree = Tree::new_tree(NodeKind::Div, tree, unary(lexer));
            }
        }
        tree
    }

    // 単行演算子 +, -
    fn unary(lexer: &mut Lexer) -> Tree {
        if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::Add)) {
            return primary(lexer);
        }
        if let Ok(_) = lexer.consume(Token::Operator(OperatorKind::Sub)) {
            return Tree::new_tree(NodeKind::Sub, Tree::Num(0.0), primary(lexer));
        }
        primary(lexer)
    }

    // 数字
    fn primary(lexer: &mut Lexer) -> Tree {
        match lexer.next_token() {
            Ok(Token::Operator(OperatorKind::LParen)) => {
                let tree = expr(lexer);
                match lexer.consume(Token::Operator(OperatorKind::RParen)) {
                    Ok(_) => return tree,
                    _ => panic!("expect ')' but disappear"),
                }
            }
            Ok(Token::Operator(OperatorKind::Operand(n))) => return Tree::new_leaf(n),
            _ => panic!("expect number or block but disappear"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::lexer::Lexer, parser::parser::*, tree::tree::*};

    #[test]
    fn test_parser() {
        let lexer1 = &mut Lexer::new("1+1;");
        assert_eq!(
            program(lexer1),
            Tree::Node(
                NodeKind::Add,
                Box::new(Tree::Num(1.0)),
                Box::new(Tree::Num(1.0))
            )
        );

        let lexer2 = &mut Lexer::new("1+1*2;");
        assert_eq!(
            program(lexer2),
            Tree::Node(
                NodeKind::Add,
                Box::new(Tree::Num(1.0)),
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Num(1.0)),
                    Box::new(Tree::Num(2.0))
                ))
            )
        );

        let lexer3 = &mut Lexer::new("3 * (2 + 3) - (6 / 2 + 2);");
        assert_eq!(
            program(lexer3),
            Tree::Node(
                NodeKind::Sub,
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Num(3.0)),
                    Box::new(Tree::Node(
                        NodeKind::Add,
                        Box::new(Tree::Num(2.0)),
                        Box::new(Tree::Num(3.0))
                    ))
                )),
                Box::new(Tree::Node(
                    NodeKind::Add,
                    Box::new(Tree::Node(
                        NodeKind::Div,
                        Box::new(Tree::Num(6.0)),
                        Box::new(Tree::Num(2.0))
                    )),
                    Box::new(Tree::Num(2.0))
                ))
            )
        );

        let lexer4 = &mut Lexer::new("5 + 6 * 7;");
        assert_eq!(
            program(lexer4),
            Tree::Node(
                NodeKind::Add,
                Box::new(Tree::Num(5.0)),
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Num(6.0)),
                    Box::new(Tree::Num(7.0))
                ))
            )
        );

        let lexer5 = &mut Lexer::new("2 * 3 == 3 + 1;");
        assert_eq!(
            program(lexer5),
            Tree::Node(
                NodeKind::Equality,
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Num(2.0)),
                    Box::new(Tree::Num(3.0)),
                )),
                Box::new(Tree::Node(
                    NodeKind::Add,
                    Box::new(Tree::Num(3.0)),
                    Box::new(Tree::Num(1.0)),
                ))
            )
        );
    }
}
