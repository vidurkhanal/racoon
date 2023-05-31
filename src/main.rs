mod abstract_tree;
mod evaluator;
pub mod lexer;
pub mod parser;
mod repl;
pub mod tkn;

use repl::REPL;

fn main() {
    REPL::default().run();
}
