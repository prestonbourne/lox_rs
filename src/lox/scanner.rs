use super::token::{Literal, Token, TokenType};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error {
    line: usize,
    col: usize,
    message: String,
}

/// Represents a scanner for the Lox programming language.
/// Lexer that scans the source code and returns a list of tokens.
///
/// We currently ignore whitespace and comments.
pub struct Scanner {
    // Scanner fields here
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    err: Option<Error>,
    current: usize,
    line: usize,
    col: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Self {
        let keywords = vec![
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ]
        .into_iter()
        .map(|(k, v)| (String::from(k), v))
        .collect();

        Scanner {
            source,
            tokens: vec![],
            current: 0,
            line: 1,
            err: None,
            col: 0,
            start: 0,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
            self.col,
        ));

        match self.err.clone() {
            Some(e) => Err(e),
            None => Ok(self.tokens.clone()),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.col += 1;

        let cur = char::from(self.source[self.current - 1]);
        cur
    }

    fn scan_token(&mut self) {
        let char = self.advance();

        match char {
            // Single character tokens
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),

            // One or two character tokens
            '!' => {
                if self.matches_next('=') {
                    self.advance();
                    self.add_token(TokenType::BangEqual, None);
                } else {
                    self.add_token(TokenType::Bang, None);
                }
            }

            '=' => {
                if self.matches_next('=') {
                    self.advance();
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            }

            '<' => {
                if self.matches_next('=') {
                    self.advance();
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }

            '>' => {
                if self.matches_next('=') {
                    self.advance();
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }

            '/' => {
                if self.matches_next('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }

            // Ignore whitespace
            ' ' | '\r' | '\t' => {}

            // digits
            '0'..='9' => self.number(),

            // strings
            '"' => self.string(),

            // identifiers
            c if is_identifier(c) => self.identifier(),

            '\n' => self.newline(),

            _ => {
                // lox::error(self.line, "Unexpected character.");
                todo!("Handle unexpected character")
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let next_char = char::from(self.source[self.current]);

        next_char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        let next_char = char::from(self.source[self.current + 1]);

        next_char
    }

    fn is_identifier(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let byte_slice = self.source[self.start..self.current].to_vec();
        let text = String::from_utf8(byte_slice.to_vec())
            .expect("Unexpected Error: Invalid UTF-8 in source code.");

        let new_token = Token::new(
            token_type,
            text,
            literal.map(|lit| lit),
            self.line,
            self.col,
        );

        self.tokens.push(new_token);
    }

    fn matches_next(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek() == expected;
    }

    fn newline(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            // supports multi-line strings
            if self.peek() == '\n' {
                self.newline();
            }
            self.advance();
        }

        // Unterminated string.
        if self.is_at_end() {
            // lox::error(self.line, "Unterminated string.");
            todo!("Handle unterminated string")
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[(self.start + 1)..self.current - 1].to_vec();
        let text =
            String::from_utf8(value).expect("Unexpected Error: Invalid UTF-8 in source code.");

        self.add_token(TokenType::String, Some(Literal::Str(text)));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the "."
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }
  

        let val: f64 = String::from_utf8(self.source[self.start..self.current].to_vec())
            .expect("Unexpected Error: Invalid UTF-8 in source code.")
            .parse()
            .unwrap();

        self.add_token(TokenType::Number, Some(Literal::Number(val)));
    }

    fn identifier(&mut self) {
        while is_identifier(self.peek()) {
            self.advance();
        }

        let value = self.source[self.start..self.current].to_vec();
        let text =
            String::from_utf8(value).expect("Unexpected Error: Invalid UTF-8 in source code.");

        let token_type = self.keywords.get(&text).unwrap_or(&TokenType::Identifier);

        self.add_token(token_type.clone(), None);
    }
}

fn is_identifier(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}
