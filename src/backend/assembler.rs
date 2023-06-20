use crate::instr::{CarbonASMProgram, CarbonConds, CarbonInstrVariants, CarbonOperand};

struct PageWriter {
    current_page: usize,
    current_page_ptr: usize,
    pages: Vec<Vec<PageOutput>>,
    comments: Vec<(String, usize)>
}

impl PageWriter {
    pub fn new() -> PageWriter {
        PageWriter {
            current_page: 0,
            current_page_ptr: 0,
            pages: vec![vec![PageOutput::Lit(0); 32]; 32],
            comments: vec![]
        }
    }

    pub fn set_page(&mut self, page: usize) {
        self.current_page = page;
        self.current_page_ptr = 0;
    }

    pub fn write(&mut self, value: u8) {
        self.pages[self.current_page][self.current_page_ptr] = PageOutput::Lit(value);
        self.current_page_ptr += 1;
    }

    pub fn write_comment(&mut self, value: String) {
        self.comments.push((value, self.current_page*32+self.current_page_ptr));
    }

    pub fn get_pages(self) -> Vec<PageOutput> {
        let mut ret: Vec<PageOutput> = self.pages.into_iter().flatten().collect();
        for (pos, comment) in self.comments.into_iter().enumerate() {
            ret.insert(comment.1+pos+1, PageOutput::Comment(comment.0));
        }
        ret
    }
}

#[derive(Debug, Clone)]
pub enum PageOutput {
    Lit(u8),
    Comment(String),
}

pub fn assemble(ast: Vec<CarbonASMProgram>) -> Vec<PageOutput> {
    let mut pages = PageWriter::new();
    for node in ast {
        let mut word = 0;
        match node {
            CarbonASMProgram::Immediate(i) => {
                word = i;
            }
            CarbonASMProgram::Instruction(i) => {
                match i.opcode {
                    CarbonInstrVariants::Hlt => word = 0b11111000,
                    CarbonInstrVariants::Add => word |= 0b00001000,
                    CarbonInstrVariants::Sub => word |= 0b00010000,
                    CarbonInstrVariants::Bsb => word |= 0b00011000,
                    CarbonInstrVariants::Or => word |= 0b00100000,
                    CarbonInstrVariants::Nor => word |= 0b00101000,
                    CarbonInstrVariants::And => word |= 0b00110000,
                    CarbonInstrVariants::Nand => word |= 0b00111000,
                    CarbonInstrVariants::Xor => word |= 0b01000000,
                    CarbonInstrVariants::Xnr => word |= 0b01001000,
                    CarbonInstrVariants::Ldi => word |= 0b01010000,
                    CarbonInstrVariants::Adr => word |= 0b01011000,
                    CarbonInstrVariants::Rld => word |= 0b01100000,
                    CarbonInstrVariants::Rst => word |= 0b01101000,
                    CarbonInstrVariants::Mst => word |= 0b01110000,
                    CarbonInstrVariants::Mld => word |= 0b01111000,
                    CarbonInstrVariants::Ics => word |= 0b10000000,
                    CarbonInstrVariants::Jid => word |= 0b10001000,
                    CarbonInstrVariants::Brc => word |= 0b10010000,
                    CarbonInstrVariants::Dec => word |= 0b10011000,
                    CarbonInstrVariants::Cmp => word |= 0b10100000,
                    CarbonInstrVariants::Bsr => word |= 0b10101000,
                    CarbonInstrVariants::Bsl => word |= 0b10110000,
                    CarbonInstrVariants::Pst => word |= 0b10111000,
                    CarbonInstrVariants::Pld => word |= 0b11000000,
                    CarbonInstrVariants::Inc => word |= 0b11001000,
                }
                pages.write_comment(format!("# {:?}", i.opcode));
                match i.operand {
                    Some(v) => {
                        for operand in v {
                            match operand {
                                CarbonOperand::Cond(c) => word |= write_cond(c),
                                CarbonOperand::Reg(r) => word |= r,
                                CarbonOperand::JmpAddr(a) => {
                                    pages.write(word);
                                    word = a.unwrap();
                                }
                            }
                        }
                    }
                    None => (),
                }
            }
            CarbonASMProgram::Comment(c) => {
                pages.write_comment(c);
                continue;
            }
            CarbonASMProgram::Label(_) => unreachable!(),
            CarbonASMProgram::PageLabel(n) => {
                pages.set_page(n);
                continue;
            }
            CarbonASMProgram::LabelDeref(_) => unreachable!(),
        }
        pages.write(word);
    }
    pages.get_pages()
}

fn write_cond(cond: CarbonConds) -> u8 {
    let mut word = 0;
    match cond {
        CarbonConds::ZR => {
            word |= 0b001;
        }
        CarbonConds::NZR => {
            word |= 0b010;
        }
        CarbonConds::MSB => {
            word |= 0b011;
        }
        CarbonConds::NMSB => {
            word |= 0b100;
        }
        CarbonConds::COUT => {
            word |= 0b101;
        }
        CarbonConds::NCOUT => {
            word |= 0b110;
        }
        CarbonConds::UCD => {
            word |= 0b111;
        }
    }
    word
}
