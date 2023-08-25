pub mod rlox_callable {
    use std::any::Any;
    use crate::{stmt::stmt::Stmt, interpreter::interpreter::Interpreter};


    pub trait RLoxCallable {
        fn arity(&self) -> usize;
        fn call(&self, interpreter: &mut Interpreter, args: &mut Vec<Box<dyn Any>>) -> Result<Box<dyn Any>, String>;
    }

    pub struct Clock {}

    impl RLoxCallable for Clock {
        fn arity(&self) -> usize {
            0
        }

        fn call(&self, _interpreter: &mut Interpreter, _args: &mut Vec<Box<dyn Any>>) -> Result<Box<dyn Any>, String>{
            Ok(Box::new(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| panic!("Could not get time since epoch"))
                .as_secs_f64()))
        }
    }

    #[derive(Clone)]
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
            

        fn call(&self, interpreter: &mut Interpreter, args: &mut Vec<Box<dyn Any>>) -> Result<Box<dyn Any>, String> {
            let env = interpreter.get_global_environment().clone();
            match &self.declaration {
                Stmt::Function(_, params, body) => {
                    // use GLOBAL_ENVIRONMENT from interpreter
                    for (_, param) in params.iter().enumerate() {
                        /*
                         * Note for future me: - Initially I am removing the first element,
                         * but once removed the second element becomes the first element and so
                         * on. Therefore, the correct way in order not to violate the bounds of the 
                         * vector is to remove the first element every time.
                         */
                        env.borrow_mut().define(param.get_token_type().to_string(), args.remove(0));
                    }

                    let env_clone = env.clone();
                    let last_env = env_clone.borrow_mut().peek();

                    match last_env {
                        Some(last_env_val) => {
                            interpreter.execute_block(body, last_env_val)
                        }
                        None => panic!("Could not get last environment")
                    }
                },
                _ => panic!("Cannot call non-function")
            }
        }
    }
}

