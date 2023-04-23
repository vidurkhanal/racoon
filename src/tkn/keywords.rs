use super::token_kind::TokenKind;

static KEYWORDS_MAP: phf::Map<&'static str, TokenKind> = phf_map! {
    "func" => TokenKind::FUNCTION,
    "let" => TokenKind::LET,
    "true" => TokenKind::TRUE,
    "false" => TokenKind::FALSE,
    "if" => TokenKind::IF,
    "else" => TokenKind::ELSE,
    "return" => TokenKind::RETURN,
};

pub fn lookup_ident(ident: &str) -> TokenKind {
    match KEYWORDS_MAP.get(ident) {
        Some(tok) => tok.clone(),
        _ => TokenKind::IDENT,
    }
}
