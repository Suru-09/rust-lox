pub mod rlox_callable {
    use crate::environment::environment::Environment;
    use crate::stmt::stmt::LiteralValue;
    use crate::{
        interpreter::interpreter::{Error, Interpreter},
        stmt::stmt::Stmt,
    };
    use chrono;
    use std::borrow::Borrow;
    use std::collections::HashMap;
    use std::{borrow::BorrowMut, cell::RefCell, fmt, rc::Rc};
    use crate::scanner::scan::Token;
    use crate::function_name;
    use crate::error_handling::error_handling::{error, RLoxErrorType};

    #[derive(Debug, PartialEq)]
    pub enum Callable {
        Class(RLoxClass),
        Instance(Rc<RefCell<RLoxInstance>>),
        Function(RLoxFunction),
        Clock(Clock),
        UnixTClock(UnixTClock),
    }

    impl Clone for Callable {
        fn clone(&self) -> Self {
            match self {
                Callable::Function(lox_function) => Callable::Function(lox_function.clone()),
                Callable::Instance(rlox_instance) => Callable::Instance(rlox_instance.clone()),
                Callable::Class(class) => Callable::Class(class.clone()),
                Callable::Clock(clock) => Callable::Clock(clock.clone()),
                Callable::UnixTClock(unix_t_clock) => Callable::UnixTClock(unix_t_clock.clone()),
            }
        }
    }

    impl fmt::Display for Callable {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Callable::Class(rlox_clas) => write!(f, "{}", rlox_clas.to_string()),
                Callable::Instance(rlox_instance) => write!(f, "{}", rlox_instance.clone().as_ref().borrow_mut().to_string()),
                Callable::Function(rlox_fun) => write!(f, "{}", rlox_fun.to_string()),
                Callable::Clock(clock) => write!(f, "{}", clock.to_string()),
                Callable::UnixTClock(unix_tclock) => write!(f, "{}", unix_tclock.to_string()),
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

    impl Clock {
        pub fn to_string(&self) -> String {
            String::from("clock")
        }
    }

    impl RLoxCallable for Clock {
        fn arity(&self) -> usize {
            0
        }

        fn call(
            &self,
            _interpreter: &mut Interpreter,
            _args: &mut Vec<LiteralValue>,
        ) -> Result<LiteralValue, Error> {
            Ok(LiteralValue::String(
                chrono::offset::Local::now().to_string(),
            ))
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct UnixTClock {}

    impl UnixTClock {
        pub fn to_string(&self) -> String {
            String::from("unixClock")
        }
    }

    impl RLoxCallable for UnixTClock {
        fn arity(&self) -> usize {
            0
        }

        fn call(
            &self,
            _interpreter: &mut Interpreter,
            _args: &mut Vec<LiteralValue>,
        ) -> Result<LiteralValue, Error> {
            Ok(LiteralValue::Number(
                chrono::offset::Local::now().timestamp_millis() as f64,
            ))
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct RLoxFunction {
        pub declaration: Box<Stmt>,
        pub closure: Rc<RefCell<Environment>>,
    }

    impl RLoxFunction {
        pub fn new(declaration: Stmt, closure: Rc<RefCell<Environment>>) -> Self {
            Self {
                declaration: Box::new(declaration),
                closure,
            }
        }

        pub fn to_string(&self) -> String {
            match self.declaration.borrow() {
                Stmt::Function(name, _, _) => format!("<fn {}>", name.get_token_type()),
                _ => panic!("Cannot call non-function"),
            }
        }
    }

    impl RLoxCallable for RLoxFunction {
        fn arity(&self) -> usize {
            match self.declaration.borrow() {
                Stmt::Function(_, params, _) => params.len(),
                _ => 0,
            }
        }

        fn call(
            &self,
            interpreter: &mut Interpreter,
            args: &mut Vec<LiteralValue>,
        ) -> Result<LiteralValue, Error> {
            let mut env = Environment::new(self.closure.clone());
            match self.declaration.borrow() {
                Stmt::Function(_, params, body) => {
                    for (idx, param) in params.iter().enumerate() {
                        env.borrow_mut().define(param, args[idx].clone());
                    }

                    match interpreter.execute_block(&body, env) {
                        Ok(_) => return Ok(LiteralValue::Nil),
                        Err(err) => match err {
                            Error::Return(ret_val) => {
                                return Ok(ret_val);
                            }
                            _ => return Err(err),
                        },
                    };
                }
                _ => panic!("Cannot call non-function"),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct RLoxClass {
        pub name: String,
        pub methods: HashMap<String, RLoxFunction>
    }

    impl RLoxClass {
        pub fn new(name: String, methods: HashMap<String, RLoxFunction>) -> Self {
            Self {
                name,
                methods
            }
        }

        pub fn find_method(&self, name: &str) -> Option<RLoxFunction> 
        {
            if self.methods.contains_key(name) {
                return Some(self.methods.get(name).unwrap().clone())
            }
            None
        }

        pub fn to_string(&self) -> String {
            format!("<class {}>", self.name)
        }
    }

    impl RLoxCallable for RLoxClass {
        fn arity(&self) -> usize {
            0
        }

        fn call(
            &self,
            _interpreter: &mut Interpreter,
            _args: &mut Vec<LiteralValue>,
        ) -> Result<LiteralValue, Error> {
            let instance = Rc::new(RefCell::new(RLoxInstance::new(Rc::new(self.clone()))));
            Ok(LiteralValue::Callable(Callable::Instance(Rc::clone(&instance))))
         }
        }

    #[derive(Clone, Debug, PartialEq)]
    pub struct RLoxInstance {
        pub rlox_class: Rc<RLoxClass>,
        pub fields: HashMap<String, LiteralValue>
    }

    impl RLoxInstance {
        pub fn new(rlox_class: Rc<RLoxClass>) -> Self {
            Self { 
                rlox_class,
                fields: HashMap::new()
            }
        }

        pub fn get(&self, name: &Token) -> Result<LiteralValue, Error> {
            let name_str = &name.get_token_type().to_string();
            if self.fields.contains_key(name_str) {
                return Ok(self.fields.get(name_str).unwrap().clone())
            }

            if let Some(method) = self.rlox_class.find_method(name_str) {
                return Ok(LiteralValue::Callable(Callable::Function(method)))
            }

            error(
                name.get_line(),
                name.get_column(),
                format!("Undefined property '{}'.", name.get_token_type()),
                function_name!(),
                Some(RLoxErrorType::RuntimeError)
            );
            Err(Error::LoxRuntimeError)
        }

        pub fn set(&mut self, name: &Token, value: LiteralValue) {
            self.fields.insert(name.get_token_type().to_string(), value);
            ()
        }

        pub fn to_string(&self) -> String {
            format!("<{} instance>", self.rlox_class.name)
        }
    }
}
