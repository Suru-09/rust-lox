pub mod interpreter {

    use crate::environment::environment::Environment;
    use crate::error_handling::error_handling::{error, RLoxErrorType};
    use crate::expr::expr::{Expr, Visitor};
    use crate::function_name;
    use crate::rlox_callable::rlox_callable::{
        Callable, Clock, RLoxCallable, RLoxClass, RLoxFunction, UnixTClock,
    };
    use crate::scanner::scan::{Token, TokenType};
    use crate::stmt::stmt::{LiteralValue, Stmt, StmtVisitor};
    use rustc_hash::FxHashMap as HashMap;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct Interpreter {
        pub environment: Rc<RefCell<Environment>>,
        pub locals: Vec<(Expr, usize)>,
        pub globals: Rc<RefCell<Environment>>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Error {
        LoxRuntimeError,
        Return(Rc<LiteralValue>),
    }

    impl Interpreter {
        pub fn new() -> Interpreter {
            let globals = Rc::new(RefCell::new(Environment::new_without_enclosing()));
            globals.borrow_mut().define(
                &Token::new(
                    TokenType::Identifier("clock".to_string()),
                    "clock".to_string(),
                    999,
                    999,
                    999,
                ),
                Rc::new(LiteralValue::Callable(Callable::Clock(Clock {}))),
            );

            globals.borrow_mut().define(
                &Token::new(
                    TokenType::Identifier("unixClock".to_string()),
                    "unixClock".to_string(),
                    999,
                    999,
                    999,
                ),
                Rc::new(LiteralValue::Callable(Callable::UnixTClock(UnixTClock {}))),
            );

            let mut locals = Vec::new();
            locals.push((
                Expr::Variable(Token::new(
                    TokenType::Identifier("clock".to_string()),
                    "".to_string(),
                    0,
                    0,
                    0,
                )),
                0,
            ));

            locals.push((
                Expr::Variable(Token::new(
                    TokenType::Identifier("unixClock".to_string()),
                    "".to_string(),
                    0,
                    0,
                    0,
                )),
                0,
            ));

            Interpreter {
                environment: Rc::clone(&globals),
                locals,
                globals: globals,
            }
        }

        fn evaluate(&mut self, expr: &Expr) -> Result<Rc<LiteralValue>, Error> {
            expr.accept(self)
        }

        fn is_truthy_lval(l_val: &LiteralValue) -> bool {
            match l_val {
                LiteralValue::Bool(b) => *b,
                LiteralValue::Nil => false,
                _ => true,
            }
        }

        pub fn resolve(&mut self, expr: Expr, depth: usize) {
            self.locals.push((expr, depth));
        }

        fn get_depth(&mut self, token: &Token) -> Option<usize> {
            for (_, (e, depth)) in self.locals.iter().enumerate() {
                if Expr::Variable(token.clone()) == *e {
                    return Some(*depth);
                }
            }
            None
        }

        fn look_up_variable(&mut self, token: &Token) -> Result<Rc<LiteralValue>, Error> {
            match self.get_depth(token) {
                Some(depth) => {
                    let mut env = self.environment.as_ref().borrow_mut();
                    env.get_at(depth, token)
                }
                None => self.globals.borrow_mut().get(token),
            }
        }

        fn substract(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
            operator: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(Rc::new(LiteralValue::Number(number1 - number2)))
                }
                _ => {
                    error(
                        operator.get_line(),
                        operator.get_column(),
                        format!("Operands must be numbers."),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    Err(Error::LoxRuntimeError)
                }
            }
        }

        fn add(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
            operator: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            match (operand1, operand2) {
                (LiteralValue::String(s1), LiteralValue::String(s2)) => Ok(Rc::new(
                    LiteralValue::String(String::from(s1.to_string() + s2)),
                )),
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(Rc::new(LiteralValue::Number(number1 + number2)))
                }
                (LiteralValue::String(str), LiteralValue::Number(num)) => Ok(Rc::new(
                    LiteralValue::String(str.to_owned() + &num.to_string()),
                )),
                (LiteralValue::Number(num), LiteralValue::String(str)) => Ok(Rc::new(
                    LiteralValue::String(str.to_owned() + &num.to_string()),
                )),
                _ => {
                    error(
                        operator.get_line(),
                        operator.get_column(),
                        format!("Operands must be two numbers or two strings."),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    return Err(Error::LoxRuntimeError);
                }
            }
        }

        fn multiply(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
            operator: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(Rc::new(LiteralValue::Number(number1 * number2)))
                }
                _ => {
                    error(
                        operator.get_line(),
                        operator.get_column(),
                        format!("Operands must be numbers."),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    Err(Error::LoxRuntimeError)
                }
            }
        }

        fn divide(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
            operator: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(Rc::new(LiteralValue::Number(number1 / number2)))
                }
                _ => {
                    error(
                        operator.get_line(),
                        operator.get_column(),
                        format!("Operands must be numbers."),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    Err(Error::LoxRuntimeError)
                }
            }
        }

        fn greater(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
            operator: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(Rc::new(LiteralValue::Bool(number1 > number2)))
                }
                _ => {
                    error(
                        operator.get_line(),
                        operator.get_column(),
                        format!("Operands must be numbers."),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    Err(Error::LoxRuntimeError)
                }
            }
        }

        fn greater_equal(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
            operator: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(Rc::new(LiteralValue::Bool(number1 >= number2)))
                }
                _ => {
                    error(
                        operator.get_line(),
                        operator.get_column(),
                        format!("Operands must be numbers."),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    Err(Error::LoxRuntimeError)
                }
            }
        }

        fn less(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
            operator: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(Rc::new(LiteralValue::Bool(number1 < number2)))
                }
                _ => {
                    error(
                        operator.get_line(),
                        operator.get_column(),
                        format!("Operands must be numbers."),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    Err(Error::LoxRuntimeError)
                }
            }
        }

        fn less_equal(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
            operator: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(Rc::new(LiteralValue::Bool(number1 <= number2)))
                }
                _ => {
                    error(
                        operator.get_line(),
                        operator.get_column(),
                        format!("Operands must be numbers."),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    Err(Error::LoxRuntimeError)
                }
            }
        }

        fn equal_equal(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
            _operator: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            match (operand1, operand2) {
                (LiteralValue::String(s1), LiteralValue::String(s2)) => {
                    Ok(Rc::new(LiteralValue::Bool(s1 == s2)))
                }
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(Rc::new(LiteralValue::Bool(number1 == number2)))
                }
                (LiteralValue::Bool(bool1), LiteralValue::Bool(bool2)) => {
                    Ok(Rc::new(LiteralValue::Bool(bool1 == bool2)))
                }
                (LiteralValue::Nil, LiteralValue::Nil) => Ok(Rc::new(LiteralValue::Bool(true))),
                _ => Ok(Rc::new(LiteralValue::Bool(false))),
            }
        }

        fn bang_equal(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
            _operator: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            match (operand1, operand2) {
                (LiteralValue::String(s1), LiteralValue::String(s2)) => {
                    Ok(Rc::new(LiteralValue::Bool(s1 != s2)))
                }
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(Rc::new(LiteralValue::Bool(number1 != number2)))
                }
                (LiteralValue::Bool(bool1), LiteralValue::Bool(bool2)) => {
                    Ok(Rc::new(LiteralValue::Bool(bool1 != bool2)))
                }
                (LiteralValue::Nil, LiteralValue::Nil) => Ok(Rc::new(LiteralValue::Bool(false))),
                _ => Ok(Rc::new(LiteralValue::Bool(true))),
            }
        }

        pub fn execute(&mut self, stmt: &Stmt) -> Result<(), Error> {
            stmt.accept(self)
        }

        pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), Error> {
            for stmt in statements {
                self.execute(stmt)?;
            }
            Ok(())
        }

        pub fn execute_block(&mut self, stmts: &Vec<Stmt>, env: Environment) -> Result<(), Error> {
            let previous = Rc::clone(&self.environment);
            self.environment = Rc::new(RefCell::new(env));

            let result = stmts.iter().try_for_each(|stmt| self.execute(stmt));

            self.environment = previous;
            result
        }
    }

    impl Visitor<Result<Rc<LiteralValue>, Error>> for Interpreter {
        fn visit_literal_expr(&mut self, value: &LiteralValue) -> Result<Rc<LiteralValue>, Error> {
            Ok(Rc::new(value.clone()))
        }

        fn visit_binary_expr(
            &mut self,
            left: &Expr,
            operator: &Token,
            right: &Expr,
        ) -> Result<Rc<LiteralValue>, Error> {
            // ? is the try operator, used to propagate errors.
            let left = self.evaluate(left)?;
            let right = self.evaluate(right)?;

            match operator.get_token_type() {
                TokenType::Greater => Interpreter::greater(&left, &right, &operator),
                TokenType::GreaterEqual => Interpreter::greater_equal(&left, &right, &operator),
                TokenType::Less => Interpreter::less(&left, &right, &operator),
                TokenType::LessEqual => Interpreter::less_equal(&left, &right, &operator),
                TokenType::BangEqual => Interpreter::bang_equal(&left, &right, &operator),
                TokenType::EqualEqual => Interpreter::equal_equal(&left, &right, &operator),
                TokenType::Minus => Interpreter::substract(&left, &right, &operator),
                TokenType::Plus => Interpreter::add(&left, &right, &operator),
                TokenType::Slash => Interpreter::divide(&left, &right, &operator),
                TokenType::Star => Interpreter::multiply(&left, &right, &operator),
                _ => Err(Error::LoxRuntimeError),
            }
        }

        fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Rc<LiteralValue>, Error> {
            self.evaluate(expr)
        }

        fn visit_unary_expr(
            &mut self,
            operator: &Token,
            right: &Expr,
        ) -> Result<Rc<LiteralValue>, Error> {
            let right_l = self.evaluate(right)?;
            match operator.get_token_type() {
                TokenType::Minus => match &*right_l {
                    LiteralValue::Number(number) => Ok(Rc::new(LiteralValue::Number(-number))),
                    _ => {
                        error(
                            operator.get_line(),
                            operator.get_column(),
                            format!("Operand must be a number."),
                            function_name!(),
                            Some(RLoxErrorType::RuntimeError),
                        );
                        Err(Error::LoxRuntimeError)
                    }
                },
                TokenType::Bang => Ok(Rc::new(LiteralValue::Bool(!Interpreter::is_truthy_lval(
                    &right_l,
                )))),
                _ => Err(Error::LoxRuntimeError),
            }
        }

        fn visit_variable_expr(&mut self, name: &Token) -> Result<Rc<LiteralValue>, Error> {
            self.look_up_variable(name)
        }

        fn visit_assign_expr(
            &mut self,
            name: &Token,
            value: &Expr,
        ) -> Result<Rc<LiteralValue>, Error> {
            let value_evaluated = self.evaluate(value)?;
            let distance = self.get_depth(name);

            match distance {
                Some(depth) => {
                    let mut env = self.environment.as_ref().borrow_mut();
                    env.assign_at(depth, name, value_evaluated.into());
                }
                None => {
                    self.globals
                        .borrow_mut()
                        .assign(name, value_evaluated.into())?;
                }
            }

            return self.visit_variable_expr(name);
        }

        fn visit_logical_expr(
            &mut self,
            left: &Expr,
            operator: &Token,
            right: &Expr,
        ) -> Result<Rc<LiteralValue>, Error> {
            let left_val = self.evaluate(left)?;
            let is_truthy = Interpreter::is_truthy_lval(&left_val);

            if let TokenType::Or = operator.get_token_type() {
                if is_truthy {
                    return Ok(left_val);
                }
            } else if !is_truthy {
                return Ok(left_val);
            }

            self.evaluate(right)
        }

        fn visit_call_expr(
            &mut self,
            callee: &Expr,
            parent: &Token,
            arguments: &Vec<Expr>,
        ) -> Result<Rc<LiteralValue>, Error> {
            let calle_local = self.evaluate(callee)?;
            let mut args = Vec::new();

            for arg in arguments {
                args.push(self.evaluate(arg)?);
            }

            let handle_arity = |arguments: usize, arity: usize| -> Result<(), Error> {
                if arguments != arity {
                    error(
                        parent.get_line(),
                        parent.get_column(),
                        format!("Expected {} arguments but got {}.", arity, arguments),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    return Err(Error::LoxRuntimeError);
                }
                Ok(())
            };

            if let LiteralValue::Callable(callable_box) = &*calle_local {
                if let Callable::Function(function) = callable_box {
                    match handle_arity(arguments.len(), function.arity()) {
                        Ok(_) => return function.call(self, &mut args),
                        Err(err) => return Err(err),
                    }
                }
            }

            if let LiteralValue::Callable(callable_box) = &*calle_local {
                if let Callable::Clock(function) = callable_box {
                    match handle_arity(arguments.len(), function.arity()) {
                        Ok(_) => return function.call(self, &mut args),
                        Err(err) => return Err(err),
                    }
                }
            }

            if let LiteralValue::Callable(callable_box) = &*calle_local {
                if let Callable::UnixTClock(function) = callable_box {
                    match handle_arity(arguments.len(), function.arity()) {
                        Ok(_) => return function.call(self, &mut args),
                        Err(err) => return Err(err),
                    }
                }
            }

            if let LiteralValue::Callable(callable_box) = &*calle_local {
                if let Callable::Class(function) = callable_box {
                    match handle_arity(arguments.len(), function.arity()) {
                        Ok(_) => return function.call(self, &mut args),
                        Err(err) => return Err(err),
                    }
                }
            }

            error(
                parent.get_line(),
                parent.get_column(),
                String::from("Can only call functions and classes"),
                function_name!(),
                Some(RLoxErrorType::RuntimeError),
            );
            Err(Error::LoxRuntimeError)
        }

        fn visit_get_expr(
            &mut self,
            object: &Expr,
            name: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            match &*self.evaluate(object)? {
                LiteralValue::Callable(call_box) => {
                    if let Callable::Instance(instance) = call_box {
                        return instance.borrow_mut().get(name);
                    }
                }
                _ => {
                    error(
                        name.get_line(),
                        name.get_column(),
                        format!(
                            "Error at '{}': Only instances can have properties.",
                            name.get_token_type()
                        ),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    return Err(Error::LoxRuntimeError);
                }
            };
            Err(Error::LoxRuntimeError)
        }

        fn visit_set_expr(
            &mut self,
            object: &Expr,
            name: &Token,
            value: &Expr,
        ) -> Result<Rc<LiteralValue>, Error> {
            let obj_l = self.evaluate(object)?;
            let value_l = self.evaluate(value)?;
            match &*obj_l {
                LiteralValue::Callable(Callable::Instance(instance)) => {
                    instance.borrow_mut().set(name, Rc::clone(&value_l));
                    return Ok(value_l);
                }
                _ => {
                    error(
                        name.get_line(),
                        name.get_column(),
                        format!(
                            "Error at '{}': Only instances have fields.",
                            name.get_token_type()
                        ),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    return Err(Error::LoxRuntimeError);
                }
            }
        }

        fn visit_this_expr(&mut self, keyword: &Token) -> Result<Rc<LiteralValue>, Error> {
            self.look_up_variable(keyword)
        }

        fn visit_super_expr(
            &mut self,
            keyword: &Token,
            method: &Token,
        ) -> Result<Rc<LiteralValue>, Error> {
            let distance = self.get_depth(keyword).unwrap();
            let superclass = self.environment.borrow_mut().get_at(
                distance,
                &Token::new(TokenType::Super, String::from("super"), 0, 0, 0),
            )?;
            let instance_obj = self.environment.borrow_mut().get_at(
                distance - 1,
                &Token::new(TokenType::This, String::from("this"), 0, 0, 0),
            )?;

            match (
                Rc::unwrap_or_clone(superclass),
                Rc::unwrap_or_clone(instance_obj),
            ) {
                (
                    LiteralValue::Callable(Callable::Class(klass)),
                    LiteralValue::Callable(Callable::Instance(instance)),
                ) => match klass.find_method(&method.get_token_type().to_string()) {
                    Some(mut method) => {
                        return Ok(Rc::new(LiteralValue::Callable(Callable::Function(
                            method.bind(instance),
                        ))));
                    }
                    None => {
                        error(
                            method.get_line(),
                            method.get_column(),
                            format!("Undefined property '{}'.", method.get_token_type()),
                            function_name!(),
                            Some(RLoxErrorType::RuntimeError),
                        );
                        return Err(Error::LoxRuntimeError);
                    }
                },
                (_, _) => {
                    return Err(Error::LoxRuntimeError);
                }
            }
        }
    }

    impl StmtVisitor<Result<(), Error>> for Interpreter {
        fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), Error> {
            self.evaluate(expr)?;
            Ok(())
        }

        fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), Error> {
            let value = self.evaluate(expr)?;
            println!("{}", value);
            Ok(())
        }

        fn visit_return_stmt(&mut self, keyword: &Token, expr: &Expr) -> Result<(), Error> {
            if self.environment.as_ref().borrow_mut().enclosing.is_none() {
                error(
                    keyword.get_line(),
                    keyword.get_column(),
                    format!(
                        "Error at '{}': Can't return from top-level code.",
                        keyword.get_token_type()
                    ),
                    function_name!(),
                    Some(RLoxErrorType::RuntimeError),
                );
                return Err(Error::LoxRuntimeError);
            }
            let return_val = self.evaluate(expr)?;
            Err(Error::Return(return_val))
        }

        fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<(), Error> {
            let value = self.evaluate(initializer)?;
            self.environment.as_ref().borrow_mut().define(name, value);
            Ok(())
        }

        fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
            let env = Environment::new(Rc::clone(&self.environment));
            self.execute_block(stmts, env)
        }

        fn visit_class_stmt(
            &mut self,
            name: &Token,
            superclass: &Option<Expr>,
            statements: &Vec<Stmt>,
        ) -> Result<(), Error> {
            let mut super_class = None;
            let report_superclass_err = || {
                error(
                    name.get_line(),
                    name.get_column(),
                    format!(
                        "Error at '{}': Superclass must be a class.",
                        name.get_token_type()
                    ),
                    function_name!(),
                    Some(RLoxErrorType::RuntimeError),
                );
                return Err(Error::LoxRuntimeError);
            };

            if let Some(superclass) = superclass {
                super_class = match Rc::unwrap_or_clone(self.evaluate(superclass)?) {
                    LiteralValue::Callable(callable) => {
                        if let Callable::Class(klass) = callable {
                            Some(klass)
                        } else {
                            return report_superclass_err();
                        }
                    }
                    _ => return report_superclass_err(),
                }
            }

            self.environment
                .as_ref()
                .borrow_mut()
                .define(name, Rc::new(LiteralValue::Nil));

            if let Some(super_class) = super_class.clone() {
                self.environment =
                    Rc::new(RefCell::new(Environment::new(Rc::clone(&self.environment))));
                self.environment.as_ref().borrow_mut().define_str(
                    "super",
                    Rc::new(LiteralValue::Callable(Callable::Class(super_class))),
                );
            }

            let mut methods = HashMap::default();
            for method in statements {
                if let Stmt::Function(fn_name, _, _) = method {
                    let lox_fun: RLoxFunction = RLoxFunction::new(
                        method.clone(),
                        Rc::clone(&self.environment),
                        fn_name.get_token_type().to_string() == "init",
                    );
                    methods.insert(fn_name.get_token_type().to_string(), lox_fun);
                }
            }

            let klass: RLoxClass = RLoxClass::new(
                name.get_token_type().to_string(),
                methods,
                match super_class.clone() {
                    Some(super_klass) => Some(super_klass),
                    None => None,
                },
            );

            if let Some(_) = super_class {
                let previous = self.environment.borrow().clone().enclosing.unwrap();
                self.environment = previous;
            }

            self.environment.as_ref().borrow_mut().assign(
                name,
                Rc::new(LiteralValue::Callable(Callable::Class(klass))),
            )?;
            Ok(())
        }

        fn visit_function_stmt(
            &mut self,
            name: &Token,
            params: &Vec<Token>,
            body: &Vec<Stmt>,
        ) -> Result<(), Error> {
            let func: RLoxFunction = RLoxFunction::new(
                Stmt::Function(name.clone(), params.clone(), body.clone()),
                Rc::clone(&self.environment),
                false,
            );
            self.environment.as_ref().borrow_mut().define(
                name,
                Rc::new(LiteralValue::Callable(Callable::Function(func))),
            );
            Ok(())
        }

        fn visit_if_stmt(
            &mut self,
            expr: &Expr,
            stmt: &Stmt,
            else_stmt: &Option<Box<Stmt>>,
        ) -> Result<(), Error> {
            let value = self.evaluate(expr)?;
            if Interpreter::is_truthy_lval(&value) {
                return self.execute(stmt);
            } else if let Some(else_) = else_stmt {
                return self.execute(&else_);
            }

            Ok(())
        }

        fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> Result<(), Error> {
            let mut l_val = self.evaluate(expr)?;
            while Interpreter::is_truthy_lval(&l_val) {
                self.execute(stmt)?;
                l_val = self.evaluate(expr)?;
            }
            Ok(())
        }
    }
}
