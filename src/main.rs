mod errors;
mod extensions;
mod lexer;

use clap::Parser;

use crate::lexer::lexer::tokenize;

#[derive(Parser, Debug)]
#[command(
    version="0.0.1",
    author="Asbjørn Eriksen",
    about="Partially Applied Functions language",
    long_about = None
)]
struct Args {
    #[arg()]
    input: String,
}

fn main() {
    let args = Args::parse();
    let tokens = tokenize(&args.input);
    println!("{tokens:?}");
}
