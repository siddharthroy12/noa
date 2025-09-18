use std::any::Any;

use crate::lox::{token::Token, types::Object};

enum Expression {
    Binary(BinaryExpression),
    Group(GroupExpression),
    Literal(LiteralExpression),
    Unary(UnaryExpression),
}

struct BinaryExpression {
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
}

struct GroupExpression {
    expression: Box<Expression>,
}

struct LiteralExpression {
    value: Option<Object>,
}

struct UnaryExpression {
    operator: Token,
    right: Box<Expression>,
}

impl Expression {
    fn parenthesize(self: &Self, name: &String, expressions: &[&Box<Expression>]) -> String {
        let mut res: String = String::new();

        res.push('(');
        res.push_str(&name);

        for expr in expressions {
            res.push(' ');
            res.push_str(&self.print());
        }

        res.push(')');

        return res;
    }
    pub fn print(self: &Self) -> String {
        match self {
            Expression::Binary(binary) => {
                return self.parenthesize(&binary.operator.lexeme, &[&binary.left, &binary.right]);
            }
            Expression::Group(group) => {
                return self.parenthesize(&String::from("group"), &[&group.expression]);
            }
            Expression::Literal(literal) => match &literal.value {
                Some(value) => {
                    return value.to_string();
                }
                None => {
                    return "nil".to_owned();
                }
            },
            Expression::Unary(unary) => {
                return self.parenthesize(&unary.operator.lexeme, &[&unary.right]);
            }
        }
    }
}
