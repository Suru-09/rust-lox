pub mod expr {

    use crate::scanner::scan::Token;
    use std::fmt;

    pub trait Visitable {
        fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T;
    }

    #[derive(Clone)]
    pub enum Expr {
        Binary(Box<Expr>, Token, Box<Expr>),
        Grouping(Box<Expr>),
        Literal(Token),
        Unary(Token, Box<Expr>),
        Variable(Token),
        Assign(Token, Box<Expr>),
    }

    impl Expr {
        pub fn name(&self) -> String {
            match self {
                Expr::Binary(_, _, _) => "Binary".to_string(),
                Expr::Grouping(_) => "Grouping".to_string(),
                Expr::Literal(_) => "Literal".to_string(),
                Expr::Unary(_, _) => "Unary".to_string(),
                Expr::Variable(_) => "Variable".to_string(),
                Expr::Assign(_, _) => "Assign".to_string(),
            }
        }
    }

    impl fmt::Display for Expr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Expr::Binary(left_expr, operand, right_expr) => write!(f, "({} {} {})", left_expr, operand.token_type_value() , right_expr),
                Expr::Grouping(expression) => write!(f, "(group {})", expression),
                Expr::Literal(value) => write!(f, "{}", value.token_type_value()),
                Expr::Unary(operand, right_expr) => write!(f, "({} {})", operand.token_type_value(), right_expr),
                Expr::Variable(token) => write!(f, "{}", token.token_type_value()),
                Expr::Assign(token, expr) => write!(f, "{} = {}", token.token_type_value(), expr),
            }
        }
    }

    impl  Visitable for Expr {
        fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
            match self {
                Expr::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
                Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
                Expr::Literal(value) => visitor.visit_literal_expr(value),
                Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
                Expr::Variable(token) => visitor.visit_variable_expr(token),
                Expr::Assign(token, expr) => visitor.visit_assign_expr(token, expr),
            }
        }
    }

    pub trait Visitor<T> {
        fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
        fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
        fn visit_literal_expr(&mut self, value: &Token) -> T;
        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
        fn visit_variable_expr(&mut self, token: &Token) -> T;
        fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> T;
    }

    impl Expr {
        pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
            match self {
                Expr::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
                Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
                Expr::Literal(value) => visitor.visit_literal_expr(value),
                Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
                Expr::Variable(token) => visitor.visit_variable_expr(token),
                Expr::Assign(token, expr) => visitor.visit_assign_expr(token, expr),
            }
        }
    }

    pub struct AstPrinter;
    impl Visitor<String> for AstPrinter {

        fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
            let left = left.accept(self);
            let right = right.accept(self);
            format!("({} {} {})", operator.token_type_value(), left, right)
        }

        fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
            format!("(group {})", expression.accept(self))
        }

        fn visit_literal_expr(&mut self, value: &Token) -> String {
            format!("{}", value.token_type_value())
        }

        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String {
            format!("({} {})", operator.token_type_value(), right.accept(self))
        }

        fn visit_variable_expr(&mut self, token: &Token) -> String {
            format!("{}", token.token_type_value())
        }

        fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> String {
            format!("{} = {}", token.token_type_value(), expr.accept(self))
        }
    }
}
