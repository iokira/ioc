pub mod myarchitecture {
    use core::fmt;

    use crate::numtype::mynumtype::NumType;

    pub enum Register {
        /// rax, w0
        R0,
        // rdi, w1
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
        /// Immidiate operand
        Num(NumType),
    }

    impl fmt::Display for Register {
        #[cfg(target_arch = "x86_64")]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name: String = match self {
                Register::R0 => "rax".to_string(),
                Register::R1 => "rdi".to_string(),
                Register::R2 => "rsi".to_string(),
                Register::R3 => "rdx".to_string(),
                Register::R4 => "rcx".to_string(),
                Register::R5 => "rbp".to_string(),
                Register::R6 => "rsp".to_string(),
                Register::R7 => "rbx".to_string(),
                Register::R8 => "r8".to_string(),
                Register::R9 => "r9".to_string(),
                Register::R10 => "r10".to_string(),
                Register::R11 => "r11".to_string(),
                Register::R12 => "r12".to_string(),
                Register::R13 => "r13".to_string(),
                Register::R14 => "r14".to_string(),
                Register::R15 => "r15".to_string(),
                Register::Num(n) => n.to_string(),
            };
            write!(f, "{}", name)
        }

        #[cfg(target_arch = "aarch64")]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name: String = match self {
                Register::R0 => "w0".to_string(),
                Register::R1 => "w1".to_string(),
                Register::R2 => "w2".to_string(),
                Register::R3 => "w3".to_string(),
                Register::R4 => "w4".to_string(),
                Register::R5 => "w5".to_string(),
                Register::R6 => "w6".to_string(),
                Register::R7 => "w7".to_string(),
                Register::R8 => "w8".to_string(),
                Register::R9 => "w9".to_string(),
                Register::R10 => "w10".to_string(),
                Register::R11 => "w11".to_string(),
                Register::R12 => "w12".to_string(),
                Register::R13 => "sp".to_string(),
                Register::R14 => "lr".to_string(),
                Register::R15 => "pc".to_string(),
                Register::Num(n) => n.to_string(),
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
            push(Register::R5),
            mov(Register::R5, Register::R6),
            sub(Register::R6, Register::Num(bytes))
        )
    }

    #[cfg(target_arch = "aarch64")]
    pub fn memory_allocate(bytes: usize) -> String {
        format!(
            "{}{}{}",
            push(Register::R15),
            mov(Register::R15, Register::R13),
            sub(Register::R13, Register::R13, Register::Num(bytes))
        )
    }

    #[cfg(target_arch = "x86_64")]
    pub fn stmt_epilogue() -> String {
        pop(Register::R0)
    }

    #[cfg(target_arch = "aarch64")]
    pub fn stmt_epilogue() -> String {
        pop(Register::R0)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn program_epilogue() -> String {
        format!(
            "{}{}{}",
            mov(Register::R6, Register::R5),
            pop(Register::R5),
            ret()
        )
    }

    #[cfg(target_arch = "aarch64")]
    pub fn program_epilogue() -> String {
        format!(
            "{}{}{}",
            mov(Register::R13, Register::R15),
            pop(Register::R15),
            ret()
        )
    }

    #[cfg(target_arch = "x86_64")]
    pub fn push(rd: Register) -> String {
        format!("\tpush {}\n", rd)
    }

    #[cfg(target_arch = "aarch64")]
    fn push(rd: Register) -> String {
        unimplemented!()
    }

    #[cfg(target_arch = "x86_64")]
    fn pop(rd: Register) -> String {
        format!("\tpop {}\n", rd)
    }

    #[cfg(target_arch = "aarch64")]
    fn pop(rd: Register) -> String {
        unimplemented!()
    }

    #[cfg(target_arch = "x86_64")]
    fn add(rd: Register, rn: Register) -> String {
        format!("\tadd {}, {}\n", rd, rn) // rd <- rd + rn
    }

    #[cfg(target_arch = "aarch64")]
    pub fn add(rd: Register, rn: Register, src2: Register) -> String {
        format!("\tadd {}, {}, {}\n", rd, rn, src2) // rd <- rn + src2
    }

    #[cfg(target_arch = "x86_64")]
    pub fn sub(rd: Register, rn: Register) -> String {
        format!("\tsub {}, {}\n", rd, rn) // rd <- rd - rn
    }

    #[cfg(target_arch = "aarch64")]
    pub fn sub(rd: Register, rn: Register, src2: Register) -> String {
        format!("\tsub {}, {}, {}\n", rd, rn, src2) // rd <- rn - src2
    }

    #[cfg(target_arch = "x86_64")]
    pub fn mul(rd: Register, rn: Register) -> String {
        format!("\timul {}, {}\n", rd, rn) // rd <- rd x rn
    }

    #[cfg(target_arch = "aarch64")]
    pub fn mul(rd: Register, rn: Register, rm: Register) -> String {
        format!("\tmul {}, {}, {}\n", rd, rn, rm) // rd <- rn x rm
    }

    #[cfg(target_arch = "x86_64")]
    pub fn div(rd: Register, _rn: Register) -> String {
        format!("\tcqo\n\tidiv {}\n", rd)
    }

    #[cfg(target_arch = "aarch64")]
    pub fn div(rd: Register, rn: Register, rm: Register) -> String {
        format!("\tudiv {}, {}, {}\n", rd, rn, rm)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn mov(rd: Register, src2: Register) -> String {
        format!("\tmov {}, {}\n", rd, src2)
    }

    #[cfg(target_arch = "aarch64")]
    pub fn mov(rd: Register, src2: Register) -> String {
        format!("\tmov {}, {}\n", rd, src2)
    }

    fn ret() -> String {
        "\tret\n".to_string()
    }
}
