pub mod mygenerator {
    use crate::{architecture::myarchitecture::*, tree::mytree::*};

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
                NodeKind::Add => assembly.push_str(&add_arg()),
                NodeKind::Sub => assembly.push_str(&sub_arg()),
                NodeKind::Mul => assembly.push_str(&mul_arg()),
                NodeKind::Div => assembly.push_str(&div_arg()),
                NodeKind::Assign => unimplemented!(),
                NodeKind::VAL => unimplemented!(),
            }
            assembly.push_str(&push(Operand::Register(Register::R0)));
        }
    }
}
