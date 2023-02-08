mod frontend;
mod backend;
mod instr;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(name = "Input file")]
    input_file: String,

    #[arg(short, long, name = "Output file", default_value_t = String::from("out.b"))]
    output: String
}

fn main() {
    let args = Args::parse();
    let src = std::fs::read_to_string(args.input_file).unwrap();
    let toks = frontend::lexer::tokenise(&src);
    let ast = frontend::parser::parse(toks);
    backend::assembler::assemble(&mut std::fs::File::create(args.output).unwrap(), ast);
}
