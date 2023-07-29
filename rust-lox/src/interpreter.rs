pub mod interpreter {

    use crate::expr::expr::{Expr, Visitor};
    use crate::scanner::scan::{Token, TokenType};
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

 pub struct Interpreter;

 impl Interpreter {
    fn evaluate(&mut self, expr: &Expr) -> Result<Box<dyn Any>, String> {
        expr.accept(self)
    }

    // TODO: This does not work due to the fact that I should first downcast to token
    // TODO: and after to bool.
    fn is_truthy(&mut self, obj: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        match obj.downcast::<bool>() {
            Ok(d_obj) => Ok(Box::new(*d_obj)),
            Err(_) => Err("Could not downcast object to bool".to_string()),
        }
    }

    fn downcast_to_token(&mut self, obj_1: Box<dyn Any>, obj_2: Box<dyn Any>) -> Result<(Token, Token), String> {
        match obj_1.downcast::<Token>() {
            Ok(d_obj_1) => {
                match obj_2.downcast::<Token>() {
                    Ok(d_obj_2) => Ok((*d_obj_1, *d_obj_2)),
                    Err(_) => Err("Could not downcast object to Token".to_string()),
                }
            },
            Err(_) => Err("Could not downcast object to Token".to_string()),
        }
    }

    fn downcast_token_to_f64(&mut self, token1: &Token, token2: &Token) -> Result<(f64, f64), String> {
        match token1.get_token_type() {
            TokenType::Number(tok1_f64) => {
                match token2.get_token_type() {
                    TokenType::Number(tok2_f64) => Ok((tok1_f64, tok2_f64)),
                    _ => Err("Could not downcast token2 to f64".to_string()),
                }
            },
            _ => Err("Could not downcast token1 to f64".to_string()),
        }
    }

    fn downcast_token_to_string(&mut self, token1: &Token, token2: &Token) -> Result<(String, String), String> {
        match token1.get_token_type() {
            TokenType::String(tok1_str) => {
                match token2.get_token_type() {
                    TokenType::String(tok2_str) => Ok((tok1_str, tok2_str)),
                    _ => Err("Could not downcast token2 to String".to_string()),
                }
            },
            _ => Err("Could not downcast token1 to String".to_string()),
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
        let (tok1, tok2) = self.downcast_to_token(operand1, operand2)?;
        if self.is_token_number(&tok1) && self.is_token_number(&tok2) {
            let (tok1_f64, tok2_f64) = self.downcast_token_to_f64(&tok1, &tok2)?;
            return Ok(Box::new(tok1_f64 - tok2_f64));
        }
        Err("In order to substract them, operands must be two numbers.".to_string())
    }

    fn add(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        let (tok1, tok2) = self.downcast_to_token(operand1, operand2)?;

        if self.is_token_number(&tok1) && self.is_token_number(&tok2) {
            let (tok1_f64, tok2_f64) = self.downcast_token_to_f64(&tok1, &tok2)?;
            return Ok(Box::new(tok1_f64 + tok2_f64));
        }

        if self.is_token_string(&tok1) && self.is_token_string(&tok2) {
            let (tok1_str, tok2_str) = self.downcast_token_to_string(&tok1, &tok2)?;
            return Ok(Box::new(tok1_str + &tok2_str));
        }
        
        Err("In order to add them, operands must be two numbers or two strings.".to_string())
    }

    fn multiply(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        let (tok1, tok2) = self.downcast_to_token(operand1, operand2)?;
        if self.is_token_number(&tok1) && self.is_token_number(&tok2) {
            let (tok1_f64, tok2_f64) = self.downcast_token_to_f64(&tok1, &tok2)?;
            return Ok(Box::new(tok1_f64 * tok2_f64));
        }
        Err("In order to multiply them, operands must be two numbers.".to_string())
    }

    fn divide(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        let (tok1, tok2) = self.downcast_to_token(operand1, operand2)?;
        if self.is_token_number(&tok1) && self.is_token_number(&tok2) {
            let (tok1_f64, tok2_f64) = self.downcast_token_to_f64(&tok1, &tok2)?;
            return Ok(Box::new(tok1_f64 / tok2_f64));
        }
        Err("In order to divide them, operands must be two numbers.".to_string())
    }

    fn greater(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        let (tok1, tok2) = self.downcast_to_token(operand1, operand2)?;

        if self.is_token_number(&tok1) && self.is_token_number(&tok2) {
            let (tok1_f64, tok2_f64) = self.downcast_token_to_f64(&tok1, &tok2)?;
            return Ok(Box::new(tok1_f64 > tok2_f64));
        }
        Err("In order to compare them, operands must be two numbers.".to_string())
    }

    fn greater_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        let (tok1, tok2) = self.downcast_to_token(operand1, operand2)?;
        if self.is_token_number(&tok1) && self.is_token_number(&tok2) {
            let (tok1_f64, tok2_f64) = self.downcast_token_to_f64(&tok1, &tok2)?;
            return Ok(Box::new(tok1_f64 >= tok2_f64));
        }
        Err("In order to compare them, operands must be two numbers.".to_string())
    }

    fn less(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        let (tok1, tok2) = self.downcast_to_token(operand1, operand2)?;
        if self.is_token_number(&tok1) && self.is_token_number(&tok2) {
            let (tok1_f64, tok2_f64) = self.downcast_token_to_f64(&tok1, &tok2)?;
            return Ok(Box::new(tok1_f64 < tok2_f64));
        }
        Err("In order to compare them, operands must be two numbers.".to_string())
    }

    fn less_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        let (tok1, tok2) = self.downcast_to_token(operand1, operand2)?;
        if self.is_token_number(&tok1) && self.is_token_number(&tok2) {
            let (tok1_f64, tok2_f64) = self.downcast_token_to_f64(&tok1, &tok2)?;
            return Ok(Box::new(tok1_f64 <= tok2_f64));
        }
        Err("In order to compare them, operands must be two numbers.".to_string())
    }

    fn equal_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        if operand1.type_id() == operand2.type_id() {
            return Ok(Box::new(true));
        }
        Err("Could not compare objects of different types".to_string())
    }

    fn bang_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, String> {
        if operand1.type_id() != operand2.type_id() {
            return Ok(Box::new(true));
        }
        Err("Could not compare objects of different types".to_string())
    }

    pub fn interpret(&mut self, expr: &Expr) -> Result<Box<dyn Any>, String> {
        self.evaluate(expr)
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
 }

}
