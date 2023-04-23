use crate::{
    ast::{self, LetStatement, Program, ReturnStatement},
    lexer::Lexer,
    tkn::{Token, TokenKind},
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    curr_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer) -> Self {
        let mut parser = Self {
            lexer: l,
            curr_token: Token::default(),
            peek_token: Token::default(),
            errors: Vec::<String>::default(),
        };

        // Read two tokens, so curr_token and peek_token are both set
        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn get_errors(&mut self) -> &Vec<String> {
        &self.errors
    }

    fn peek_errors(&mut self, tok: TokenKind) {
        let msg = format!(
            "expected next token to be {:?}, but got {:?}",
            tok, self.peek_token.kind
        );
        self.errors.push(msg)
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::default();
        while self.curr_token.kind != TokenKind::EOF {
            let stmt = self.parse_statement();

            // TODO TO APPEND PROGRAM STATEMENTS.
            if stmt.is_some() {
                let stmt = stmt.unwrap();
                program.statements.push(stmt);
            }
            self.next_token();
        }
        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.curr_token.kind {
            TokenKind::LET => {
                if let Some(let_st) = self.parse_let_statement() {
                    return Some(let_st as Box<dyn ast::Statement>);
                }
                None
            }
            TokenKind::RETURN => {
                if let Some(return_st) = self.parse_return_statement() {
                    return Some(return_st as Box<dyn ast::Statement>);
                }
                None
            }
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<LetStatement>> {
        let mut stmt = Box::new(LetStatement {
            token: self.curr_token.clone(),
            value: None,
            name: None,
        });

        if !self.expect_peek(TokenKind::IDENT) {
            return None;
        }

        stmt.name = Some(ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        });

        if !self.expect_peek(TokenKind::ASSIGN) {
            return None;
        }

        // TODOD: UPDATE THIS , RN SKIPPING TILL SEMICOLON
        while !self.curr_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    fn parse_return_statement(&mut self) -> Option<Box<ReturnStatement>> {
        let statement = Box::new(ReturnStatement {
            token: self.curr_token.clone(),
            return_value: None,
        });

        self.next_token();

        // TODOD: UPDATE THIS , RN SKIPPING TILL SEMICOLON
        while !self.curr_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }

        Some(statement)
    }

    fn expect_peek(&mut self, tok: TokenKind) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            true
        } else {
            self.peek_errors(tok);
            false
        }
    }
    fn curr_token_is(&mut self, tok: TokenKind) -> bool {
        self.curr_token.kind == tok
    }
    fn peek_token_is(&mut self, tok: TokenKind) -> bool {
        self.peek_token.kind == tok
    }
}

#[cfg(test)]
mod test_parser {
    use crate::ast::{Node, Statement};

    use super::*;

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let  y = 10;
        let  foobar= 838383;
    ";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        // eprintln!("Vector--> {:?}", p.get_errors());
        if program.is_none() {
            panic!("parse_program() returned None")
        }
        let program = program.unwrap();
        // eprintln!("LENGTH--> {:?}", program.statements.len());
        check_parse_errors(&mut p);
        if program.statements.len() != 3 {
            panic!(
                "program.statements does not contain 3 statements. got {}",
                program.statements.len()
            )
        }
        let tests = vec!["x", "y", "foobar"];
        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            assert!(test_let_statement(stmt, *tt))
        }
    }

    fn test_let_statement(statement: &Box<dyn Statement>, name: &str) -> bool {
        if statement.token_literal() != "let" {
            panic!(
                "s.token_literal not 'let'. got={}",
                statement.token_literal()
            );
        }

        if let Some(let_stmt) = statement.as_any().downcast_ref::<ast::LetStatement>() {
            let ident = let_stmt.name.as_ref().unwrap();
            if ident.value != name {
                eprintln!(
                    "Identifier name are not same. Expected {} but received {}",
                    name, ident.value
                );
                return false;
            }
            if ident.token_literal() != name {
                eprintln!(
                    "Token Literal are not same. Expected {} but received {}",
                    ident.token_literal(),
                    name
                );
                return false;
            }
        }
        true
    }

    #[test]
    fn test_return_statements() {
        let input = "
        return 5;
        return 10;
        return 25;
        ";

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        if program.is_none() {
            panic!("parse_program() returned None")
        }
        let program = program.unwrap();
        check_parse_errors(&mut p);
        if program.statements.len() != 3 {
            panic!(
                "program.statements does not contain 3 statements. got {}",
                program.statements.len()
            )
        }
        for (_, stmt) in program.statements.iter().enumerate() {
            if let Some(stmt) = stmt.as_any().downcast_ref::<ast::ReturnStatement>() {
                if stmt.token_literal() != "return" {
                    panic!(
                        "Parsing Error: Token Literal expected return but got {}.",
                        stmt.token_literal()
                    );
                }
            } else {
                panic!("Parsing Error: The provided statement was not a return statement.")
            }
        }
    }

    fn check_parse_errors(p: &mut Parser) {
        let errors = p.get_errors();
        if errors.len() == 0 {
            return;
        }

        for (i, err_msg) in errors.iter().enumerate() {
            eprintln!("Parsing Error - LNum {} ; {} ", i + 1, err_msg);
        }

        panic!()
    }
}
