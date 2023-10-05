pub mod parser {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::*;
    use crate::tree::tree::*;

    // 式
    pub fn expr(lexer: &mut Lexer) -> Tree {
        equality(lexer)
    }

    // 比較 ==, !=
    fn equality(lexer: &mut Lexer) -> Tree {
        let mut tree = relational(lexer);
        while lexer.expect(OperatorKind::Equality) || lexer.expect(OperatorKind::Nonequality) {
            if let Ok(_) = lexer.consume(OperatorKind::Equality) {
                tree = Tree::new_tree(NodeKind::Equality, tree, relational(lexer));
            }
            if let Ok(_) = lexer.consume(OperatorKind::Nonequality) {
                tree = Tree::new_tree(NodeKind::Nonequality, tree, relational(lexer));
            }
        }
        tree
    }

    // 比較 <, <=, >, >=
    fn relational(lexer: &mut Lexer) -> Tree {
        let mut tree = add(lexer);
        while lexer.expect(OperatorKind::Less)
            || lexer.expect(OperatorKind::LessOrEqual)
            || lexer.expect(OperatorKind::Greater)
            || lexer.expect(OperatorKind::GreaterOrEqual)
        {
            if let Ok(_) = lexer.consume(OperatorKind::Less) {
                tree = Tree::new_tree(NodeKind::Less, tree, add(lexer));
            }
            if let Ok(_) = lexer.consume(OperatorKind::LessOrEqual) {
                tree = Tree::new_tree(NodeKind::LessOrEqual, tree, add(lexer));
            }
            if let Ok(_) = lexer.consume(OperatorKind::Greater) {
                tree = Tree::new_tree(NodeKind::Less, add(lexer), tree);
            }
            if let Ok(_) = lexer.consume(OperatorKind::GreaterOrEqual) {
                tree = Tree::new_tree(NodeKind::LessOrEqual, add(lexer), tree);
            }
        }
        tree
    }

    // 加減算 +, -
    fn add(lexer: &mut Lexer) -> Tree {
        let mut tree = mul(lexer);
        while lexer.expect(OperatorKind::Add) || lexer.expect(OperatorKind::Sub) {
            if let Ok(_) = lexer.consume(OperatorKind::Add) {
                tree = Tree::new_tree(NodeKind::Add, tree, mul(lexer));
            }
            if let Ok(_) = lexer.consume(OperatorKind::Sub) {
                tree = Tree::new_tree(NodeKind::Sub, tree, mul(lexer));
            }
        }
        tree
    }

    // 乗除算 *, /
    fn mul(lexer: &mut Lexer) -> Tree {
        let mut tree = unary(lexer);
        while lexer.expect(OperatorKind::Mul) || lexer.expect(OperatorKind::Div) {
            if let Ok(_) = lexer.consume(OperatorKind::Mul) {
                tree = Tree::new_tree(NodeKind::Mul, tree, unary(lexer));
            }
            if let Ok(_) = lexer.consume(OperatorKind::Div) {
                tree = Tree::new_tree(NodeKind::Div, tree, unary(lexer));
            }
        }
        tree
    }

    // 単行演算子 +, -
    fn unary(lexer: &mut Lexer) -> Tree {
        if let Ok(_) = lexer.consume(OperatorKind::Add) {
            return primary(lexer);
        }
        if let Ok(_) = lexer.consume(OperatorKind::Sub) {
            return Tree::new_tree(NodeKind::Sub, Tree::Leaf(0.0), primary(lexer));
        }
        primary(lexer)
    }

    // 数字
    fn primary(lexer: &mut Lexer) -> Tree {
        match lexer.next_token() {
            Ok(Token::Operator(OperatorKind::LParen)) => {
                let tree = expr(lexer);
                match lexer.consume(OperatorKind::RParen) {
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
        let lexer1 = &mut Lexer::new("1+1");
        assert_eq!(
            expr(lexer1),
            Tree::Node(
                NodeKind::Add,
                Box::new(Tree::Leaf(1.0)),
                Box::new(Tree::Leaf(1.0))
            )
        );

        let lexer2 = &mut Lexer::new("1+1*2");
        assert_eq!(
            expr(lexer2),
            Tree::Node(
                NodeKind::Add,
                Box::new(Tree::Leaf(1.0)),
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Leaf(1.0)),
                    Box::new(Tree::Leaf(2.0))
                ))
            )
        );

        let lexer3 = &mut Lexer::new("3 * (2 + 3) - (6 / 2 + 2)");
        assert_eq!(
            expr(lexer3),
            Tree::Node(
                NodeKind::Sub,
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Leaf(3.0)),
                    Box::new(Tree::Node(
                        NodeKind::Add,
                        Box::new(Tree::Leaf(2.0)),
                        Box::new(Tree::Leaf(3.0))
                    ))
                )),
                Box::new(Tree::Node(
                    NodeKind::Add,
                    Box::new(Tree::Node(
                        NodeKind::Div,
                        Box::new(Tree::Leaf(6.0)),
                        Box::new(Tree::Leaf(2.0))
                    )),
                    Box::new(Tree::Leaf(2.0))
                ))
            )
        );

        let lexer4 = &mut Lexer::new("5 + 6 * 7");
        assert_eq!(
            expr(lexer4),
            Tree::Node(
                NodeKind::Add,
                Box::new(Tree::Leaf(5.0)),
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Leaf(6.0)),
                    Box::new(Tree::Leaf(7.0))
                ))
            )
        );

        let lexer5 = &mut Lexer::new("2 * 3 == 3 + 1");
        assert_eq!(
            expr(lexer5),
            Tree::Node(
                NodeKind::Equality,
                Box::new(Tree::Node(
                    NodeKind::Mul,
                    Box::new(Tree::Leaf(2.0)),
                    Box::new(Tree::Leaf(3.0)),
                )),
                Box::new(Tree::Node(
                    NodeKind::Add,
                    Box::new(Tree::Leaf(3.0)),
                    Box::new(Tree::Leaf(1.0)),
                ))
            )
        );
    }
}
