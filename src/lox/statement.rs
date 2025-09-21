use crate::lox::{
    environment::Environment, error::LoxError, expression::Expression, token::Token, types::Object,
};

pub enum Statement {
    ExpressionStatement(ExpressionStatement),
    PrintStatement(PrintStatement),
    VarStatement(VarStatement),
}

pub struct ExpressionStatement {
    pub expression: Box<Expression>,
}

pub struct PrintStatement {
    pub expression: Box<Expression>,
}

pub struct VarStatement {
    pub initializer: Option<Box<Expression>>,
    pub identifier: Token,
}

impl Statement {
    pub fn execute(self: &Self, environment: &mut Environment) -> Result<(), LoxError> {
        match self {
            Statement::ExpressionStatement(expression_statement) => {
                expression_statement.expression.evaluate(environment)?;
                Ok(())
            }
            Statement::PrintStatement(print_statement) => {
                let value = print_statement.expression.evaluate(environment)?;
                println!("{}", value.to_string());
                Ok(())
            }
            Statement::VarStatement(var_statement) => {
                let mut value = Object::Nil;
                if let Some(initializer) = &var_statement.initializer {
                    value = initializer.evaluate(environment)?;
                }
                environment.define(var_statement.identifier.lexeme.clone(), value);
                Ok(())
            }
        }
    }
}
