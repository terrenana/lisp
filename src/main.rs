use std::{fs::File, io::Read};

mod env;
mod eval;
mod lexer;
mod object;
mod parser;

fn main() -> std::io::Result<()> {
    let mut f = File::open("src/test.ls")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let tokens = lexer::tokenize(&input).unwrap();

    println!("{:#?}", tokens);

    Ok(())
}
