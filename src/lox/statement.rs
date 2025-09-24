use std::sync::{Arc, Mutex};

use crate::lox::{
    environment::Environment, error::LoxError, expression::Expression, token::Token, types::Object,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(ExpressionStatement),
    Var(VarStatement),
    Block(BlockStatement),
    If(IfStatement),
    While(WhileStatement),
}
#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub check: Box<Expression>,
    pub if_true: Box<Statement>,
}
#[derive(Debug, Clone, PartialEq)]

pub struct IfStatement {
    pub check: Box<Expression>,
    pub if_true: Box<Statement>,
    pub if_false: Option<Box<Statement>>,
}
#[derive(Debug, Clone, PartialEq)]

pub struct BlockStatement {
    pub statements: Vec<Statement>,
}
#[derive(Debug, Clone, PartialEq)]

pub struct ExpressionStatement {
    pub expression: Box<Expression>,
}
#[derive(Debug, Clone, PartialEq)]

pub struct VarStatement {
    pub initializer: Option<Box<Expression>>,
    pub identifier: Token,
}

impl Statement {
    pub fn execute(self: &Self, environment: Arc<Mutex<Environment>>) -> Result<(), LoxError> {
        match self {
            Statement::Expression(expression_statement) => {
                expression_statement.expression.evaluate(environment)?;
                Ok(())
            }
            Statement::Var(var_statement) => {
                let mut value = Object::Nil;
                if let Some(initializer) = &var_statement.initializer {
                    value = initializer.evaluate(environment.clone())?;
                }
                match environment.lock() {
                    Ok(mut mutex) => {
                        mutex.define(var_statement.identifier.lexeme.clone(), value);
                    }
                    Err(_) => {
                        return Err(LoxError {
                            line: var_statement.identifier.line,
                            location: var_statement.identifier.lexeme.clone(),
                            message: format!(
                                "Failed to get local scope memory to assign the value"
                            ),
                        });
                    }
                }
                Ok(())
            }
            Statement::Block(block_statement) => {
                let scope = Arc::new(Mutex::new(Environment::new()));
                match scope.lock() {
                    Ok(mut mutex) => {
                        mutex.enclose(environment);
                    }
                    Err(_) => {
                        return Err(LoxError {
                            line: 0,
                            location: format!("N/A"),
                            message: format!("Unable to create local scope for block"),
                        });
                    }
                }
                for statement in &block_statement.statements {
                    statement.execute(scope.clone())?;
                }
                Ok(())
            }
            Statement::If(if_statement) => {
                let value = if_statement.check.evaluate(environment.clone())?;

                if value.is_truthy() {
                    return if_statement.if_true.execute(environment.clone());
                } else {
                    if let Some(if_false) = &if_statement.if_false {
                        return if_false.execute(environment.clone());
                    }
                }

                Ok(())
            }
            Statement::While(while_statement) => {
                let mut value: Object = while_statement.check.evaluate(environment.clone())?;

                while value.is_truthy() {
                    while_statement.if_true.execute(environment.clone())?;
                    value = while_statement.check.evaluate(environment.clone())?;
                }

                return Ok(());
            }
        }
    }
}
