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
    Xnr,
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
}

#[derive(PartialEq, Debug, Clone)]
pub enum CarbonConds {
    ZR,
    NZR,
    MSB,
    NMSB,
    COUT,
    NCOUT,
    UCD,
}

#[derive(PartialEq, Debug, Clone)]
pub enum CarbonOperand {
    Cond(CarbonConds),
    Reg(u8),
    JmpAddr(u8),
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
}
