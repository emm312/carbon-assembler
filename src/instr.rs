#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CarbonInstrVariants {
    Hlt,
    Add,
    Sub,
    Bsb,
    Or,
    Nor,
    And,
    Nand,
    Xor,
    Lia,
    Ldi,
    Adr,
    Rld,
    Rst,
    Mst,
    Mld,
    Ics,
    Jid,
    Brc,
    Cmp,
    Bsr,
    Bsl,
    Pst,
    Pld,
    Inc,
    Dec,
    Nop,
}

#[derive(PartialEq, Debug, Clone)]
pub enum CarbonConds {
    EVEN = 0,
    JMP,
    EQ,
    NEQ,
    LT,
    GT,
    GTEQ,
    LTEQ,
}

#[derive(PartialEq, Debug, Clone)]
pub enum CarbonOperand {
    Cond(CarbonConds),
    Reg(u8),
    JmpAddr(JmpAddr),
    Label(String)
}

#[derive(PartialEq, Debug, Clone)]
pub enum JmpAddr {
    Literal(u8),
    Label(String),
}

impl JmpAddr {
    pub fn unwrap(&self) -> u8 {
        match self {
            JmpAddr::Literal(a) => *a << 3,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CarbonInstr {
    pub opcode: CarbonInstrVariants,
    pub operand: Option<Vec<CarbonOperand>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CarbonASMProgram {
    Instruction(CarbonInstr),
    Immediate(u8),
    Comment(String),
    Label(String),
    PageLabel(usize),
    LabelDeref(String),
}
