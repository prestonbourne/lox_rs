use std::fs;
use std::io::{self, stdin, stdout, Write};

pub mod expr;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod interpreter;

use scanner::Scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&self, path: &str) -> io::Result<()> {
        let content = fs::read_to_string(path)?;
        self.run(&content);
        Ok(())
    }

    pub fn run_prompt(&self) -> io::Result<()> {
        let stdin = stdin();
        let mut stdout = stdout();
        let mut buffer = String::new();

        loop {
            print!("> ");
            stdout.flush()?;
            buffer.clear();
            stdin.read_line(&mut buffer)?;
            self.run(&buffer.trim().to_string());
        }
    }

    fn run(&self, source: &str) {
        let tokens = Scanner::new(source.as_bytes().to_vec())
            .scan_tokens()
            .unwrap();

       
        let mut parser = parser::Parser::new(tokens);
        let expr = parser.parse().unwrap();
        interpreter::Interpreter::interpret(&expr);

        if self.had_error {
            return;
        }
        
        // let ast_str = parser::stringify_ast(&expr);
        // println!("{}", ast_str);
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
