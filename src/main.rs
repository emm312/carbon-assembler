mod backend;
mod frontend;
mod instr;

use std::io::Write;

use clap::Parser;

use crate::backend::assembler::PageOutput;

#[derive(Parser)]
struct Args {
    #[arg(name = "Input file")]
    input_file: String,

    #[arg(short, long, name = "Output file", default_value_t = String::from("out.b"))]
    output: String,
}

fn main() {
    let args = Args::parse();
    let src = std::fs::read_to_string(args.input_file).unwrap();
    let toks = frontend::lexer::tokenise(&src);
    let mut ast = frontend::parser::parse(toks.clone());
    ast = frontend::parser::transform_labels(ast);
    let asm = backend::assembler::assemble(ast);
    let out_file = &mut std::fs::File::create(args.output).unwrap();
    write!(out_file, "// PAGE 0").unwrap();
    let mut ctr = 0;
    for word in asm.iter() {
        if ctr % 32 == 0 {
            if ctr / 32 != 0 {
                write!(out_file, "\n// PAGE {}", ctr / 32).unwrap();
            }
        }

        match word {
            PageOutput::Lit(n) => { ctr += 1; write!(out_file, "\n{:08b}", n).unwrap(); },
            PageOutput::Comment(n) => write!(out_file, " {}", *n).unwrap(),
        };
    }
}
