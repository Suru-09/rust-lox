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
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct Interpreter {
        pub environment: Rc<RefCell<Environment>>,
        pub locals: Vec<(Expr, usize)>,
        pub globals: Rc<RefCell<Environment>>,
    }

    #[derive(Debug, PartialEq)]
    pub enum Error {
        LoxRuntimeError(String),
        Return(LiteralValue),
    }

    impl Error {
        pub fn from_string(str: &str) -> Error {
            Error::LoxRuntimeError(String::from(str))
        }
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
                LiteralValue::Callable(Box::new(Callable::Clock(Clock {}))),
            );

            globals.borrow_mut().define(
                &Token::new(
                    TokenType::Identifier("unixClock".to_string()),
                    "unixClock".to_string(),
                    999,
                    999,
                    999,
                ),
                LiteralValue::Callable(Box::new(Callable::UnixTClock(UnixTClock {}))),
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

        fn evaluate(&mut self, expr: &Expr) -> Result<LiteralValue, Error> {
            expr.accept(self)
        }

        fn is_truthy(expr: &Expr) -> bool {
            match Interpreter::convert_expr_to_literal_value(expr) {
                Ok(l_val) => match l_val {
                    LiteralValue::Bool(b) => b,
                    LiteralValue::Nil => false,
                    _ => true,
                },
                Err(_) => false,
            }
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

        fn look_up_variable(&mut self, token: &Token) -> Result<LiteralValue, Error> {
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
        ) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Number(number1 - number2))
                }
                _ => Err(Error::from_string(
                    "In order to substract two things they need to be numbers",
                )),
            }
        }

        fn add(operand1: &LiteralValue, operand2: &LiteralValue) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::String(s1), LiteralValue::String(s2)) => {
                    Ok(LiteralValue::String(String::from(s1.to_string() + s2)))
                }
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Number(number1 + number2))
                }
                (LiteralValue::String(str), LiteralValue::Number(num)) => {
                    Ok(LiteralValue::String(str.clone() + &num.to_string()))
                }
                (LiteralValue::Number(num), LiteralValue::String(str)) => {
                    Ok(LiteralValue::String(str.clone() + &num.to_string()))
                }
                _ => Err(Error::from_string(
                    "In order to add two things they need to be numbers or strings",
                )),
            }
        }

        fn multiply(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Number(number1 * number2))
                }
                _ => Err(Error::from_string(
                    "In order to multiply two things they need to be numbers",
                )),
            }
        }

        fn divide(operand1: &LiteralValue, operand2: &LiteralValue) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Number(number1 / number2))
                }
                _ => Err(Error::from_string(
                    "In order to divide two things they need to be numbers",
                )),
            }
        }

        fn greater(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 > number2))
                }
                _ => Err(Error::from_string(
                    "In order to compare them, operands must be two numbers.",
                )),
            }
        }

        fn greater_equal(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 >= number2))
                }
                _ => Err(Error::from_string(
                    "In order to compare them, operands must be two numbers.",
                )),
            }
        }

        fn less(operand1: &LiteralValue, operand2: &LiteralValue) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 < number2))
                }
                _ => Err(Error::from_string(
                    "In order to compare them, operands must be two numbers.",
                )),
            }
        }

        fn less_equal(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 <= number2))
                }
                _ => Err(Error::from_string(
                    "In order to compare them, operands must be two numbers.",
                )),
            }
        }

        fn equal_equal(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::String(s1), LiteralValue::String(s2)) => {
                    Ok(LiteralValue::Bool(s1 == s2))
                }
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 == number2))
                }
                (LiteralValue::Bool(bool1), LiteralValue::Bool(bool2)) => {
                    Ok(LiteralValue::Bool(bool1 == bool2))
                }
                _ => Ok(LiteralValue::Bool(false)),
            }
        }

        fn bang_equal(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::String(s1), LiteralValue::String(s2)) => {
                    Ok(LiteralValue::Bool(s1 != s2))
                }
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 != number2))
                }
                (LiteralValue::Bool(bool1), LiteralValue::Bool(bool2)) => {
                    Ok(LiteralValue::Bool(bool1 != bool2))
                }
                _ => Ok(LiteralValue::Bool(true)),
            }
        }

        fn convert_expr_to_literal_value(expr: &Expr) -> Result<LiteralValue, Error> {
            match expr {
                Expr::Literal(ltype) => Ok(ltype.clone()),
                _ => Err(Error::from_string("Expression has to be a literal!!!")),
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

    impl Visitor<Result<LiteralValue, Error>> for Interpreter {
        fn visit_literal_expr(&mut self, value: &LiteralValue) -> Result<LiteralValue, Error> {
            Ok(value.clone())
        }

        fn visit_binary_expr(
            &mut self,
            left: &Expr,
            operator: &Token,
            right: &Expr,
        ) -> Result<LiteralValue, Error> {
            // ? is the try operator, used to propagate errors.
            let left = self.evaluate(left)?.clone();
            let right = self.evaluate(right)?.clone();

            match operator.get_token_type() {
                TokenType::Greater => Interpreter::greater(&left, &right),
                TokenType::GreaterEqual => Interpreter::greater_equal(&left, &right),
                TokenType::Less => Interpreter::less(&left, &right),
                TokenType::LessEqual => Interpreter::less_equal(&left, &right),
                TokenType::BangEqual => Interpreter::bang_equal(&left, &right),
                TokenType::EqualEqual => Interpreter::equal_equal(&left, &right),
                TokenType::Minus => Interpreter::substract(&left, &right),
                TokenType::Plus => Interpreter::add(&left, &right),
                TokenType::Slash => Interpreter::divide(&left, &right),
                TokenType::Star => Interpreter::multiply(&left, &right),
                _ => Err(Error::from_string(
                    "The given operator is not a binary operator.",
                )),
            }
        }

        fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<LiteralValue, Error> {
            self.evaluate(expr)
        }

        fn visit_unary_expr(
            &mut self,
            operator: &Token,
            right: &Expr,
        ) -> Result<LiteralValue, Error> {
            match operator.get_token_type() {
                TokenType::Minus => match right {
                    Expr::Literal(ltype) => match ltype {
                        LiteralValue::Number(number) => Ok(LiteralValue::Number(-number)),
                        _ => Err(Error::from_string("Operand must be a number!")),
                    },
                    _ => Err(Error::from_string("Operand must be a number!")),
                },
                TokenType::Bang => Ok(LiteralValue::Bool(!Interpreter::is_truthy(right))),
                _ => Err(Error::from_string(
                    "The given operator is not a unary operator.",
                )),
            }
        }

        fn visit_variable_expr(&mut self, name: &Token) -> Result<LiteralValue, Error> {
            self.look_up_variable(name)
        }

        fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<LiteralValue, Error> {
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
        ) -> Result<LiteralValue, Error> {
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
        ) -> Result<LiteralValue, Error> {
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
                        format!(
                            "Function signature has {} parameters, however {} args are received",
                            arity, arguments
                        ),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    return Err(Error::from_string(&format!(
                        "Function signature has {} parameters, however {} args are received",
                        arity, arguments
                    )));
                }
                Ok(())
            };

            if let LiteralValue::Callable(callable_box) = calle_local.clone() {
                if let Callable::Function(function) = *callable_box {
                    match handle_arity(arguments.len(), function.arity()) {
                        Ok(_) => return function.call(self, &mut args),
                        Err(err) => return Err(err),
                    }
                }
            }

            if let LiteralValue::Callable(callable_box) = calle_local.clone() {
                if let Callable::Clock(function) = *callable_box {
                    match handle_arity(arguments.len(), function.arity()) {
                        Ok(_) => return function.call(self, &mut args),
                        Err(err) => return Err(err),
                    }
                }
            }

            if let LiteralValue::Callable(callable_box) = calle_local.clone() {
                if let Callable::UnixTClock(function) = *callable_box {
                    match handle_arity(arguments.len(), function.arity()) {
                        Ok(_) => return function.call(self, &mut args),
                        Err(err) => return Err(err),
                    }
                }
            }

            Err(Error::from_string("Function has not been implemented yet."))
        }
    }

    impl StmtVisitor<Result<(), Error>> for Interpreter {
        fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), Error> {
            self.evaluate(expr)?;
            Ok(())
        }

        fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), Error> {
            let value: LiteralValue = self.evaluate(expr)?;
            println!("{}", value);
            Ok(())
        }

        fn visit_return_stmt(&mut self, _keyword: &Token, expr: &Expr) -> Result<(), Error> {
            let return_val = self.evaluate(expr)?;
            Err(Error::Return(return_val))
        }

        fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<(), Error> {
            let value: LiteralValue = self.evaluate(initializer)?;

            self.environment.as_ref().borrow_mut().define(name, value);
            Ok(())
        }

        fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
            let env = Environment::new(Rc::clone(&self.environment));
            self.execute_block(stmts, env)
        }

        fn visit_class_stmt(&mut self, name: &Token, _: &Vec<Stmt>) -> Result<(), Error> {
            let klass: RLoxClass = RLoxClass::new(name.get_token_type().to_string().clone());
            self.environment.as_ref().borrow_mut().define(
                name,
                LiteralValue::Callable(Box::new(Callable::Class(klass))),
            );
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
            );
            self.environment.as_ref().borrow_mut().define(
                name,
                LiteralValue::Callable(Box::new(Callable::Function(func))),
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
            match value {
                LiteralValue::Bool(is_true) => {
                    if is_true {
                        return self.execute(stmt);
                    }
                }
                _ => {
                    return Err(Error::from_string(
                        "Could not visit IF statement, truthy might be a reason.",
                    ))
                }
            }

            if let Some(else_) = else_stmt {
                return self.execute(&else_);
            }

            Ok(())
        }

        fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> Result<(), Error> {
            let value = self.evaluate(expr)?;
            match value {
                LiteralValue::Bool(value_bool) => {
                    if value_bool {
                        self.execute(stmt)?;
                        return self.visit_while_stmt(expr, stmt);
                    }
                }
                _ => return Err(Error::from_string("While condition is not a boolean!")),
            }
            Ok(())
        }
    }
}
