use crate::lox::{
    error::LoxError,
    token::{Token, TokenType},
    types::{Number, Object},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Binary(BinaryExpression),
    Group(GroupExpression),
    Literal(LiteralExpression),
    Unary(UnaryExpression),
    Ternary(TernaryExpression),
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
                    location: format!(" At '{}'", object.to_string()),
                    message: format!("{} is not a valid number", object.to_string()),
                });
            }
        }
    }

    pub fn evaluate(self: &Self) -> Result<Object, LoxError> {
        match self {
            Expression::Binary(binary_expression) => {
                let left_value = binary_expression.left.evaluate()?;
                let right_value = binary_expression.right.evaluate()?;

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
                        if (n2 == 0.0) {
                            return Err(LoxError {
                                line: binary_expression.operator.line,
                                location: format!(" At '{}'", n2),
                                message: format!("Cannot divide by zero"),
                            });
                        }
                        return Ok(Object::Number(n1 / n2));
                    }

                    _ => {
                        return Err(LoxError {
                            line: binary_expression.operator.line,
                            location: format!("At {}", binary_expression.operator.lexeme),
                            message: format!("Unknown binary operator"),
                        });
                    }
                }
            }
            Expression::Group(group_expression) => {
                return group_expression.expression.evaluate();
            }
            Expression::Literal(literal_expression) => return Ok(literal_expression.value.clone()),
            Expression::Unary(unary_expression) => {
                let right_value = unary_expression.right.evaluate()?;

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
                            location: format!("At {}", unary_expression.operator.lexeme),
                            message: format!("Unknown unary operator"),
                        });
                    }
                }
            }
            Expression::Ternary(ternary_expression) => {
                let check = ternary_expression.check.evaluate()?;

                if check.is_truthy() {
                    return Ok(ternary_expression.if_true.evaluate()?);
                } else {
                    return Ok(ternary_expression.if_false.evaluate()?);
                }
            }
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
        }
    }
}
