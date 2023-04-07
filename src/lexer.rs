use crate::Token::{lookup_ident, Token, TokenKind};

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: String::from(input),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let next_tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let mut literal = String::from(ch as char);
                    literal.push(self.ch as char);
                    Token::new(TokenKind::EQ, literal)
                } else {
                    Token::new(TokenKind::ASSIGN, String::from(self.ch as char))
                }
            }
            b';' => Token::new(TokenKind::SEMICOLON, String::from(self.ch as char)),
            b'(' => Token::new(TokenKind::LPAREN, String::from(self.ch as char)),
            b')' => Token::new(TokenKind::RPAREN, String::from(self.ch as char)),
            b',' => Token::new(TokenKind::COMMA, String::from(self.ch as char)),
            b'+' => Token::new(TokenKind::PLUS, String::from(self.ch as char)),
            b'{' => Token::new(TokenKind::LBRACE, String::from(self.ch as char)),
            b'}' => Token::new(TokenKind::RBRACE, String::from(self.ch as char)),
            b'-' => Token::new(TokenKind::MINUS, String::from(self.ch as char)),
            b'!' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let mut literal = String::from(ch as char);
                    literal.push(self.ch as char);
                    Token::new(TokenKind::NEQ, literal)
                } else {
                    Token::new(TokenKind::BANG, String::from(self.ch as char))
                }
            }
            b'*' => Token::new(TokenKind::ASTERISK, String::from(self.ch as char)),
            b'/' => Token::new(TokenKind::SLASH, String::from(self.ch as char)),
            b'>' => Token::new(TokenKind::GT, String::from(self.ch as char)),
            b'<' => Token::new(TokenKind::LT, String::from(self.ch as char)),
            _ => {
                if Self::is_letter(self.ch) {
                    let ident = self.read_identifier();
                    self.position -= 1;
                    self.read_position -= 1;
                    Token::new(lookup_ident(&ident), ident.clone())
                } else if Self::is_digit(self.ch) {
                    let int = self.read_number();
                    self.position -= 1;
                    self.read_position -= 1;
                    Token::new(TokenKind::INT, int)
                } else {
                    Token::new(TokenKind::EOF, " ".into())
                }
            }
        };

        self.read_char();

        next_tok
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\n' || self.ch == b'\t' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;

        while Self::is_letter(self.ch) {
            self.read_char();
        }

        String::from(&self.input[pos..self.position])
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;

        while Self::is_digit(self.ch) {
            self.read_char();
        }

        String::from(&self.input[pos..self.position])
    }

    fn is_letter(char: u8) -> bool {
        (b'a' <= char && char <= b'z') || (b'A' <= char && char <= b'Z') || char == b'_'
    }

    fn is_digit(char: u8) -> bool {
        b'0' <= char && char <= b'9'
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
            (TokenKind::TRUE, "true"),
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
}
