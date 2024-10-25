use crate::{scanner::{Token, TokenType::{self,*}}, stmt};
use crate::expr::{*, Expr::*};
use crate::stmt::Stmt;

// macro_rules! match_tokens {
//     ($parser:ident, $($token:ident),+) => {
//         {
//             let mut result = false;
//             {$( result |= $parser.match_token($tokens); )*}
//             result
//         }
//     }
// }

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(self: &mut Self) -> Result<Vec<Stmt>, String> {
        let mut stmts = vec![];
        let mut errs = vec![];

        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => errs.push(msg),
            }
        };

        if errs.len() <= 0 { Ok(stmts)}
        else { Err(errs.join("\n")) }
    }

    fn declaration(self: &mut Self) -> Result<Stmt, String> {
        if self.match_token(Var) { 
            match self.var_declaration() {
                Ok(stmt) => Ok(stmt),
                Err(msg) => {
                    self.synchronise();
                    Err(msg)
                }
            }
        } else { self.statement() }
    }

    fn var_declaration(self: &mut Self) -> Result<Stmt, String>{
        let token = self.consume(Identifier, "Expect variable name.")?;
        
        let initialiser;
        if self.match_token(Equal) { initialiser = self.expression()?; } 
        else { initialiser = Literal { value: LiteralValue::Nil }; }

        self.consume(Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var { 
            name: token,
            initialiser: initialiser 
        })
    }

    fn statement(self: &mut Self) -> Result<Stmt, String> {
        // double statements at the same time?
        if self.match_tokens(&[Print, Show]) { self.print_stmt() }
        else if self.match_token(LeftBrace) { self.block_statement() }
        else { self.expression_stmt() }

    }

    fn block_statement(self: &mut Self) -> Result<Stmt, String> {
        let mut stmts = vec![];

        while !self.check(RightBrace) && !self.is_at_end() {
            let decl = self.declaration()?;
            stmts.push(decl);
            // match decl {
            //     Ok(s) => stmts.push(s),
            //     Err(_) => todo!(),
            // }
        }
        self.consume(RightBrace, "Expect '}' to end the block.");
        // Ok(stmts)
        Ok(Stmt::Block { statements: stmts })
        // stmts
        // todo!()
    }

    fn print_stmt(self: &mut Self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print { expression: value })
    }

    fn expression_stmt(self: &mut Self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after expression.")?;

        Ok(Stmt::Expression { expression: expr })
    }

    fn expression(self: &mut Self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(self: &mut Self) -> Result<Expr, String> {
        let expr = self.equality()?;

        if self.match_token(Equal) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Variable { name } => Ok(Assignment { name: name, value: Box::from(value) }),
                _ => panic!("Invalid assignment target: {}", equals.to_string()),
            }
        } else { Ok(expr) }
    }

    fn equality(self: &mut Self) -> Result<Expr, String>  {
        let mut expr = self.comparison()?;
        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison()?;
            expr = Binary { 
                left: Box::from(expr),
                right: Box::from(rhs),
                operator: operator,
            };
            // matches_eq =  self.match_tokens(&[BangEqual, EqualEqual])
        }
        Ok(expr)
    }

    fn comparison(self: &mut Self) -> Result<Expr, String>  {
        let mut expr = self.term()?;
        
        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let rhs = self.term()?;
            expr = Binary { 
                left: Box::from(expr),
                operator: operator,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }

    fn term(self: &mut Self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        
        while self.match_tokens(&[Minus, Plus]) {
            let operator = self.previous();
            let rhs = self.factor()?;
            expr = Binary { 
                left: Box::from(expr),
                operator: operator,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }

    fn factor(self: &mut Self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        
        while self.match_tokens(&[Slash, Star]) {
            let operator = self.previous();
            let rhs = self.unary()?;
            expr = Binary { 
                left: Box::from(expr),
                operator: operator,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }

    fn unary(self: &mut Self) -> Result<Expr, String> {
        if self.match_tokens(&[Bang, Minus]) {
            let operator = self.previous();
            let rhs = self.unary()?;
            Ok(Unary { 
                operator: operator,
                right: Box::from(rhs),
            })
        } else { self.primary() }
    }

    fn primary(self: &mut Self) -> Result<Expr, String> {
        let token = self.peek();

        let result;
        match token.token_type {
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expected ')'")?;
                result = Grouping { expression: Box::from(expr) }
            },
            False | True | Nil | Number | StringLit => {
                self.advance();
                result = Literal { value: LiteralValue::from_token(token) }
            },
            Identifier => {
                self.advance();
                result = Variable { name: self.previous() }
            },
            _ => return Err("Expected expression".to_string()),
        }

        Ok(result)
    }

    fn consume(self: &mut Self, token_type: TokenType, msg: &str) -> Result<Token, String>{
        let token = self.peek();
        if token.token_type == token_type { 
            self.advance(); 
            let token = self.previous();
            Ok(token)
        }
        else { 
            println!("Missing token");
            Err(msg.to_string())
        }
    }

    fn check(self: &mut Self,t_type: TokenType) -> bool {
        self.peek().token_type == t_type
    }

    fn match_token(self: &mut Self, t_type: TokenType) -> bool {
        if self.is_at_end() { false } 
        else { 
            if self.peek().token_type == t_type {
                self.advance();
                true
            } /*stupid*/ else { false }
        }
    }

    fn match_tokens(self: &mut Self, t_types: &[TokenType]) -> bool {
        for &t_type in t_types {
            if self.match_token(t_type) { return true; }
        }

        false 
    }

    fn advance(self: &mut Self) -> Token {
        if !self.is_at_end() { self.current+=1; }
        self.previous()
    }

    fn peek(self: &mut Self) -> Token{
        self.tokens[self.current].clone()
    }

    fn previous(self: &mut Self) -> Token{
        self.tokens[self.current-1].clone()
    }

    fn is_at_end(self: &mut Self) -> bool {
        self.peek().token_type == Eof
    }

    fn synchronise(self: &mut Self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {return;}

            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => (),
            }
            self.advance();
        }
    }

}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::scanner::{self, LiteralValue::*, Scanner};

//     #[test]
//     fn test_addition() {
//         let one = Token {
//             token_type: Number,
//             lexeme: "1".to_string(),
//             literal: Some(NumberValue(1)),
//             line_number: 0
//         };

//         let plus = Token {
//             token_type: Plus,
//             lexeme: "+".to_string(),
//             literal: None,
//             line_number: 0
//         };

//         let two = Token {
//             token_type: Number,
//             lexeme: "2".to_string(),
//             literal: Some(NumberValue(2)),
//             line_number: 0
//         };

//         let semi_colon = Token {
//             token_type: Semicolon,
//             lexeme: ";".to_string(),
//             literal: None,
//             line_number: 0
//         };

//         let tokens = vec![one, plus, two, semi_colon];
//         let mut parser = Parser::new(tokens);

//         let parsed_expr = parser.parse().unwrap();
//         let string_expr = parsed_expr.to_string();

//         // print!("{}", string_expr);
//         assert_eq!(string_expr, "(1 2 +)");
//     }

//     #[test]
//     fn test_comparison() {
//         let source = "1 + 2 == 5 + 7";
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens().unwrap();
//         let mut parser = Parser::new(tokens);
//         let parsed_expr = parser.parse().unwrap();
//         let string_expr = parsed_expr.to_string();
//         // print!("{}", string_expr);
//         assert_eq!(string_expr, "((1 2 +) (5 7 +) ==)");
//     }

//     #[test]
//     fn test_eq_with_paren() {
//         let source = "1 == (2 + 2)";
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens().unwrap();
//         let mut parser = Parser::new(tokens);
//         let parsed_expr = parser.parse().unwrap();
//         let string_expr = parsed_expr.to_string();

//         // print!("{}", string_expr);
//         assert_eq!(string_expr, "(1 ((2 2 +)) ==)");
//     }
// }