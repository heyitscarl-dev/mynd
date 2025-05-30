use compiler::compile;

pub mod lexer;
pub mod parser;
pub mod compiler;

fn main() {
    let source      = include_str!("../res/main.bf");
    let compiled    = compile(source).unwrap();
    println!("{}", compiled)
}
