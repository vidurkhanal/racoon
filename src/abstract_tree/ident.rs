use crate::tkn::Token;

#[derive(PartialEq, Clone, Debug)]
pub struct Identifier {
    pub literal: String,
    pub token: Token,
}
