use std::{env, fs::{self, File}, io::Write, path::Path, process::exit};

use compiler::compile;

pub mod lexer;
pub mod parser;
pub mod compiler;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1);
    let output = args.get(2);
    
    if let None = input {
        eprintln!("Usage: mynd <input> [output]");
        exit(1);
    }

    let source = fs::read_to_string(input.unwrap());

    if let Err(e) = source {
        eprintln!("Could not open '{}': {:?}", input.unwrap(), e);
        exit(2);
    }

    let compiled = compile(source.unwrap()).unwrap();
    
    if let Some(output) = output {
        let path = Path::new(output);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Could not create parents of output file.");
        }

        let mut file = File::create(path).expect("Could not create the output file.");
        let _ = file.write(compiled.as_bytes());

        println!("Successfully wrote file.");
    } else {
        println!("{}", compiled)
    }
}
