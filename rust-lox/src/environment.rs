pub mod environment {
    use crate::stmt::stmt::LiteralValue;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::interpreter::interpreter::Error;
    use crate::scanner::scan::Token;
    use crate::error_handling::error_handling::error;
    use crate::function_name;

    #[derive(Debug, PartialEq)]
    pub struct Environment {
        values: HashMap<String, LiteralValue>,
    }

    impl Environment {
        pub fn new() -> Environment {
            Environment {
                values: HashMap::new(),
            }
        }

        pub fn define(&mut self, token: &Token, value: LiteralValue) {
            self.values.insert(token.get_token_type().to_string().clone(), value);
        }

        pub fn get(&mut self, token: &Token) -> Option<&LiteralValue> {
            let token_name = token.get_token_type().to_string().clone();
            if self.values.contains_key(&token_name) {
                return self.values.get(&token_name);
            }
            None
        }

        /**
         * ! Very important to note that using '?' is mandatory because on succeess it returns the value
         * ! is void and on failure it returns an error String which should be sent back to the caller.
         */
        pub fn assign(&mut self, token: &Token, value: LiteralValue) -> Result<(), Error> {
            let token_name = token.get_token_type().to_string().clone();
            if self.values.contains_key(&token_name) {
                self.values.insert(token_name, value);
                return Ok(());
            }
            Err(Error::from_string(&format!("Variable '{}' is undefined.", token_name)))
        }
    }

    pub struct EnvironmentStack {
        stack: Vec<Rc<RefCell<Environment>>>,
    }

    impl EnvironmentStack {
        pub fn new() -> EnvironmentStack {
            EnvironmentStack { stack: Vec::new() }
        }

        pub fn push_env(&mut self, env: Rc<RefCell<Environment>>) {
            self.stack.push(env);
        }

        pub fn len(&mut self) -> usize {
            self.stack.len()
        }

        pub fn pop(&mut self) -> Option<Rc<RefCell<Environment>>> {
            self.stack.pop()
        }

        pub fn peek(&mut self) -> Option<Rc<RefCell<Environment>>> {
            self.stack.last().map(|env| env.clone())
        }

        pub fn get(&mut self, token: &Token) -> Option<LiteralValue> {
            for env in self.stack.iter().rev() {
                if let Some(value) = env.as_ref().borrow_mut().get(token) {
                    return Some(value.clone());
                }
            }
            None
        }

        fn ancestor(&mut self, distance: usize) -> Option<Rc<RefCell<Environment>>> {
            if distance >= self.stack.len() {
                return None;
            }
            Some(self.stack[self.stack.len() - distance - 1].clone())
        }

        pub fn get_at(&mut self, distance: usize, token: &Token) -> Option<LiteralValue> {
            if distance >= self.stack.len() {
                return None;
            }

            if let Some(env) = self.ancestor(distance) {
                if let Some(value) = env.as_ref().borrow_mut().get(token) {
                    return Some(value.clone());
                }
            }

            None
        }

        pub fn assign_at(
            &mut self,
            distance: usize,
            token: &Token,
            value: LiteralValue,
        ) -> Result<(), Error> {
            if distance >= self.stack.len() {
                error(
                    token.get_line(),
                    token.get_column(),
                    format!(
                        "Variable '{}' is undefined.",
                        token.get_token_type().to_string()
                    ),
                    function_name!(),
                );
                return Err(Error::from_string(&format!("Variable '{}' is undefined.", token.get_token_type().to_string().clone())));
            }

            if let Some(env) = self.ancestor(distance) {
                if let Ok(_) = env.as_ref().borrow_mut().assign(token, value) {
                    return Ok(());
                }
            }

            error(
                token.get_line(),
                token.get_column(),
                format!(
                    "Variable '{}' is undefined.",
                    token.get_token_type().to_string()
                ),
                function_name!(),
            );
            Err(Error::from_string(&format!("Variable '{}' is undefined.", token.get_token_type().to_string().clone())))
        }

        pub fn define(&mut self, token: &Token, value: LiteralValue) {
            if let Some(env) = self.stack.last() {
                env.as_ref().borrow_mut().define(token, value);
            }
        }

        pub fn assign(&mut self, token: &Token, value: LiteralValue) -> Result<(), Error> {
            for env in self.stack.iter().rev() {
                if let Ok(_) = env
                    .as_ref()
                    .borrow_mut()
                    .assign(token, value.clone())
                {
                    return Ok(());
                }
            }


            error(
                token.get_line(),
                token.get_column(),
                format!(
                    "Variable '{}' is undefined.",
                    token.get_token_type().to_string()
                ),
                function_name!(),
            );
            Err(Error::from_string(&format!("Variable '{}' is undefined.", token.get_token_type().to_string().clone())))
        }
    }
}
