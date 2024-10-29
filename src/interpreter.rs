use crate::environment::Environment;
use crate::literals::LiteralValue;
use crate::stmt::Stmt;
use crate::Token;

use crate::functions::func::clock_impl;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    specials: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}


impl Interpreter {
    pub fn new() -> Self {
        let mut natives = Environment::new();
        // clock func 
        natives.define( 
            "clock".to_string(),
            LiteralValue::Callable { 
                name: "clock".to_string(),
                arity: 0, 
                func: Rc::new(clock_impl)
            });

        Self {
            // globals: globals,
            specials: Rc::new(RefCell::new(Environment::new())),
            environment: Rc::new(RefCell::new(natives)),
        }
    }

    fn for_closure(parent: Rc<RefCell<Environment>>) -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));
        environment.borrow_mut().enclosing = Some(parent);
        Self { 
            specials: Rc::new(RefCell::new(Environment::new())),
            environment
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
                Stmt::Function { 
                    name,
                    params,
                    body 
                } => {
                    let arity = params.len();

                    let params: Vec<Token> = params.iter().map(|t| (*t).clone()).collect();
                    let body: Vec<Box<Stmt>> = body.iter().map(|b| (*b).clone()).collect();
                    let name_clone = name.lexeme.clone();

                    let func_impl = 
                        move |parent_env, args: &Vec<LiteralValue>| 
                    {
                        let mut clos_int = Interpreter::for_closure(parent_env);

                        for (i, arg) in args.iter().enumerate() {
                            clos_int
                                .environment
                                .borrow_mut()
                                .define(params[i].lexeme.clone(), (*arg).clone());
                        }

                        for i in 0..(body.len()) {
                            clos_int.interpret(vec![body[i].as_ref()]).expect(&format!(
                                "Evaluating failed inside {}",
                                name_clone
                            ));
                            if let Some(value) = clos_int.specials.borrow().get("return") {
                                return value;
                            }
                        }
                        LiteralValue::Non
                    };
                    let callable = LiteralValue::Callable { 
                        name: name.lexeme.clone(),
                        arity,
                        func: Rc::new(func_impl),
                    };

                    self.environment.borrow_mut().define(name.lexeme.clone(), callable);
                },
                Stmt::ReturnStmt { 
                    keyword: _,
                    value
                } => {
                    let eval_val;
                    if let Some(value) = value {
                        eval_val = value.evaluate(self.environment.clone())?;
                    } else {
                        eval_val = LiteralValue::Non;
                    }
                    self.specials
                        .borrow_mut()
                        .define_top_level("return".to_string(), eval_val);

                },
            };
        }
        Ok(())
    }
}
