use std::{cmp::min, io::Write};

use crate::{lexer::Lexer, tkn::TokenKind};

pub struct REPL {
    pub command_buffer: Vec<String>,
}

impl Default for REPL {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::print_literal)]
impl REPL {
    pub fn new() -> Self {
        Self {
            command_buffer: vec![],
        }
    }
    pub fn run(&mut self) {
        println!("Welcome to Racoon v{}!! [Rust] ", env!("CARGO_PKG_VERSION"),);
        let mut buffer = String::new();
        loop {
            buffer.clear();
            let stdin = std::io::stdin();

            print!(">>> ");

            if let Err(e) = std::io::stdout().flush() {
                eprintln!("Unable to flush stdout \n Error Details: {}", e);
            }

            if let Err(e) = stdin.read_line(&mut buffer) {
                eprintln!(
                    "Unable to take input from the user. \n Error Details: {}",
                    e
                );
            }

            let buffer = buffer.trim();
            self.command_buffer.push(String::from(buffer));
            match buffer {
                ":quit" => {
                    eprintln!("See you again!!");
                    std::process::exit(0);
                }
                ":clear" => {
                    println!("{}", "b\x1B[2J\x1B[1;1H");
                }
                ":history" => {
                    println!("-- Command History --");
                    let n = self.command_buffer.len();
                    for i in 0..min(n, 10_usize) {
                        println!(" -> {}", self.command_buffer[n - i - 1])
                    }
                    println!("-- END --")
                }

                _ => {
                    let mut l = Lexer::new(buffer);
                    let mut tok = l.next_token();
                    while tok.kind != TokenKind::EOF {
                        println!("{:?}", tok);
                        tok = l.next_token();
                    }
                }
            }
        }
    }
}
