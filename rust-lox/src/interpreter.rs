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
    fn evaluate(&mut self, expr: &Expr) -> Result<Box<dyn Any>,  &'static str> {
        expr.accept(self)
    }

    fn is_truthy(&mut self, obj: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        match obj.downcast::<bool>() {
            Ok(d_obj) => Ok(Box::new(*d_obj)),
            Err(_) => Err("Could not downcast object to bool"),
        }
    }

    fn downcast_to_f64(&mut self, obj_1: Box<dyn Any>, obj_2: Box<dyn Any>) -> Result<(f64, f64), &'static str> {
        println!("Type of: {:?} is not f64", obj_1.type_id());
        match obj_1.downcast::<f64>() {
            Ok(d_obj_1) => {
                println!("Type of: {:?} is not f64", obj_2.type_id());
                match obj_2.downcast::<f64>() {
                    Ok(d_obj_2) => Ok((*d_obj_1, *d_obj_2)),
                    Err(_) => {
                        println!("Type of: {:?} is not f64", d_obj_1.type_id());
                        Err("Could not downcast object to f64")
                    },
                }
            },
            Err(_) => {
                println!("Type of: {:?} is not f64", obj_2.type_id());
                Err("Could not downcast object to f64")
            },
        }
    }

    fn substract(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        let (op1, op2) = self.downcast_to_f64(operand1, operand2)?;
        Ok(Box::new(op1 - op2))
    }

    fn add(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        // if let Some(op1) = operand1.downcast_ref::<f64>() {
        //     if let Some(op2) = operand2.downcast_ref::<f64>() {
        //         return Ok(Box::new(*op1 + *op2));
        //     }
        // }
    
        // if let Some(op1) = operand1.downcast_ref::<String>() {
        //     if let Some(op2) = operand2.downcast_ref::<String>() {
        //         return Ok(Box::new(op1.clone() + op2));
        //     }
        // }
        let (op1, op2) = self.downcast_to_f64(operand1, operand2)?;
        Ok(Box::new(op1 + op2))

        //Err("In order to add them, operands must be two numbers or two strings.")
    }

    fn multiply(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        let (op1, op2) = self.downcast_to_f64(operand1, operand2)?;
        Ok(Box::new(op1 * op2))
    }

    fn divide(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        let (op1, op2) = self.downcast_to_f64(operand1, operand2)?;
        Ok(Box::new(op1 / op2))
    }

    fn greater(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        let (op1, op2) = self.downcast_to_f64(operand1, operand2)?;
        Ok(Box::new(op1 > op2))
    }

    fn greater_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        let (op1, op2) = self.downcast_to_f64(operand1, operand2)?;
        Ok(Box::new(op1 >= op2))
    }

    fn less(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        let (op1, op2) = self.downcast_to_f64(operand1, operand2)?;
        Ok(Box::new(op1 < op2))
    }

    fn less_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        let (op1, op2) = self.downcast_to_f64(operand1, operand2)?;
        Ok(Box::new(op1 <= op2))
    }

    fn equal_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        if operand1.type_id() != operand2.type_id() {
            return Ok(Box::new(false));
        }
        Err("Could not compare objects of different types")
    }

    fn bang_equal(&mut self, operand1: Box<dyn Any>, operand2: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
        if operand1.type_id() != operand2.type_id() {
            return Ok(Box::new(true));
        }
        Err("Could not compare objects of different types")
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
            _ => Err("The given operator is not a binary operator.")
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Box<dyn Any>, &'static str> {
        self.evaluate(expr)
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Box<dyn Any>, &'static str> {
        match operator.get_token_type() {
            TokenType::Minus => Ok(Box::new(right.clone())),
            TokenType::Bang => Ok(Box::new(self.is_truthy(Box::new(right.clone())))),
            _ => Err("The given operator is not a unary operator.")
        }
    }
 }

}
