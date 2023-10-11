pub mod architecture {
    use core::fmt;

    pub enum Register {
        R0,
        R1,
        R2,
        R3,
        R4,
        R5,
        R6,
        R7,
        R8,
        R9,
        R10,
        R11,
        R12,
        R13,
        R14,
        R15,
    }

    impl fmt::Display for Register {
        #[cfg(target_arch = "x86_64")]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match self {
                Register::R0 => "r0",
                Register::R1 => "r1",
                Register::R2 => "r2",
                Register::R3 => "r3",
                Register::R4 => "r4",
                Register::R5 => "r5",
                Register::R6 => "r6",
                Register::R7 => "r7",
                Register::R8 => "r8",
                Register::R9 => "r9",
                Register::R10 => "r10",
                Register::R11 => "r11",
                Register::R12 => "r12",
                Register::R13 => "sp",
                Register::R14 => "lr",
                Register::R15 => "pc",
            };
            write!(f, "{}", name)
        }

        #[cfg(target_arch = "aarch64")]
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
    fn push(rd: &str) -> String {
        format!("\tpush {}\n", rd)
    }

    #[cfg(target_arch = "aarch64")]
    fn push() -> String {
        unimplemented!()
    }

    #[cfg(target_arch = "x86_64")]
    fn pop(rd: &str) -> String {
        format!("\tpop {}\n", rd)
    }

    #[cfg(target_arch = "aarch64")]
    fn pop() -> String {
        unimplemented!()
    }

    #[cfg(target_arch = "x86_64")]
    fn add(rd: &str, rn: &str, src2: &str) -> String {
        format!("\tadd {}, {}\n", rd, rn) // rd <- rd + rn
    }

    #[cfg(target_arch = "aarch64")]
    pub fn add(rd: &str, rn: &str, src2: &str) -> String {
        format!("\tadd {}, {}, {}\n", rd, rn, src2) // rd <- rn + src2
    }

    #[cfg(target_arch = "x86_64")]
    pub fn sub(rd: &str, rn: &str, src2: &str) -> String {
        format!("\tsub {}, {}\n", rd, rn) // rd <- rd - rn
    }

    #[cfg(target_arch = "aarch64")]
    pub fn sub(rd: &str, rn: &str, src2: &str) -> String {
        format!("\tsub {}, {}, {}\n", rd, rn, src2) // rd <- rn - src2
    }

    #[cfg(target_arch = "x86_64")]
    pub fn mul(rd: &str, rn: &str, rm: &str) -> String {
        format!("\timul {}, {}\n", rd, rn) // rd <- rd x rn
    }

    #[cfg(target_arch = "aarch64")]
    pub fn mul(rd: &str, rn: &str, rm: &str) -> String {
        format!("\tmul {}, {}, {}\n", rd, rn, rm) // rd <- rn x rm
    }

    #[cfg(target_arch = "x86_64")]
    pub fn div(rd: &str, rn: &str, rm: &str) -> String {
        format!("\tcqo\n\tidiv {}", rd)
    }

    #[cfg(target_arch = "aarch64")]
    pub fn div(rd: &str, rn: &str, rm: &str) -> String {
        format!("\tudiv {}, {}, {}", rd, rn, rm)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn mov(rd: &str, src2: &str) -> String {
        format!("\tmov {}, {}", rd, src2)
    }

    #[cfg(target_arch = "aarch64")]
    pub fn mov(rd: &str, src2: &str) -> String {
        format!("\tmov {}, {}", rd, src2)
    }

    pub fn ret() -> String {
        "\tret\n".to_string()
    }
}
