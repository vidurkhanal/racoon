use super::token_kind::TokenKind;

static KEYWORDS_MAP: phf::Map<&'static str, TokenKind> = phf_map! {
    "func" => TokenKind::FUNCTION,
    "let" => TokenKind::LET,
};

pub fn lookup_ident(ident: &str) -> TokenKind {
    match KEYWORDS_MAP.get(ident) {
        Some(tok) => tok.clone(),
        _ => TokenKind::IDENT,
    }
}
