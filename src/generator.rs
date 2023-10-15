pub mod mygenerator {
    use crate::{
        architecture::myarchitecture::{gen_val, pop_arg, pop_lvar, pop_val, push, Operand},
        tree::mytree::*,
    };

    fn generate_val(assembly: &mut String, offset: usize) {
        assembly.push_str(&gen_val(offset));
    }

    // 構文木をアセンブリに変換する
    pub fn generate_assembly(assembly: &mut String, tree: Tree) {
        if let Tree::Num(n) = tree {
            let str = &push(Operand::Num(n));
            assembly.push_str(&str);
            return;
        }

        if let Tree::Val(o) = tree {
            generate_val(assembly, o);
            assembly.push_str(&pop_val());
            return;
        }

        if let Tree::Node(kind, lhs, rhs) = tree {
            if let NodeKind::Assign = kind {
                if let Tree::Val(o) = *lhs {
                    generate_val(assembly, o);
                } else {
                    panic!("The left-hand side value of the assignment is not a variable")
                }
                generate_assembly(assembly, *rhs);
                assembly.push_str(&pop_lvar());
                return;
            }

            generate_assembly(assembly, *lhs);
            generate_assembly(assembly, *rhs);

            assembly.push_str(&pop_arg());

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
                NodeKind::Assign => unimplemented!(),
                NodeKind::VAL => unimplemented!(),
            }
            assembly.push_str("\tpush rax\n");
        }
    }
}
