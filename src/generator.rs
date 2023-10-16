pub mod mygenerator {
    use crate::{architecture::myarchitecture::*, tree::mytree::*};

    fn generate_val(assembly: &mut String, offset: usize) {
        assembly.push_str(&gen_val(offset));
    }

    // 構文木をアセンブリに変換する
    pub fn generate_assembly(assembly: &mut String, tree: Tree) {
        if let Tree::Num(n) = tree {
            assembly.push_str(&push(Operand::Num(n)));
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
                NodeKind::Equality => assembly.push_str(&eq_arg()),
                NodeKind::Nonequality => assembly.push_str(&neq_arg()),
                NodeKind::Less => assembly.push_str(&less_arg()),
                NodeKind::LessOrEqual => assembly.push_str(&less_or_eq_arg()),
                NodeKind::Add => assembly.push_str(&add_arg()),
                NodeKind::Sub => assembly.push_str(&sub_arg()),
                NodeKind::Mul => assembly.push_str(&mul_arg()),
                NodeKind::Div => assembly.push_str(&div_arg()),
                _ => panic!("unexpected node"),
            }
            assembly.push_str(&push(Operand::Register(Register::R0)));
        }
    }
}
