use std::sync::{Arc, Mutex};

use crate::noa::{
    environment::Environment,
    error::{NoaError, NoaTermination},
    expression::Expression,
    token::Token,
    types::{Function, Object},
};

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(ExpressionStatement),
    Var(VarStatement),
    Block(BlockStatement),
    If(IfStatement),
    While(WhileStatement),
    Function(FunctionStatement),
    Return(ReturnStatement),
}
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub keyword: Token,
    pub value: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct FunctionStatement {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Box<Statement>,
}
#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub check: Box<Expression>,
    pub if_true: Box<Statement>,
}
#[derive(Debug, Clone)]

pub struct IfStatement {
    pub check: Box<Expression>,
    pub if_true: Box<Statement>,
    pub if_false: Option<Box<Statement>>,
}
#[derive(Debug, Clone)]

pub struct BlockStatement {
    pub statements: Vec<Statement>,
}
#[derive(Debug, Clone)]

pub struct ExpressionStatement {
    pub expression: Box<Expression>,
}
#[derive(Debug, Clone)]

pub struct VarStatement {
    pub initializer: Option<Box<Expression>>,
    pub identifier: Token,
}

impl Statement {
    pub fn execute(
        self: &Self,
        environment: Arc<Mutex<Environment>>,
    ) -> Result<(), NoaTermination> {
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
                        return Err(NoaTermination::Error(NoaError {
                            line: var_statement.identifier.line,
                            location: var_statement.identifier.lexeme.clone(),
                            message: format!(
                                "Failed to get local scope memory to assign the value"
                            ),
                        }));
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
                        return Err(NoaTermination::Error(NoaError {
                            line: 0,
                            location: format!("N/A"),
                            message: format!("Unable to create local scope for block"),
                        }));
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
            Statement::Function(function_statement) => {
                let params: Vec<String> = function_statement
                    .params
                    .iter()
                    .map(|p| p.lexeme.to_owned())
                    .collect();

                match environment.lock() {
                    Ok(mut mutex) => {
                        let snapshot = mutex.snapshot()?;

                        let snapshot = Arc::new(Mutex::new(snapshot));

                        let func = Object::Function(Box::new(Function {
                            body: Some(function_statement.body.clone()),
                            params: params,
                            callback: None,
                            environment: snapshot.clone(),
                        }));

                        match snapshot.lock() {
                            Ok(mut mutex) => {
                                mutex.define(function_statement.name.lexeme.clone(), func.clone())
                            }
                            Err(_) => {
                                return Err(NoaTermination::Error(NoaError {
                                    line: 0,
                                    location: "N/A".to_owned(),
                                    message: "Failed to lock environment".to_owned(),
                                }));
                            }
                        }
                        mutex.define(function_statement.name.lexeme.clone(), func)
                    }
                    Err(_) => {
                        return Err(NoaTermination::Error(NoaError {
                            line: function_statement.name.line,
                            location: function_statement.name.lexeme.to_owned(),
                            message: "Failed to lock environment".to_owned(),
                        }));
                    }
                };
                Ok(())
            }
            Statement::Return(return_statement) => {
                let value = return_statement.value.evaluate(environment)?;
                return Err(NoaTermination::Return(value));
            }
        }
    }
}
