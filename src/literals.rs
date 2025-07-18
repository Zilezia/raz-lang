
use std::rc::Rc;
use std::cell::RefCell;

use crate::environment::Environment;
use crate::scanner::{self, Token, TokenType};
use crate::digit::*;

// use raz::{
//     digit::*,
//     environment::Environment,
//     scanner::{self, Token, TokenType}
// };

#[derive(Clone)]
pub enum LiteralValue {
    NumberValue(DigitType),
    StringValue(String),
    True,
    False,
    Non,
    Callable {
        name: String,
        arity: usize,
        func: Rc<dyn Fn(
            Rc<RefCell<Environment>>,
            &Vec<LiteralValue>
        ) -> LiteralValue>,
    },
}

use LiteralValue::*;

impl std::fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NumberValue(x), NumberValue(y)) => x == y,
            (Callable { name, arity, func: _ },
            Callable { name: name2, arity: arity2, func: _}
            ) => name == name2 && arity == arity2,
            (StringValue(s1), StringValue(s2)) => s1 == s2,
            (True, True) => true,
            (False, False) => true,
            (Non, Non) => true,
            _ => false,
        }
    }
}

fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        Some(scanner::LiteralValue::StringValue(s)) => s.clone(),
        _ => panic!("Could not unwrap as string")
    }
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            Self::NumberValue(x) => x.to_string(),
            Self::StringValue(x) => format!("\"{x}\""),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),
            Self::Non => "Non".to_string(),
            Self::Callable { name, arity, func: _ } => {
                format!("{name}_{arity}")
            },
            _ => panic!("Cannot convert LiteralValue into string"),
        }
    }

    pub fn to_type(&self) -> &str {
        match self {
            Self::NumberValue(_) => "Number",
            Self::StringValue(_) => "String",
            Self::True => "Boolean",
            Self::False => "Boolean",
            Self::Non => "Non",
            Self::Callable { 
                name: _,
                arity: _,
                func: _
            } => "Callable",
            _ => panic!("Cannot check unknown LiteralValue"),
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::NumberValue(DigitType::from_string(token.lexeme)),
            TokenType::StringLit => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Non => Self::Non,
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
                if *x == DigitType::f32(0.0) { True }
                else { False }
            },
            StringValue(s) => {
                if s.len() == 0 { True }
                else { False }
            },
            True => False,
            False => True,
            Non => True,
            unkn => panic!("Cannot use {} as a falsy value.", unkn.to_string()),
        }
    }

    pub fn is_truthy(self: &Self) -> LiteralValue {
        match self {
            NumberValue(x) => {
                if *x == DigitType::f32(0.0) { False }
                else { True }
            },
            StringValue(s) => {
                if s.len() == 0 { False }
                else { True }
            },
            True => True,
            False => False,
            Non => False,
            unkn => panic!("Cannot use {} as a truthy value.", unkn.to_string()),
        }
    }
}
