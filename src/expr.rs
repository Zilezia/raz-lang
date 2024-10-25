use std::default;

use crate::{environment, scanner};
use crate::scanner::{Token, TokenType};
use crate::environment::Environment;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    NumberValue(f32),
    StringValue(String),
    True,
    False,
    Nil,
}
use LiteralValue::*;

fn unwrap_as_f32(literal: Option<scanner::LiteralValue>) -> f32 {
    match literal {
        Some(scanner::LiteralValue::NumberValue(x)) => x as f32,
        Some(scanner::LiteralValue::FValue(x)) => x as f32,
        _ => panic!("Could not unwrap as f32")
    }
}

fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        Some(scanner::LiteralValue::StringValue(s)) => s.clone(),
        Some(scanner::LiteralValue::IdentifierValue(s)) => s.clone(),
        _ => panic!("Could not unwrap as string")
    }
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            Self::NumberValue(x) => x.to_string(),
            Self::StringValue(x) => x.clone(),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),
            Self::Nil => "nil".to_string(),
        }
    }

    pub fn to_type(&self) -> &str {
        match self {
            Self::NumberValue(_) => "Number",
            Self::StringValue(_) => "String",
            Self::True => "Boolean",
            Self::False => "Boolean",
            Self::Nil => "nil",
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::NumberValue(unwrap_as_f32(token.literal)),
            TokenType::StringLit => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            _ => panic!("Could not create LiteralValue from {:?}", token),
        }
    }
    
    pub fn from_bool(b: bool) -> Self {
        if b { True } 
        else { False }
    }

    pub fn is_falsy(self: &Self) -> LiteralValue {
        match self {
            NumberValue(x) => {
                if *x == 0.0 { True }
                else { False }
            },
            StringValue(s) => {
                if s.len() == 0 { True }
                else { False }
            },
            True => False,
            False => True,
            Nil => True,
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Assignment { 
        name: Token,
        value: Box<Expr>
    },
    Binary { 
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token,
    },
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Unary { 
        operator: Token,
        right:Box<Expr>
    },
    Variable { name: Token },
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Self::Assignment { name, value } => {
                format!("({} = {})", name.lexeme, value.to_string())
            }
            Self::Binary { 
                left,
                right,
                operator,
            } => format!(
                "({} {} {})",
                left.to_string(),
                right.to_string(),
                operator.lexeme,
            ),
            Self::Grouping { expression } => format!("({})", (*expression).to_string()), // (*expression).to_string()
            Self::Literal { value } => format!("{}", value.to_string()),
            Self::Unary { 
                right,
                operator,
            } => format!("({} {})", operator.lexeme.clone(), (*right).to_string()),
            Self::Variable { name } => format!("(var {})", name.lexeme),
        }
    }

    pub fn evaluate(self: &Self, environment: &mut Environment) -> Result<LiteralValue, String> {
        match self {
            Self::Assignment { name, value } => {
                let new_value = (*value).evaluate(environment)?;
                let assign_success = environment.assign(&name.lexeme, new_value.clone());
                if assign_success { Ok(new_value) }
                else { Err(format!("Variable {:?} has not been declared", name.lexeme)) }
            },
            Self::Variable { name } => match environment.get(&name.lexeme) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Variable {:?} has not been declared", name.lexeme)),
            },
            Self::Literal { value } => Ok((*value).clone()),
            Self::Grouping { expression } => expression.evaluate(environment),
            Self::Unary { operator, right } => {
                let right = right.evaluate(environment)?;
                match (&right, operator.token_type) {
                    (NumberValue(x), TokenType::Minus) => Ok(NumberValue(-x)),
                    (NumberValue(x), TokenType::Plus) => Ok(NumberValue(*x)),
                    // have the string mirrored/reversed
                    (_, TokenType::Minus) => return Err(format!("Minus not implemented for {:?}", right.to_type())),
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (_, t_type)=> Err(format!("{} is not a valid unary operator", t_type)),
                }
            },
            Self::Binary { 
                left,
                right,
                operator 
            } => {
                let left = left.evaluate(environment)?;
                let right = right.evaluate(environment)?;
                
                match (&left, &right, operator.token_type) {
                    // Standard math calculations /* very basic stuff*/
                    (NumberValue(x), NumberValue(y), TokenType::Plus) => Ok(NumberValue(x+y)),
                    (NumberValue(x), NumberValue(y), TokenType::Minus) => Ok(NumberValue(x-y)),
                    (NumberValue(x), NumberValue(y), TokenType::Star) => Ok(NumberValue(x*y)),
                    (NumberValue(x), NumberValue(y), TokenType::Slash) => Ok(NumberValue(x/y)),
                    (NumberValue(x), NumberValue(y), TokenType::Greater) => Ok(LiteralValue::from_bool(x>y)),
                    (NumberValue(x), NumberValue(y), TokenType::GreaterEqual) => Ok(LiteralValue::from_bool(x>=y)),
                    (NumberValue(x), NumberValue(y), TokenType::Less) => Ok(LiteralValue::from_bool(x<y)),
                    (NumberValue(x), NumberValue(y), TokenType::LessEqual) => Ok(LiteralValue::from_bool(x<=y)),                    
                    
                    // (NumberValue(x), NumberValue(y), TokenType::EqualEqual) => Ok(LiteralValue::from_bool(x==y)),
                    // (NumberValue(x), NumberValue(y), TokenType::BangEqual) => Ok(LiteralValue::from_bool(x!=y)),
                    // /* starting to get interesting */
                    (StringValue(s1), StringValue(s2), TokenType::Plus) => Ok(StringValue(format!("{}{}",s1,s2))),
                    (StringValue(s1), StringValue(s2), TokenType::Minus) => {
                        if s1 == s2 { Ok(Nil) }
                        // take away the chars from the other
                        // s1 = "hello"
                        // s2 = "world"
                        // s1 - s2 = "hel" 
                        else { todo!() }
                    },

                    // (StringValue(s1), StringValue(s2), TokenType::EqualEqual) => Ok(LiteralValue::from_bool(s1==s2)),
                    // (StringValue(s1), StringValue(s2), TokenType::BangEqual) => Ok(LiteralValue::from_bool(s1!=s2)),

                    // Number and String calculations
                    (StringValue(s), NumberValue(x), TokenType::Plus) => {Ok(StringValue(format!("{}{}",s,x.to_string())))},
                    (NumberValue(x), StringValue(s), TokenType::Plus) => {Ok(StringValue(format!("{}{}",x.to_string(),s)))},
                    // (StringValue(s), NumberValue(x), TokenType::Star) => {
                    //     Ok(StringValue((s*x)))
                    // },

                    (x,y, TokenType::BangEqual) => Ok(LiteralValue::from_bool(x!=y)),
                    (x,y, TokenType::EqualEqual) => Ok(LiteralValue::from_bool(x==y)),
                    
                    (x,y,t_type) => Err(format!("{} is not implemented for operands {:?} and {:?}", t_type,x.to_string(),y.to_string()))
                    // _ => Err("Never accomplished operation!!!!"),
                }
            },
            _ => todo!(),
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use super::Expr::*;
//     use super::LiteralValue::*;

//     #[test]
//     fn pretty_print_ast() { // uses RPN? 
//         let minus_token = Token { 
//             token_type: Minus,
//             lexeme: "-".to_string(),
//             literal: None,
//             line_number: 0,
//         };
//         let one_two_three = Literal {
//             value: NumberValue(123.0),
//         };
//         let group = Grouping { 
//             expression: Box::from(Literal { 
//                 value: NumberValue(45.67),
//             }),
//         };
//         let multi = Token { 
//             token_type: Star,
//             lexeme: "*".to_string(),
//             literal: None,
//             line_number: 0
//         };
//         let ast = Binary { 
//             left: Box::from(Unary { 
//                 operator: minus_token,
//                 right: Box::from(one_two_three)
//             }),
//             operator: multi,
//             right: Box::from(group),
//         };

//         let result = ast.to_string();
//         print!("{}", result);
//         // assert_eq!(result, "(* (- 123) (45.67))")
//     }
// }