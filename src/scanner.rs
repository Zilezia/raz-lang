use std::string::String;
use std::collections::HashMap;

fn is_digit(ch: char) -> bool {
    let uch = ch as u8;
    uch >= '0' as u8 &&
    uch <= '9' as u8
}

fn is_alpha(ch: char) -> bool {
    let uch = ch as u8;
   (uch >= 'a' as u8 && uch <= 'z' as u8) ||
   (uch >= 'A' as u8 && uch <= 'Z' as u8) ||
    ch == '_'
}

fn is_alph_numeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}

fn get_keywords_hashmap() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("and", And),
        ("class", Class),
        ("else", Else),
        ("false", False),
        ("for", For),
        ("func", Func),
        ("if", If),
        ("non", Non),
        ("or", Or),
        ("print", Print),
        ("show", Show),
        ("return", Return),
        ("super", Super),
        ("this", This),
        ("true", True),
        ("var", Var),
        ("while", While),
    ])
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: get_keywords_hashmap(),
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line
        });

        if errors.len() > 0 {
            let mut joined = "".to_string();
            for error in errors {
                joined.push_str(&error);
                joined.push_str("\n");
            }
            return Err(joined);
        }

        Ok(self.tokens.clone())
    }

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(self: &mut Self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            ';' => self.add_token(Semicolon),

            '-' => {
                let token = if self.match_token('-') {
                    MinusMinus
                } else if self.match_token('=') {
                    MinusEqual
                } else { Minus };
                self.add_token(token);
            },
            '+' => {
                let token = if self.match_token('+') {
                    PlusPlus
                } else if self.match_token('=') {
                    PlusEqual
                } else { Plus };
                self.add_token(token);
            },
            '*' => self.add_token(Star),
            '/' => {
                if self.match_token('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                else if self.match_token('*') { self.block_comment()?; }
                else if self.match_token('^') { self.add_token(Root); }
                else { self.add_token(Slash); }
            },
            '^' => self.add_token(Power),
            '%' => self.add_token(Modulo),

            '!' => {
                let token = if self.match_token('=')
                    { BangEqual } else { Bang };
                self.add_token(token);
            },
            '=' => {
                let token = if self.match_token('=')
                    { EqualEqual } else { Equal };
                self.add_token(token);
            },
            '<' => {
                let token = if self.match_token('=')
                    { LessEqual } else { Less };
                self.add_token(token);
            },
            '>' => {
                let token = if self.match_token('=')
                    { GreaterEqual } else { Greater };
                self.add_token(token);
            },

            ' ' | '\r' | '\t' => {},
            '\n' => self.line+=1,
            '"' => self.string()?,

            '&' => if self.match_token('&') { self.add_token(And); },
            '|' => if self.match_token('|') { self.add_token(Or); },

            c => {
                if is_digit(c) { self.number()?; }
                else if is_alpha(c) { self.identifier()?; }
                else { return Err(format!("Unrecognized token at line {}: {}", self.line, c)); }
            },
        }
        // println!("{}", c);
        Ok(())
    }

    fn block_comment(self: &mut Self) -> Result<(), String>{
        while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
            if self.peek() == '\n' { self.line+=1; }
            self.advance();
        }
        if self.is_at_end() { return Err("Unterminated block comment".to_string()); }

        self.advance();
        self.advance();

        Ok(())
    }

    fn string(self: &mut Self) -> Result<(), String>{
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line+=1; }
            self.advance();
        }

        if self.is_at_end() { return Err("Unterminated string".to_string()); }

        self.advance();

        let value = &self.source[self.start+1..self.current-1];

        self.add_token_lit(StringLit, Some(StringValue(value.to_string())));

        Ok(())
    }

    fn number(self: &mut Self) -> Result<(), String>{
        while is_digit(self.peek()) { self.advance(); }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) { self.advance(); }
        }

        let sub_string = &self.source[self.start..self.current];
        let value = sub_string.parse::<f64>();
        match value {
            Ok(value) => self.add_token_lit(Number, Some(FValue(value))),
            Err(_) => return Err(format!("Could not parse number: {}", sub_string))
        }

        Ok(())
    }

    fn identifier(self: &mut Self) -> Result<(), String> {
        while is_alph_numeric(self.peek()) { self.advance(); }

        let sub_string = &self.source[self.start..self.current];
        if let Some(&t_type) = self.keywords.get(sub_string) {
            self.add_token(t_type);
        } else {
            self.add_token(Identifier);
        }
        Ok(())
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() { return '\0'; }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(self: &Self) -> char {
        if self.current+1 >= self.source.len() { return '\0'; }
        return self.source.chars().nth(self.current+1).unwrap();
    }

    fn match_token(self: &mut Self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        } else {
            self.current+=1;
            return true;
        }
    }

    fn advance(self: &mut Self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
        // let mut text = "".to_string();
        // let _= self.source[self.start..self.current]
        //     .chars()
        //     .map(|ch| text.push(ch));
        let text = self.source[self.start..self.current].to_string();

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line_number: self.line,
        });
    }



}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Semicolon, Comma, Dot,
    // SAMDEB ->
    Minus, Plus, Star, Slash, Power, Root,
    Modulo,
    MinusMinus, PlusPlus,
    // Eq
    PlusEqual, MinusEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    Bang, BangEqual,
    // Literals.
    Identifier, StringLit, Number,
    // Keywords.
    And, Class, Else, False, Func, For, If, Non, Or,
    Print, Show, Return, Super, This, True, Var, While,
    Eof
}

use TokenType::*;

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    // NumberValue(i64),
    FValue(f64),
    StringValue(String),
    // IdentifierValue(String),
}
use LiteralValue::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: usize, 
}

#[allow(dead_code)]
impl Token {
    pub fn new(
        token_type: TokenType, 
        lexeme: String, 
        literal: Option<LiteralValue>,
        line_number: usize
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number
        }
    }

    pub fn to_string(self: &Self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
