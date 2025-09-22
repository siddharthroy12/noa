use std::sync::{Arc, Mutex};

use crate::lox::{
    environment::Environment,
    error::LoxError,
    token::{Token, TokenType},
    types::{Number, Object},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Binary(BinaryExpression),
    Group(GroupExpression),
    Literal(LiteralExpression),
    Variable(VariableExpression),
    Unary(UnaryExpression),
    Ternary(TernaryExpression),
    Assign(AssginExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssginExpression {
    pub token: Token,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableExpression {
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TernaryExpression {
    pub check: Box<Expression>,
    pub if_true: Box<Expression>,
    pub if_false: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupExpression {
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpression {
    pub value: Object,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

impl Expression {
    fn parenthesize(self: &Self, name: &String, expressions: &[Box<Expression>]) -> String {
        let mut res: String = String::new();

        res.push('(');
        res.push_str(&name);

        for expr in expressions {
            res.push(' ');
            res.push_str(&expr.print());
        }

        res.push(')');

        return res;
    }

    fn get_number_object(object: Object, line: usize) -> Result<Number, LoxError> {
        match object {
            Object::Number(n) => {
                return Ok(n);
            }
            _ => {
                return Err(LoxError {
                    line: line,
                    location: object.to_string(),
                    message: format!("{} is not a valid number", object.to_string()),
                });
            }
        }
    }

    pub fn evaluate(self: &Self, environment: Arc<Mutex<Environment>>) -> Result<Object, LoxError> {
        match self {
            Expression::Binary(binary_expression) => {
                let left_value = binary_expression.left.evaluate(environment.clone())?;
                let right_value = binary_expression.right.evaluate(environment.clone())?;

                match binary_expression.operator.token_type {
                    // Equality
                    TokenType::Comma => {
                        return Ok(right_value);
                    }
                    // Equality
                    TokenType::EqualEqual => {
                        return Ok(Object::Bool(left_value.is_equal(&right_value)));
                    }
                    TokenType::BangEqual => {
                        return Ok(Object::Bool(!left_value.is_equal(&right_value)));
                    }
                    // Comparison
                    TokenType::Greater => {
                        return Ok(Object::Bool(left_value.is_greater(&right_value)));
                    }
                    TokenType::GreaterEqual => {
                        return Ok(Object::Bool(
                            left_value.is_greater(&right_value)
                                || left_value.is_equal(&right_value),
                        ));
                    }
                    TokenType::Less => {
                        return Ok(Object::Bool(left_value.is_less(&right_value)));
                    }
                    TokenType::LessEqual => {
                        return Ok(Object::Bool(
                            left_value.is_less(&right_value) || left_value.is_equal(&right_value),
                        ));
                    }

                    // Term
                    TokenType::Plus => {
                        if let Object::String(str) = left_value {
                            return Ok(Object::String(str + &right_value.to_string()));
                        }
                        if let Object::String(str) = right_value {
                            return Ok(Object::String(left_value.to_string() + &str));
                        }

                        let n1 =
                            Self::get_number_object(left_value, binary_expression.operator.line)?;

                        let n2 =
                            Self::get_number_object(right_value, binary_expression.operator.line)?;

                        return Ok(Object::Number(n1 + n2));
                    }
                    TokenType::Minus => {
                        let n1 =
                            Self::get_number_object(left_value, binary_expression.operator.line)?;

                        let n2 =
                            Self::get_number_object(right_value, binary_expression.operator.line)?;

                        return Ok(Object::Number(n1 - n2));
                    }
                    // Factor
                    TokenType::Star => {
                        let n1 =
                            Self::get_number_object(left_value, binary_expression.operator.line)?;

                        let n2 =
                            Self::get_number_object(right_value, binary_expression.operator.line)?;

                        return Ok(Object::Number(n1 * n2));
                    }
                    TokenType::Slash => {
                        let n1 =
                            Self::get_number_object(left_value, binary_expression.operator.line)?;

                        let n2 =
                            Self::get_number_object(right_value, binary_expression.operator.line)?;
                        if n2 == 0.0 {
                            return Err(LoxError {
                                line: binary_expression.operator.line,
                                location: n2.to_string(),
                                message: format!("Cannot divide by zero"),
                            });
                        }
                        return Ok(Object::Number(n1 / n2));
                    }

                    _ => {
                        return Err(LoxError {
                            line: binary_expression.operator.line,
                            location: binary_expression.operator.lexeme.clone(),
                            message: format!("Unknown binary operator"),
                        });
                    }
                }
            }
            Expression::Group(group_expression) => {
                return group_expression.expression.evaluate(environment);
            }
            Expression::Literal(literal_expression) => return Ok(literal_expression.value.clone()),
            Expression::Unary(unary_expression) => {
                let right_value = unary_expression.right.evaluate(environment)?;

                match unary_expression.operator.token_type {
                    TokenType::Bang => {
                        return Ok(Object::Bool(!right_value.is_truthy()));
                    }
                    TokenType::Minus => {
                        let n1 =
                            Self::get_number_object(right_value, unary_expression.operator.line)?;

                        return Ok(Object::Number(-n1));
                    }
                    _ => {
                        return Err(LoxError {
                            line: unary_expression.operator.line,
                            location: unary_expression.operator.lexeme.clone(),
                            message: format!("Unknown unary operator"),
                        });
                    }
                }
            }
            Expression::Ternary(ternary_expression) => {
                let check = ternary_expression.check.evaluate(environment.clone())?;

                if check.is_truthy() {
                    return Ok(ternary_expression.if_true.evaluate(environment.clone())?);
                } else {
                    return Ok(ternary_expression.if_false.evaluate(environment.clone())?);
                }
            }
            Expression::Assign(assgin_expression) => {
                let value = assgin_expression
                    .expression
                    .evaluate(environment.clone())?
                    .clone();
                match environment.lock() {
                    Ok(mut mutex) => {
                        mutex.assign(&assgin_expression.token, value.clone())?;
                    }
                    Err(_) => {
                        return Err(LoxError {
                            line: assgin_expression.token.line,
                            location: assgin_expression.token.lexeme.clone(),
                            message: format!("Failed to get local scope memory"),
                        });
                    }
                }
                return Ok(value);
            }
            Expression::Variable(variable_expression) => match environment.lock() {
                Ok(mut mutex) => {
                    let value = mutex.get(&variable_expression.token)?;
                    return Ok(value);
                }
                Err(_) => {
                    return Err(LoxError {
                        line: variable_expression.token.line,
                        location: variable_expression.token.lexeme.clone(),
                        message: format!("Failed to get local scope memory"),
                    });
                }
            },
        }
    }
    pub fn print(self: &Self) -> String {
        match self {
            Expression::Ternary(ternary) => {
                return self.parenthesize(
                    &String::from("ternary"),
                    &[
                        ternary.check.clone(),
                        ternary.if_false.clone(),
                        ternary.if_true.clone(),
                    ],
                );
            }
            Expression::Binary(binary) => {
                return self.parenthesize(
                    &binary.operator.lexeme,
                    &[binary.left.clone(), binary.right.clone()],
                );
            }
            Expression::Group(group) => {
                return self.parenthesize(&String::from("group"), &[group.expression.clone()]);
            }
            Expression::Literal(literal) => return literal.value.to_string(),
            Expression::Unary(unary) => {
                return self.parenthesize(&unary.operator.lexeme, &[unary.right.clone()]);
            }
            Expression::Assign(assgin_expression) => {
                return self.parenthesize(
                    &format!("= {}", assgin_expression.token.lexeme),
                    &[assgin_expression.expression.clone()],
                );
            }
            Expression::Variable(variable_expression) => {
                return format!("var {}", variable_expression.token.lexeme);
            }
        }
    }
}
