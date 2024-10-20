pub mod interpreter {

    use crate::environment::environment::{Environment, EnvironmentStack};
    use crate::error_handling::error_handling::error;
    use crate::expr::expr::{Expr, Visitor};
    use crate::function_name;
    use crate::rlox_callable::rlox_callable::{Callable, RLoxCallable, RLoxClass, RLoxFunction, Clock, UnixTClock};
    use crate::scanner::scan::{Token, TokenType};
    use crate::stmt::stmt::{LiteralValue, Stmt, StmtVisitor};
    use std::cell::RefCell;
    use std::rc::Rc;

    /**
     * ! Notes to my self:
     * ! No. 1:
     * * The final result of the interpretor visitor is a Literal, therefore If I try to downcast the result of the
     * * f64 or String for example, I will fail miserably, I would need to downcast the result to a Literal and then
     * * check the type of the literal.
     * ! No. 2:
     * * Could there be improvements in error handling? Everything seems too verbose.
     * ! No. 3:
     * TODO: At the moment it is not possible to keep track of the outermost environment,
     * TODO: therefore it is not possible to use the environment to define variables in the global scope.
     */
    pub struct Interpreter {
        pub environment: Rc<RefCell<EnvironmentStack>>,
        pub locals: Vec<(Expr, usize)>,
    }

    #[derive(Debug, PartialEq)]
    pub enum Error {
        LoxRuntimeError(String),
        Return(LiteralValue)
    }

    impl Error {
        pub fn from_string(str: &str) -> Error {
            Error::LoxRuntimeError(String::from(str))
        }
    }

    impl Interpreter {
        pub fn new() -> Interpreter {
            let env = Rc::new(RefCell::new(EnvironmentStack::new()));
            // add the clock function to the global environment.
            // it will be available in all the scopes.
            env.borrow_mut()
                .push_env(Rc::new(RefCell::new(Environment::new())));

            env.borrow_mut().define(
                &Token::new(TokenType::Identifier("clock".to_string()), "clock".to_string(), 999, 999, 999),
                LiteralValue::Callable(Box::new(Callable::Clock(Clock {}))),
            );

            env.borrow_mut().define(
                &Token::new(TokenType::Identifier("unixClock".to_string()), "unixClock".to_string(), 999, 999, 999),
                LiteralValue::Callable(Box::new(Callable::UnixTClock(UnixTClock {}))),
            );

            // push clock into locals.
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
                environment: env,
                locals,
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

        fn get_depth(&mut self, token: &Token, expr: Expr) -> Result<usize, String> {
            for (_, (e, depth)) in self.locals.iter().enumerate() {
                if *e == expr {
                    return Ok(*depth);
                }
            }
            error(
                token.get_line(),
                token.get_column(),
                format!(
                    "Could not find variable '{}' in the environment",
                    token.get_token_type().to_string()
                ),
                function_name!(),
            );
            Err(format!(
                "Could not find variable '{}' in the environment",
                token.get_token_type().to_string()
            ))
        }

        fn look_up_variable(&mut self, token: &Token, expr: Expr) -> Result<LiteralValue, Error> {
            match self.get_depth(token, expr) {
                Ok(depth) => {
                    let variable = self.get_at(depth, token)?;
                    Ok(variable)
                }
                Err(err) => Err(Error::LoxRuntimeError(err)),
            }
        }

        fn get_at(&mut self, distance: usize, token: &Token) -> Result<LiteralValue, Error> {
            let mut env = self.environment.as_ref().borrow_mut();
            match env.get_at(distance, token) {
                Some(value) => Ok(value),
                None => match env.get(token) {
                    Some(value) => Ok(value),
                    None => {
                        error(
                            token.get_line(),
                            token.get_column(),
                            format!(
                                "Could not find variable '{}' in the environment",
                                token.get_token_type().to_string()
                            ),
                            function_name!(),
                        );
                        Err(Error::LoxRuntimeError(format!("Variable '{}' is undefined.", token.get_token_type().to_string().clone())))
                    }
                },
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
                _ => Err(Error::from_string("In order to substract two things they need to be numbers")),
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
                _ => {
                    Err(Error::from_string("In order to add two things they need to be numbers or strings"))
                }
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
                _ => Err(Error::from_string("In order to multiply two things they need to be numbers")),
            }
        }

        fn divide(
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Number(number1 / number2))
                }
                _ => Err(Error::from_string("In order to divide two things they need to be numbers")),
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
                _ => Err(Error::from_string("In order to compare them, operands must be two numbers.")),
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
                _ => Err(Error::from_string("In order to compare them, operands must be two numbers.")),
            }
        }

        fn less(operand1: &LiteralValue, operand2: &LiteralValue) -> Result<LiteralValue, Error> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 < number2))
                }
                _ => Err(Error::from_string("In order to compare them, operands must be two numbers.")),
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
                _ => Err(Error::from_string("In order to compare them, operands must be two numbers.")),
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
                _ => Err(Error::from_string("Could do perform == on object of different types")),
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
                _ => Err(Error::from_string("Could do perform != on object of different types")),
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

        pub fn execute_block(
            &mut self,
            stmts: &Vec<Stmt>,
            env: Rc<RefCell<Environment>>,
        ) -> Result<(), Error> {
            self.environment.as_ref().borrow_mut().push_env(env);
            for stmt in stmts {
                self.execute(stmt)?;
            }
            self.environment.as_ref().borrow_mut().pop();
            Ok(())
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
                _ => Err(Error::from_string("The given operator is not a binary operator.")),
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
                TokenType::Bang => Ok(LiteralValue::Bool(Interpreter::is_truthy(right))),
                _ => Err(Error::from_string("The given operator is not a unary operator.")),
            }
        }

        fn visit_variable_expr(&mut self, name: &Token) -> Result<LiteralValue, Error> {
            let expr = Expr::Variable(name.clone());
            self.look_up_variable(name, expr)
        }

        fn visit_assign_expr(
            &mut self,
            name: &Token,
            value: &Expr,
        ) -> Result<LiteralValue, Error> {
            let value_evaluated = self.evaluate(value)?;
            //let distance = self.get_depth(name, value.clone());

            // match distance {
            //     Ok(depth) => {
            //         let mut env = self.environment.as_ref().borrow_mut();
            //         env.assign_at(
            //             depth,
            //             name.get_token_type().to_string(),
            //             value_evaluated.into(),
            //         )?;
            //     }
            //     Err(_) => {
            //         let mut env = self.environment.as_ref().borrow_mut();
            //         env.assign(name.get_token_type().to_string(), value_evaluated.into())?;
            //     }
            // }
            {
                let mut env = self.environment.as_ref().borrow_mut();
                env.assign(name, value_evaluated.into())?;
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
            _: &Token,
            arguments: &Vec<Expr>,
        ) -> Result<LiteralValue, Error> {
            let calle_local = self.evaluate(callee)?;

            // ! TODO: I will delay the arity check until I implement the functions.

            let mut args = Vec::new();
            for arg in arguments {
                args.push(self.evaluate(arg)?);
            }

            if let LiteralValue::Callable(callable_box) = calle_local.clone() {
                if let Callable::Function(function) = *callable_box {
                    return function.call(self, &mut args);
                }
            }

            // to do !!! reimplement clock
            // if let Some(callee) = calle_local.downcast_ref::<Clock>() {
            //     return callee.call(self, &mut args);
            // }

            if let LiteralValue::Callable(callable_box) = calle_local.clone() {
                if let Callable::Clock(function) = *callable_box {
                    return function.call(self, &mut args);
                }
            }

            if let LiteralValue::Callable(callable_box) = calle_local.clone() {
                if let Callable::UnixTClock(function) = *callable_box {
                    return function.call(self, &mut args);
                }
            }

            println!("calle_local: {:?}", calle_local);
            Err(Error::from_string("Function has not been implemented yet."))
        }
    }

    impl StmtVisitor<Result<(), Error>> for Interpreter {
        fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), Error> {
            self.evaluate(expr)?;
            Ok(())
        }

        fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), Error> {
            let value = self.evaluate(expr)?;
            println!("{:?}", value);
            Ok(())
        }

        fn visit_return_stmt(&mut self, _keyword: &Token, expr: &Expr) -> Result<(), Error> {
            let return_val = self.evaluate(expr)?;
            Err(Error::Return(return_val))
        }

        fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<(), Error> {
            let value = self.evaluate(initializer)?;

            let stack_len = self.environment.as_ref().borrow_mut().len();
            self.resolve(initializer.clone(), stack_len);

            self.environment
                .as_ref()
                .borrow_mut()
                .define(name, value.clone());
            Ok(())
        }

        fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
            let env = Rc::new(RefCell::new(Environment::new()));
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
                self.environment.as_ref().borrow_mut().peek().unwrap(),
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
            let is_truthy = Interpreter::is_truthy_lval(&value);
            if is_truthy {
                match value {
                    LiteralValue::Bool(is_true) => {
                        if is_true {
                            return self.execute(stmt);
                        }
                    }
                    _ => {
                        return Err(
                            Error::from_string("Could not visit IF statement, truthy might be a reason.")
                        )
                    }
                }
            }

            if let Some(else_) = else_stmt {
                return self.execute(&else_);
            }

            Ok(())
        }

        fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> Result<(), Error> {
            let value = self.evaluate(expr)?;
            let is_truthy = Interpreter::is_truthy_lval(&value);
            if is_truthy {
                match value {
                    LiteralValue::Bool(value_bool) => {
                        if value_bool {
                            self.execute(stmt)?;
                            return self.visit_while_stmt(expr, stmt);
                        } else {
                            return Ok(());
                        }
                    }
                    _ => return Err(Error::from_string("While condition is not a boolean!")),
                }
            }
            Ok(())
        }
    }
}
