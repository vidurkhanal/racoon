mod abstract_tree;
pub mod lexer;
pub mod parser;
mod repl;
pub mod tkn;

#[macro_use(phf_map)]
extern crate phf;
use repl::REPL;

fn main() {
    REPL::default().run();
}
