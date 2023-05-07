mod identifier;
mod program;
mod statements;
use std::{any::Any, fmt::Debug};

pub use identifier::Identifier;
pub use program::Program;
pub use statements::{ExpressionStatement, LetStatement, ReturnStatement};

pub trait Node: ToString {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node + Debug {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node + Debug {
    fn expression_node(&self);
}

#[cfg(test)]
mod ast_test {
    use super::*;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Box::new(LetStatement {
                token: crate::tkn::Token {
                    kind: crate::tkn::TokenKind::ASSIGN,
                    literal: String::from("let"),
                },
                name: Some(Identifier::new(
                    crate::tkn::Token {
                        kind: crate::tkn::TokenKind::IDENT,
                        literal: String::from("let"),
                    },
                    String::from("myVar"),
                )),
                value: Some(Box::new(Identifier::new(
                    crate::tkn::Token {
                        kind: crate::tkn::TokenKind::IDENT,
                        literal: String::from("let"),
                    },
                    String::from("anotherVar"),
                ))),
            })],
        };
        let str = program.to_string();
        assert_eq!(
            str.trim_end(),
            "let myVar = anotherVar;",
            "i implemented the to_string trait incorrectly. Need to fix it then."
        )
    }
}
