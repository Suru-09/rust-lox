pub mod environment {
    use std::borrow::BorrowMut;
    use std::collections::HashMap;
    use std::any::Any;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::expr::expr::Expr;
    use crate::stmt::stmt::Stmt;
    use crate::scanner::scan::Token;

    pub struct Environment {
        values: HashMap<String, Box<dyn Any>>,
    }

    impl Environment {

        pub fn new() -> Environment {
            Environment {
                values: HashMap::new(),
            }
        }

        pub fn define(&mut self, name: String, value: Box<dyn Any>) {
            self.values.insert(name, value);
        }

        pub fn get(&mut self, name: String) -> Option<&Box<dyn Any>> {
            if self.values.contains_key(&name) {
                return self.values.get(&name);
            } 
            None
        }

        /**
         * ! Very important to note that using '?' is mandatory because on succeess it returns the value
         * ! is void and on failure it returns an error String which should be sent back to the caller.
         */
        pub fn assign(&mut self, name: String, value: Box<dyn Any>) -> Result<(), String> {
            if self.values.contains_key(&name) {
                self.values.insert(name, value);
                return Ok(());
            } 
            Err(format!("Variable '{}' is undefined.", name))
        }
    }

    pub struct EnvironmentStack {
        stack: Vec<Rc<RefCell<Environment>>>,
    }

    impl EnvironmentStack {
        pub fn new() -> EnvironmentStack {
            EnvironmentStack {
                stack: Vec::new(),
            }
        }

        pub fn push_env(&mut self) {
            self.stack.push(Rc::new(RefCell::new(Environment::new())));
        }

        pub fn pop(&mut self) -> Option<Rc<RefCell<Environment>>> {
            self.stack.pop()
        }

        pub fn peek(&mut self) -> Option<Rc<RefCell<Environment>>> {
            self.stack.last().map(|env| env.clone())
        }

        fn get_value(&self, value: &Box<dyn Any>) -> Option<Box<dyn Any>> {
            if let Some(token) = value.downcast_ref::<Token>() {
                return Some(Box::new(token.clone()))
            }

            if let Some(expr) = value.downcast_ref::<Expr>() {
                return Some(Box::new(expr.clone()))
            }

            if let Some(stmt) = value.downcast_ref::<Stmt>() {
                return Some(Box::new(stmt.clone()))
            }
            None
        }

        pub fn get(&mut self, name: String) -> Option<Box<dyn Any>> {
            for env in self.stack.iter().rev() {
                if let Some(value) = env.as_ref().borrow_mut().get(name.clone()) {
                    return self.get_value(value);
                }
            }
            None
        }

        pub fn define(&mut self, name: String, value: Box<dyn Any>) {
            if let Some(env) = self.stack.last() {
                env.as_ref().borrow_mut().define(name, value);
            }
        }

        pub fn assign(&mut self, name: String, value: Box<dyn Any>) -> Result<(), String> {
            for env in self.stack.iter().rev() {
                let val_copied = match self.get_value(&value) {
                    Some(val) => {
                        val
                    },
                    None => return Err(format!("Variable '{}' is undefined.", name)),
                };

                if let Ok(_) = env.as_ref().borrow_mut().assign(name.clone(), val_copied) {
                    return Ok(());
                }
            }
            Err(format!("Variable '{}' is undefined.", name))
        }
    }
}