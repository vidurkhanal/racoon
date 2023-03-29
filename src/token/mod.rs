mod keywords;
mod token_kind;
pub use token_kind::TokenKind;

pub use keywords::lookup_ident;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}
