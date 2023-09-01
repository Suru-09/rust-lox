pub mod rlox_callable {
    use std::any::Any;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::environment::environment::Environment;
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
        pub closure: Rc<RefCell<Environment>>,
    }

    impl RLoxFunction {
        pub fn new(declaration: Stmt, closure: Rc<RefCell<Environment>>) -> Self {
            Self {
                declaration,
                closure,
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
            let env = self.closure.clone();
            match &self.declaration {
                Stmt::Function(_, params, body) => {
                    for (_, param) in params.iter().enumerate() {
                        /*
                         * Note for future me: - Initially I am removing the first element,
                         * but once removed the second element becomes the first element and so
                         * on. Therefore, the correct way in order not to violate the bounds of the 
                         * vector is to remove the first element every time.
                         */
                        env.borrow_mut().define(param.get_token_type().to_string(), args.remove(0));
                    }

                    match interpreter.execute_block(body, env) {
                        Ok(return_val) => Ok(return_val),
                        Err(err_str) => {
                            if err_str.starts_with("Return") && interpreter.return_value.is_some() {
                                let ret_val = interpreter.return_value.take().unwrap();
                                interpreter.return_value = None;
                                return Interpreter::extract_return_value(ret_val);
                            } else {
                                return Err(err_str);
                            }
                        }
                    }
                        
                },
                _ => panic!("Cannot call non-function"),
            }
        }
    }
}

