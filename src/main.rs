use std::env;
use std::io::{self};
use std::process;

mod lox;
use lox::Lox;

const REPL_ARGS: usize = 1;
const FILE_ARGS: usize = 2;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let lox = Lox::new();

    match args.len() {
        REPL_ARGS => lox.run_prompt()?,
        FILE_ARGS => lox.run_file(&args[1])?,
        _ => {
            println!("Usage: rlox [script]");
            process::exit(64);
        }
    }

    Ok(())
}
