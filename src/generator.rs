pub mod generator {
    use crate::tree::tree::*;

    // 構文木をアセンブリに変換する
    pub fn generate_assembly(assembly: &mut String, tree: Tree) {
        if let Tree::Leaf(n) = tree {
            let str = format!("\tpush {}\n", n);
            assembly.push_str(&str);
            return;
        }

        if let Tree::Node(kind, lhs, rhs) = tree {
            generate_assembly(assembly, *lhs);
            generate_assembly(assembly, *rhs);

            assembly.push_str("\tpop rdi\n");
            assembly.push_str("\tpop rax\n");

            match kind {
                NodeKind::Equality => {
                    assembly.push_str("\tcmp rax, rdi\n\tsete al\n\tmovzb rax, al\n")
                }
                NodeKind::Nonequality => {
                    assembly.push_str("\tcmp rax, rdi\n\tsetne al\n\tmovzb rax, al\n")
                }
                NodeKind::Less => assembly.push_str("\tcmp rax, rdi\n\tsetl al\n\tmovzb rax, al\n"),
                NodeKind::LessOrEqual => {
                    assembly.push_str("\tcmp rax, rdi\n\tsetle al\n\tmovzb rax, al\n")
                }
                NodeKind::Add => assembly.push_str("\tadd rax, rdi\n"),
                NodeKind::Sub => assembly.push_str("\tsub rax, rdi\n"),
                NodeKind::Mul => assembly.push_str("\timul rax, rdi\n"),
                NodeKind::Div => assembly.push_str("\tcqo\n\tidiv rdi\n"),
            }
            assembly.push_str("\tpush rax\n");
        }
    }
}
