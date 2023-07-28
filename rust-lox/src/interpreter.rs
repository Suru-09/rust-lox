pub mod interpreter {

    use crate::expr::expr::{Expr, Visitor};
    use crate::scanner::scan::{Token, TokenType};
    use std::any::Any;

 pub struct Interpreter;

 impl Interpreter {
    fn evaluate(&mut self, expr: &Expr) -> Result<Box<dyn Any>,  &'static str> {
        expr.accept(self)
    }

    fn is_truthy(&mut self, obj: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        match obj.downcast::<bool>() {
            Ok(d_obj) => Ok(Box::new(*d_obj)),
            Err(_) => Err("Could not downcast object to bool"),
        }
    }

    fn substract(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        if let Ok(op1_d) = operand1.downcast::<f64>() {
            if let Ok(op2_d) = operand2.downcast::<f64>() {
               return Ok(Box::new(*op1_d - *op2_d))
            }
        }
        Err("Substract not available for the given operands")
    }

    pub fn interpret(&mut self, expr: &Expr) -> Result<Box<dyn Any>, &'static str> {
        self.evaluate(expr)
    }
 }

 impl Visitor<Result<Box<dyn Any>,  &'static str>> for Interpreter {
    fn visit_literal_expr(&mut self, value: &Token) -> Result<Box<dyn Any>,  &'static str> {
        Ok(Box::new(value.clone()))
    }

   fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Box<dyn Any>,  &'static str> {
        let left = self.evaluate(left);
        let right = self.evaluate(right);

        match left {
            Ok(l) => {
                match right {
                    Ok(r) => {
                        match operator.get_token_type() {
                            TokenType::Minus => self.substract(l, r),
                            _ => Err("Wrong operator for binary expression")
                        }
                    },
                    Err(r) => Err(r)    // propagate error further
                }
            },
            Err(l) => Err(l),   // propagate error further
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Box<dyn Any>, &'static str> {
        self.evaluate(expr)
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Box<dyn Any>, &'static str> {
        match operator.get_token_type() {
            TokenType::Minus => Ok(Box::new(right.clone())),
            TokenType::Bang => Ok(Box::new(self.is_truthy(Box::new(right.clone())))),
            _ => panic!("Unary Interpreter not finding a minus")
        }
    }
 }

}
