pub mod bos_context;
mod token;
mod scanner;
mod error_list;
mod parser;

use std::fs;

use crate::scanner::Scanner;
use crate::token::Token;

/// Load and execute a bos source file.
///
/// # Arguments
///
/// * 'path' - A filepath pointing to the source file to execute.
pub fn run_file(source: String) {
    let mut scan: Scanner = Scanner::new(source);
    
    let tokens = scan.scan_tokens();
    
    match tokens {
        Ok(t) => {println!("No errors!"); println!("{:?}", t);}
        Err(t) => {println!("{}", t);}
    }
}


/// Run a single line of bos code.
pub fn run_line(line: String) -> String {
    return line;
}


/// Read a specified file into memory.
pub fn read_file(fp: String) -> Result<String, String> {
    let f = fs::read_to_string(fp);

    return match f {
        Ok(s) => { Ok(s) }
        Err(_) => { Err("Unable to read file.".to_string()) }
    }
}

