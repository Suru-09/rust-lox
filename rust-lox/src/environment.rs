pub mod environment {
    use std::collections::HashMap;
    use std::any::Any;

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

        pub fn get(&self, name: String) -> Option<&Box<dyn Any>> {
            match self.values.get(&name) {
                Some(value) => Some(value),
                None => None,
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
            Err(format!("Variable '{}' is undefined.", name))
        }
    }
}