use crate::environment::Environment;
use crate::literals::LiteralValue::{self, *};
use crate::scanner::{Token, TokenType};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]

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
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments:Vec<Expr>,
    },
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Logical { 
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token,
    },
    Unary { 
        operator: Token,
        val: Box<Expr>
    },
    Variable { name: Token },
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Expr {
    #[allow(dead_code)]
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
            Self::Call { 
                callee,
                paren: _,
                arguments: _
            } => format!(
                "({})",
                callee.to_string()
            ),
            Self::Grouping { expression } => format!("({})", (*expression).to_string()), // (*expression).to_string()
            Self::Literal { value } => format!("{}", value.to_string()),
            Self::Logical {
                left,
                right,
                operator
            } => format!(
                "({} {} {})",
                left.to_string(),
                right.to_string(),
                operator.lexeme,
            ),
            Self::Unary { 
                val,
                operator,
            } => format!("({} {})", operator.lexeme.clone(), (*val).to_string()),
            Self::Variable { name } => format!("(var {})", name.lexeme),
        }
    }

    pub fn evaluate(self: &Self, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, String> {
        match self {
            Self::Assignment { name, value } => {
                let new_value = (*value).evaluate(environment.clone())?;
                let assign_success = environment.borrow_mut().assign(&name.lexeme, new_value.clone());
                
                if assign_success { Ok(new_value) }
                else { Err(format!("Variable {:?} has not been declared", name.lexeme)) }
            },
            Self::Variable { name } => match environment.borrow().get(&name.lexeme) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Variable {:?} has not been declared", name.lexeme)),
            },
            Self::Call { 
                callee,
                paren:_,
                arguments
            } => {
                let callable = (*callee).evaluate(environment.clone())?;
                match callable {
                    Callable { name, arity, func } => {
                        if arguments.len() != arity {
                            return Err(format!(
                                "Callable {} expected {} arguments but got {}.", 
                                name, arity, arguments.len()
                            ));
                        }
                        let mut arg_vals = vec![];
                        for arg in arguments {
                            let val = arg.evaluate(environment.clone())?;
                            arg_vals.push(val);
                        }

                        Ok(func(environment.clone(), &arg_vals))
                    },
                    unkn => Err(format!("{} is not callable.", unkn.to_type())),
                }
            }
            Self::Literal { value } => Ok((*value).clone()),
            Self::Grouping { expression } => expression.evaluate(environment),
            Self::Unary { operator, val } => {
                let val = val.evaluate(environment)?;
                match (&val, operator.token_type) {
                    (NumberValue(x), TokenType::Minus) => Ok(NumberValue(-x)),
                    (NumberValue(x), TokenType::Plus) => Ok(NumberValue(*x)),
                    (NumberValue(x), TokenType::MinusMinus) => Ok(NumberValue(x-1.0)),
                    (NumberValue(x), TokenType::PlusPlus) => Ok(NumberValue(x+1.0)),
                    (NumberValue(x), TokenType::Root) => {
                        let res = f64::sqrt(*x);
                        Ok(NumberValue(res))
                    },
                    // have the string mirrored/reversed
                    (_, TokenType::Minus) => return Err(format!("Minus not implemented for {:?}", val.to_type())),
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (_, t_type)=> Err(format!("{} is not a valid unary operator", t_type)),
                }
            },
            Self::Logical {
                left,
                right,
                operator
            } => {
                match operator.token_type {
                    TokenType::Or => {
                        let lhs_value = left.evaluate(environment.clone())?;
                        let lhs_true = lhs_value.is_truthy();
                        if lhs_true == True { Ok(lhs_value) }
                        else { right.evaluate(environment.clone()) }
                    },
                    TokenType::And => {
                        let lhs_true = left.evaluate(environment.clone())?.is_truthy();
                        let rhs_true = right.evaluate(environment.clone())?.is_truthy();
                        if lhs_true == True && rhs_true == True { Ok(True) }
                        else { Ok(False) }
                    },
                    ttype => Err(format!("Invalid token in logical expression: {}", ttype)),
                }
            },
            Self::Binary { 
                left,
                right,
                operator 
            } => {
                let left = left.evaluate(environment.clone())?;
                let right = right.evaluate(environment.clone())?;
                
                match (&left, &right, operator.token_type) {
                    // Standard math calculations /* very basic stuff*/
                    (NumberValue(x), NumberValue(y), TokenType::Plus)           => Ok(NumberValue(x+y)),
                    (NumberValue(x), NumberValue(y), TokenType::Minus)          => Ok(NumberValue(x-y)),
                    (NumberValue(x), NumberValue(y), TokenType::Star)           => Ok(NumberValue(x*y)),
                    (NumberValue(x), NumberValue(y), TokenType::Slash)          => Ok(NumberValue(x/y)),
                    (NumberValue(x), NumberValue(y), TokenType::Power) => {
                        let res = f64::powf(*x, *y);
                        Ok(NumberValue(res))
                    },
                    (NumberValue(x), NumberValue(y), TokenType::Root) => {
                        let res = f64::powf(*y, 1.0/(*x));
                        Ok(NumberValue(res))
                    },
                    (NumberValue(x), NumberValue(y), TokenType::Modulo)          => Ok(NumberValue(x%y)),
                    
                    (NumberValue(x), NumberValue(y), TokenType::Greater)        => Ok(LiteralValue::from_bool(x>y)),
                    (NumberValue(x), NumberValue(y), TokenType::GreaterEqual)   => Ok(LiteralValue::from_bool(x>=y)),
                    (NumberValue(x), NumberValue(y), TokenType::Less)           => Ok(LiteralValue::from_bool(x<y)),
                    (NumberValue(x), NumberValue(y), TokenType::LessEqual)      => Ok(LiteralValue::from_bool(x<=y)),                    
                    
                    // (NumberValue(x), NumberValue(y), TokenType::EqualEqual) => Ok(LiteralValue::from_bool(x==y)),
                    // (NumberValue(x), NumberValue(y), TokenType::BangEqual) => Ok(LiteralValue::from_bool(x!=y)),
                    // /* starting to get interesting */
                    (StringValue(s1), StringValue(s2), TokenType::Plus) => Ok(StringValue(format!("{}{}",s1,s2))),
                    /* RED.*/(StringValue(s1), StringValue(s2), TokenType::Minus) => {
                        let mut result = String::new();
                        // let mut char_freq = HashMap::new();
                        
                        // for s2_char in s2.chars() {
                        //     let count = char_freq.entry(s2_char).or_insert(0);
                        //     *count+=1;
                        // }
                        // for s1_char in s1.chars() {
                        //     let count = char_freq.entry(s1_char).or_insert(0);
                        //     if *count > 0 {
                        //         *count-=1;
                        //     } else {
                        //         result.push(s1_char);
                        //     }
                        // }

                        if result.is_empty() { Ok(Non) }
                        else { Ok(StringValue(result)) }
                    },


                    // Combos
                    // Number and String calculations
                    (StringValue(s), NumberValue(x), TokenType::Plus) => {Ok(StringValue(format!("{}{}",s,x.to_string())))},
                    (NumberValue(x), StringValue(s), TokenType::Plus) => {Ok(StringValue(format!("{}{}",x.to_string(),s)))},
                    
                    // update thess for float numbers to work
                    /* YELL.*/(StringValue(s), NumberValue(x), TokenType::Star) => {
                        let string_res;
                        string_res = s.repeat(*x as usize);
                        Ok(StringValue(string_res))
                    },
                    /* YELL.*/(NumberValue(x), StringValue(s), TokenType::Star) => {
                        let string_res;
                        string_res = s.repeat(*x as usize);
                        Ok(StringValue(string_res))
                    },
                    
                    // num / string, could not work? 
                    /* YELL.*/(StringValue(s), NumberValue(x), TokenType::Slash) => {
                        let mut string_res = String::new();
                        // divide the string/string_length by number
                        // loop through the leng
                        let div_string_len = s.len() / *x as usize;
                        if div_string_len == 1 {
                            let first_char = s.chars().nth(0).unwrap().to_string();
                            Ok(StringValue(first_char))
                        }
                        else if div_string_len > 1 {
                            for it in 0..div_string_len {
                                let nth_char = s.chars().nth(it).unwrap();
                                string_res.push(nth_char);
                            }
                            Ok(StringValue(string_res)) 
                        }
                        else { Ok(Non) } 
                    },


                    (x,y, TokenType::BangEqual) => Ok(LiteralValue::from_bool(x!=y)),
                    (x,y, TokenType::EqualEqual) => Ok(LiteralValue::from_bool(x==y)),
                    
                    (x,y,t_type) => Err(format!(
                        "{} is not implemented for operands {:?} and {:?}", t_type,x.to_string(),y.to_string()))
                }
            },
            _ => todo!(),
        }
    }

    // pub fn print(&self) {
    //     println!("{}", self.to_string());
    // }
}
