pub mod rlox_callable {
    use crate::environment::environment::Environment;
    use crate::error_handling::error_handling::{error, RLoxErrorType};
    use crate::function_name;
    use crate::scanner::scan::{Token, TokenType};
    use crate::stmt::stmt::LiteralValue;
    use crate::{
        interpreter::interpreter::{Error, Interpreter},
        stmt::stmt::Stmt,
    };
    use chrono;
    use rustc_hash::FxHashMap as HashMap;
    use std::borrow::Borrow;
    use std::{borrow::BorrowMut, cell::RefCell, fmt, rc::Rc};

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
                Callable::Instance(rlox_instance) => write!(
                    f,
                    "{}",
                    rlox_instance.clone().as_ref().borrow_mut().to_string()
                ),
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
            args: &mut Vec<Rc<LiteralValue>>,
        ) -> Result<Rc<LiteralValue>, Error>;
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
            _args: &mut Vec<Rc<LiteralValue>>,
        ) -> Result<Rc<LiteralValue>, Error> {
            Ok(Rc::new(LiteralValue::String(
                chrono::offset::Local::now().to_string(),
            )))
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
            _args: &mut Vec<Rc<LiteralValue>>,
        ) -> Result<Rc<LiteralValue>, Error> {
            Ok(Rc::new(LiteralValue::Number(
                chrono::offset::Local::now().timestamp_millis() as f64,
            )))
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct RLoxFunction {
        pub declaration: Box<Stmt>,
        pub closure: Rc<RefCell<Environment>>,
        pub is_initializer: bool,
    }

    impl RLoxFunction {
        pub fn new(
            declaration: Stmt,
            closure: Rc<RefCell<Environment>>,
            is_initializer: bool,
        ) -> Self {
            Self {
                declaration: Box::new(declaration),
                closure,
                is_initializer,
            }
        }

        pub fn to_string(&self) -> String {
            match self.declaration.borrow() {
                Stmt::Function(name, _, _) => format!("<fn {}>", name.get_token_type()),
                _ => panic!("Cannot call non-function"),
            }
        }

        pub fn bind(&mut self, instance: Rc<RefCell<RLoxInstance>>) -> RLoxFunction {
            let env = Rc::new(RefCell::new(Environment::new(Rc::clone(&self.closure))));
            env.as_ref().borrow_mut().define_str(
                "this",
                Rc::new(LiteralValue::Callable(Callable::Instance(instance))),
            );
            Self {
                declaration: self.declaration.clone(),
                closure: env,
                is_initializer: self.is_initializer,
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
            args: &mut Vec<Rc<LiteralValue>>,
        ) -> Result<Rc<LiteralValue>, Error> {
            let mut env = Environment::new(self.closure.clone());
            match self.declaration.borrow() {
                Stmt::Function(_, params, body) => {
                    for (idx, param) in params.iter().enumerate() {
                        env.borrow_mut().define(param, Rc::clone(&args[idx]));
                    }

                    match interpreter.execute_block(&body, env) {
                        Ok(_) => (),
                        Err(err) => match err {
                            Error::Return(ret_val) => {
                                return Ok(ret_val);
                            }
                            _ => return Err(err),
                        },
                    };
                }
                _ => (),
            }

            if self.is_initializer {
                return self.closure.as_ref().borrow_mut().get_at(
                    0,
                    &Token::new(TokenType::This, String::from("this"), 0, 0, 0),
                );
            }

            Ok(Rc::new(LiteralValue::Nil))
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct RLoxClass {
        pub name: String,
        pub super_class: Rc<Option<RLoxClass>>,
        pub methods: HashMap<String, RLoxFunction>,
    }

    impl RLoxClass {
        pub fn new(
            name: String,
            methods: HashMap<String, RLoxFunction>,
            super_class: Option<RLoxClass>,
        ) -> Self {
            Self {
                name,
                methods,
                super_class: Rc::new(super_class),
            }
        }

        pub fn find_method(&self, name: &str) -> Option<RLoxFunction> {
            if self.methods.contains_key(name) {
                return Some(self.methods.get(name).unwrap().clone());
            }

            if let Some(superclass) = self.super_class.borrow() {
                return superclass.find_method(name);
            }

            None
        }

        pub fn to_string(&self) -> String {
            format!("<class {}>", self.name)
        }
    }

    impl RLoxCallable for RLoxClass {
        fn arity(&self) -> usize {
            if let Some(ctor) = self.find_method("init") {
                return ctor.arity();
            }
            0
        }

        fn call(
            &self,
            interpreter: &mut Interpreter,
            args: &mut Vec<Rc<LiteralValue>>,
        ) -> Result<Rc<LiteralValue>, Error> {
            let instance = Rc::new(RefCell::new(RLoxInstance::new(Rc::new(self.clone()))));

            let c_tor = self.find_method("init");
            if let Some(mut ctor) = c_tor {
                ctor.bind(Rc::clone(&instance)).call(interpreter, args)?;
            }

            Ok(Rc::new(LiteralValue::Callable(Callable::Instance(
                Rc::clone(&instance),
            ))))
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct RLoxInstance {
        pub rlox_class: Rc<RLoxClass>,
        pub fields: HashMap<String, Rc<LiteralValue>>,
    }

    impl RLoxInstance {
        pub fn new(rlox_class: Rc<RLoxClass>) -> Self {
            Self {
                rlox_class,
                fields: HashMap::default(),
            }
        }

        pub fn get(&mut self, name: &Token) -> Result<Rc<LiteralValue>, Error> {
            let name_str = &name.get_token_type().to_string();
            if self.fields.contains_key(name_str) {
                return Ok(self.fields.get(name_str).unwrap().clone());
            }

            if let Some(mut method) = self.rlox_class.find_method(name_str) {
                return Ok(Rc::new(LiteralValue::Callable(Callable::Function(
                    method.bind(Rc::new(RefCell::new(self.to_owned()))),
                ))));
            }

            error(
                name.get_line(),
                name.get_column(),
                format!("Undefined property '{}'.", name.get_token_type()),
                function_name!(),
                Some(RLoxErrorType::RuntimeError),
            );
            Err(Error::LoxRuntimeError)
        }

        pub fn set(&mut self, name: &Token, value: Rc<LiteralValue>) {
            self.fields.insert(name.get_token_type().to_string(), value);
        }

        pub fn to_string(&self) -> String {
            format!("<{} instance>", self.rlox_class.name)
        }
    }
}
