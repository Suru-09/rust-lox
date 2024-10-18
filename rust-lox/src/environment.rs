pub mod environment {
    use crate::stmt::stmt::LiteralValue;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::interpreter::interpreter::Error;

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

        pub fn define(&mut self, name: String, value: LiteralValue) {
            self.values.insert(name, value);
        }

        pub fn get(&mut self, name: String) -> Option<&LiteralValue> {
            if self.values.contains_key(&name) {
                return self.values.get(&name);
            }
            None
        }

        /**
         * ! Very important to note that using '?' is mandatory because on succeess it returns the value
         * ! is void and on failure it returns an error String which should be sent back to the caller.
         */
        pub fn assign(&mut self, name: String, value: LiteralValue) -> Result<(), Error> {
            if self.values.contains_key(&name) {
                self.values.insert(name, value);
                return Ok(());
            }
            Err(Error::from_string(&format!("Variable '{}' is undefined.", name)))
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

        pub fn get(&mut self, name: String) -> Option<LiteralValue> {
            for env in self.stack.iter().rev() {
                if let Some(value) = env.as_ref().borrow_mut().get(name.clone()) {
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

        pub fn get_at(&mut self, distance: usize, name: String) -> Option<LiteralValue> {
            if distance >= self.stack.len() {
                return None;
            }

            if let Some(env) = self.ancestor(distance) {
                if let Some(value) = env.as_ref().borrow_mut().get(name.clone()) {
                    return Some(value.clone());
                }
            }

            None
        }

        pub fn assign_at(
            &mut self,
            distance: usize,
            name: String,
            value: LiteralValue,
        ) -> Result<(), Error> {
            if distance >= self.stack.len() {
                return Err(Error::from_string(&format!("Variable '{}' is undefined.", name)));
            }

            if let Some(env) = self.ancestor(distance) {
                if let Ok(_) = env.as_ref().borrow_mut().assign(name.clone(), value) {
                    return Ok(());
                }
            }
            Err(Error::from_string(&format!("Variable '{}' is undefined.", name)))
        }

        pub fn define(&mut self, name: String, value: LiteralValue) {
            if let Some(env) = self.stack.last() {
                env.as_ref().borrow_mut().define(name, value);
            }
        }

        pub fn assign(&mut self, name: String, value: LiteralValue) -> Result<(), Error> {
            for env in self.stack.iter().rev() {
                if let Ok(_) = env
                    .as_ref()
                    .borrow_mut()
                    .assign(name.clone(), value.clone())
                {
                    return Ok(());
                }
            }
            Err(Error::from_string(&format!("Variable '{}' is undefined.", name)))
        }
    }
}
