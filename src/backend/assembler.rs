use std::io::Write;

use crate::instr::{CarbonASMProgram, CarbonConds, CarbonInstrVariants, CarbonOperand};

pub fn assemble(file: &mut impl Write, ast: Vec<CarbonASMProgram>) {
    for node in ast {
        match node {
            CarbonASMProgram::Immediate(i) => {
                write!(file, "{:0>8b}", i).unwrap();
            }
            CarbonASMProgram::Instruction(i) => {
                match i.opcode {
                    CarbonInstrVariants::Hlt =>  write!(file, "11111000").unwrap(),
                    CarbonInstrVariants::Add =>  write!(file, "00001").unwrap(),
                    CarbonInstrVariants::Sub =>  write!(file, "00010").unwrap(),
                    CarbonInstrVariants::Bsb =>  write!(file, "00011").unwrap(),
                    CarbonInstrVariants::Or =>   write!(file, "00100").unwrap(),
                    CarbonInstrVariants::Nor =>  write!(file, "00101").unwrap(),
                    CarbonInstrVariants::And =>  write!(file, "00110").unwrap(),
                    CarbonInstrVariants::Nand => write!(file, "00111").unwrap(),
                    CarbonInstrVariants::Xor =>  write!(file, "01000").unwrap(),
                    CarbonInstrVariants::Xnr =>  write!(file, "01001").unwrap(),
                    CarbonInstrVariants::Ldi =>  write!(file, "01010").unwrap(),
                    CarbonInstrVariants::Adr =>  write!(file, "01011").unwrap(),
                    CarbonInstrVariants::Rld =>  write!(file, "01100").unwrap(),
                    CarbonInstrVariants::Rst =>  write!(file, "01101").unwrap(),
                    CarbonInstrVariants::Mst =>  write!(file, "01110").unwrap(),
                    CarbonInstrVariants::Mld =>  write!(file, "01111").unwrap(),
                    CarbonInstrVariants::Ics =>  write!(file, "10000000").unwrap(),
                    CarbonInstrVariants::Jid =>  write!(file, "10001").unwrap(),
                    CarbonInstrVariants::Brc =>  write!(file, "10010").unwrap(),
                    CarbonInstrVariants::Jmp =>  write!(file, "10011000").unwrap(),
                    CarbonInstrVariants::Cmp =>  write!(file, "10100").unwrap(),
                    CarbonInstrVariants::Bsr =>  write!(file, "10101").unwrap(),
                    CarbonInstrVariants::Bsl =>  write!(file, "10110").unwrap(),
                    CarbonInstrVariants::Pst =>  write!(file, "10111").unwrap(),
                    CarbonInstrVariants::Pld =>  write!(file, "11000").unwrap()
                }
                match i.operand {
                    Some(v) => {
                        match v {
                            CarbonOperand::Cond(c) => write_cond(file, c),
                            CarbonOperand::Reg(r) => write!(file, "{:0>3b}", r).unwrap(),
                        }
                    },
                    None => (),
                }
            }
        }
        write!(file, "\n").unwrap();
    }
}

fn write_cond(file: &mut impl Write, cond: CarbonConds) {
    match cond {
        CarbonConds::ZR => {
            write!(file, "001").unwrap();
        }
        CarbonConds::NZR => {
            write!(file, "010").unwrap();
        }
        CarbonConds::MSB => {
            write!(file, "011").unwrap();
        }
        CarbonConds::NMSB => {
            write!(file, "100").unwrap();
        }
        CarbonConds::COUT => {
            write!(file, "101").unwrap();
        }
        CarbonConds::NCOUT => {
            write!(file, "110").unwrap();
        }
        CarbonConds::UCD => {
            write!(file, "111").unwrap();
        }
    }
}
