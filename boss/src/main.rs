use clap::Parser;

use boslib::{read_file, run_file};

/// BO Script (bos) servicer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Extraneous filepath
    filename: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.filename {
        Some(p) => {
            println!("Path: {}", p);
            let f = read_file(p);
            match f {
                Ok(s) => {
                    println!("Contents:\n{}", s);
                    println!("Here goes nothing...");
                    run_file(s);
                },
                Err(s) => {println!("ERROR: {}", s)}
            }
        },
        None => println!("No path given, reading from stdin."),
    }
}
