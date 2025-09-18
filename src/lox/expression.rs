use crate::lox::{token::Token, types::Object};

pub enum Expression {
    Binary(BinaryExpression),
    Group(GroupExpression),
    Literal(LiteralExpression),
    Unary(UnaryExpression),
}

pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

pub struct GroupExpression {
    pub expression: Box<Expression>,
}

pub struct LiteralExpression {
    pub value: Option<Object>,
}

pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

impl Expression {
    fn parenthesize(self: &Self, name: &String, expressions: &[&Box<Expression>]) -> String {
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
