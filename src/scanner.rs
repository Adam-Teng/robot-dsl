use crate::error::error;
use crate::token::{Token, TokenType, KEYWORDS};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "", self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    self.block_comment();
                }
            }
            '*' => {}
            ' ' | '\r' | '\t' => (), // Ignore whitespace
            '\n' => self.line += 1,
            '"' => self.string(),
            c => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    error(self.line, "Unexpected character.");
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        // See if the identifier is a reserved word.
        let text = self
            .source
            .get(self.start..self.current)
            .expect("Unexpected end.");

        let tpe: TokenType = KEYWORDS.get(text).cloned().unwrap_or(TokenType::Identifier);
        self.add_token(tpe);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consumer the ".".
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let n: f64 = self
            .source
            .get(self.start..self.current)
            .expect("Unexpected end.")
            .parse()
            .expect("Scanned number could not be parsed.");
        self.add_token(TokenType::Number { literal: n })
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        // Unterminated string.
        if self.is_at_end() {
            error(self.line, "Unterminated string.");
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let literal = self
            .source
            .get((self.start + 1)..(self.current - 1))
            .expect("Unexpected end.")
            .to_string();
        self.add_token(TokenType::String { literal });
    }

    fn block_comment(&mut self) {
        while !self.is_at_end() {
            let c = self.advance();
            match c {
                '/' => {
                    if self.match_char('*') {
                        self.block_comment();
                    }
                }
                '*' => {
                    if self.match_char('/') {
                        return;
                    }
                }
                '\n' => {
                    self.line += 1;
                }
                _ => { break; },
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self
            .source
            .chars()
            .nth(self.current)
            .expect("Unexpected end of source.")
            != expected
        {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let char_vec: Vec<char> = self.source.chars().collect();
        char_vec[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, tpe: TokenType) {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("Source token is empty.");
        self.tokens.push(Token::new(tpe, text, self.line))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scan_tokens_keywords() {
        let source = "true false speak listen branch step input var exit".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].tpe, TokenType::True);
        assert_eq!(tokens[1].tpe, TokenType::False);
        assert_eq!(tokens[2].tpe, TokenType::Speak);
        assert_eq!(tokens[3].tpe, TokenType::Listen);
        assert_eq!(tokens[4].tpe, TokenType::Branch);
        assert_eq!(tokens[5].tpe, TokenType::Step);
        assert_eq!(tokens[6].tpe, TokenType::Input);
        assert_eq!(tokens[7].tpe, TokenType::Var);
        assert_eq!(tokens[8].tpe, TokenType::Exit);
        assert_eq!(tokens[9].tpe, TokenType::EOF);
    }

    #[test]
    fn test_scan_tokens_operators() {
        let source = "!= == = + -".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].tpe, TokenType::BangEqual);
        assert_eq!(tokens[1].tpe, TokenType::EqualEqual);
        assert_eq!(tokens[2].tpe, TokenType::Equal);
        assert_eq!(tokens[3].tpe, TokenType::Plus);
        assert_eq!(tokens[4].tpe, TokenType::Minus);
        assert_eq!(tokens[5].tpe, TokenType::EOF);
    }

    #[test]
    fn test_scan_tokens_literals() {
        let source = "\"hello\" 123.45".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(
            tokens[0].tpe,
            TokenType::String {
                literal: "hello".to_string()
            }
        );
        assert_eq!(tokens[1].tpe, TokenType::Number { literal: 123.45 });
        assert_eq!(tokens[2].tpe, TokenType::EOF);
    }

    #[test]
    fn test_scan_tokens_identifiers() {
        let source = "hello world".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].tpe, TokenType::Identifier);
        assert_eq!(tokens[1].tpe, TokenType::Identifier);
        assert_eq!(tokens[2].tpe, TokenType::EOF);
    }

    #[test]
    fn test_scan_tokens_comments() {
        let source = "/* hello 
        /* world 
        */ */".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[2].tpe, TokenType::EOF);
    }

    #[test]
    fn test_scan_tokens() {
        let source = "
            var hello = \"world\"
            var a = 1
            var b = 2
            var c = a + b
            speak c
            input d
            var e = d - c
            var f = true
            var g = false
            step welcome
                branch cat
            step cat
                speak \"meow\"
                exit
            speak hello
            exit
            ";
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 49);
        assert_eq!(tokens[0].tpe, TokenType::Var);
        assert_eq!(tokens[1].tpe, TokenType::Identifier);
        assert_eq!(tokens[2].tpe, TokenType::Equal);
        assert_eq!(
            tokens[3].tpe,
            TokenType::String {
                literal: "world".to_string()
            }
        );
        assert_eq!(tokens[4].tpe, TokenType::Var);
        assert_eq!(tokens[5].tpe, TokenType::Identifier);
        assert_eq!(tokens[6].tpe, TokenType::Equal);
        assert_eq!(tokens[7].tpe, TokenType::Number { literal: 1.0 });
        assert_eq!(tokens[8].tpe, TokenType::Var);
        assert_eq!(tokens[9].tpe, TokenType::Identifier);
        assert_eq!(tokens[10].tpe, TokenType::Equal);
        assert_eq!(tokens[11].tpe, TokenType::Number { literal: 2.0 });
        assert_eq!(tokens[12].tpe, TokenType::Var);
        assert_eq!(tokens[13].tpe, TokenType::Identifier);
        assert_eq!(tokens[14].tpe, TokenType::Equal);
        assert_eq!(tokens[15].tpe, TokenType::Identifier);
        assert_eq!(tokens[16].tpe, TokenType::Plus);
        assert_eq!(tokens[17].tpe, TokenType::Identifier);
        assert_eq!(tokens[18].tpe, TokenType::Speak);
        assert_eq!(tokens[19].tpe, TokenType::Identifier);
        assert_eq!(tokens[20].tpe, TokenType::Input);
        assert_eq!(tokens[21].tpe, TokenType::Identifier);
        assert_eq!(tokens[22].tpe, TokenType::Var);
        assert_eq!(tokens[23].tpe, TokenType::Identifier);
        assert_eq!(tokens[24].tpe, TokenType::Equal);
        assert_eq!(tokens[25].tpe, TokenType::Identifier);
        assert_eq!(tokens[26].tpe, TokenType::Minus);
        assert_eq!(tokens[27].tpe, TokenType::Identifier);
        assert_eq!(tokens[28].tpe, TokenType::Var);
        assert_eq!(tokens[29].tpe, TokenType::Identifier);
        assert_eq!(tokens[30].tpe, TokenType::Equal);
        assert_eq!(tokens[31].tpe, TokenType::True);
        assert_eq!(tokens[32].tpe, TokenType::Var);
        assert_eq!(tokens[33].tpe, TokenType::Identifier);
        assert_eq!(tokens[34].tpe, TokenType::Equal);
        assert_eq!(tokens[35].tpe, TokenType::False);
        assert_eq!(tokens[36].tpe, TokenType::Step);
        assert_eq!(tokens[37].tpe, TokenType::Identifier);
        assert_eq!(tokens[38].tpe, TokenType::Branch);
        assert_eq!(tokens[39].tpe, TokenType::Identifier);
        assert_eq!(tokens[40].tpe, TokenType::Step);
        assert_eq!(tokens[41].tpe, TokenType::Identifier);
        assert_eq!(tokens[42].tpe, TokenType::Speak);
        assert_eq!(
            tokens[43].tpe,
            TokenType::String {
                literal: "meow".to_string()
            }
        );
        assert_eq!(tokens[44].tpe, TokenType::Exit);
        assert_eq!(tokens[45].tpe, TokenType::Speak);
        assert_eq!(tokens[46].tpe, TokenType::Identifier);
        assert_eq!(tokens[47].tpe, TokenType::Exit);
        assert_eq!(tokens[48].tpe, TokenType::EOF);
    }
}
