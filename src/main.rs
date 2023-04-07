pub mod ast;
pub mod lexer;
mod repl;
pub mod token;

#[macro_use(phf_map)]
extern crate phf;

fn main() {
    repl::REPL::default().run();
}
