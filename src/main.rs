#![feature(let_chains)]

use std::fs::read_to_string;
use std::io::Write;
use std::process::exit;
use std::path::Path;
use std::env;

use repl::{repl, exec};

mod lexer;
mod parser;
mod interpreter;
mod repl;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<_> = env::args().collect();
    for i in &args {
        if matches!(i.as_str(), "--version" | "-v" | "--versiune") {
            println!("{} v{}", args[0], VERSION);
            exit(0);
        }
    }
    if args.len() > 1 {
        let mut filep = args[1].clone();
        let mut file = Path::new(&filep);
        if !file.exists() {
            filep.push_str(".ro");
            file = Path::new(&filep);
            if !file.exists() {
                std::io::stderr().write(format!("Nu există fișierul `{}`!\n", filep).as_bytes()).unwrap();
                exit(1);
            }
        }
        let input = read_to_string(file).unwrap();

        exec(&input).unwrap();
        return;
    }

    repl();
}
