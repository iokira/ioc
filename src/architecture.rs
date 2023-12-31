pub mod myarchitecture {
    use core::fmt;

    use crate::numtype::mynumtype::NumType;

    pub enum Register {
        /// rax, x0
        R0,
        /// rdi, x1
        R1,
        /// rsi, x2
        R2,
        /// rdx, x3
        R3,
        /// rcx, x4
        R4,
        /// rbp, x5
        R5,
        /// rsp, x6
        R6,
        /// rbx, x7
        R7,
        /// r8, x8(rbp)
        R8,
        /// r9, x9(tsp)
        R9,
        /// r10, x10
        R10,
        /// r11, x11
        R11,
        /// r12, x12
        R12,
        /// r13, sp
        R13,
        /// r14, lr
        R14,
        /// r15, pc
        R15,
    }

    pub enum Operand {
        Num(NumType),
        Register(Register),
        Address(Register),
    }

    impl fmt::Display for Register {
        #[cfg(target_arch = "x86_64")]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match self {
                Register::R0 => "rax",
                Register::R1 => "rdi",
                Register::R2 => "rsi",
                Register::R3 => "rdx",
                Register::R4 => "rcx",
                Register::R5 => "rbp",
                Register::R6 => "rsp",
                Register::R7 => "rbx",
                Register::R8 => "r8",
                Register::R9 => "r9",
                Register::R10 => "r10",
                Register::R11 => "r11",
                Register::R12 => "r12",
                Register::R13 => "r13",
                Register::R14 => "r14",
                Register::R15 => "r15",
            };
            write!(f, "{}", name)
        }

        #[cfg(target_arch = "aarch64")]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match self {
                Register::R0 => "x0",
                Register::R1 => "x1",
                Register::R2 => "x2",
                Register::R3 => "x3",
                Register::R4 => "x4",
                Register::R5 => "x5",
                Register::R6 => "x6",
                Register::R7 => "x7",
                Register::R8 => "x8",
                Register::R9 => "x9",
                Register::R10 => "x10",
                Register::R11 => "x11",
                Register::R12 => "x12",
                Register::R13 => "sp",
                Register::R14 => "lr",
                Register::R15 => "pc",
            };
            write!(f, "{}", name)
        }
    }

    impl fmt::Display for Operand {
        #[cfg(target_arch = "x86_64")]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name: String = match self {
                Operand::Register(r) => r.to_string(),
                Operand::Address(r) => format!("[{}]", r),
                Operand::Num(n) => n.to_string(),
            };
            write!(f, "{}", name)
        }

        #[cfg(target_arch = "aarch64")]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name: String = match self {
                Operand::Register(r) => r.to_string(),
                Operand::Address(r) => format!("[{}]", r),
                Operand::Num(n) => format!("#{}", n),
            };
            write!(f, "{}", name)
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn program_prologue() -> String {
        ".intel_syntax noprefix\n".to_string()
    }

    #[cfg(target_arch = "aarch64")]
    pub fn program_prologue() -> String {
        ".text\n".to_string()
    }

    #[cfg(target_arch = "x86_64")]
    pub fn main_func() -> String {
        ".globl main\nmain:\n".to_string()
    }

    #[cfg(target_arch = "aarch64")]
    pub fn main_func() -> String {
        ".globl _main\n_main:\n".to_string()
    }

    /// push rbp
    /// mov rbp, rsp
    /// sub rsp, #bytes
    #[cfg(target_arch = "x86_64")]
    pub fn memory_allocate(bytes: usize) -> String {
        format!(
            "{}{}{}",
            push(Operand::Register(Register::R5)),
            mov(
                Operand::Register(Register::R5),
                Operand::Register(Register::R6)
            ),
            sub(Operand::Register(Register::R6), Operand::Num(bytes))
        )
    }

    /// mov rbp(r8), sp(r13)
    /// mov rsp(r9), sp(r13)
    /// push rbp(r8)
    /// mov rbp(r8), rsp(r9)
    /// sub rsp(r9), $bytes
    #[cfg(target_arch = "aarch64")]
    pub fn memory_allocate(bytes: usize) -> String {
        format!(
            "{}{}{}{}{}",
            mov(
                Operand::Register(Register::R8),
                Operand::Register(Register::R13)
            ),
            mov(
                Operand::Register(Register::R9),
                Operand::Register(Register::R13)
            ),
            push(Operand::Register(Register::R8)),
            mov(
                Operand::Register(Register::R8),
                Operand::Register(Register::R9)
            ),
            sub(Operand::Register(Register::R9), Operand::Num(bytes))
        )
    }

    #[cfg(target_arch = "aarch64")]
    fn ldr(rd: Operand, rn: Operand) -> String {
        format!("\tldr {}, {}\n", rd, rn)
    }

    #[cfg(target_arch = "aarch64")]
    fn str(rd: Operand, rn: Operand) -> String {
        format!("\tstr {}, {}\n", rd, rn)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn stmt_epilogue() -> String {
        pop(Operand::Register(Register::R0))
    }

    #[cfg(target_arch = "aarch64")]
    pub fn stmt_epilogue() -> String {
        pop(Operand::Register(Register::R0))
    }

    /// mov rsp, rbp
    /// pop rbp
    /// ret
    #[cfg(target_arch = "x86_64")]
    pub fn program_epilogue() -> String {
        format!(
            "{}{}{}",
            mov(
                Operand::Register(Register::R6),
                Operand::Register(Register::R5)
            ),
            pop(Operand::Register(Register::R5)),
            ret()
        )
    }

    /// mov rsp(r9), rbp(r8)
    /// pop rbp(r8)
    /// ret
    #[cfg(target_arch = "aarch64")]
    pub fn program_epilogue() -> String {
        format!(
            "{}{}{}",
            mov(
                Operand::Register(Register::R9),
                Operand::Register(Register::R8)
            ),
            pop(Operand::Register(Register::R8)),
            ret()
        )
    }

    /// mov rax, rbp
    /// sub rax, offset
    /// push rax
    #[cfg(target_arch = "x86_64")]
    pub fn gen_val(offset: usize) -> String {
        format!(
            "{}{}{}",
            mov(
                Operand::Register(Register::R0),
                Operand::Register(Register::R5)
            ),
            sub(Operand::Register(Register::R0), Operand::Num(offset)),
            push(Operand::Register(Register::R0))
        )
    }

    /// mov r0, rbp(r8)
    /// sub rax, offset
    /// push rax
    #[cfg(target_arch = "aarch64")]
    pub fn gen_val(offset: usize) -> String {
        format!(
            "{}{}{}",
            mov(
                Operand::Register(Register::R0),
                Operand::Register(Register::R8)
            ),
            sub(Operand::Register(Register::R0), Operand::Num(offset)),
            push(Operand::Register(Register::R0))
        )
    }

    /// pop r0
    /// mov r0, [r0]
    /// push r0
    #[cfg(target_arch = "x86_64")]
    pub fn pop_val() -> String {
        format!(
            "{}{}{}",
            pop(Operand::Register(Register::R0)),
            mov(
                Operand::Register(Register::R0),
                Operand::Address(Register::R0)
            ),
            push(Operand::Register(Register::R0))
        )
    }

    /// pop r0
    /// mov r0, [r0]
    /// push r0
    #[cfg(target_arch = "aarch64")]
    pub fn pop_val() -> String {
        format!(
            "{}{}{}",
            pop(Operand::Register(Register::R0)),
            ldr(
                Operand::Register(Register::R0),
                Operand::Address(Register::R0)
            ),
            push(Operand::Register(Register::R0))
        )
    }

    /// pop r1
    /// pop r0
    /// mov [r0], r1
    /// push r1
    #[cfg(target_arch = "x86_64")]
    pub fn pop_lvar() -> String {
        format!(
            "{}{}{}{}",
            pop(Operand::Register(Register::R1)),
            pop(Operand::Register(Register::R0)),
            mov(
                Operand::Address(Register::R0),
                Operand::Register(Register::R1)
            ),
            push(Operand::Register(Register::R1))
        )
    }

    /// pop r1
    /// pop r0
    /// mov [r0], r1
    /// push r1
    #[cfg(target_arch = "aarch64")]
    pub fn pop_lvar() -> String {
        format!(
            "{}{}{}{}",
            pop(Operand::Register(Register::R1)),
            pop(Operand::Register(Register::R0)),
            str(
                Operand::Register(Register::R1),
                Operand::Address(Register::R0)
            ),
            push(Operand::Register(Register::R1))
        )
    }

    /// pop r1
    /// pop r0
    pub fn pop_arg() -> String {
        format!(
            "{}{}",
            pop(Operand::Register(Register::R1)),
            pop(Operand::Register(Register::R0))
        )
    }

    /// push rd
    #[cfg(target_arch = "x86_64")]
    pub fn push(rd: Operand) -> String {
        format!("\tpush {}\n", rd)
    }

    /// sub r9, #8
    /// mov r0, rd
    /// str r0, r9
    #[cfg(target_arch = "aarch64")]
    pub fn push(rd: Operand) -> String {
        format!(
            "{}{}{}",
            sub(Operand::Register(Register::R9), Operand::Num(8)),
            mov(Operand::Register(Register::R1), rd),
            str(
                Operand::Register(Register::R1),
                Operand::Address(Register::R9)
            )
        )
    }

    /// pop rd
    #[cfg(target_arch = "x86_64")]
    fn pop(rd: Operand) -> String {
        format!("\tpop {}\n", rd)
    }

    /// ldr rd, r9
    /// add r9, #8
    #[cfg(target_arch = "aarch64")]
    pub fn pop(rd: Operand) -> String {
        format!(
            "{}{}",
            ldr(rd, Operand::Address(Register::R9)),
            add(Operand::Register(Register::R9), Operand::Num(8))
        )
    }

    pub fn add_arg() -> String {
        add(
            Operand::Register(Register::R0),
            Operand::Register(Register::R1),
        )
    }

    #[cfg(target_arch = "x86_64")]
    fn add(rd: Operand, rn: Operand) -> String {
        format!("\tadd {}, {}\n", rd, rn) // rd <- rd + rn
    }

    #[cfg(target_arch = "aarch64")]
    fn add(rd: Operand, rn: Operand) -> String {
        format!("\tadd {}, {}, {}\n", rd, rd, rn) // rd <- rn + src2
    }

    pub fn sub_arg() -> String {
        sub(
            Operand::Register(Register::R0),
            Operand::Register(Register::R1),
        )
    }

    #[cfg(target_arch = "x86_64")]
    fn sub(rd: Operand, rn: Operand) -> String {
        format!("\tsub {}, {}\n", rd, rn) // rd <- rd - rn
    }

    #[cfg(target_arch = "aarch64")]
    fn sub(rd: Operand, rn: Operand) -> String {
        format!("\tsub {}, {}, {}\n", rd, rd, rn) // rd <- rn - src2
    }

    pub fn mul_arg() -> String {
        mul(
            Operand::Register(Register::R0),
            Operand::Register(Register::R1),
        )
    }

    #[cfg(target_arch = "x86_64")]
    fn mul(rd: Operand, rn: Operand) -> String {
        format!("\timul {}, {}\n", rd, rn) // rd <- rd x rn
    }

    #[cfg(target_arch = "aarch64")]
    fn mul(rd: Operand, rn: Operand) -> String {
        format!("\tmul {}, {}, {}\n", rd, rd, rn) // rd <- rn x rm
    }

    pub fn div_arg() -> String {
        div(
            Operand::Register(Register::R0),
            Operand::Register(Register::R1),
        )
    }

    #[cfg(target_arch = "x86_64")]
    fn div(_: Operand, rn: Operand) -> String {
        format!("\tcqo\n\tidiv {}\n", rn)
    }

    #[cfg(target_arch = "aarch64")]
    fn div(rd: Operand, rn: Operand) -> String {
        format!("\tudiv {}, {}, {}\n", rd, rd, rn)
    }

    #[cfg(target_arch = "x86_64")]
    fn mov(rd: Operand, src2: Operand) -> String {
        format!("\tmov {}, {}\n", rd, src2)
    }

    #[cfg(target_arch = "aarch64")]
    fn mov(rd: Operand, src2: Operand) -> String {
        format!("\tmov {}, {}\n", rd, src2)
    }

    pub fn eq_arg() -> String {
        eq(
            Operand::Register(Register::R0),
            Operand::Register(Register::R1),
        )
    }

    #[cfg(target_arch = "x86_64")]
    fn eq(rd: Operand, rn: Operand) -> String {
        format!("\tcmp {}, {}\n\tsete al\n\tmovzb {}, al\n", rd, rn, rd)
    }

    #[cfg(target_arch = "aarch64")]
    fn eq(rd: Operand, rn: Operand) -> String {
        format!("\tcmp {}, {}\n\tcset {}, EQ\n", rd, rn, rd)
    }

    pub fn neq_arg() -> String {
        neq(
            Operand::Register(Register::R0),
            Operand::Register(Register::R1),
        )
    }

    #[cfg(target_arch = "x86_64")]
    fn neq(rd: Operand, rn: Operand) -> String {
        format!("\tcmp {}, {}\n\tsetne al\n\tmovzb {}, al\n", rd, rn, rd)
    }

    #[cfg(target_arch = "aarch64")]
    fn neq(rd: Operand, rn: Operand) -> String {
        format!("\tcmp {}, {}\n\tcset {}, NE\n", rd, rn, rd)
    }

    pub fn less_arg() -> String {
        less(
            Operand::Register(Register::R0),
            Operand::Register(Register::R1),
        )
    }

    #[cfg(target_arch = "x86_64")]
    fn less(rd: Operand, rn: Operand) -> String {
        format!("\tcmp {}, {}\n\tsetl al\n\tmovzb {}, al\n", rd, rn, rd)
    }

    #[cfg(target_arch = "aarch64")]
    fn less(rd: Operand, rn: Operand) -> String {
        format!("\tcmp {}, {}\n\tcset {}, LT\n", rd, rn, rd)
    }

    pub fn less_or_eq_arg() -> String {
        less_or_eq(
            Operand::Register(Register::R0),
            Operand::Register(Register::R1),
        )
    }

    #[cfg(target_arch = "x86_64")]
    fn less_or_eq(rd: Operand, rn: Operand) -> String {
        format!("\tcmp {}, {}\n\tsetle al\n\tmovzb {}, al\n", rd, rn, rd)
    }

    #[cfg(target_arch = "aarch64")]
    fn less_or_eq(rd: Operand, rn: Operand) -> String {
        format!("\tcmp {}, {}\n\tcset {}, LS\n", rd, rn, rd)
    }

    fn ret() -> String {
        "\tret\n".to_string()
    }
}
