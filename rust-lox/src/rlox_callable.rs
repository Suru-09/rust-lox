pub mod rlox_callable {
    use std::any::Any;
    use crate::{stmt::stmt::Stmt, interpreter::interpreter::Interpreter};


    pub trait RLoxCallable {
        fn arity(&self) -> usize;
        fn call(&self, interpreter: &mut Interpreter, args: Vec<Box<dyn Any>>) -> Result<Box<dyn Any>, String>;
    }

    pub struct Clock {}

    impl RLoxCallable for Clock {
        fn arity(&self) -> usize {
            0
        }

        fn call(&self, _interpreter: &mut Interpreter, _args: Vec<Box<dyn Any>>) -> Result<Box<dyn Any>, String>{
            Ok(Box::new(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| panic!("Could not get time since epoch"))
                .as_secs_f64()))
        }
    }

    pub struct RLoxFunction {
        pub declaration: Stmt,
    }

    impl RLoxFunction {
        pub fn new(declaration: Stmt) -> Self {
            Self {
                declaration
            }
        }

        pub fn to_string(&self) -> String {
            match &self.declaration {
                Stmt::Function(name, _, _) => format!("<fn {}>", name.get_token_type()),
                _ => panic!("Cannot call non-function")
            }
        }
    }

    impl RLoxCallable for RLoxFunction {
        fn arity(&self) -> usize {
            match &self.declaration {
                Stmt::Function(_, params, _ ) => params.len(),
                _ => 0
            }  
        }
            

        fn call(&self, interpreter: &mut Interpreter, args: Vec<Box<dyn Any>>) -> Result<Box<dyn Any>, String> {
            let env = interpreter.get_global_environment().clone();
            match &self.declaration {
                Stmt::Function(_, params, body) => {
                    // use GLOBAL_ENVIRONMENT from interpreter
                    for (i, param) in params.iter().enumerate() {
                        env.borrow_mut().define(param.get_token_type().to_string(), args[i]);
                    }

                    match env.borrow_mut().peek() {
                        Some(last_env) => {
                            interpreter.execute_block(&body, last_env)
                        },
                        None => panic!("No environment found")
                    }
                },
                _ => panic!("Cannot call non-function")
            }
        }
    }
}

