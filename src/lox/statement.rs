use crate::lox::{error::LoxError, expression::Expression};

pub enum Statement {
    ExpressionStatement(ExpressionStatement),
    PrintStatement(PrintStatement),
}

pub struct ExpressionStatement {
    pub expression: Box<Expression>,
}

pub struct PrintStatement {
    pub expression: Box<Expression>,
}

impl Statement {
    pub fn execute(self: &Self) -> Result<(), LoxError> {
        match self {
            Statement::ExpressionStatement(expression_statement) => {
                expression_statement.expression.evaluate()?;
                Ok(())
            }
            Statement::PrintStatement(print_statement) => {
                let value = print_statement.expression.evaluate()?;
                println!("{}", value.to_string());
                Ok(())
            }
        }
    }
}
