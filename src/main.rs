pub mod lexer;
mod repl;
mod token;

#[macro_use(phf_map)]
extern crate phf;

fn main() {
    repl::REPL::default().run();
}
