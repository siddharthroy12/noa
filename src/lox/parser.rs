use std::collections::HashMap;

use crate::lox::error::LoxError;
use crate::lox::expression::{
    AssginExpression, BinaryExpression, CallExpression, Expression, GroupExpression,
    KeyAccessAssignExpression, KeyAccessExpression, LiteralExpression, LogicalExpression,
    TableExpression, TernaryExpression, UnaryExpression, VariableExpression,
};
use crate::lox::statement::{
    BlockStatement, ExpressionStatement, FunctionStatement, IfStatement, ReturnStatement,
    Statement, VarStatement, WhileStatement,
};
use crate::lox::token::{Token, TokenType};
use crate::lox::types::Object;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Parser { tokens, current: 0 };
    }

    pub fn parse(self: &mut Self) -> Result<Vec<Statement>, LoxError> {
        let mut statements: Vec<Statement> = Vec::new();

        loop {
            if self.is_at_end() {
                break;
            }
            match self.parse_declaration() {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(err) => {
                    return Err(LoxError {
                        line: self.peek().line,
                        location: format!(
                            "{}",
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

        return Ok(statements);
    }

    pub fn parse_declaration(self: &mut Self) -> Result<Statement, String> {
        if self.match_token_types(&[TokenType::Var]) {
            return self.parse_var_declaration();
        }
        if self.match_token_types(&[TokenType::Fun]) {
            return self.parse_func_declaration("function".to_owned());
        }

        return self.parse_statement();
    }

    pub fn parse_var_declaration(self: &mut Self) -> Result<Statement, String> {
        let identifier = self
            .consume(TokenType::Identifier, "Invalid identifier".to_string())?
            .clone();
        let mut initializer: Option<Box<Expression>> = None;
        if self.match_token_types(&[TokenType::Equal]) {
            let expression = self.parse_expression()?;
            initializer = Some(Box::new(expression));
        }
        self.consume(TokenType::Semicolon, "Expect ';' after value".to_string())?;

        return Ok(Statement::Var(VarStatement {
            initializer,
            identifier: identifier,
        }));
    }

    pub fn parse_func_declaration(self: &mut Self, kind: String) -> Result<Statement, String> {
        let name = self
            .consume(TokenType::Identifier, format!("Expect {} name", kind))?
            .clone();

        self.consume(TokenType::LeftParen, "Expect ( after name".to_owned())?;

        let mut parameters: Vec<Token> = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err("Cannot have more than 255 parameters".to_owned());
                }

                parameters.push(
                    self.consume(TokenType::Identifier, "Expect parameter name".to_owned())?
                        .clone(),
                );

                if !self.match_token_types(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(
            TokenType::RightParen,
            "Expect ) after parameters".to_owned(),
        )?;
        self.consume(TokenType::LeftBrace, "Expect { after )".to_owned())?;
        let body = self.parse_block_statement()?;
        return Ok(Statement::Function(FunctionStatement {
            name: name.clone(),
            params: parameters,
            body: Box::new(body),
        }));
    }

    pub fn parse_statement(self: &mut Self) -> Result<Statement, String> {
        if self.match_token_types(&[TokenType::LeftBrace]) {
            return self.parse_block_statement();
        }

        if self.match_token_types(&[TokenType::If]) {
            return self.parse_if_statement();
        }

        if self.match_token_types(&[TokenType::While]) {
            return self.parse_while_statement();
        }

        if self.match_token_types(&[TokenType::For]) {
            return self.parse_for_statement();
        }

        if self.match_token_types(&[TokenType::Return]) {
            return self.parse_return_statement();
        }

        return self.parse_expression_statement();
    }

    pub fn parse_return_statement(self: &mut Self) -> Result<Statement, String> {
        let token = self.previous().clone();
        let mut expr = Expression::Literal(LiteralExpression { value: Object::Nil });
        if !self.check(&TokenType::Semicolon) {
            expr = self.parse_expression()?;
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ; after return value".to_owned(),
        )?;

        return Ok(Statement::Return(ReturnStatement {
            keyword: token,
            value: Box::new(expr),
        }));
    }

    pub fn parse_if_statement(self: &mut Self) -> Result<Statement, String> {
        self.consume(TokenType::LeftParen, "Expect ( after if.".to_string())?;

        let expression = self.parse_expression()?;

        self.consume(
            TokenType::RightParen,
            "Expect ) after expression".to_string(),
        )?;

        let if_true = self.parse_statement()?;

        if self.match_token_types(&[TokenType::Else]) {
            let if_false = self.parse_statement()?;

            return Ok(Statement::If(IfStatement {
                check: Box::new(expression),
                if_true: Box::new(if_true),
                if_false: Some(Box::new(if_false)),
            }));
        }

        return Ok(Statement::If(IfStatement {
            check: Box::new(expression),
            if_true: Box::new(if_true),
            if_false: None,
        }));
    }

    pub fn parse_while_statement(self: &mut Self) -> Result<Statement, String> {
        self.consume(TokenType::LeftParen, "Expect ( after while.".to_string())?;

        let expression = self.parse_expression()?;

        self.consume(
            TokenType::RightParen,
            "Expect ) after expression".to_string(),
        )?;

        let if_true = self.parse_statement()?;

        return Ok(Statement::While(WhileStatement {
            check: Box::new(expression),
            if_true: Box::new(if_true),
        }));
    }

    pub fn parse_for_statement(self: &mut Self) -> Result<Statement, String> {
        self.consume(TokenType::LeftParen, "Expect ( after for.".to_string())?;

        let mut initializer: Statement = Statement::Expression(ExpressionStatement {
            expression: Box::new(Expression::Literal(LiteralExpression {
                value: Object::Nil,
            })),
        });
        let mut check: Expression = Expression::Literal(LiteralExpression {
            value: Object::Bool(true),
        });

        let mut post_loop: Expression = Expression::Literal(LiteralExpression {
            value: Object::Bool(true),
        });

        // First part
        if !self.match_token_types(&[TokenType::Semicolon]) {
            if self.match_token_types(&[TokenType::Var]) {
                initializer = self.parse_var_declaration()?;
            } else {
                initializer = self.parse_expression_statement()?;
            }
        }

        // Second part
        if !self.match_token_types(&[TokenType::Semicolon]) {
            check = self.parse_expression()?;
            self.consume(
                TokenType::Semicolon,
                "Expect ; after expression".to_string(),
            )?;
        }

        // Third part
        if !self.match_token_types(&[TokenType::RightParen]) {
            post_loop = self.parse_expression()?;
            self.consume(
                TokenType::RightParen,
                "Expect ) after expression".to_string(),
            )?;
        }

        let if_true = self.parse_statement()?;

        return Ok(Statement::Block(BlockStatement {
            statements: vec![
                initializer,
                Statement::While(WhileStatement {
                    check: Box::new(check),
                    if_true: Box::new(Statement::Block(BlockStatement {
                        statements: vec![
                            if_true,
                            Statement::Expression(ExpressionStatement {
                                expression: Box::new(post_loop),
                            }),
                        ],
                    })),
                }),
            ],
        }));
    }

    pub fn parse_block_statement(self: &mut Self) -> Result<Statement, String> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect } after block.".to_owned())?;

        return Ok(Statement::Block(BlockStatement {
            statements: statements,
        }));
    }

    pub fn parse_expression_statement(self: &mut Self) -> Result<Statement, String> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value".to_string())?;

        return Ok(Statement::Expression(ExpressionStatement {
            expression: Box::new(expr),
        }));
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
    fn previous(self: &Self) -> Token {
        return self.tokens[self.current - 1].clone();
    }
    fn advance(self: &mut Self) -> Token {
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
        return self.parse_assignment();
    }

    fn parse_assignment(self: &mut Self) -> Result<Expression, String> {
        let expr = self.parse_or()?;

        if self.match_token_types(&[TokenType::Equal]) {
            let value = self.parse_assignment()?;
            let expr_clone = expr.clone();

            match expr {
                Expression::Variable(variable) => {
                    return Ok(Expression::Assign(AssginExpression {
                        token: variable.token.clone(),
                        expression: Box::new(value),
                    }));
                }
                Expression::KeyAccess(_) => {
                    return Ok(Expression::KeyAccessAssign(KeyAccessAssignExpression {
                        key_access: Box::new(expr_clone),
                        expression: Box::new(value),
                    }));
                }
                _ => return Err("Invalid assignment target".to_string()),
            }
        }
        return Ok(expr);
    }

    fn parse_or(self: &mut Self) -> Result<Expression, String> {
        let mut expr = self.parse_and()?;

        while self.match_token_types(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.parse_and()?;
            expr = Expression::Logical(LogicalExpression {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            })
        }

        return Ok(expr);
    }

    fn parse_and(self: &mut Self) -> Result<Expression, String> {
        let mut expr = self.parse_ternary()?;

        while self.match_token_types(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.parse_ternary()?;
            expr = Expression::Logical(LogicalExpression {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            })
        }

        return Ok(expr);
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
            let right = self.parse_call()?;
            return Ok(Expression::Unary(UnaryExpression {
                operator: operator,
                right: Box::new(right),
            }));
        }

        return self.parse_key_access();
    }

    fn parse_key_access(self: &mut Self) -> Result<Expression, String> {
        let mut expression = self.parse_call()?;
        if self.match_token_types(&[TokenType::LeftSquareBracket]) {
            let key = self.parse_expression()?;
            self.consume(
                TokenType::RightSqureBracket,
                "Expect ] after arguments.".to_owned(),
            )?;
            return Ok(Expression::KeyAccess(KeyAccessExpression {
                target: Box::new(expression),
                left_bracket: self.previous().clone(),
                key: Box::new(key),
            }));
        }

        return Ok(expression);
    }

    fn parse_call(self: &mut Self) -> Result<Expression, String> {
        let mut expression = self.parse_primary()?;

        loop {
            if self.match_token_types(&[TokenType::LeftParen]) {
                expression = self.finish_call(expression)?;
            } else {
                break;
            }
        }

        return Ok(expression);
    }

    fn finish_call(self: &mut Self, expression: Expression) -> Result<Expression, String> {
        let mut arguments: Vec<Expression> = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err("Can't have more than 255 arguments".to_owned());
                }
                arguments.push(self.parse_expression()?);
                if !self.match_token_types(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(
            TokenType::RightParen,
            "Expect ) after arguments.".to_owned(),
        )?;

        return Ok(Expression::Call(CallExpression {
            callee: Box::new(expression),
            paren: paren.clone(),
            arguments: arguments,
        }));
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

        if self.match_token_types(&[TokenType::Identifier]) {
            return Ok(Expression::Variable(VariableExpression {
                token: self.previous().clone(),
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
        if self.match_token_types(&[TokenType::LeftBrace]) {
            return self.parse_table();
        }

        return Err(format!("Unexpected token"));
    }

    fn parse_table(self: &mut Self) -> Result<Expression, String> {
        let mut map: HashMap<String, Expression> = HashMap::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let key = self.consume(TokenType::String, "Expect string as a key".to_owned())?;
            self.consume(TokenType::Colon, "Expect ':' after key".to_owned())?;

            let expr = self.parse_expression().clone()?;
            self.consume(TokenType::Comma, "Expect ',' after value".to_owned())?;

            map.insert(key.litral.to_string(), expr);
        }

        self.consume(
            TokenType::RightBrace,
            "Expect '}' at the end of the table".to_owned(),
        )?;

        return Ok(Expression::Table(TableExpression { values: map }));
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

    fn consume(self: &mut Self, token_type: TokenType, message: String) -> Result<Token, String> {
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
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }
}
