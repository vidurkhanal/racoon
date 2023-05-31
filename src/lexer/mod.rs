use crate::tkn::{Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    peek_pos: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            pos: 0,
            peek_pos: 0,
            ch: 0,
        };

        lexer.read_char();

        return lexer;
    }

    fn read_char(&mut self) {
        if self.peek_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.peek_pos];
        }
        self.pos = self.peek_pos;
        self.peek_pos += 1;
    }

    fn nextch(&mut self) -> u8 {
        if self.peek_pos >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.peek_pos];
        }
    }

    fn nextch_is(&mut self, ch: u8) -> bool {
        self.nextch() == ch
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::EQ,
                        literal: "==".to_string(),
                    }
                } else {
                    Token {
                        kind: TokenKind::ASSIGN,
                        literal: "=".to_string(),
                    }
                }
            }
            b'+' => Token {
                kind: TokenKind::PLUS,
                literal: "+".to_string(),
            },
            b'-' => Token {
                kind: TokenKind::MINUS,
                literal: "-".to_string(),
            },
            b'!' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::NEQ,
                        literal: "!=".to_string(),
                    }
                } else {
                    Token {
                        kind: TokenKind::BANG,
                        literal: "!".to_string(),
                    }
                }
            }
            b'/' => Token {
                kind: TokenKind::SLASH,
                literal: "/".to_string(),
            },
            b'*' => Token {
                kind: TokenKind::ASTERISK,
                literal: "*".to_string(),
            },
            b'<' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::LTE,
                        literal: "<=".to_string(),
                    }
                } else {
                    Token {
                        kind: TokenKind::LT,
                        literal: "<".to_string(),
                    }
                }
            }
            b'>' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::GTE,
                        literal: ">=".to_string(),
                    }
                } else {
                    Token {
                        kind: TokenKind::GT,
                        literal: ">".to_string(),
                    }
                }
            }
            b'(' => Token {
                kind: TokenKind::LPAREN,
                literal: "(".to_string(),
            },
            b')' => Token {
                kind: TokenKind::RPAREN,
                literal: ")".to_string(),
            },
            b'{' => Token {
                kind: TokenKind::LBRACE,
                literal: "{".to_string(),
            },
            b'}' => Token {
                kind: TokenKind::RBRACE,
                literal: "}".to_string(),
            },
            b'[' => Token {
                kind: TokenKind::LBRACKET,
                literal: "[".to_string(),
            },
            b']' => Token {
                kind: TokenKind::RBRACKET,
                literal: "]".to_string(),
            },
            b',' => Token {
                kind: TokenKind::COMMA,
                literal: ",".to_string(),
            },
            b';' => Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            // b':' => Token{kind: TokenKind::COLON,literal:"".to_string()},
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.consume_identifier();
            }
            b'0'..=b'9' => {
                return self.consume_number();
            }
            b'"' => {
                return self.consume_string();
            }
            b'\n' => {
                if self.nextch_is(b'\n') {
                    Token {
                        kind: TokenKind::BLANK,
                        literal: "\n".to_string(),
                    }
                } else {
                    self.read_char();
                    return self.next_token();
                }
            }
            0 => Token {
                kind: TokenKind::EOF,
                literal: "EOF".to_string(),
            },
            _ => Token {
                kind: TokenKind::ILLEGAL,
                literal: "".to_string(),
            },
        };

        self.read_char();

        return tok;
    }

    fn consume_identifier(&mut self) -> Token {
        let start_pos = self.pos;

        loop {
            match self.ch {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[start_pos..self.pos];

        match literal {
            "func" => Token {
                kind: TokenKind::FUNCTION,
                literal: "func".to_string(),
            },
            "let" => Token {
                kind: TokenKind::LET,
                literal: "let".to_string(),
            },
            "true" => Token {
                kind: TokenKind::TRUE(true),
                literal: "true".to_string(),
            },
            "false" => Token {
                kind: TokenKind::FALSE(false),
                literal: "false".to_string(),
            },
            "if" => Token {
                kind: TokenKind::IF,
                literal: "if".to_string(),
            },
            "else" => Token {
                kind: TokenKind::ELSE,
                literal: "else".to_string(),
            },
            "return" => Token {
                kind: TokenKind::RETURN,
                literal: "return".to_string(),
            },

            _ => Token {
                kind: TokenKind::IDENT,
                literal: literal.to_string(),
            },
        }
    }

    fn consume_number(&mut self) -> Token {
        let start_pos = self.pos;

        loop {
            match self.ch {
                b'0'..=b'9' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[start_pos..self.pos];

        Token {
            kind: TokenKind::INT,
            literal: literal.to_string(),
        }
    }

    fn consume_string(&mut self) -> Token {
        self.read_char();

        let start_pos = self.pos;

        loop {
            match self.ch {
                b'"' | 0 => {
                    let literal = &self.input[start_pos..self.pos];
                    self.read_char();
                    return Token {
                        kind: TokenKind::STRING,
                        literal: literal.to_string(),
                    };
                }
                _ => {
                    self.read_char();
                }
            }
        }
    }
}
#[cfg(test)]
mod lexer_test {
    use super::*;

    #[test]
    fn test_next_token1() {
        let input = "  let x = 5,";
        let tests = vec![
            (TokenKind::LET, "let"),
            (TokenKind::IDENT, "x"),
            (TokenKind::ASSIGN, "="),
            (TokenKind::INT, "5"),
            (TokenKind::COMMA, ","),
            (TokenKind::EOF, " "),
        ];
        let mut l = Lexer::new(input);
        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert_eq!(
                tok.kind, tt.0,
                "tests[{0}] - tokentype wrong. expected={1:?}, got={2:?}",
                i, tt.0, tok.kind
            );
            assert_eq!(
                tok.literal, tt.1,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, tt.1, tok.literal
            );
        }
    }

    #[test]
    fn test_next_token2() {
        let input = "let five = 5;
        let ten = 10;
           let add = func(x, y) {
             x + y;
        };
           let result = add(five, ten);";
        let tests = vec![
            (TokenKind::LET, "let"),
            (TokenKind::IDENT, "five"),
            (TokenKind::ASSIGN, "="),
            (TokenKind::INT, "5"),
            (TokenKind::SEMICOLON, ";"),
            (TokenKind::LET, "let"),
            (TokenKind::IDENT, "ten"),
            (TokenKind::ASSIGN, "="),
            (TokenKind::INT, "10"),
            (TokenKind::SEMICOLON, ";"),
            (TokenKind::LET, "let"),
            (TokenKind::IDENT, "add"),
            (TokenKind::ASSIGN, "="),
            (TokenKind::FUNCTION, "func"),
            (TokenKind::LPAREN, "("),
            (TokenKind::IDENT, "x"),
            (TokenKind::COMMA, ","),
            (TokenKind::IDENT, "y"),
            (TokenKind::RPAREN, ")"),
            (TokenKind::LBRACE, "{"),
            (TokenKind::IDENT, "x"),
            (TokenKind::PLUS, "+"),
            (TokenKind::IDENT, "y"),
            (TokenKind::SEMICOLON, ";"),
            (TokenKind::RBRACE, "}"),
            (TokenKind::SEMICOLON, ";"),
            (TokenKind::LET, "let"),
            (TokenKind::IDENT, "result"),
            (TokenKind::ASSIGN, "="),
            (TokenKind::IDENT, "add"),
            (TokenKind::LPAREN, "("),
            (TokenKind::IDENT, "five"),
            (TokenKind::COMMA, ","),
            (TokenKind::IDENT, "ten"),
            (TokenKind::RPAREN, ")"),
            (TokenKind::SEMICOLON, ";"),
            (TokenKind::EOF, " "),
        ];
        let mut l = Lexer::new(input);
        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert_eq!(
                tok.kind, tt.0,
                "tests[{0}] - tokentype wrong. expected={1:?}, got={2:?}",
                i, tt.0, tok.kind
            );
            assert_eq!(
                tok.literal, tt.1,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, tt.1, tok.literal
            );
        }
    }

    #[test]
    fn test_next_token3() {
        let input = "if x > 4{
            return true;
        };";
        let tests = vec![
            (TokenKind::IF, "if"),
            (TokenKind::IDENT, "x"),
            (TokenKind::GT, ">"),
            (TokenKind::INT, "4"),
            (TokenKind::LBRACE, "{"),
            (TokenKind::RETURN, "return"),
            (TokenKind::TRUE(true), "true"),
            (TokenKind::SEMICOLON, ";"),
            (TokenKind::RBRACE, "}"),
            (TokenKind::SEMICOLON, ";"),
        ];
        let mut l = Lexer::new(input);
        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert_eq!(
                tok.kind, tt.0,
                "tests[{0}] - tokentype wrong. expected={1:?}, got={2:?}",
                i, tt.0, tok.kind
            );
            assert_eq!(
                tok.literal, tt.1,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, tt.1, tok.literal
            );
        }
    }

    #[test]
    fn test_next_token4() {
        let input = "
            !-/*5;
            5 < 10 > 5;

            10 == 10;
            10!=9;
        ";
        let tests = vec![
            (TokenKind::BANG, "!"),
            (TokenKind::MINUS, "-"),
            (TokenKind::SLASH, "/"),
            (TokenKind::ASTERISK, "*"),
            (TokenKind::INT, "5"),
            (TokenKind::SEMICOLON, ";"),
            (TokenKind::INT, "5"),
            (TokenKind::LT, "<"),
            (TokenKind::INT, "10"),
            (TokenKind::GT, ">"),
            (TokenKind::INT, "5"),
            (TokenKind::SEMICOLON, ";"),
            (TokenKind::INT, "10"),
            (TokenKind::EQ, "=="),
            (TokenKind::INT, "10"),
            (TokenKind::SEMICOLON, ";"),
            (TokenKind::INT, "10"),
            (TokenKind::NEQ, "!="),
            (TokenKind::INT, "9"),
            (TokenKind::SEMICOLON, ";"),
            (TokenKind::EOF, " "),
        ];
        let mut l = Lexer::new(input);
        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert_eq!(
                tok.kind, tt.0,
                "tests[{0}] - tokentype wrong. expected={1:?}, got={2:?}",
                i, tt.0, tok.kind
            );
            assert_eq!(
                tok.literal, tt.1,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, tt.1, tok.literal
            );
        }
    }

    #[test]
    fn test_string() {
        let input = "
            \"foobar\"
            [1,2]
        ";
        let tests = vec![
            (TokenKind::STRING, "foobar"),
            (TokenKind::LBRACKET, "["),
            (TokenKind::INT, "1"),
            (TokenKind::COMMA, ","),
            (TokenKind::INT, "2"),
            (TokenKind::RBRACKET, "]"),
        ];
        let mut l = Lexer::new(input);
        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next_token();
            println!("TOKEN--> {:#?}", tok);
            assert_eq!(
                tok.kind,
                tt.0,
                "tests[{0}] - tokentype wrong. expected={1:?}, got={2}",
                i,
                tt.0,
                tok.kind.to_string()
            );
            assert_eq!(
                tok.literal, tt.1,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, tt.1, tok.literal
            );
        }
    }
}
