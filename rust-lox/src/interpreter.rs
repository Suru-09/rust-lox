pub mod interpreter {

    use crate::expr::expr::{Expr, Visitor};
    use crate::stmt::stmt::{Stmt, StmtVisitor};
    use crate::scanner::scan::{Token, TokenType};
    use crate::environment::environment::{EnvironmentStack, Environment};
    use std::any::Any;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::rlox_callable::rlox_callable::{Clock, RLoxFunction, RLoxCallable, RLoxClass};

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


static mut GLOBAL_ENVIRONMENT: Option<Rc<RefCell<EnvironmentStack>>> = None;

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
        env.borrow_mut().push_env(Rc::new(RefCell::new(Environment::new())));
        env.borrow_mut().define("clock".to_string(), Box::new(Clock{}));

        // push clock into locals.
        let mut locals = Vec::new();
        locals.push((Expr::Variable(Token::new(TokenType::Identifier("clock".to_string()), "".to_string(), 0, 0, 0)), 0));

        unsafe {
            GLOBAL_ENVIRONMENT = Some(env.clone());
        }
        
        Interpreter {
            environment: env,
            return_value: None,
            locals,
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Box<dyn Any>, String> {
        expr.accept(self)
    }

    pub fn get_global_environment(&mut self) -> Rc<RefCell<EnvironmentStack>> {
        unsafe {
            GLOBAL_ENVIRONMENT.as_ref().unwrap().clone()
        }
    }

    fn is_truthy(&mut self, obj: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        match obj.downcast::<Token>() {
            Ok(d_obj) => {
                match d_obj.get_token_type() {
                    TokenType::Nil => Ok(Box::new(Token::new(TokenType::False, "".to_string(), 0, 0, 0))),
                    TokenType::True => Ok(Box::new(Token::new(TokenType::True, "".to_string(), 0, 0, 0))),
                    TokenType::False => Ok(Box::new(Token::new(TokenType::False, "".to_string(), 0, 0, 0))),
                    _ => Err("Given token can't be considered as a boolean".to_string()),
                }
            },
            Err(_) => Err("Could not downcast object to Token".to_string()),
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
        Err(format!("Could not find variable '{}' in the environment", token.get_token_type().to_string()))
    }

    fn look_up_variable(&mut self, token: &Token, expr: Expr) -> Result<Box<dyn Any>, String> {
        match self.get_depth(token, expr) {
            Ok(depth) => {
                let variable = self.get_at(depth, token.get_token_type().to_string())?;
                Ok(variable)
            },
            Err(err) => Err(err)
        }
        

    }

    fn get_at(&mut self, distance: usize, name: String) -> Result<Box<dyn Any>, String> {
        let mut env = self.environment.as_ref().borrow_mut();
        match env.get_at(distance, name.clone()) {
            Some(value) => Ok(value),
            None => {
                match env.get(name.clone()) {
                    Some(value) => Ok(value),
                    None => Err(format!("Variable '{}' is undefined.", name))
                }
            }
        }
    }

    fn downcast_to_token(&mut self, obj_1: Box<dyn Any>, obj_2: Box<dyn Any>) -> Result<(Token, Token), String> {
        match (obj_1.downcast::<Token>(), obj_2.downcast::<Token>()) {
            (Ok(d_obj_1), Ok(d_obj_2)) => Ok((*d_obj_1, *d_obj_2)),
            _ => Err("Could not downcast object to Token".to_string()),
        }
    }

    fn downcast_to_token_to_f64(&mut self, token1: Box<dyn Any>, token2: Box<dyn Any>) -> Result<(f64, f64), String> {
        let (tok1, tok2) = self.downcast_to_token(token1, token2)?;        
        match self.downcast_token_to_f64(tok1, tok2) {
            Ok((tok1_f64, tok2_f64)) => Ok((tok1_f64, tok2_f64)),
            Err(_) => Err("Could not downcast token1 and token2 to f64".to_string()),
        }
    }

    fn downcast_token_to_f64(&mut self, token1: Token, token2: Token) -> Result<(f64, f64), String> {
        match (token1.get_token_type(), token2.get_token_type()) {
            (TokenType::Number(tok1_f64), TokenType::Number(tok2_f64)) => Ok((tok1_f64, tok2_f64)),
            _ => Err("Could not downcast token1/token2 to f64".to_string()),
        }
    }
    
    fn downcast_token_to_string(&mut self, token1: Token, token2: Token) -> Result<(String, String), String> {
        match (token1.get_token_type(), token2.get_token_type()) {
            (TokenType::String(tok1_str), TokenType::String(tok2_str)) => Ok((tok1_str, tok2_str)),
            _ => Err("Could not downcast token1/token2 to String".to_string()),
        }
    }

    fn downcast_to_token_to_string(&mut self, token1: Box<dyn Any>, token2: Box<dyn Any>) -> Result<(String, String), String> {
        let (tok1, tok2) = self.downcast_to_token(token1, token2)?;
        match self.downcast_token_to_string(tok1, tok2) {
            Ok((tok1_str, tok2_str)) => Ok((tok1_str, tok2_str)),
            Err(_) => Err("Could not downcast token1 and token2 to String".to_string()),
        }
    }

    fn is_token_string(&mut self, token: &Token) -> bool {
        match token.get_token_type() {
            TokenType::String(_) => true,
            _ => false,
        }
    }

    fn is_token_number(&mut self, token: &Token) -> bool {
        match token.get_token_type() {
            TokenType::Number(_) => true,
            _ => false,
        }
    }

    fn substract(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
            match self.downcast_to_token_to_f64(operand1, operand2) {
                Ok((tok1_f64, tok2_f64)) => Ok(Box::new(Token::new(TokenType::Number(tok1_f64 - tok2_f64), "".to_string(), 0, 0, 0))),
                _ => Err("In order to substract two things they need to be numbers".to_string()),
            }
    }

    fn add(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        let (tok1, tok2) = self.downcast_to_token(operand1, operand2)?;
        if self.is_token_string(&tok1) && self.is_token_string(&tok2) {
            let (tok1_str, tok2_str) = self.downcast_to_token_to_string(Box::new(tok1), Box::new(tok2))?;
            return Ok(Box::new(Token::new(TokenType::String(tok1_str + &tok2_str), "".to_string(), 0, 0, 0)))
        } else if self.is_token_number(&tok1) && self.is_token_number(&tok2) {
            let (tok1_f64, tok2_f64) = self.downcast_to_token_to_f64(Box::new(tok1), Box::new(tok2))?;
            return Ok(Box::new(Token::new(TokenType::Number(tok1_f64 + tok2_f64), "".to_string(), 0, 0, 0)))
        } 
        Err("In order to add two things they need to be numbers or strings".to_string())     
    }

    fn multiply(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        match self.downcast_to_token_to_f64(operand1, operand2) {
            Ok((tok1_f64, tok2_f64)) => Ok(Box::new(Token::new(TokenType::Number(tok1_f64 * tok2_f64), "".to_string(), 0, 0, 0))),
            _ => Err("In order to multiply two things they need to be numbers".to_string()),
        }
    }

    fn divide(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        match self.downcast_to_token_to_f64(operand1, operand2) {
            Ok((tok1_f64, tok2_f64)) => Ok(Box::new(Token::new(TokenType::Number(tok1_f64 / tok2_f64), "".to_string(), 0, 0, 0))),
            _ => Err("In order to divide two things they need to be numbers".to_string()),
        }
    }

    fn greater(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        match self.downcast_to_token_to_f64(operand1, operand2) {
            Ok((tok1_f64, tok2_f64)) => {
                if tok1_f64 > tok2_f64 {
                    return Ok(Box::new(Token::new(TokenType::True, "".to_string(), 0, 0, 0)));
                }
                Ok(Box::new(Token::new(TokenType::False, "".to_string(), 0, 0, 0)))
            },
            _ => Err("In order to compare them, operands must be two numbers.".to_string()),
        }
    }

    fn greater_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        match self.downcast_to_token_to_f64(operand1, operand2) {
            Ok((tok1_f64, tok2_f64)) => {
                if tok1_f64 >= tok2_f64 {
                    return Ok(Box::new(Token::new(TokenType::True, "".to_string(), 0, 0, 0)));
                }
                Ok(Box::new(Token::new(TokenType::False, "".to_string(), 0, 0, 0)))
            },
            _ => Err("In order to compare them, operands must be two numbers.".to_string()),
        }
    }

    fn less(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        match self.downcast_to_token_to_f64(operand1, operand2) {
            Ok((tok1_f64, tok2_f64)) => {
                if tok1_f64 < tok2_f64 {
                    return Ok(Box::new(Token::new(TokenType::True, "".to_string(), 0, 0, 0)));
                }
                Ok(Box::new(Token::new(TokenType::False, "".to_string(), 0, 0, 0)))
            },
            _ => Err("In order to compare them, operands must be two numbers.".to_string()),
        }
    }

    fn less_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        match self.downcast_to_token_to_f64(operand1, operand2) {
            Ok((tok1_f64, tok2_f64)) => {
                if tok1_f64 <= tok2_f64 {
                    return Ok(Box::new(Token::new(TokenType::True, "".to_string(), 0, 0, 0)));
                }
                Ok(Box::new(Token::new(TokenType::False, "".to_string(), 0, 0, 0)))
            },
            _ => Err("In order to compare them, operands must be two numbers.".to_string()),
        }
    }

    fn equal_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        if operand1.type_id() != operand2.type_id() {
            return Err("Could not compare objects of different types".to_string());
        }
        let (tok1, tok2) = self.downcast_to_token(operand1, operand2)?;

        if self.is_token_string(&tok1) && self.is_token_string(&tok2) {
            let (tok1_str, tok2_str) = self.downcast_token_to_string(tok1, tok2)?;
            if tok1_str == tok2_str {
                return Ok(Box::new(Token::new(TokenType::True, "".to_string(), 0, 0, 0)));
            }
            return Ok(Box::new(Token::new(TokenType::False, "".to_string(), 0, 0, 0)));
        }

        if self.is_token_number(&tok1) && self.is_token_number(&tok2) {
            let (tok1_f64, tok2_f64) = self.downcast_token_to_f64(tok1, tok2)?;
            if tok1_f64 == tok2_f64 {
                return Ok(Box::new(Token::new(TokenType::True, "".to_string(), 0, 0, 0)));
            }
            return Ok(Box::new(Token::new(TokenType::False, "".to_string(), 0, 0, 0)));
        }
        Err("Could not compare(EqualEqual) objects of different types".to_string())
    }

    fn bang_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        if operand1.type_id() != operand2.type_id() {
            return Err("Could not compare objects of different types".to_string());
        }
        let (tok1, tok2) = self.downcast_to_token(operand1, operand2)?;

        if self.is_token_number(&tok1) && self.is_token_number(&tok2) {
            let (tok1_f64, tok2_f64) = self.downcast_token_to_f64(tok1, tok2)?;
            if tok1_f64 != tok2_f64 {
                return Ok(Box::new(Token::new(TokenType::True, "".to_string(), 0, 0, 0)));
            }
            return Ok(Box::new(Token::new(TokenType::False, "".to_string(), 0, 0, 0)));
        }

        if self.is_token_string(&tok1) && self.is_token_string(&tok2) {
            let (tok1_str, tok2_str) = self.downcast_token_to_string(tok1, tok2)?;
            if tok1_str != tok2_str {
                return Ok(Box::new(Token::new(TokenType::True, "".to_string(), 0, 0, 0)));
            }
            return Ok(Box::new(Token::new(TokenType::False, "".to_string(), 0, 0, 0)));
        }

        Err("Could not compare(BangEqual) objects of different types".to_string())
    }

    pub fn extract_return_value(ret_val: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        if let Some(token) = ret_val.downcast_ref::<Token>() {
            return Ok(Box::new(token.clone()));
        }

        if let Some(expr) = ret_val.downcast_ref::<Expr>() {
            return Ok(Box::new(expr.clone()));
        }

        if let Some(rlox_func) = ret_val.downcast_ref::<RLoxFunction>() {
            return Ok(Box::new(rlox_func.clone()));
        }

        if let Some(clock_fun) = ret_val.downcast_ref::<Clock>() {
            return Ok(Box::new(clock_fun.clone()));
        }

        Err("Could not extract return value".to_string())
    }

    pub fn execute(&mut self, stmt: Stmt) -> Result<Box<dyn Any>, String> {
        stmt.accept(self)
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<Vec<Box<dyn Any>>, String> {
        let mut vec = Vec::new();
        for stmt in statements {
            vec.push(self.execute(stmt)?);
        }
        Ok(vec)
    }

    pub fn execute_block(&mut self, stmts: &Vec<Stmt>, env: Rc<RefCell<Environment>>) -> Result<Box<dyn Any>, String> {
        self.environment.as_ref().borrow_mut().push_env(env);
        let mut block_return_value: Box<dyn Any> = Box::new(Token::new(TokenType::Nil, "".to_string(), 0, 0, 0));
        for stmt in stmts {
            match stmt {
                Stmt::ReturnStmt(_, _) => block_return_value = self.execute(stmt.clone())?,
                _ => { self.execute(stmt.clone())?; },
            }
        }
        self.environment.as_ref().borrow_mut().pop();
        Ok(block_return_value)
    }

 }

 impl Visitor<Result<Box<dyn Any>,  String>> for Interpreter {
    fn visit_literal_expr(&mut self, value: &Token) -> Result<Box<dyn Any>, String> {
        Ok(Box::new(value.clone()))
    }

   fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Box<dyn Any>, String> {
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
            _ => Err("The given operator is not a binary operator.".to_string())
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Box<dyn Any>, String> {
        self.evaluate(expr)
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Box<dyn Any>, String> {
        match operator.get_token_type() {
            TokenType::Minus => Ok(Box::new(right.clone())),
            TokenType::Bang => Ok(Box::new(self.is_truthy(Box::new(right.clone())))),
            _ => Err("The given operator is not a unary operator.".to_string())
        }
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Box<dyn Any>, String> {
        let expr = Expr::Variable(name.clone());
        return self.look_up_variable(name, expr);
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Box<dyn Any>, String> {
        let value_evaluated = self.evaluate(value)?;
        let distance = self.get_depth(name, value.clone());

        match distance {
            Ok(depth) => {
                let mut env = self.environment.as_ref().borrow_mut();
                env.assign_at(depth, name.get_token_type().to_string(), value_evaluated.into())?;
            },
            Err(_) => {
                let mut env = self.environment.as_ref().borrow_mut();
                env.assign(name.get_token_type().to_string(), value_evaluated.into())?;
            }
        }


        return self.visit_variable_expr(name);
    }

    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Box<dyn Any>,  String> {
        let left_val = self.evaluate(left)?;
        let is_truthy = self.is_truthy(left_val)?;
        if let Some(truth) = is_truthy.downcast_ref::<Token>() {
            if truth.get_token_type() == TokenType::True {
                if operator.get_token_type() == TokenType::Or {
                    return Ok(Box::new(truth.clone()));
                }
                return self.evaluate(right);
            }
            else if truth.get_token_type() == TokenType::False {
                if operator.get_token_type() == TokenType::Or {
                    return self.evaluate(right);
                }
                return Ok(Box::new(truth.clone()));
            }
        }
        return Err("Could not visit Logical Expression, truthy might be a reason.".to_string());
    }

    fn visit_call_expr(&mut self, callee: &Expr, _: &Token, arguments: &Vec<Expr>) -> Result<Box<dyn Any>,  String> {
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

 impl StmtVisitor<Result<Box<dyn Any>, String>> for Interpreter {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<Box<dyn Any>, String> {
        self.evaluate(expr)
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<Box<dyn Any>, String> {
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

    fn visit_return_stmt(&mut self, _keyword: &Token, expr: &Expr) -> Result<Box<dyn Any>, String> {
        let value = self.evaluate(expr)?;
        
        if let Some(token) = value.downcast_ref::<Token>() {
            self.return_value = Some(Box::new(token.clone()));
            return Err(format!("Returning {} at this level is not available", token.get_token_type()));
        }

        if let Some(expr) = value.downcast_ref::<Expr>() {
            self.return_value = Some(Box::new(expr.clone()));
            return Err(format!("Returning {} at this level is not available", expr));
        }

        if let Some(stmt) = value.downcast_ref::<Stmt>() {
            self.return_value = Some(Box::new(stmt.clone()));
            return Err(format!("Returning {} at this level is not available", stmt));
        }

        if let Some(rlox_func) = value.downcast_ref::<RLoxFunction>() {
            self.return_value = Some(Box::new(rlox_func.clone()));
            return Err(format!("Returning {} at this level is not available", "rlox_func"));
        }

        if let Some(clock_fun) = value.downcast_ref::<Clock>() {
            self.return_value = Some(Box::new(clock_fun.clone()));
            return Err(format!("Returning {} at this level is not available", "clock_fun"));
        }

        Err("Could not return value.".to_string())
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<Box<dyn Any>, String> {
        let value = self.evaluate(initializer)?;
        
        self.environment.as_ref().borrow_mut().define(name.get_token_type().to_string(), value.into());
        Ok(Box::new(name.clone()))
    }

    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<Box<dyn Any>, String> {
        let env = Rc::new(RefCell::new(Environment::new()));
        self.execute_block(stmts, env)
    }

    fn visit_class_stmt(&mut self, name: &Token, _: &Vec<Stmt>) -> Result<Box<dyn Any>, String> {
        let klass: RLoxClass = RLoxClass::new(name.get_token_type().to_string().clone());
        self.environment.as_ref().borrow_mut().define(name.get_token_type().to_string(), Box::new(klass));
        Ok(Box::new(name.clone()))
    }

    fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> Result<Box<dyn Any>, String> {
        let func: RLoxFunction = RLoxFunction::new(Stmt::Function(name.clone(), params.clone(), body.clone()), self.environment.as_ref().borrow_mut().peek().unwrap());
        self.environment.as_ref().borrow_mut().define(name.get_token_type().to_string(), Box::new(func));
        Ok(Box::new(name.clone()))
    }

    fn visit_if_stmt(&mut self, expr: &Expr, stmt: &Stmt, else_stmt: &Option<Box<Stmt>>) -> Result<Box<dyn Any>, String> {
        let value = self.evaluate(expr)?;
        let is_truthy = self.is_truthy(value)?;
        if let Some(truth) = is_truthy.downcast_ref::<Token>() {
            if truth.get_token_type() == TokenType::True {
                return self.execute(stmt.clone());
            }
            else if truth.get_token_type() == TokenType::False {
                if let Some(else_stmt) = else_stmt {
                    return self.execute(*else_stmt.clone());
                }
                return Ok(Box::new(Token::new(TokenType::Nil, "".to_string(), 0, 0, 0)));
            }
        }
        return Err("Could not visit IF statement, truthy might be a reason.".to_string());
    }

    fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> Result<Box<dyn Any>, String> {
        let value = self.evaluate(expr)?;
        let is_truthy = self.is_truthy(value)?;
        if let Some(truth) = is_truthy.downcast_ref::<Token>() {
            if truth.get_token_type() == TokenType::True {
                self.execute(stmt.clone())?;
                return self.visit_while_stmt(expr, stmt);
            }
            else if truth.get_token_type() == TokenType::False {
                return Ok(Box::new(Token::new(TokenType::Nil, "".to_string(), 0, 0, 0)));
            }
        }
        return Err("Could not visit WHILE statement, truthy might be a reason.".to_string());
    }
    
 }

}
