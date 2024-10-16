pub mod rlox_callable {
    use crate::environment::environment::Environment;
    use crate::stmt::stmt::LiteralValue;
    use crate::{interpreter::interpreter::Interpreter, stmt::stmt::Stmt};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    pub enum Callable {
        Class(RLoxClass),
        Function(RLoxFunction),
    }

    impl Clone for Callable {
        fn clone(&self) -> Self {
            match self {
                Callable::Function(lox_function) => Callable::Function(lox_function.clone()),
                Callable::Class(class) => Callable::Class(class.clone()),
            }
        }
    }

    pub trait RLoxCallable {
        fn arity(&self) -> usize;
        fn call(
            &self,
            interpreter: &mut Interpreter,
            args: &mut Vec<LiteralValue>,
        ) -> Result<LiteralValue, String>;
    }

    // #[derive(Clone)]
    // pub struct Clock {}

    // impl RLoxCallable for Clock {
    //     fn arity(&self) -> usize {
    //         0
    //     }

    //     fn call(
    //         &self,
    //         _interpreter: &mut Interpreter,
    //         _args: &mut Vec<LiteralValue>,
    //     ) -> Result<LiteralValue, String> {
    //         Ok(Box::new(Token::new(
    //             crate::scanner::scan::TokenType::Number(
    //                 std::time::SystemTime::now()
    //                     .duration_since(std::time::UNIX_EPOCH)
    //                     .unwrap_or_else(|_| panic!("Could not get time since epoch"))
    //                     .as_secs_f64(),
    //             ),
    //             "clock".to_string(),
    //             0,
    //             0,
    //             0,
    //         )))
    //     }
    // }

    #[derive(Clone, Debug)]
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
                _ => panic!("Cannot call non-function"),
            }
        }
    }

    impl RLoxCallable for RLoxFunction {
        fn arity(&self) -> usize {
            match &self.declaration {
                Stmt::Function(_, params, _) => params.len(),
                _ => 0,
            }
        }

        fn call(
            &self,
            interpreter: &mut Interpreter,
            args: &mut Vec<LiteralValue>,
        ) -> Result<LiteralValue, String> {
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
                        env.borrow_mut()
                            .define(param.get_token_type().to_string(), args.remove(0));
                    }

                    let _ = match interpreter.execute_block(body, env) {
                        Ok(return_val) => Ok(()),
                        Err(err_str) => {
                            return Err(err_str);
                        }
                    };
                }
                _ => panic!("Cannot call non-function"),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct RLoxClass {
        pub name: String,
    }

    impl RLoxClass {
        pub fn new(name: String) -> Self {
            Self { name }
        }

        pub fn to_string(&self) -> String {
            format!("<class {}>", self.name)
        }
    }
}
