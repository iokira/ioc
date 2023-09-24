pub mod parser {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::*;
    use crate::tree::tree::*;

    pub fn expr(lexer: &mut Lexer) -> Tree {
        let mut tree = mul(lexer);
        while lexer.expect(Operator::Add) || lexer.expect(Operator::Sub) {
            if let Ok(_) = lexer.consume(Operator::Add) {
                tree = Tree::new_tree(NodeKind::ADD, tree, mul(lexer));
            }
            if let Ok(_) = lexer.consume(Operator::Sub) {
                tree = Tree::new_tree(NodeKind::SUB, tree, mul(lexer));
            }
        }
        tree
    }

    fn mul(lexer: &mut Lexer) -> Tree {
        let mut tree = primary(lexer);
        while lexer.expect(Operator::Mul) || lexer.expect(Operator::Div) {
            if let Ok(_) = lexer.consume(Operator::Mul) {
                tree = Tree::new_tree(NodeKind::MUL, tree, primary(lexer));
            }
            if let Ok(_) = lexer.consume(Operator::Div) {
                tree = Tree::new_tree(NodeKind::DIV, tree, primary(lexer));
            }
        }
        tree
    }

    fn primary(lexer: &mut Lexer) -> Tree {
        match lexer.next_token() {
            Ok(Token::Operator(Operator::LParen)) => {
                let tree = expr(lexer);
                match lexer.consume(Operator::RParen) {
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
    }
}
