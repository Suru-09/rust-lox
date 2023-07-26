pub mod interpreter {

    use crate::expr::expr::{Expr, Visitor};
    use crate::scanner::scan::{Token, TokenType};
    use std::any::Any;


 pub struct Interpreter;

 impl Interpreter {
    pub fn evaluate(&mut self, expr: &Expr) -> Box<dyn Any> {
        expr.accept(self)
    }

    pub fn is_truthy(&mut self, obj: Box<dyn Any>) -> bool {
        if let Ok(obj_downcasted) = obj.downcast::<bool>() {
            *obj_downcasted
        }
        else {
            false
        }
    }
 }

 impl Visitor<Box<dyn Any>> for Interpreter {
    fn visit_literal_expr(&mut self, value: &Token) -> Box<dyn Any> {
        Box::new(value.clone())
    }

    fn visit_binary_expr(&mut self, _left: &Expr, operator: &Token, _right: &Expr) -> Box<dyn Any> {
        Box::new(operator.clone())
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Box<dyn Any> {
        self.evaluate(expr)
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Box<dyn Any> {
        match operator.get_token_type() {
            TokenType::Minus => Box::new(right.clone()),
            TokenType::Bang => Box::new(!self.is_truthy(Box::new(right.clone()))),
            _ => panic!("Unary Interpreter not finding a minus")
        }
    }
 }
 

}