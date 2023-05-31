mod ident;
mod infix;
mod literal;
mod precedence;
mod prefix;

use std::fmt::Debug;

pub use ident::Identifier;
pub use infix::Infix;
pub use literal::Literal;
pub use precedence::Precedence;
pub use prefix::Prefix;

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix(Prefix, Box<Expression>),
    Infix(Infix, Box<Expression>, Box<Expression>),
    Index(Box<Expression>, Box<Expression>),
    If {
        condition: Box<Expression>,
        consequence: BlockOfStatements,
        alternative: Option<BlockOfStatements>,
    },

    Function {
        params: Vec<Identifier>,
        body: BlockOfStatements,
    },
    Call {
        func: Box<Expression>,
        args: Option<Vec<Expression>>,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    // Blank,
    Let { name: Identifier, value: Expression },
    Return { return_value: Expression },
    Expression { expression: Expression },
}

pub type BlockOfStatements = Vec<Statement>;
pub type Program = BlockOfStatements;
