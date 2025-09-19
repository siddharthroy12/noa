use crate::lox::error::LoxError;
use crate::lox::expression::{
    BinaryExpression, Expression, GroupExpression, LiteralExpression, TernaryExpression,
    UnaryExpression,
};
use crate::lox::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Parser { tokens, current: 0 };
    }

    pub fn generate_tree(self: &mut Self) -> Result<Expression, LoxError> {
        match self.parse_expression() {
            Ok(expression) => {
                return Ok(expression);
            }
            Err(err) => {
                return Err(LoxError {
                    line: self.peek().line,
                    location: format!(
                        " at '{}'",
                        if self.peek().token_type == TokenType::EOF {
                            "eof"
                        } else {
                            &self.peek().lexeme
                        }
                    ),
                    message: err,
                });
            }
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

    fn parse_expression(self: &mut Self) -> Result<Expression, String> {
        return self.parse_ternary();
    }

    fn parse_ternary(self: &mut Self) -> Result<Expression, String> {
        let mut expr = self.parse_equality()?;

        if self.match_token_types(&[TokenType::QuestionMark]) {
            let if_true = self.parse_equality()?;
            if self.match_token_types(&[TokenType::Colon]) {
                let if_false = self.parse_equality()?;
                expr = Expression::Ternary(TernaryExpression {
                    check: Box::new(expr),
                    if_true: Box::new(if_true),
                    if_false: Box::new(if_false),
                })
            } else {
                return Err("Expected : ".to_string());
            }
        }

        return Ok(expr);
    }

    fn parse_equality(self: &mut Self) -> Result<Expression, String> {
        let mut expr: Expression = self.parse_comparison()?;

        while self.match_token_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right: Expression = self.parse_comparison()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            })
        }

        return Ok(expr);
    }

    fn parse_comparison(self: &mut Self) -> Result<Expression, String> {
        let mut expr: Expression = self.parse_term()?;

        while self.match_token_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.parse_term()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            })
        }

        return Ok(expr);
    }

    fn parse_term(self: &mut Self) -> Result<Expression, String> {
        let mut expr: Expression = self.parse_factor()?;

        while self.match_token_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.parse_term()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            })
        }

        return Ok(expr);
    }

    fn parse_factor(self: &mut Self) -> Result<Expression, String> {
        let mut expr: Expression = self.parse_unary()?;

        while self.match_token_types(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.parse_unary()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            })
        }

        return Ok(expr);
    }

    fn parse_unary(self: &mut Self) -> Result<Expression, String> {
        if self.match_token_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.parse_unary()?;
            return Ok(Expression::Unary(UnaryExpression {
                operator: operator,
                right: Box::new(right),
            }));
        }

        return self.parse_primary();
    }

    fn parse_primary(self: &mut Self) -> Result<Expression, String> {
        if self.match_token_types(&[TokenType::False]) {
            return Ok(Expression::Literal(LiteralExpression {
                value: (super::types::Object::Bool(false)),
            }));
        }

        if self.match_token_types(&[TokenType::True]) {
            return Ok(Expression::Literal(LiteralExpression {
                value: (super::types::Object::Bool(true)),
            }));
        }
        if self.match_token_types(&[TokenType::Nil]) {
            return Ok(Expression::Literal(LiteralExpression {
                value: super::types::Object::Nil,
            }));
        }

        if self.match_token_types(&[TokenType::Number, TokenType::String]) {
            return Ok(Expression::Literal(LiteralExpression {
                value: self.previous().litral.clone(),
            }));
        }

        if self.match_token_types(&[TokenType::LeftParen]) {
            let expr = self.parse_comma_operator()?;
            match self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            ) {
                Ok(_) => {
                    return Ok(Expression::Group(GroupExpression {
                        expression: Box::new(expr),
                    }));
                }
                Err(err) => {
                    // We are going to stop right here
                    return Err(err);
                }
            }
        }

        return Err(String::from("Unexpected token"));
    }

    fn parse_comma_operator(self: &mut Self) -> Result<Expression, String> {
        let mut expr = self.parse_expression()?;

        while self.match_token_types(&[TokenType::Comma]) {
            let operator = self.previous().clone();
            let right = self.parse_expression()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            })
        }

        return Ok(expr);
    }

    fn consume(self: &mut Self, token_type: TokenType, message: String) -> Result<&Token, String> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        return Err(message);
    }

    fn synchronize(self: &mut Self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
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
