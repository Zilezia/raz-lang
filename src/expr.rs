use crate::scanner;
use crate::scanner::{Token, TokenType};
use crate::environment::Environment;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub enum LiteralValue {
    NumberValue(f64),
    StringValue(String),
    True,
    False,
    Non,
    Callable { 
        name: String,
        arity: usize,
        func: Rc<dyn Fn(Vec<LiteralValue>) -> LiteralValue>,
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
            (Callable { name, arity, func: _},
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

fn unwrap_as_f64(literal: Option<scanner::LiteralValue>) -> f64 {
    match literal {
        // Some(scanner::LiteralValue::NumberValue(x)) => x as f64,
        Some(scanner::LiteralValue::FValue(x)) => x as f64,
        _ => panic!("Could not unwrap as f64")
    }
}

fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        // Some(scanner::LiteralValue::IdentifierValue(s)) => s.clone(),
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
            //Self::Callable { name, arity, func: _ } => format!("{name}|{arity}"),
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
            // Self::Callable => "Callable",
            _ => panic!("Cannot check unknown LiteralValue"),
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::NumberValue(unwrap_as_f64(token.literal)),
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
                if *x == 0.0 { True }
                else { False }
            },
            StringValue(s) => {
                if s.len() == 0 { True }
                else { False }
            },
            True => False,
            False => True,
            Non => True,
            Callable { name: _, arity: _, func: _ } => panic!("Cannot use Callable as a falsy value."),
        }
    }

    pub fn is_truthy(self: &Self) -> LiteralValue {
        match self {
            NumberValue(x) => {
                if *x == 0.0 { False }
                else { True }
            },
            StringValue(s) => {
                if s.len() == 0 { False }
                else { True }
            },
            True => True,
            False => False,
            Non => False,
            Callable { name: _, arity: _, func: _ } => panic!("Cannot use Callable as a truthy value."),
        }
    }
}

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
        right:Box<Expr>
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
                right,
                operator,
            } => format!("({} {})", operator.lexeme.clone(), (*right).to_string()),
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

                        Ok(func(arg_vals))
                    },
                    unkn => Err(format!("{} is not callable.", unkn.to_type())),
                }
            }
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

#[cfg(test)]
mod tests {
    // use crate::Interpreter;
    // use crate::Parser;
    // use crate::Scanner;


    // #[test]
    // fn divide_string_one_way() {
    //     /*var a = "hello";
    //      show a/a.len(); (non existent function yet)
    //      h*/
    //     let statement = "show \"longer_string\"/2;"; // "h" = floor(5/3)
    //     let mut scanner = Scanner::new(statement);
    //     let tokens = scanner.scan_tokens().unwrap();
    //     let mut parser = Parser::new(tokens);
    //     let stmts = parser.parse().unwrap();
    //     let mut interpreter = Interpreter::new();

    //     interpreter.interpret(stmts).unwrap();
    // }

    // #[test]
    // fn multiple_string() {
    //     let statement = "show 3*\"hello\";"; // "hellohellohello"
    //     let mut scanner = Scanner::new(statement);
    //     let tokens = scanner.scan_tokens().unwrap();
    //     let mut parser = Parser::new(tokens);
    //     let stmts = parser.parse().unwrap();
    //     let mut interpreter = Interpreter::new();

    //     interpreter.interpret(stmts).unwrap();
    // }

    // this subtraction is the hardest to implement
    // #[test]
    // fn world_from_hello() {
    //     let statement = "show \"hello\"-\"world\";";
    //     let mut scanner = Scanner::new(statement);
    //     let tokens = scanner.scan_tokens().unwrap();
    //     let mut parser = Parser::new(tokens);
    //     let stmts = parser.parse().unwrap();
    //     let mut interpreter = Interpreter::new();

    //     interpreter.interpret(stmts).unwrap();
    // }
    // #[test]
    // fn hey_from_hello() {
    //     let statement = "show \"hello\"-\"hey\";";
    //     let mut scanner = Scanner::new(statement);
    //     let tokens = scanner.scan_tokens().unwrap();
    //     let mut parser = Parser::new(tokens);
    //     let stmts = parser.parse().unwrap();
    //     let mut interpreter = Interpreter::new();

    //     interpreter.interpret(stmts).unwrap();
    // }
    // #[test]
    // fn long_string_sub() {
    //     let statement = "show \"some pretty interesting string here\"-\"small string\";";
    //     let mut scanner = Scanner::new(statement);
    //     let tokens = scanner.scan_tokens().unwrap();
    //     let mut parser = Parser::new(tokens);
    //     let stmts = parser.parse().unwrap();
    //     let mut interpreter = Interpreter::new();

    //     interpreter.interpret(stmts).unwrap();
    // }
    
    // #[test]
    // fn same_string_subtraction() {
    //     let statement = "show \"hello there\"-\"hello there\";"; // should be empty
    //     let mut scanner = Scanner::new(statement);
    //     let tokens = scanner.scan_tokens().unwrap();
    //     let mut parser = Parser::new(tokens);
    //     let stmts = parser.parse().unwrap();
    //     let mut interpreter = Interpreter::new();

    //     interpreter.interpret(stmts).unwrap();
    // }

    // #[test]
    // fn pretty_print_ast() { // uses RPN? 
    //     let minus_token = Token { 
    //         token_type: TokenType::Minus,
    //         lexeme: "-".to_string(),
    //         literal: None,
    //         line_number: 0,
    //     };
    //     let one_two_three = Literal {
    //         value: NumberValue(123.0),
    //     };
    //     let group = Grouping { 
    //         expression: Box::from(Literal { 
    //             value: NumberValue(45.67),
    //         }),
    //     };
    //     let multi = Token { 
    //         token_type: TokenType::Star,
    //         lexeme: "*".to_string(),
    //         literal: None,
    //         line_number: 0
    //     };
    //     let ast = Binary { 
    //         left: Box::from(Unary { 
    //             operator: minus_token,
    //             right: Box::from(one_two_three)
    //         }),
    //         operator: multi,
    //         right: Box::from(group),
    //     };

    //     let result = ast.to_string();
    //     // print!("{}", result);
    //     assert_eq!(result, "((- 123) (45.67) *)")
    // }
}
