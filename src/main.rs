pub mod ast;
pub mod lexer;
pub mod parser;
mod repl;
pub mod tkn;

#[macro_use(phf_map)]
extern crate phf;

fn main() {
    repl::REPL::default().run();
}
