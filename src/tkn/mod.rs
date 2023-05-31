mod token_kind;
pub use token_kind::TokenKind;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::EOF,
            literal: " ".to_string(),
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind,
            literal: self.literal.clone(),
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}
