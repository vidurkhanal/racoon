use crate::token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }
}

pub struct Identifier {
    token: token::Token,
    value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

pub struct LetStatement<'a> {
    token: token::Token,
    name: &'a Identifier,
    value: dyn Expression,
}

impl Statement for LetStatement<'_> {
    fn statement_node(&self) {}
}

impl Node for LetStatement<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
