pub mod ast;
pub mod interpreter;

use lalrpop_util::lalrpop_mod;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    tinylang
);

use interpreter::Interpreter;

#[derive(Parser)]
struct Args {
    #[arg()]
    filename: String
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let filename = args.filename;
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;


    let parser = tinylang::ProgramParser::new();
    let program = parser.parse(&contents).unwrap_or_else(|e| {
        eprintln!("Parse error: {e}");
        std::process::exit(1);
    });

    let mut interp = Interpreter::new();
    if let Err(e) = interp.run(program) {
        eprintln!("Runtime error: {e}");
        std::process::exit(1);
    }

    Ok(())
}

