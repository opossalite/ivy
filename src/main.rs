mod tokenizer;

use std::{fmt, error::Error, fs};
use clap::Parser;




#[derive(Debug, Parser)]
#[clap(name = "ivy", version = "0.0.1", about = "Compiler for Ivy")]
pub struct Args {
    /// The Ivy file to compile.
    input_file: String,
}


#[derive(Debug, Clone)]
pub enum CorvusError {
    UnmatchedCharacter(char),
}
impl fmt::Display for CorvusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Received error: {}", self)
    } 
}
impl Error for CorvusError {
}


fn main() {
    let args = Args::parse();
    let file = args.input_file;
    let raw_contents = fs::read_to_string(file);

    let contents: Vec<char> = match raw_contents {
        Err(err) => {
            println!("{:?}", err);
            return;
        },
        Ok(a) => {
            a.chars().collect()
        }
    };

    tokenizer::tokenize(&contents);

}






