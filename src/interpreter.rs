use crate::environment::Environment;
use crate::expr::LiteralValue;
use crate::stmt::Stmt;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<&Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(self.environment.clone())?;
                },
                Stmt::Print { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    println!("{}", value.to_string());
                },
                Stmt::Var { name, initialiser } => {
                    let value = initialiser.evaluate(self.environment.clone())?;

                    self.environment.borrow_mut().define(name.lexeme.clone(), value);
                },
                Stmt::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());
                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(RefCell::new(new_environment));
                    let block_result = self.interpret(
                        (*statements).iter().map(|b| b.as_ref()).collect()
                    );
                    self.environment = old_environment;

                    block_result?;
                },
                Stmt::IfStmt { 
                    condition,
                    then_branch,
                    else_branch
                } => {
                    let truth_value = condition.evaluate(self.environment.clone())?;
                    if truth_value.is_truthy() == LiteralValue::True {
                        self.interpret(vec![then_branch])?;
                    } else if let Some(else_stmt) = else_branch {
                        self.interpret(vec![else_stmt])?;
                    }
                },
                Stmt::WhileStmt { condition, body } => {
                    let mut flag = condition.evaluate(self.environment.clone())?;
                    while flag.is_truthy() == LiteralValue::True { // heh
                        self.interpret(vec![body])?;
                        flag = condition.evaluate(self.environment.clone())?;
                    }
                },
            };
        }
        Ok(())
    }
}
