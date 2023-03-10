use std::{process::exit};

use crate::instr::{self, CarbonASMProgram, CarbonInstrVariants, CarbonInstr, CarbonOperand};

use super::lexer::Token;

fn tok_compare(a: Token, b: Token) -> bool {
    std::mem::discriminant(&a) == std::mem::discriminant(&b)
}


fn buf_consume(buf: &mut TokenBuffer, toks: &[Token], err: &str) -> Token {
    for tok in toks {
        if tok_compare(buf.current(), tok.clone()) {
            return buf.current();
        }
    }
    println!("{}", err);
    exit(-1);
}

struct TokenBuffer {
    toks: Vec<Token>,
    pos: usize
}

impl TokenBuffer {
    pub fn new(toks: Vec<Token>) -> Self {
        Self { toks, pos: 0 }
    }
    pub fn has_next(&mut self) -> bool {
        self.pos < self.toks.len()
    }

    pub fn current(&mut self) -> Token {
        self.toks[self.pos].clone()
    }

    pub fn advance(&mut self) {
        if self.has_next() {
            self.pos += 1;
        } else {
            panic!("Parser called advance without having an extra element on token {:#?}", self.toks[self.pos])
        }
    }
}

pub fn parse(toks: Vec<Token>) -> Vec<CarbonASMProgram> {
    let mut ret = Vec::new();
    let mut buf = TokenBuffer::new(toks);
    while buf.has_next() {
        match buf.current() {
            Token::Immediate(val) => {
                ret.push(CarbonASMProgram::Immediate(val));
            },
            Token::Instr(val) => {
                if val == CarbonInstrVariants::Hlt || val == CarbonInstrVariants::Ics || val == CarbonInstrVariants::Jmp {
                    ret.push(
                        CarbonASMProgram::Instruction(CarbonInstr {
                            opcode: val,
                            operand: None
                        })
                    )
                } else {
                    buf.advance();
                    let tok = buf_consume(&mut buf, &[Token::Register(0), Token::Cond(instr::CarbonConds::COUT)], "expected operand");
                    let mut instr = CarbonInstr { opcode: val, operand: None };
                    match tok {
                        Token::Cond(c) => instr.operand = Some(CarbonOperand::Cond(c)),
                        Token::Register(r) => instr.operand = Some(CarbonOperand::Reg(r)),
                        _ => unreachable!("")
                    }
                    ret.push(
                        CarbonASMProgram::Instruction(instr)
                    );
                }
            }
            _ => todo!("{:#?}", buf.current())
        }
        if buf.has_next() {
            buf.advance()
        }
    }
    ret
}