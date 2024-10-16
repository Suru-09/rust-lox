pub mod interpreter {

    use crate::environment::environment::{Environment, EnvironmentStack};
    use crate::error_handling::error_handling::error;
    use crate::expr::expr::{Expr, Visitor};
    use crate::function_name;
    use crate::rlox_callable::rlox_callable::{Clock, RLoxClass, RLoxFunction};
    use crate::scanner::scan::{Token, TokenType};
    use crate::stmt::stmt::{LiteralValue, Stmt, StmtVisitor};
    use std::any::Any;
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
        pub return_value: Option<Box<dyn Any>>,
        pub locals: Vec<(Expr, usize)>,
    }

    impl Interpreter {
        pub fn new() -> Interpreter {
            let env = Rc::new(RefCell::new(EnvironmentStack::new()));
            // add the clock function to the global environment.
            // it will be available in all the scopes.
            env.borrow_mut()
                .push_env(Rc::new(RefCell::new(Environment::new())));
            env.borrow_mut()
                .define("clock".to_string(), Box::new(Clock {}));

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

            Interpreter {
                environment: env,
                return_value: None,
                locals,
            }
        }

        fn evaluate(&mut self, expr: &Expr) -> Result<LiteralValue, String> {
            expr.accept(self)
        }

        fn is_truthy(self, token: &Token) -> Result<LiteralValue, String> {
            match token.get_token_type() {
                TokenType::Nil => Ok(LiteralValue::Nil),
                TokenType::False => Ok(LiteralValue::Bool(true)),
                TokenType::True => Ok(LiteralValue::Bool(true)),
                _ => Err("Given token can't be considered as a boolean".to_string()),
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

        fn look_up_variable(&mut self, token: &Token, expr: Expr) -> Result<(), String> {
            match self.get_depth(token, expr) {
                Ok(depth) => {
                    let variable = self.get_at(depth, token.get_token_type().to_string())?;
                    Ok(variable)
                }
                Err(err) => Err(err),
            }
        }

        fn get_at(&mut self, distance: usize, name: String) -> Result<(), String> {
            let mut env = self.environment.as_ref().borrow_mut();
            match env.get_at(distance, name.clone()) {
                Some(value) => Ok(value),
                None => match env.get(name.clone()) {
                    Some(value) => Ok(value),
                    None => Err(format!("Variable '{}' is undefined.", name)),
                },
            }
        }

        fn is_token_string(self, token: &Token) -> bool {
            match token.get_token_type() {
                TokenType::String(_) => true,
                _ => false,
            }
        }

        fn is_token_number(self, token: &Token) -> bool {
            match token.get_token_type() {
                TokenType::Number(_) => true,
                _ => false,
            }
        }

        fn substract(
            self,
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, String> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Number(number1 - number2))
                }
                _ => Err("In order to substract two things they need to be numbers".to_string()),
            }
        }

        fn add(
            self,
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, String> {
            match (operand1, operand2) {
                (LiteralValue::String(s1), LiteralValue::String(s2)) => {
                    Ok(LiteralValue::String(String::from(s1.to_string() + s2)))
                }
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Number(number1 + number2))
                }
                _ => {
                    Err("In order to add two things they need to be numbers or strings".to_string())
                }
            }
        }

        fn multiply(
            self,
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, String> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Number(number1 * number2))
                }
                _ => Err("In order to multiply two things they need to be numbers".to_string()),
            }
        }

        fn divide(
            self,
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, String> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Number(number1 / number2))
                }
                _ => Err("In order to divide two things they need to be numbers".to_string()),
            }
        }

        fn greater(
            self,
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, String> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 > number2))
                }
                _ => Err("In order to compare them, operands must be two numbers.".to_string()),
            }
        }

        fn greater_equal(
            self,
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, String> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 >= number2))
                }
                _ => Err("In order to compare them, operands must be two numbers.".to_string()),
            }
        }

        fn less(
            self,
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, String> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 < number2))
                }
                _ => Err("In order to compare them, operands must be two numbers.".to_string()),
            }
        }

        fn less_equal(
            &mut self,
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, String> {
            match (operand1, operand2) {
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 < number2))
                }
                _ => Err("In order to compare them, operands must be two numbers.".to_string()),
            }
        }

        fn equal_equal(
            self,
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, String> {
            match (operand1, operand2) {
                (LiteralValue::String(s1), LiteralValue::String(s2)) => {
                    Ok(LiteralValue::Bool(s1 == s2))
                }
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 == number2))
                }
                _ => Err("Could do perform == on object of different types".to_string()),
            }
        }

        fn bang_equal(
            self,
            operand1: &LiteralValue,
            operand2: &LiteralValue,
        ) -> Result<LiteralValue, String> {
            match (operand1, operand2) {
                (LiteralValue::String(s1), LiteralValue::String(s2)) => {
                    Ok(LiteralValue::Bool(s1 != s2))
                }
                (LiteralValue::Number(number1), LiteralValue::Number(number2)) => {
                    Ok(LiteralValue::Bool(number1 != number2))
                }
                _ => Err("Could do perform != on object of different types".to_string()),
            }
        }

        pub fn execute(&mut self, stmt: &Stmt) -> Result<(), String> {
            stmt.accept(self)
        }

        pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), String> {
            for stmt in statements {
                self.execute(stmt)?;
            }
            Ok(())
        }

        pub fn execute_block(
            &mut self,
            stmts: &Vec<Stmt>,
            env: Rc<RefCell<Environment>>,
        ) -> Result<(), String> {
            self.environment.as_ref().borrow_mut().push_env(env);
            for stmt in stmts {
                match stmt {
                    Stmt::ReturnStmt(_, _) => return Ok(()),
                    _ => return Ok(self.execute(stmt)?),
                }
            }
            self.environment.as_ref().borrow_mut().pop();
            Err("Block statement err".to_string())
        }
    }

    impl Visitor<Result<LiteralValue, String>> for Interpreter {
        fn visit_literal_expr(&mut self, value: &Token) -> Result<LiteralValue, String> {
            Ok(value.clone())
        }

        fn visit_binary_expr(
            &mut self,
            left: &Expr,
            operator: &Token,
            right: &Expr,
        ) -> Result<LiteralValue, String> {
            // ? is the try operator, used to propagate errors.
            let left = self.evaluate(left)?;
            let right = self.evaluate(right)?;

            match operator.get_token_type() {
                TokenType::Greater => self.greater(left, right),
                TokenType::GreaterEqual => self.greater_equal(left, right),
                TokenType::Less => self.less(left, right),
                TokenType::LessEqual => self.less_equal(left, right),
                TokenType::BangEqual => self.bang_equal(left, right),
                TokenType::EqualEqual => self.equal_equal(left, right),
                TokenType::Minus => self.substract(left, right),
                TokenType::Plus => self.add(left, right),
                TokenType::Slash => self.divide(left, right),
                TokenType::Star => self.multiply(left, right),
                _ => Err("The given operator is not a binary operator.".to_string()),
            }
        }

        fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<LiteralValue, String> {
            self.evaluate(expr)
        }

        fn visit_unary_expr(
            &mut self,
            operator: &Token,
            right: &Expr,
        ) -> Result<LiteralValue, String> {
            match operator.get_token_type() {
                TokenType::Minus => Ok(Box::new(right.clone())),
                TokenType::Bang => Ok(Box::new(self.is_truthy(Box::new(right.clone())))),
                _ => Err("The given operator is not a unary operator.".to_string()),
            }
        }

        fn visit_variable_expr(&mut self, name: &Token) -> Result<LiteralValue, String> {
            let expr = Expr::Variable(name.clone());
            return self.look_up_variable(name, expr);
        }

        fn visit_assign_expr(
            &mut self,
            name: &Token,
            value: &Expr,
        ) -> Result<LiteralValue, String> {
            let value_evaluated = self.evaluate(value)?;
            let distance = self.get_depth(name, value.clone());

            match distance {
                Ok(depth) => {
                    let mut env = self.environment.as_ref().borrow_mut();
                    env.assign_at(
                        depth,
                        name.get_token_type().to_string(),
                        value_evaluated.into(),
                    )?;
                }
                Err(_) => {
                    let mut env = self.environment.as_ref().borrow_mut();
                    env.assign(name.get_token_type().to_string(), value_evaluated.into())?;
                }
            }

            return self.visit_variable_expr(name);
        }

        fn visit_logical_expr(
            &mut self,
            left: &Expr,
            operator: &Token,
            right: &Expr,
        ) -> Result<LiteralValue, String> {
            let left_val = self.evaluate(left)?;
            let is_truthy = self.is_truthy(left_val)?;
            if let Some(truth) = is_truthy.downcast_ref::<Token>() {
                if truth.get_token_type() == TokenType::True {
                    if operator.get_token_type() == TokenType::Or {
                        return Ok(Box::new(truth.clone()));
                    }
                    return self.evaluate(right);
                } else if truth.get_token_type() == TokenType::False {
                    if operator.get_token_type() == TokenType::Or {
                        return self.evaluate(right);
                    }
                    return Ok(Box::new(truth.clone()));
                }
            }
            return Err(
                "Could not visit Logical Expression, truthy might be a reason.".to_string(),
            );
        }

        fn visit_call_expr(
            &mut self,
            callee: &Expr,
            _: &Token,
            arguments: &Vec<Expr>,
        ) -> Result<LiteralValue, String> {
            let calle_local = self.evaluate(callee)?;

            // ! TODO: I will delay the arity check until I implement the functions.

            let mut args = Vec::new();
            for arg in arguments {
                args.push(self.evaluate(arg)?);
            }

            if let Some(callee) = calle_local.downcast_ref::<RLoxFunction>() {
                return callee.call(self, &mut args);
            }

            if let Some(callee) = calle_local.downcast_ref::<Clock>() {
                return callee.call(self, &mut args);
            }

            Err("Function has not been implemented yet.".to_string())
        }
    }

    impl StmtVisitor<Result<(), String>> for Interpreter {
        fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), String> {
            self.evaluate(expr)?;
            Ok(())
        }

        fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), String> {
            let value = self.evaluate(expr)?;

            if let Some(token) = value.downcast_ref::<Token>() {
                println!("{}", token.get_token_type());
                return Ok(Box::new(token.clone()));
            }

            if let Some(expr) = value.downcast_ref::<Expr>() {
                println!("{}", expr);
                return Ok(Box::new(expr.clone()));
            }

            if let Some(stmt) = value.downcast_ref::<Stmt>() {
                println!("{}", stmt);
                return Ok(Box::new(stmt.clone()));
            }

            if let Some(rlox_func) = value.downcast_ref::<RLoxFunction>() {
                println!("{}", rlox_func.to_string());
                return Ok(Box::new(rlox_func.clone()));
            }

            if let Some(rlox_class) = value.downcast_ref::<RLoxClass>() {
                println!("{}", rlox_class.to_string());
                return Ok(Box::new(rlox_class.clone()));
            }

            Err("Could not print value.".to_string())
        }

        fn visit_return_stmt(&mut self, _keyword: &Token, expr: &Expr) -> Result<(), String> {
            let _: Expr = self.evaluate(expr)?;
            Err("Could not return value.".to_string())
        }

        fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<(), String> {
            let value = self.evaluate(initializer)?;

            self.environment
                .as_ref()
                .borrow_mut()
                .define(name.get_token_type().to_string(), Box::new(value.clone()));
            Ok(value)
        }

        fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), String> {
            let env = Rc::new(RefCell::new(Environment::new()));
            self.execute_block(stmts, env)
        }

        fn visit_class_stmt(&mut self, name: &Token, _: &Vec<Stmt>) -> Result<(), String> {
            let klass: RLoxClass = RLoxClass::new(name.get_token_type().to_string().clone());
            self.environment
                .as_ref()
                .borrow_mut()
                .define(name.get_token_type().to_string(), Box::new(klass));
            Ok(Box::new(name.clone()))
        }

        fn visit_function_stmt(
            &mut self,
            name: &Token,
            params: &Vec<Token>,
            body: &Vec<Stmt>,
        ) -> Result<(), String> {
            let func: RLoxFunction = RLoxFunction::new(
                Stmt::Function(name.clone(), params.clone(), body.clone()),
                self.environment.as_ref().borrow_mut().peek().unwrap(),
            );
            self.environment
                .as_ref()
                .borrow_mut()
                .define(name.get_token_type().to_string(), Box::new(func));
            Ok(Box::new(name.clone()))
        }

        fn visit_if_stmt(
            &mut self,
            expr: &Expr,
            stmt: &Stmt,
            else_stmt: &Option<Box<Stmt>>,
        ) -> Result<(), String> {
            let value = self.evaluate(expr)?;
            let is_truthy = self.is_truthy(value)?;
            if let Some(truth) = is_truthy.downcast_ref::<Token>() {
                if truth.get_token_type() == TokenType::True {
                    return self.execute(stmt);
                } else if truth.get_token_type() == TokenType::False {
                    if let Some(else_stmt) = else_stmt {
                        return self.execute(&*else_stmt);
                    }
                    return Ok(Box::new(Token::new(
                        TokenType::Nil,
                        "".to_string(),
                        0,
                        0,
                        0,
                    )));
                }
            }
            return Err("Could not visit IF statement, truthy might be a reason.".to_string());
        }

        fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> Result<(), String> {
            let value = self.evaluate(expr)?;
            let is_truthy = self.is_truthy(value)?;
            if let Some(truth) = is_truthy.downcast_ref::<Token>() {
                if truth.get_token_type() == TokenType::True {
                    self.execute(stmt)?;
                    return self.visit_while_stmt(expr, stmt);
                } else if truth.get_token_type() == TokenType::False {
                    return Ok(Box::new(Token::new(
                        TokenType::Nil,
                        "".to_string(),
                        0,
                        0,
                        0,
                    )));
                }
            }
            return Err("Could not visit WHILE statement, truthy might be a reason.".to_string());
        }
    }
}
