use parser::parse;

pub mod lexer;
pub mod parser;

fn main() {
    let source  = include_str!("../res/main.bf");
    let nodes   = parse(source).unwrap();
    println!("{:#?}", nodes);
}
