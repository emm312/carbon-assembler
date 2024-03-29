use std::{collections::HashMap, process::exit};

use crate::instr::{
    self, CarbonASMProgram, CarbonInstr, CarbonInstrVariants, CarbonOperand, JmpAddr,
};

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
    pos: usize,
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
            panic!(
                "Parser called advance without having an extra element on token {:#?}",
                self.toks[self.pos]
            )
        }
    }

    pub fn advance_over_skips(&mut self, ret: &mut Vec<CarbonASMProgram>) {
        while tok_compare(self.current(), Token::Comment(String::new())) {
            if let Token::Comment(c) = self.current() {
                ret.push(CarbonASMProgram::Comment(c));
            }
            self.advance();
        }
    }

    pub fn get_labels(&mut self) -> Vec<CarbonOperand> {
        let mut ret = Vec::new();
        while tok_compare(self.current(), Token::Label(String::new())) {
            match self.current() {
                Token::Label(l) => ret.push(CarbonOperand::Label(l.to_string())),
                _ => unreachable!(),
            }
            self.advance();
        }
        ret
    }
}

pub fn parse(toks: Vec<Token>) -> Vec<CarbonASMProgram> {
    let mut ret = Vec::new();
    let mut buf = TokenBuffer::new(toks);
    while buf.has_next() {
        match buf.current() {
            Token::Immediate(val) => {
                ret.push(CarbonASMProgram::Immediate(val));
            }
            Token::Instr(val) => {
                if val == CarbonInstrVariants::Hlt || val == CarbonInstrVariants::Nop {
                    ret.push(CarbonASMProgram::Instruction(CarbonInstr {
                        opcode: val,
                        operand: None,
                    }))
                } else if val == CarbonInstrVariants::Ics {
                    buf.advance();
                    let cond = match buf_consume(
                        &mut buf,
                        &[Token::Cond(instr::CarbonConds::Jmp)],
                        "Expected cond after ics",
                    ) {
                        Token::Cond(c) => c,
                        _ => unreachable!(),
                    };
                    buf.advance();
                    buf.advance_over_skips(&mut ret);
                    let mut labels = buf.get_labels();

                    labels.append(&mut vec![
                        instr::CarbonOperand::Cond(cond),
                        CarbonOperand::JmpAddr(
                            match buf_consume(
                                &mut buf,
                                &[Token::Immediate(0), Token::LabelDeref(String::new())],
                                "Expected jump address after jump inst",
                            ) {
                                Token::Immediate(a) => JmpAddr::Literal(a),
                                Token::LabelDeref(a) => JmpAddr::Label(a),
                                _ => unreachable!(),
                            },
                        ),
                    ]);
                    ret.push(CarbonASMProgram::Instruction(CarbonInstr {
                        opcode: CarbonInstrVariants::Ics,
                        operand: Some(labels),
                    }))
                } else if val == CarbonInstrVariants::Brc {
                    buf.advance();
                    buf.advance_over_skips(&mut ret);
                    let cond = match buf_consume(
                        &mut buf,
                        &[Token::Cond(instr::CarbonConds::Jmp)],
                        "Expected cond after brc",
                    ) {
                        Token::Cond(c) => c,
                        _ => unreachable!(),
                    };
                    buf.advance();
                    buf.advance_over_skips(&mut ret);
                    let mut labels = buf.get_labels();
                    labels.append(&mut vec![
                        instr::CarbonOperand::Cond(cond),
                        CarbonOperand::JmpAddr(
                            match buf_consume(
                                &mut buf,
                                &[Token::Immediate(0), Token::LabelDeref(String::new())],
                                "Expected jump address after jump inst",
                            ) {
                                Token::Immediate(a) => JmpAddr::Literal(a),
                                Token::LabelDeref(a) => JmpAddr::Label(a),
                                _ => unreachable!(),
                            },
                        ),
                    ]);
                    ret.push(CarbonASMProgram::Instruction(CarbonInstr {
                        opcode: CarbonInstrVariants::Brc,
                        operand: Some(labels),
                    }))
                } else if val == CarbonInstrVariants::Inc
                    || val == CarbonInstrVariants::Dec
                    || val == CarbonInstrVariants::Lia
                {
                    ret.push(CarbonASMProgram::Instruction(CarbonInstr {
                        opcode: val,
                        operand: None,
                    }))
                } else {
                    buf.advance();
                    buf.advance_over_skips(&mut ret);
                    let err = &format!("expected address ($NUMBER) got {:?}", buf.current());
                    let tok = buf_consume(&mut buf, &[Token::Register(0)], err);
                    let mut instr = CarbonInstr {
                        opcode: val,
                        operand: None,
                    };
                    match tok {
                        Token::Register(r) => instr.operand = Some(vec![CarbonOperand::Reg(r)]),
                        _ => unreachable!(""),
                    }
                    ret.push(CarbonASMProgram::Instruction(instr));
                }
            }
            Token::Comment(c) => ret.push(CarbonASMProgram::Comment(c)),
            Token::PageLabel(n) => ret.push(CarbonASMProgram::PageLabel(n)),
            Token::Label(n) => ret.push(CarbonASMProgram::Label(n)),
            Token::LabelDeref(label) => ret.push(CarbonASMProgram::LabelDeref(label)),
            _ => todo!("{:#?}", buf.current()),
        }
        if buf.has_next() {
            buf.advance()
        }
    }
    ret
}

pub fn transform_labels(ast: Vec<CarbonASMProgram>) -> Vec<CarbonASMProgram> {
    // first pass; put label PC positions into a HashMap
    let mut label_map: HashMap<String, u8> = HashMap::new();
    let mut pc: i8 = -1;
    for instr in ast.iter() {
        match instr {
            CarbonASMProgram::Immediate(_) => pc += 1,
            CarbonASMProgram::Instruction(n) => {
                pc += 1 + n
                    .operand
                    .as_ref()
                    .map(|e| {
                        e.iter().fold(0, |acc, elem| {
                            if let CarbonOperand::JmpAddr(_) = elem {
                                acc + 1
                            } else if let CarbonOperand::Label(l) = elem {
                                label_map.insert(l.clone(), (pc + acc + 1) as u8);
                                acc
                            } else {
                                acc
                            }
                        })
                    })
                    .unwrap_or(0)
            }
            CarbonASMProgram::LabelDeref(_) => pc += 1,
            CarbonASMProgram::PageLabel(_) => pc = -1,

            _ => (),
        }
        if let CarbonASMProgram::Label(name) = instr {
            label_map.insert(name.clone(), pc as u8);
        }
    }
    println!("{:#?}", label_map);
    // second pass, use said map to transform all label refs to the other thingy
    let mut ret: Vec<CarbonASMProgram> = Vec::new();
    for instr in ast {
        match instr {
            CarbonASMProgram::LabelDeref(n) => ret.push(CarbonASMProgram::Immediate(label_map[&n])),
            CarbonASMProgram::Instruction(instr) => {
                let mut instr_ret = instr.clone();
                if let Some(operands) = instr.operand {
                    for (pos, operand) in operands.into_iter().enumerate() {
                        if let CarbonOperand::JmpAddr(JmpAddr::Label(n)) = operand {
                            if let Some(ops) = instr_ret.operand.as_deref_mut() {
                                ops[pos] = CarbonOperand::JmpAddr(JmpAddr::Literal(label_map[&n]));
                            }
                        }
                    }
                }
                ret.push(CarbonASMProgram::Instruction(instr_ret));
            }
            CarbonASMProgram::Label(_) => (),
            _ => ret.push(instr),
        }
    }
    ret
}
