use crate::Token;
use crate::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initialiser: Expr },
    Block { statements: Vec<Box<Stmt>>},
    IfStmt { 
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    WhileStmt { 
        condition: Expr,
        body: Box<Stmt>,
    },
    // ForStmt {
    //     var_decl: Option<Box<Stmt>>,
    //     expr_stmt: Option<Box<Stmt>>,
    //     condition: Option<Expr>,
    //     incrementer: Option<Expr>,
    //     body: Box<Stmt>,
    // },
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
            IfStmt { condition, then_branch, else_branch} => {
                todo!();
            },
            WhileStmt { condition, body } => {
                todo!();
            },
        }
    }
}