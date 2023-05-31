use crate::{
    abstract_tree::{
        Expression, Identifier, Infix, Literal, Precedence, Prefix, Program, Statement,
    },
    lexer::Lexer,
    tkn::{Token, TokenKind},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer: l,
            curr_token: Token::default(),
            peek_token: Token::default(),
            errors: vec![],
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn token_to_precedence(tok: &Token) -> Precedence {
        match tok.kind {
            TokenKind::EQ | TokenKind::NEQ => Precedence::Equals,
            TokenKind::LT => Precedence::LessGreater,
            TokenKind::GT => Precedence::LessGreater,
            TokenKind::PLUS | TokenKind::MINUS => Precedence::Sum,
            TokenKind::SLASH | TokenKind::ASTERISK => Precedence::Product,
            TokenKind::LBRACKET => Precedence::Index,
            TokenKind::LPAREN => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    pub fn peek_precedence(&mut self) -> Precedence {
        Parser::token_to_precedence(&self.peek_token)
    }

    pub fn curr_precedence(&mut self) -> Precedence {
        Parser::token_to_precedence(&self.curr_token)
    }

    pub fn get_errors(&mut self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, token_kind: TokenKind) {
        let msg = format!(
            "Expected {:?}, but received {:?}",
            token_kind, self.peek_token.kind
        );
        self.errors.push(msg)
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = vec![];
        while self.curr_token.kind != TokenKind::EOF {
            let statement = self.parse_statement();
            if statement.is_some() {
                let statement = statement.unwrap();
                program.push(statement);
            }
            self.next_token()
        }
        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token.kind {
            TokenKind::LET => self.parse_let_statement(),
            TokenKind::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        match self.peek_token.kind {
            TokenKind::IDENT => self.next_token(),
            _ => return None,
        }

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.expect_peek(TokenKind::ASSIGN) {
            return None;
        }

        self.next_token();

        let value = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        // println!("Name --> {:?}", name);
        // println!("Value --> {:?}", value);
        if self.peek_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }

        let st = Statement::Let { name, value };
        Some(st)
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        // let return_value = Expression::Identifier(Identifier {
        //     literal: self.curr_token.literal.clone(),
        //     token: self.curr_token.clone(),
        // });

        let return_value = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        while !self.curr_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }

        Some(Statement::Return { return_value })
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        match self.parse_expression(Precedence::Lowest) {
            Some(expr) => {
                if self.peek_token_is(TokenKind::SEMICOLON) {
                    self.next_token();
                }
                return Some(Statement::Expression { expression: expr });
            }
            None => None,
        }
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // PREFIX
        let mut left = match self.curr_token.kind {
            TokenKind::IDENT => self.parse_ident_expr(),
            TokenKind::INT => self.parse_int_expr(),
            TokenKind::TRUE(_) | TokenKind::FALSE(_) => self.parse_bool_expr(),
            TokenKind::BANG | TokenKind::MINUS | TokenKind::PLUS => self.parse_prefix_expr(),
            TokenKind::LPAREN => self.parse_grouped_expr(),
            TokenKind::LBRACKET => self.parse_array_expr(),
            TokenKind::IF => self.parse_if_expr(),
            TokenKind::FUNCTION => self.parse_function_expr(),
            TokenKind::STRING => Some(Expression::Literal(Literal::String(
                self.curr_token.literal.clone(),
            ))),
            _ => {
                self.errors.push(format!(
                    "No Prefix Parsing Function available for {:?}",
                    self.curr_token.kind
                ));
                None
            }
        };

        // INFIX
        while !self.peek_token_is(TokenKind::SEMICOLON) && precedence < self.peek_precedence() {
            match self.peek_token.kind {
                TokenKind::PLUS
                | TokenKind::MINUS
                | TokenKind::SLASH
                | TokenKind::ASTERISK
                | TokenKind::EQ
                | TokenKind::NEQ
                | TokenKind::LT
                | TokenKind::GT
                | TokenKind::LTE
                | TokenKind::GTE => {
                    self.next_token();
                    left = self.parse_infix_expr(left.unwrap());
                }
                TokenKind::LPAREN => {
                    self.next_token();
                    left = self.parse_call_expr(left.unwrap())
                }
                TokenKind::LBRACKET => {
                    self.next_token();
                    left = self.parse_index_expr(left.unwrap())
                }
                _ => return left,
            }
        }

        left
    }

    fn parse_infix_expr(&mut self, left: Expression) -> Option<Expression> {
        let infix = match self.curr_token.kind {
            TokenKind::PLUS => Infix::Plus,
            TokenKind::MINUS => Infix::Minus,
            TokenKind::SLASH => Infix::Divide,
            TokenKind::ASTERISK => Infix::Multiply,
            TokenKind::EQ => Infix::Equal,
            TokenKind::NEQ => Infix::NotEqual,
            TokenKind::LT => Infix::LessThan,
            TokenKind::GT => Infix::GreaterThan,
            _ => return None,
        };

        let precedence = self.curr_precedence();

        self.next_token();

        match self.parse_expression(precedence) {
            Some(expr) => Some(Expression::Infix(infix, Box::new(left), Box::new(expr))),
            None => None,
        }
    }

    fn parse_ident(&mut self) -> Option<Identifier> {
        match self.curr_token.kind {
            TokenKind::IDENT => Some(Identifier {
                literal: self.curr_token.literal.clone(),
                token: self.curr_token.clone(),
            }),
            _ => None,
        }
    }

    fn parse_ident_expr(&mut self) -> Option<Expression> {
        match self.parse_ident() {
            Some(ident) => Some(Expression::Identifier(ident)),
            None => None,
        }
    }

    pub fn parse_bool_expr(&mut self) -> Option<Expression> {
        match self.curr_token.kind {
            TokenKind::TRUE(val) | TokenKind::FALSE(val) => {
                Some(Expression::Literal(Literal::Bool(val == true)))
            }
            _ => None,
        }
    }

    pub fn parse_int_expr(&mut self) -> Option<Expression> {
        if self.curr_token_is(TokenKind::INT) {
            let int_value = self.curr_token.literal.parse::<i64>();

            if int_value.is_err() {
                return None;
            }
            let int_value = int_value.unwrap();
            return Some(Expression::Literal(Literal::Int {
                token: self.curr_token.clone(),
                value: int_value,
            }));
        }
        None
    }

    pub fn parse_prefix_expr(&mut self) -> Option<Expression> {
        let prefix = match self.curr_token.kind {
            TokenKind::BANG => Prefix::Not,
            TokenKind::MINUS => Prefix::Minus,
            TokenKind::PLUS => Prefix::Plus,
            _ => return None,
        };

        self.next_token();

        match self.parse_expression(Precedence::Prefix) {
            Some(expr) => Some(Expression::Prefix(prefix, Box::new(expr))),
            None => None,
        }
    }

    pub fn parse_function_expr(&mut self) -> Option<Expression> {
        if !self.expect_peek(TokenKind::LPAREN) {
            return None;
        }
        let params = match self.parse_function_params() {
            Some(params) => params,
            None => return None,
        };

        if !self.expect_peek(TokenKind::LBRACE) {
            return None;
        }

        // println!("Function_Body -->{:?}", self.curr_token);
        let function_body = self.parse_block_statement();

        Some(Expression::Function {
            params,
            body: function_body,
        })
    }

    fn parse_function_params(&mut self) -> Option<Vec<Identifier>> {
        let mut params = vec![];

        if self.peek_token_is(TokenKind::RPAREN) {
            self.next_token();
            return Some(params);
        }

        self.next_token();

        match self.parse_ident() {
            Some(ident) => params.push(ident),
            None => return None,
        }
        while self.peek_token_is(TokenKind::COMMA) {
            self.next_token();
            self.next_token();

            match self.parse_ident() {
                Some(ident) => params.push(ident),
                None => return None,
            }
        }

        if !self.expect_peek(TokenKind::RPAREN) {
            return None;
        }

        Some(params)
    }

    pub fn parse_grouped_expr(&mut self) -> Option<Expression> {
        self.next_token();

        let expression = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(TokenKind::RPAREN) {
            None
        } else {
            expression
        }
    }

    pub fn parse_if_expr(&mut self) -> Option<Expression> {
        // if x>4{true;} else { false;}
        // println!("CURR TOKEN --> {:?}", self.curr_token);
        self.next_token();
        // println!("CURR TOKEN --> {:?}", self.curr_token);
        let condition = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_peek(TokenKind::LBRACE) {
            return None;
        }

        let consequence = self.parse_block_statement();
        let mut alternative: Option<Vec<Statement>> = None;

        if self.peek_token_is(TokenKind::ELSE) {
            self.next_token();
            if !self.expect_peek(TokenKind::LBRACE) {
                return None;
            }
            alternative = Some(self.parse_block_statement());
        }

        Some(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    pub fn parse_block_statement(&mut self) -> Vec<Statement> {
        self.next_token();
        let mut block = vec![];
        while !self.curr_token_is(TokenKind::RBRACE) && !self.curr_token_is(TokenKind::EOF) {
            match self.parse_statement() {
                Some(stmt) => block.push(stmt),
                None => {}
            }

            self.next_token()
        }

        block
    }

    fn parse_call_expr(&mut self, left: Expression) -> Option<Expression> {
        let args = self.parse_expr_list(TokenKind::RPAREN);

        Some(Expression::Call {
            func: Box::new(left),
            args,
        })
    }

    fn parse_index_expr(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();

        let index = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_peek(TokenKind::RBRACKET) {
            return None;
        }

        let res = Expression::Index(Box::new(left), Box::new(index));
        // println!("{:?}", res);
        Some(res)
    }

    // pub fn parse_call_args(&mut self) -> Option<Vec<Expression>> {
    //     let mut args: Vec<Expression> = vec![];
    //     if self.peek_token_is(TokenKind::RPAREN) {
    //         self.next_token();
    //         return Some(args);
    //     }

    //     self.next_token();

    //     match self.parse_expression(Precedence::Lowest) {
    //         Some(expr) => args.push(expr),
    //         None => return None,
    //     }
    //     while self.peek_token_is(TokenKind::COMMA) {
    //         self.next_token();
    //         self.next_token();

    //         match self.parse_expression(Precedence::Lowest) {
    //             Some(expr) => args.push(expr),
    //             None => return None,
    //         }
    //     }

    //     if !self.expect_peek(TokenKind::RPAREN) {
    //         return None;
    //     }

    //     Some(args)
    // }

    fn parse_array_expr(&mut self) -> Option<Expression> {
        match self.parse_expr_list(TokenKind::RBRACKET) {
            Some(list) => Some(Expression::Literal(Literal::Array(list))),
            None => None,
        }
    }

    fn parse_expr_list(&mut self, end_token: TokenKind) -> Option<Vec<Expression>> {
        let mut list = vec![];

        if self.peek_token_is(end_token) {
            self.next_token();
            return Some(list);
        }

        self.next_token();

        match self.parse_expression(Precedence::Lowest) {
            Some(expr) => {
                list.push(expr);
            }
            None => return None,
        }

        while self.peek_token_is(TokenKind::COMMA) {
            self.next_token();
            self.next_token();

            match self.parse_expression(Precedence::Lowest) {
                Some(expr) => {
                    list.push(expr);
                }
                None => return None,
            }
        }

        if !self.expect_peek(end_token) {
            return None;
        }

        Some(list)
    }

    fn expect_peek(&mut self, token_kind: TokenKind) -> bool {
        if self.peek_token_is(token_kind) {
            self.next_token();
            return true;
        }
        self.peek_error(token_kind);
        false
    }
    fn curr_token_is(&mut self, token_kind: TokenKind) -> bool {
        self.curr_token.kind == token_kind
    }
    fn peek_token_is(&mut self, token_kind: TokenKind) -> bool {
        self.peek_token.kind == token_kind
    }
}

#[cfg(test)]
mod parser_test {

    use super::*;
    #[test]
    fn test_let_statement() {
        let input = "
        let x = 5;
        let y =6;
        let foobar = 1512;
        ";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&mut p);
        assert_ne!(program.len(), 0, "Sorry, Couldn't parse the program.");
        assert_eq!(
            program.len(),
            3,
            "Expected to have 3 statements , but received {}",
            program.len()
        );

        let tests = vec!["x", "y", "foobar"];

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program[i];
            if let Statement::Let { name, value: _ } = stmt {
                let Identifier { literal, token } = name;
                assert_eq!(literal, *tt, "Expected {} but received {}", tt, literal);
                assert_eq!(token.kind, TokenKind::IDENT, "Unmatching token types");
            }
        }
    }

    #[test]
    fn test_return_statement() {
        let input = "
        return x;
        return y;
        return 1512;
        ";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&mut p);
        assert_ne!(program.len(), 0, "Sorry, Couldn't parse the program.");
        assert_eq!(
            program.len(),
            3,
            "Expected to have 3 statements , but received {}",
            program.len()
        );

        println!("PROGRAM --> {:?}", program);

        let tests = vec!["x", "y", "1512"];

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program[i];
            // println!("{:?}", stmt);
            if let Statement::Return { return_value } = stmt {
                if let Expression::Identifier(Identifier { literal, token: _ }) = return_value {
                    assert_eq!(literal, *tt, "Expected {} but received {}", tt, literal);
                };
            }
        }
    }

    #[test]
    fn test_ident_expr() {
        let input = "
        foobar;
        ";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&mut p);
        assert_ne!(program.len(), 0, "Sorry, Couldn't parse the program.");
        assert_eq!(
            program.len(),
            1,
            "Expected to have 1 statements , but received {}",
            program.len()
        );

        let stmt = &program[0];
        println!("{:?}", stmt);
        if let Statement::Expression { expression } = stmt {
            if let Expression::Identifier(ident) = expression {
                assert_eq!(
                    ident.literal, "foobar",
                    "Expected foobar but received {}",
                    ident.literal
                );
                assert_eq!(
                    ident.token.literal, "foobar",
                    "Expected foobar but received {}",
                    ident.token.literal
                );
            }
        } else {
            panic!("Expected a Expression Statement but received.. {:?}", stmt)
        }
    }

    #[test]
    fn test_int_literal() {
        let input = "
        5344;
        ";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&mut p);
        assert_ne!(program.len(), 0, "Sorry, Couldn't parse the program.");
        assert_eq!(
            program.len(),
            1,
            "Expected to have 1 statements , but received {}",
            program.len()
        );

        let stmt = &program[0];
        println!("{:?}", stmt);
        if let Statement::Expression { expression } = stmt {
            if let Expression::Literal(Literal::Int { token: _, value }) = expression {
                assert_eq!(&5344, value);
            }
        } else {
            panic!("Expected a Expression Statement but received.. {:?}", stmt)
        }
    }

    fn check_parser_errors(p: &mut Parser) {
        let errors = p.get_errors();
        if errors.len() == 0 {
            return;
        }

        eprintln!("The parser had {} errors.", errors.len());
        for msg in errors.iter() {
            eprintln!("Parser Error : {}", msg);
        }
        panic!()
    }

    #[test]
    fn test_prefix_expr() {
        let l = Lexer::new("!5");
        let mut p = Parser::new(l);

        let program = p.parse_program();

        assert_ne!(
            program.len(),
            0,
            "Parse Error: Couldn't parse the given expression."
        );
        assert_eq!(
            program.len(),
            1,
            "Expected 1 statement, instead received {}",
            program.len()
        );

        println!("{:?}", program)
    }

    #[test]
    fn test_infix_expr() {
        let l = Lexer::new("x==y");
        let mut p = Parser::new(l);

        let program = p.parse_program();

        assert_ne!(
            program.len(),
            0,
            "Parse Error: Couldn't parse the given expression."
        );
        println!("{:?}", program);
        assert_eq!(
            program.len(),
            1,
            "Expected 1 statement, instead received {}",
            program.len()
        );
    }

    #[test]
    fn test_bool_expr() {
        let l = Lexer::new("let foobar = !true;");
        let mut p = Parser::new(l);

        let program = p.parse_program();

        println!("{:?}", program);
        assert_ne!(
            program.len(),
            0,
            "Parse Error: Couldn't parse the given expression."
        );
        assert_eq!(
            program.len(),
            1,
            "Expected 1 statement, instead received {}",
            program.len()
        );
    }

    #[test]
    fn test_grouped_expr() {
        let l = Lexer::new("(5+5)*3)");
        let mut p = Parser::new(l);

        let program = p.parse_program();

        assert_ne!(
            program.len(),
            0,
            "Parse Error: Couldn't parse the given expression."
        );
        assert_eq!(
            program.len(),
            1,
            "Expected 1 statement, instead received {}",
            program.len()
        );
        println!("{:?}", program);
    }

    #[test]
    fn test_if_expr() {
        let l = Lexer::new("if x<3 {let y =5;} else {let z =7;}");
        let mut p = Parser::new(l);

        let program = p.parse_program();

        assert_ne!(
            program.len(),
            0,
            "Parse Error: Couldn't parse the given expression."
        );
        assert_eq!(
            program.len(),
            1,
            "Expected 1 statement, instead received {}",
            program.len()
        );
        println!("{:?}", program);
    }

    #[test]
    fn test_function_expr() {
        let l = Lexer::new(
            "
        func(x,y){ let y = 5;  return y+2;};
        ",
        );
        let mut p = Parser::new(l);

        let program = p.parse_program();

        assert_ne!(
            program.len(),
            0,
            "Parse Error: Couldn't parse the given expression."
        );
        assert_eq!(
            program.len(),
            1,
            "Expected 1 statement, instead received {}",
            program.len()
        );
        println!("{:?}", program);
    }

    #[test]
    fn test_call_expr() {
        let l = Lexer::new(
            "
        [1,2,3,4][1];
        ",
        );
        let mut p = Parser::new(l);

        let program = p.parse_program();
        println!("{:?}", program);

        assert_ne!(
            program.len(),
            0,
            "Parse Error: Couldn't parse the given expression."
        );
        assert_eq!(
            program.len(),
            1,
            "Expected 1 statement, instead received {}",
            program.len()
        );
    }
}
