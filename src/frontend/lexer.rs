use logos::{Lexer, Logos};
use std::process::exit;

use crate::instr::*;

pub fn register(lex: &mut Lexer<Token>) -> Option<u8> {
    let mut slice = lex.slice();
    slice = &slice.trim();
    let mut tmp = slice.to_string();
    tmp.remove(0);
    slice = tmp.as_str();
    Some(
        slice
            .parse::<u8>()
            .expect(&format!("Invalid reg: {}", slice)),
    )
}

pub fn immediate(lex: &mut Lexer<Token>) -> Option<u8> {
    let slice = lex.slice();
    Some(slice.parse::<u8>().unwrap())
}

pub fn cond(lex: &mut Lexer<Token>) -> Option<CarbonConds> {
    let slice = lex.slice();
    match slice {
        "EQ" => Some(CarbonConds::NZR),
        "NEQ" => Some(CarbonConds::MSB),
        "LT" => Some(CarbonConds::NMSB),
        "GTEQ" => Some(CarbonConds::COUT),
        "LTEQ" => Some(CarbonConds::NCOUT),
        "GT" => Some(CarbonConds::UCD),
        "JMP" => Some(CarbonConds::ZR),
        _ => unreachable!(),
    }
}

pub fn instr(lex: &mut Lexer<Token>) -> Option<CarbonInstrVariants> {
    let slice = lex.slice();
    Some(match slice.to_uppercase().as_str() {
        "HLT" => CarbonInstrVariants::Hlt,
        "ADD" => CarbonInstrVariants::Add,
        "SUB" => CarbonInstrVariants::Sub,
        "BSUB" => CarbonInstrVariants::Bsb,
        "OR" => CarbonInstrVariants::Or,
        "NOR" => CarbonInstrVariants::Nor,
        "AND" => CarbonInstrVariants::And,
        "NAND" => CarbonInstrVariants::Nand,
        "XOR" => CarbonInstrVariants::Xor,
        "XNR" => CarbonInstrVariants::Xnr,
        "LDI" => CarbonInstrVariants::Ldi,
        "ADR" => CarbonInstrVariants::Adr,
        "RLD" => CarbonInstrVariants::Rld,
        "RST" => CarbonInstrVariants::Rst,
        "MST" => CarbonInstrVariants::Mst,
        "MLD" => CarbonInstrVariants::Mld,
        "ICS" => CarbonInstrVariants::Ics,
        "JID" => CarbonInstrVariants::Jid,
        "BRC" => CarbonInstrVariants::Brc,
        "CMP" => CarbonInstrVariants::Cmp,
        "BSR" => CarbonInstrVariants::Bsr,
        "BSL" => CarbonInstrVariants::Bsl,
        "PST" => CarbonInstrVariants::Pst,
        "PLD" => CarbonInstrVariants::Pld,
        "INC" => CarbonInstrVariants::Inc,
        "DEC" => CarbonInstrVariants::Dec,
        _ => {
            println!("Invalid instruction: {}", slice);
            exit(-1)
        }
    })
}

#[derive(Debug, PartialEq, Logos, Clone)]
pub enum Token {
    #[error]
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    Error,

    #[regex("JMP|EQ|NEQ|LT|GTEQ|LTEQ|GT", cond, priority = 1)]
    Cond(CarbonConds),

    #[regex("\\$[0-9]+", register, priority = 4)]
    #[regex("(R|r)[0-7]", register, priority = 4)]
    Register(u8),

    #[regex("[0-9]+", immediate, priority = 1)]
    Immediate(u8),

    #[regex("\\w+", instr, priority = 0)]
    Instr(CarbonInstrVariants),

    #[regex("(#|//).*", |lexer| lexer.slice().to_string())]
    Comment(String),

    #[regex(r"\..[^\s]*", |lexer| { let mut s = lexer.slice().to_string(); s.remove(0); s })]
    Label(String),

    #[regex(r"\[\w*\]", |lexer| lexer.slice()[1..lexer.slice().len() - 1].to_string())]
    LabelDeref(String),

    #[regex(r">.[^\s]*", |lexer| lexer.slice()[1..].parse::<usize>().expect("invalid int on pageno"))]
    PageLabel(usize),
}

pub fn tokenise(src: &str) -> Vec<Token> {
    let mut lexer = Token::lexer(src);
    let mut ret = Vec::new();
    'l: loop {
        let cur_tok = lexer.next();
        match cur_tok {
            Some(tok) => ret.push(tok),
            None => break 'l,
        }
    }
    ret
}
