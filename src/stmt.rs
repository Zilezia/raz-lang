use crate::Token;
use crate::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block { statements: Vec<Box<Stmt>>},
    Expression { expression: Expr },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>
    },
    IfStmt { 
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>
    },
    Print { expression: Expr },
    ReturnStmt { 
        keyword: Token,
        value: Option<Expr>
    },
    WhileStmt { 
        condition: Expr,
        body: Box<Stmt>
    },
    Var {
        name: Token,
        initialiser: Expr
    },
}
impl Stmt {
    #[allow(dead_code)]
    pub fn to_string(self: &mut Self) -> String {
        use Stmt::*;
        match self {
            Expression { expression } => expression.to_string(),
            Print { expression } => format!("(print {})", expression.to_string()),
            Var { name, initialiser: _ } => format!("(var {})", name.lexeme),
            Block { statements } => format!(
                "(block {})",
                statements.iter_mut().map(
                    |stmt| stmt.to_string()
                ).collect::<String>()
            ),
            // TODO those, but unusable except when testing
            // started debugging in interactive mode however, so..
            IfStmt { condition: _, then_branch: _, else_branch: _ } => {
                todo!();
            },
            WhileStmt { condition: _, body: _ } => {
                todo!();
            },
            Function { name: _, params: _, body: _} => {
                todo!();
            },
            ReturnStmt { keyword: _, value: _} => {
                todo!();
            }
        }
    }
}