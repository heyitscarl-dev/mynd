use std::{env, fs::{self, File}, io::Write, path::Path, process::exit};

use compiler::compile;

pub mod lexer;
pub mod parser;
pub mod compiler;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1).expect("Usage: mynd <input> [output?]");
    let output = args.get(2);

    let source = fs::read_to_string(input).expect("Could not open input file.");
    let compiled = compile(source).expect("Could not compile input file.");
    
    if let Some(output) = output {
        let path = Path::new(output);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Could not create parents of output file.");
        }

        let mut file = File::create(path).expect("Could not create the output file.");
        let _ = file.write_all(compiled.as_bytes());

        println!("Successfully wrote file.");
    } else {
        println!("{}", compiled)
    }
}
