use crate::Token;
use crate::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initialiser: Expr },
    Block { statements: Vec<Stmt>},
}

impl Stmt {
    pub fn to_string(self: &mut Self) -> String {
        use Stmt::*;
        match self {
            Expression { expression } => expression.to_string(),
            Print { expression } => format!("(print {})", expression.to_string()),
            Var { name, initialiser } => format!("(var {})", name.lexeme),
            Block { statements } => format!(
                "(block {})",
                statements.iter_mut().map(
                    |stmt| stmt.to_string()
                ).collect::<String>()
            ),
        }
    }
}