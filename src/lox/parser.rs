use std::primitive;

use crate::lox::error::LoxError;
use crate::lox::expression::{
    BinaryExpression, Expression, GroupExpression, LiteralExpression, UnaryExpression,
};
use crate::lox::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    failed: bool,
    failed_message: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Parser {
            tokens,
            current: 0,
            failed: false,
            failed_message: String::new(),
        };
    }

    pub fn generateTree(self: &mut Self) -> Result<Expression, LoxError> {
        let expression = self.parse_expression();

        if self.failed {
            return Err(LoxError {
                line: self.peek().line,
                location: format!(" at {}", self.peek().lexeme),
                message: self.failed_message.clone(),
            });
        } else {
            return Ok(expression);
        }
    }
    fn is_at_end(self: &Self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }
    fn peek(self: &Self) -> &Token {
        return &self.tokens[self.current];
    }
    fn check(self: &Self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == *token_type;
    }
    fn previous(self: &Self) -> &Token {
        return &self.tokens[self.current - 1];
    }
    fn advance(self: &mut Self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }
    fn match_token_types(self: &mut Self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn parse_expression(self: &mut Self) -> Expression {
        print!("here");
        return self.parse_equality();
    }

    fn parse_equality(self: &mut Self) -> Expression {
        let mut expr: Expression = self.parse_comparison();

        while self.match_token_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right: Expression = self.parse_comparison();
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            })
        }

        return expr;
    }

    fn parse_comparison(self: &mut Self) -> Expression {
        let mut expr: Expression = self.parse_term();

        while self.match_token_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.parse_term();
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            })
        }

        return expr;
    }

    fn parse_term(self: &mut Self) -> Expression {
        let mut expr: Expression = self.parse_factor();

        while self.match_token_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.parse_term();
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            })
        }

        return expr;
    }

    fn parse_factor(self: &mut Self) -> Expression {
        let mut expr: Expression = self.parse_unary();

        while self.match_token_types(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.parse_unary();
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            })
        }

        return expr;
    }

    fn parse_unary(self: &mut Self) -> Expression {
        if self.match_token_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.parse_unary();
            return Expression::Unary(UnaryExpression {
                operator: operator,
                right: Box::new(right),
            });
        }

        return self.parse_primary();
    }

    fn parse_primary(self: &mut Self) -> Expression {
        match self.peek().token_type {
            TokenType::False => {
                return Expression::Literal(LiteralExpression {
                    value: Some(super::types::Object::Bool(true)),
                });
            }
            TokenType::True => {
                return Expression::Literal(LiteralExpression {
                    value: Some(super::types::Object::Bool(false)),
                });
            }
            TokenType::Nil => {
                return Expression::Literal(LiteralExpression { value: None });
            }
            TokenType::Number => {
                return Expression::Literal(LiteralExpression {
                    value: self.previous().litral.clone(),
                });
            }
            TokenType::String => {
                return Expression::Literal(LiteralExpression {
                    value: self.previous().litral.clone(),
                });
            }
            TokenType::LeftParen => {
                let expr = self.parse_expression();
                match self.consume(
                    TokenType::RightParen,
                    "Expect ')' after expression.".to_string(),
                ) {
                    Some(_) => {
                        return Expression::Group(GroupExpression {
                            expression: Box::new(expr),
                        });
                    }
                    None => {
                        // We are going to stop right here
                    }
                }
            }
            _ => {}
        }
        return Expression::Literal(LiteralExpression { value: None });
    }

    fn consume(self: &mut Self, token_type: TokenType, message: String) -> Option<&Token> {
        if self.check(&token_type) {
            return Some(self.advance());
        }
        self.error(self.peek().clone(), message);
        return None;
    }

    fn error(self: &mut Self, token: Token, message: String) {
        self.failed = true;
        if token.token_type == TokenType::EOF {
            self.failed_message = format!("{} at end", message);
        } else {
            self.failed_message = format!("{} at '{}'", message, token.lexeme);
        }
    }

    fn synchronize(self: &mut Self) {
        self.advance();

        while !self.is_at_end() {
            if (self.previous().token_type == TokenType::Semicolon) {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }
}
