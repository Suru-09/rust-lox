pub mod environment {
    use crate::error_handling::error_handling::{error, RLoxErrorType};
    use crate::function_name;
    use crate::interpreter::interpreter::Error;
    use crate::scanner::scan::Token;
    use crate::stmt::stmt::LiteralValue;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Clone, Default)]
    pub struct Environment {
        values: HashMap<String, LiteralValue>,
        enclosing: Option<Rc<RefCell<Environment>>>,
    }

    impl Environment {
        pub fn new(enclosing: Rc<RefCell<Environment>>) -> Self {
            Environment {
                values: HashMap::new(),
                enclosing: Some(enclosing),
            }
        }

        pub fn new_without_enclosing() -> Self {
            Environment {
                values: HashMap::new(),
                enclosing: None,
            }
        }

        pub fn define(&mut self, token: &Token, value: LiteralValue) {
            self.values
                .insert(token.get_token_type().to_string().clone(), value);
        }

        pub fn get(&mut self, token: &Token) -> Result<LiteralValue, Error> {
            let token_name = token.get_token_type().to_string().clone();
            if self.values.contains_key(&token_name) {
                return Ok(self.values.get(&token_name).unwrap().clone());
            } else if let Some(enclosing) = &self.enclosing {
                Ok(enclosing.as_ref().borrow_mut().get(&token.clone())?)
            } else {
                error(
                    token.get_line(),
                    token.get_column(),
                    format!(
                        "Variable '{}' is not defined",
                        token.get_token_type().to_string()
                    ),
                    function_name!(),
                    Some(RLoxErrorType::RuntimeError),
                );
                Err(Error::from_string(&format!(
                    "Variable '{}' is not defined",
                    token.get_token_type().to_string()
                )))
            }
        }

        pub fn assign(&mut self, token: &Token, value: LiteralValue) -> Result<(), Error> {
            let token_name = token.get_token_type().to_string().clone();
            if self.values.contains_key(&token_name) {
                self.values.insert(token_name, value);
                Ok(())
            } else if let Some(enclosing) = &self.enclosing {
                enclosing.as_ref().borrow_mut().assign(token, value)?;
                Ok(())
            } else {
                error(
                    token.get_line(),
                    token.get_column(),
                    format!("Variable '{}' is undefined.", token_name),
                    function_name!(),
                    Some(RLoxErrorType::RuntimeError),
                );
                Err(Error::from_string(&format!(
                    "Variable '{}' is undefined.",
                    token_name
                )))
            }
        }

        pub fn get_at(&mut self, distance: usize, token: &Token) -> Result<LiteralValue, Error> {
            if distance == 0 {
                self.get(&token)
            } else {
                self.enclosing
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .get_at(distance - 1, token)
            }
        }

        pub fn assign_at(&mut self, distance: usize, token: &Token, value: LiteralValue) {
            if distance == 0 {
                self.define(token, value);
            } else {
                self.enclosing
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .assign_at(distance - 1, token, value);
            }
        }
    }
}
