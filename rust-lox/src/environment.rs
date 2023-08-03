pub mod environment {
    use std::collections::HashMap;
    use std::any::Any;

    pub struct Environment {
        values: HashMap<String, Box<dyn Any>>,
        enclosing: Option<Box<Environment>>
    }

    impl Environment {
        pub fn empty_env() -> Environment {
            Environment {
                values: HashMap::new(),
                enclosing: None,
            }
        }

        pub fn new(env: Environment) -> Environment {
            Environment {
                values: HashMap::new(),
                enclosing: Some(Box::new(env)),
            }
        }

        pub fn define(&mut self, name: String, value: Box<dyn Any>) {
            self.values.insert(name, value);
        }

        pub fn get(&self, name: String) -> Option<&Box<dyn Any>> {
            match self.values.get(&name) {
                Some(value) => Some(value),
                None => {
                    match &self.enclosing {
                        Some(enclosing) => enclosing.get(name),
                        None => None,
                    }
                },
            }
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

            match &mut self.enclosing {
                Some(enclosing) => {
                    match enclosing.assign(name, value) {
                        Ok(_) => return Ok(()),
                        Err(err) => return Err(err),
                    }
                },
                None => (),
            }
            Err(format!("Variable '{}' is undefined.", name))
        }
    }
}