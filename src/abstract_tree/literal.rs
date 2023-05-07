use crate::tkn::Token;

use super::Expression;

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Int { token: Token, value: i64 },
    String(String),
    Bool(bool),
    Array(Vec<Expression>),
    Hash(Vec<(Expression, Expression)>),
}
