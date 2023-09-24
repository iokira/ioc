pub mod parser {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::*;
    use crate::tree::tree::*;

    pub fn expr(lexer: &mut Lexer) -> Tree {
        let mut tree = mul(lexer);
        while lexer.expect(OperatorKind::Add) || lexer.expect(OperatorKind::Sub) {
            if let Ok(_) = lexer.consume(OperatorKind::Add) {
                tree = Tree::new_tree(NodeKind::ADD, tree, mul(lexer));
            }
            if let Ok(_) = lexer.consume(OperatorKind::Sub) {
                tree = Tree::new_tree(NodeKind::SUB, tree, mul(lexer));
            }
        }
        tree
    }

    fn mul(lexer: &mut Lexer) -> Tree {
        let mut tree = unary(lexer);
        while lexer.expect(OperatorKind::Mul) || lexer.expect(OperatorKind::Div) {
            if let Ok(_) = lexer.consume(OperatorKind::Mul) {
                tree = Tree::new_tree(NodeKind::MUL, tree, unary(lexer));
            }
            if let Ok(_) = lexer.consume(OperatorKind::Div) {
                tree = Tree::new_tree(NodeKind::DIV, tree, unary(lexer));
            }
        }
        tree
    }

    fn unary(lexer: &mut Lexer) -> Tree {
        if let Ok(_) = lexer.consume(OperatorKind::Add) {
            return primary(lexer);
        }
        if let Ok(_) = lexer.consume(OperatorKind::Sub) {
            return Tree::new_tree(NodeKind::SUB, Tree::Leaf(0.0), primary(lexer));
        }
        primary(lexer)
    }

    fn primary(lexer: &mut Lexer) -> Tree {
        match lexer.next_token() {
            Ok(Token::Operator(OperatorKind::LParen)) => {
                let tree = expr(lexer);
                match lexer.consume(OperatorKind::RParen) {
                    Ok(_) => return tree,
                    _ => panic!("expect ')' but disappear"),
                }
            }
            Ok(Token::Operand(n)) => return Tree::new_leaf(n),
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
                NodeKind::ADD,
                Box::new(Tree::Leaf(1.0)),
                Box::new(Tree::Leaf(1.0))
            )
        );

        let lexer2 = &mut Lexer::new("1+1*2");
        assert_eq!(
            expr(lexer2),
            Tree::Node(
                NodeKind::ADD,
                Box::new(Tree::Leaf(1.0)),
                Box::new(Tree::Node(
                    NodeKind::MUL,
                    Box::new(Tree::Leaf(1.0)),
                    Box::new(Tree::Leaf(2.0))
                ))
            )
        );

        let lexer3 = &mut Lexer::new("3 * (2 + 3) - (6 / 2 + 2)");
        assert_eq!(
            expr(lexer3),
            Tree::Node(
                NodeKind::SUB,
                Box::new(Tree::Node(
                    NodeKind::MUL,
                    Box::new(Tree::Leaf(3.0)),
                    Box::new(Tree::Node(
                        NodeKind::ADD,
                        Box::new(Tree::Leaf(2.0)),
                        Box::new(Tree::Leaf(3.0))
                    ))
                )),
                Box::new(Tree::Node(
                    NodeKind::ADD,
                    Box::new(Tree::Node(
                        NodeKind::DIV,
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
                NodeKind::ADD,
                Box::new(Tree::Leaf(5.0)),
                Box::new(Tree::Node(
                    NodeKind::MUL,
                    Box::new(Tree::Leaf(6.0)),
                    Box::new(Tree::Leaf(7.0))
                ))
            )
        );
    }
}
