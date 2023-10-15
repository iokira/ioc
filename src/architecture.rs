pub mod myarchitecture {
    use core::fmt;

    use crate::numtype::mynumtype::NumType;

    pub enum Register {
        /// rax, w0
        R0,
        /// rdi, w1
        R1,
        /// rsi, w2
        R2,
        /// rdx, w3
        R3,
        /// rcx, w4
        R4,
        /// rbp, w5
        R5,
        /// rsp, w6
        R6,
        /// rbx, w7
        R7,
        /// r8, w8
        R8,
        /// r9, w9
        R9,
        /// r10, w10
        R10,
        /// r11, w11
        R11,
        /// r12, w12
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
                Register::R0 => "w0",
                Register::R1 => "w1",
                Register::R2 => "w2",
                Register::R3 => "w3",
                Register::R4 => "w4",
                Register::R5 => "w5",
                Register::R6 => "w6",
                Register::R7 => "w7",
                Register::R8 => "w8",
                Register::R9 => "w9",
                Register::R10 => "w10",
                Register::R11 => "w11",
                Register::R12 => "w12",
                Register::R13 => "sp",
                Register::R14 => "lr",
                Register::R15 => "pc",
            };
            write!(f, "{}", name)
        }
    }

    impl fmt::Display for Operand {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name: String = match self {
                Operand::Register(r) => r.to_string(),
                Operand::Address(r) => format!("[{}]", r),
                Operand::Num(n) => n.to_string(),
            };
            write!(f, "{}", name)
        }
    }

    pub fn intel_syntax() -> String {
        ".intel_syntax noprefix\n".to_string()
    }

    #[cfg(target_arch = "x86_64")]
    pub fn main_func() -> String {
        ".globl main\nmain:\n".to_string()
    }

    #[cfg(target_arch = "aarch64")]
    pub fn main_func() -> String {
        ".globl _main\n_main:\n".to_string()
    }

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

    #[cfg(target_arch = "aarch64")]
    pub fn memory_allocate(bytes: usize) -> String {
        format!(
            "{}{}{}",
            push(Operand::Register(Register::R15)),
            mov(
                Operand::Register(Register::R15),
                Operand::Register(Register::R13)
            ),
            sub(
                Operand::Register(Register::R13),
                Operand::Register(Register::R13),
                Operand::Num(bytes)
            )
        )
    }

    #[cfg(target_arch = "x86_64")]
    pub fn stmt_epilogue() -> String {
        pop(Operand::Register(Register::R0))
    }

    #[cfg(target_arch = "aarch64")]
    pub fn stmt_epilogue() -> String {
        pop(Operand::Register(Register::R0))
    }

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

    #[cfg(target_arch = "aarch64")]
    pub fn program_epilogue() -> String {
        format!(
            "{}{}{}",
            mov(
                Operand::Register(Register::R13),
                Operand::Register(Register::R15)
            ),
            pop(Operand::Register(Register::R15)),
            ret()
        )
    }

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

    #[cfg(target_arch = "aarch64")]
    pub fn gen_val(offset: usize) -> String {
        format!(
            "{}{}{}",
            mov(
                Operand::Register(Register::R0),
                Operand::Register(Register::R15)
            ),
            sub(Operand::Register(Register::R0), Operand::Num(offset)),
            push(Operand::Register(Register::R0))
        )
    }

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

    #[cfg(target_arch = "aarch64")]
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

    #[cfg(target_arch = "x86_64")]
    pub fn push(rd: Operand) -> String {
        format!("\tpush {}\n", rd)
    }

    #[cfg(target_arch = "aarch64")]
    pub fn push(rd: Operand) -> String {
        unimplemented!()
    }

    #[cfg(target_arch = "x86_64")]
    fn pop(rd: Operand) -> String {
        format!("\tpop {}\n", rd)
    }

    #[cfg(target_arch = "aarch64")]
    fn pop(rd: Operand) -> String {
        unimplemented!()
    }

    #[cfg(target_arch = "x86_64")]
    pub fn add(rd: Operand, rn: Operand) -> String {
        format!("\tadd {}, {}\n", rd, rn) // rd <- rd + rn
    }

    #[cfg(target_arch = "aarch64")]
    pub fn add(rd: Operand, rn: Operand, src2: Operand) -> String {
        format!("\tadd {}, {}, {}\n", rd, rn, src2) // rd <- rn + src2
    }

    #[cfg(target_arch = "x86_64")]
    pub fn sub(rd: Operand, rn: Operand) -> String {
        format!("\tsub {}, {}\n", rd, rn) // rd <- rd - rn
    }

    #[cfg(target_arch = "aarch64")]
    pub fn sub(rd: Operand, rn: Operand, src2: Operand) -> String {
        format!("\tsub {}, {}, {}\n", rd, rn, src2) // rd <- rn - src2
    }

    #[cfg(target_arch = "x86_64")]
    pub fn mul(rd: Operand, rn: Operand) -> String {
        format!("\timul {}, {}\n", rd, rn) // rd <- rd x rn
    }

    #[cfg(target_arch = "aarch64")]
    pub fn mul(rd: Operand, rn: Operand, rm: Operand) -> String {
        format!("\tmul {}, {}, {}\n", rd, rn, rm) // rd <- rn x rm
    }

    #[cfg(target_arch = "x86_64")]
    pub fn div(rd: Operand, _rn: Operand) -> String {
        format!("\tcqo\n\tidiv {}\n", rd)
    }

    #[cfg(target_arch = "aarch64")]
    pub fn div(rd: Operand, rn: Operand, rm: Operand) -> String {
        format!("\tudiv {}, {}, {}\n", rd, rn, rm)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn mov(rd: Operand, src2: Operand) -> String {
        format!("\tmov {}, {}\n", rd, src2)
    }

    #[cfg(target_arch = "aarch64")]
    pub fn mov(rd: Operand, src2: Operand) -> String {
        format!("\tmov {}, {}\n", rd, src2)
    }

    fn ret() -> String {
        "\tret\n".to_string()
    }
}
