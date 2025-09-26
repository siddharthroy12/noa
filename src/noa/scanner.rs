use std::collections::HashMap;

use crate::noa::{
    error::NoaError,
    token::TokenType,
    types::{Number, Object},
};

use super::token::Token;

pub struct Scanner {
    keywords: HashMap<String, TokenType>,
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);

        return Scanner {
            keywords: keywords,
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
    }

    pub fn debug_print(self: &Self) {
        println!("---Tokens Start---");
        for (i, token) in self.tokens.iter().enumerate() {
            if token.lexeme.len() > 0 {
                println!("{}: {} {:?}", i, token.lexeme, token.token_type)
            } else {
                println!("{}: {:?}", i, token.token_type)
            }
        }
        println!("---Tokens End---");
    }

    fn is_at_end(self: &Self) -> bool {
        return self.current >= self.source.len();
    }

    pub fn scan_tokens(self: &mut Self) -> Result<(), NoaError> {
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(message) = self.scan_token() {
                return Err(NoaError {
                    line: self.line,
                    location: self.peek().to_string(),
                    message: message,
                });
            }
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            line: self.line,
            lexeme: "".to_owned(),
            litral: Object::Nil,
        });
        return Ok(());
    }

    fn scan_string_literal(self: &mut Self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string".to_string());
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_with_literal(TokenType::String, Object::String(String::from(value)));
        Ok(())
    }

    fn scan_identifier_token(self: &mut Self) -> Result<(), String> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let value = &self.source[self.start..self.current];
        let token_type: TokenType = self
            .keywords
            .get(value)
            .unwrap_or(&TokenType::Identifier)
            .clone();
        self.add_token_with_literal(token_type, Object::String(String::from(value)));
        Ok(())
    }

    fn skip_single_line_comment(self: &mut Self) -> Result<(), String> {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
        Ok(())
    }

    fn skip_multi_line_comment(self: &mut Self) -> Result<(), String> {
        while !(self.peek() == '*' && self.peek_next() == '/') && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            if self.peek() == '/' && self.peek_next() == '*' {
                self.advance();
                self.advance();

                self.skip_multi_line_comment()?;
            } else {
                self.advance();
            }
        }

        if self.is_at_end() {
            return Err(String::from("Unterminated comment"));
        }
        // Consume *
        self.advance();

        if self.is_at_end() {
            return Err(String::from("Unterminated comment"));
        }
        // Consume /
        self.advance();
        Ok(())
    }

    fn scan_number_literal(self: &mut Self) -> Result<(), String> {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let value = &self.source[self.start..self.current];
        let value: Number = match value.parse() {
            Ok(v) => v,
            Err(_) => {
                return Err(String::from("Failed to parse number"));
            }
        };
        self.add_token_with_literal(TokenType::Number, Object::Number(value));
        Ok(())
    }

    fn scan_token(self: &mut Self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftSquareBracket),
            ']' => self.add_token(TokenType::RightSqureBracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '?' => self.add_token(TokenType::QuestionMark),
            ':' => self.add_token(TokenType::Colon),
            '!' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_next_char('/') {
                    self.skip_single_line_comment()?;
                } else if self.match_next_char('*') {
                    self.skip_multi_line_comment()?;
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {} // Ignore the whitespaces
            '\n' => {
                self.line += 1;
            }
            '"' => self.scan_string_literal()?,

            _ => {
                if c.is_digit(10) {
                    self.scan_number_literal()?;
                } else if c.is_alphanumeric() || c == '_' {
                    self.scan_identifier_token()?;
                } else {
                    return Err(format!("Unexpected character '{}'", c));
                }
            }
        }
        return Ok(());
    }

    fn add_token_with_literal(self: &mut Self, token_type: TokenType, literal: Object) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            line: self.line,
            litral: literal,
        });
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Object::Nil);
    }

    fn advance(self: &mut Self) -> char {
        self.current += 1;

        return self.source.chars().nth(self.current - 1).unwrap_or('\0');
    }

    fn peek(self: &mut Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap_or('\0');
    }

    fn peek_next(self: &mut Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap_or('\0');
    }

    fn match_next_char(self: &mut Self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap_or('\0') != expected {
            return false;
        }
        self.current += 1;
        return true;
    }
}
