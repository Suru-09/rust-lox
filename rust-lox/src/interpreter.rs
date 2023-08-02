pub mod interpreter {

    use crate::expr::expr::{Expr, Visitor};
    use crate::stmt::stmt::{Stmt, StmtVisitor};
    use crate::scanner::scan::{Token, TokenType};
    use crate::environment::environment::Environment;
    use std::any::Any;

    /**
     * ! Notes to my self:
     * ! No. 1:
     * * The final result of the interpretor visitor is a Literal, therefore If I try to downcast the result of the
     * * f64 or String for example, I will fail miserably, I would need to downcast the result to a Literal and then
     * * check the type of the literal.
     * ! No. 2:
     * * Could there be improvements in error handling? Everything seems too verbose.
     */

 pub struct Interpreter {
    pub environment: Environment,
 }

 impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Box<dyn Any>, String> {
        expr.accept(self)
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
        let value = self.environment.get(name.get_token_type().to_string());
        match value {
            Some(value) => {
                // try to downcast the value to a Token/Stmt/Expr and return it.
                let val_downcast = value.downcast_ref::<Token>();
                match val_downcast {
                    Some(token) => return Ok(Box::new(token.clone())),
                    None => {
                        let val_downcast = value.downcast_ref::<Stmt>();
                        match val_downcast {
                            Some(stmt) => return Ok(Box::new(stmt.clone())),
                            None => {
                                let val_downcast = value.downcast_ref::<Expr>();
                                match val_downcast {
                                    Some(expr) => return Ok(Box::new(expr.clone())),
                                    None => return Err("Could not downcast value".to_string())
                                }
                            }
                        }
                    }
                }
            },
            None => Err(format!("Variable {} is not defined", name.get_token_type()))
        }
    }
 }

 impl StmtVisitor<Result<Box<dyn Any>, String>> for Interpreter {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<Box<dyn Any>, String> {
        self.evaluate(expr)
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<Box<dyn Any>, String> {
        let value = self.evaluate(expr)?;
        
       let val_downcast = value.downcast_ref::<Token>();
       // match val_downcast without borrowing it and downcast to Token, Stmt and Expr
         match val_downcast {
              Some(token) => {
                println!("{}", token.get_token_type());
                return Ok(Box::new(token.clone()));
            },
            None => {
                let val_downcast = value.downcast_ref::<Stmt>();
                match val_downcast {
                    Some(stmt) => {
                        println!("{}", stmt);
                        //return Ok(stmt);
                    },
                    None => {
                        let val_downcast = value.downcast_ref::<Expr>();
                        match val_downcast {
                            Some(expr) =>{ 
                                println!("{}", expr);
                            },
                            None => return Err("Could not print value".to_string())
                        }
                    }
                }
            }
        }
        Err("Could not print value".to_string())
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<Box<dyn Any>, String> {
        let value = self.evaluate(initializer)?;
        
        self.environment.define(name.get_token_type().to_string(), value);
        Ok(Box::new(name.clone()))
    }
 }

}
