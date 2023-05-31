use std::{cell::RefCell, cmp::min, io::Write, rc::Rc};

use crate::{
    evaluator::{self, builltin_funcs::new_builtins, Environment, Evaluator, Object},
    lexer::Lexer,
    parser::Parser,
    tkn::TokenKind,
};

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

    pub fn print_errors(&mut self, errors: &Vec<String>) {
        let ascii_art = "   
    /\\_/\\  
  ( o   o )
   =\\  /=";
        println!("\t {} \n Racoon fell into some errors. \n ", ascii_art);
        for msg in errors.iter() {
            eprintln!("\t{}\n", msg);
        }
    }

    pub fn run(&mut self) {
        println!("{}", "b\x1B[2J\x1B[1;1H");
        println!("Welcome to Racoon v{}!! [Rust] ", env!("CARGO_PKG_VERSION"),);
        let mut buffer = String::new();
        let mut env = Environment::from(new_builtins());

        env.set(
            "putln".to_string(),
            &Object::BUILTIN {
                arity: -1,
                builtInFunc: |args| {
                    for arg in args.iter() {
                        println!("{}", arg);
                    }
                    Object::NIL
                },
            },
        );
        let mut evaluator = Evaluator::new(Rc::new(RefCell::new(env)));

        loop {
            buffer.clear();
            let stdin = std::io::stdin();

            print!(">>> ");

            if let Err(e) = std::io::stdout().flush() {
                self.print_errors(&vec![format!(
                    "InputBufferError: Unable to flush stdout \n Error Details: {}",
                    e
                )]);
            }

            if let Err(e) = stdin.read_line(&mut buffer) {
                self.print_errors(&vec![format!(
                    "InputBufferError: Unable to take input from the user. \n Error Details: {}",
                    e
                )]);
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
                    let l = Lexer::new(buffer);

                    // let mut tok = l.next_token();
                    // while tok.kind != TokenKind::EOF {
                    //     println!("{:?}", tok);
                    //     tok = l.next_token();
                    // }
                    let mut parser = Parser::new(l);
                    let program = parser.parse_program();
                    if !parser.get_errors().is_empty() {
                        self.print_errors(parser.get_errors());
                        continue;
                    }
                    // for statement in program.iter() {
                    //     println!("{:?}", statement);
                    // }
                    match evaluator.evaluate(program) {
                        Some(obj) => match obj {
                            Object::ERROR(error_msg) => self.print_errors(&vec![error_msg]),
                            _ => println!("{:?}", obj),
                        },

                        None => {}
                    }
                    // println!("DEBUGGING: {:?}", env);'
                }
            }
        }
    }
}
