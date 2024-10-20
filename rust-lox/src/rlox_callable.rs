pub mod rlox_callable {
    use crate::environment::environment::Environment;
    use crate::stmt::stmt::LiteralValue;
    use crate::{interpreter::interpreter::{Interpreter, Error}, stmt::stmt::Stmt};
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::rc::Rc;
    use chrono;

    #[derive(Debug, PartialEq)]
    pub enum Callable {
        Class(RLoxClass),
        Function(RLoxFunction),
        Clock(Clock),
        UnixTClock(UnixTClock),
    }

    impl Clone for Callable {
        fn clone(&self) -> Self {
            match self {
                Callable::Function(lox_function) => Callable::Function(lox_function.clone()),
                Callable::Class(class) => Callable::Class(class.clone()),
                Callable::Clock(clock) => Callable::Clock(clock.clone()),
                Callable::UnixTClock(unix_t_clock) => Callable::UnixTClock(unix_t_clock.clone()),
            }
        }
    }

    pub trait RLoxCallable {
        fn arity(&self) -> usize;
        fn call(
            &self,
            interpreter: &mut Interpreter,
            args: &mut Vec<LiteralValue>,
        ) -> Result<LiteralValue, Error>;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Clock {}

    impl RLoxCallable for Clock {
        fn arity(&self) -> usize {
            0
        }

        fn call(
            &self,
            _interpreter: &mut Interpreter,
            _args: &mut Vec<LiteralValue>,
        ) -> Result<LiteralValue, Error> {
            Ok(
                LiteralValue::String(chrono::offset::Local::now().to_string())
            )
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct UnixTClock {}

    impl RLoxCallable for UnixTClock {
        fn arity(&self) -> usize {
            0
        }

        fn call(
            &self,
            _interpreter: &mut Interpreter,
            _args: &mut Vec<LiteralValue>,
        ) -> Result<LiteralValue, Error> {
            Ok(
                LiteralValue::Number(chrono::offset::Local::now().timestamp_millis() as f64)
            )
        }
    }

    #[derive(Clone, Debug, PartialEq)]
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
        ) -> Result<LiteralValue, Error> {
            let env = self.closure.clone();
            match &self.declaration {
                Stmt::Function(_, params, body) => {
                    for (idx, param) in params.iter().enumerate() {
                        env.clone().borrow_mut()
                            .as_ref()
                            .borrow_mut()
                            .define(param.get_token_type().to_string(), args[idx].clone());
                    }

                    match interpreter.execute_block(body, env) {
                        Ok(_) => return Ok(LiteralValue::Nil),
                        Err(err) => {
                            match err {
                                Error::Return(ret_val) => return Ok(ret_val),
                                _ => return Err(err)
                            }
                        }
                    };
                }
                _ => panic!("Cannot call non-function"),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
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
