pub mod expr {

    use crate::scanner::scan::Token;
    use crate::stmt::stmt::LiteralValue;
    use std::fmt;

    pub trait Visitable {
        fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Expr {
        Binary(Box<Expr>, Token, Box<Expr>),
        Call(
            Box<Expr>, /*callee*/
            Token,     /*parent*/
            Vec<Expr>, /*arguments*/
        ),
        Logical(Box<Expr>, Token, Box<Expr>),
        Grouping(Box<Expr>),
        Literal(LiteralValue),
        Unary(Token, Box<Expr>),
        Variable(Token),
        Assign(Token, Box<Expr>),
        Get(Box<Expr> /*obj*/, Token /*name*/),
        Set(
            Box<Expr>, /*obj*/
            Token,     /*name*/
            Box<Expr>, /*value*/
        ),
        This(Token /*keyword*/),
        Super(Token /*keyword*/, Token /*method*/),
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
                Expr::Logical(_, _, _) => "Logical".to_string(),
                Expr::Call(_, _, _) => "Call".to_string(),
                Expr::Get(_, _) => "Get".to_string(),
                Expr::Set(_, _, _) => "Set".to_string(),
                Expr::This(_) => "This".to_string(),
                Expr::Super(_, _) => "super".to_string(),
            }
        }
    }

    impl fmt::Display for Expr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Expr::Binary(left_expr, operand, right_expr) => write!(
                    f,
                    "({} {} {})",
                    left_expr,
                    operand.token_type_value(),
                    right_expr
                ),
                Expr::Grouping(expression) => write!(f, "(group {})", expression),
                Expr::Literal(value) => write!(f, "{:?}", value),
                Expr::Unary(operand, right_expr) => {
                    write!(f, "({} {})", operand.token_type_value(), right_expr)
                }
                Expr::Variable(token) => write!(f, "{}", token.token_type_value()),
                Expr::Assign(token, expr) => write!(f, "{} = {}", token.token_type_value(), expr),
                Expr::Logical(left, operator, right) => {
                    write!(f, "({} {} {})", left, operator.token_type_value(), right)
                }
                Expr::Call(calle, _, args) => {
                    let mut args_str = String::new();
                    for arg in args {
                        args_str.push_str(&format!("{}, ", arg));
                    }
                    write!(f, "{}({})", calle, args_str)
                }
                Expr::Get(obj, name) => {
                    write!(f, "{}.{}", obj, name.get_token_type())
                }
                Expr::Set(obj, name, value) => {
                    write!(f, "{}.{} = {}", obj, name.get_token_type(), value)
                }
                Expr::This(keyword) => {
                    write!(f, "{}", keyword.get_token_type())
                }
                Expr::Super(keyword, method) => {
                    write!(
                        f,
                        "{} {}",
                        keyword.get_token_type(),
                        method.get_token_type()
                    )
                }
            }
        }
    }

    impl Visitable for Expr {
        fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
            match self {
                Expr::Binary(left, operator, right) => {
                    visitor.visit_binary_expr(left, operator, right)
                }
                Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
                Expr::Literal(value) => visitor.visit_literal_expr(value),
                Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
                Expr::Variable(token) => visitor.visit_variable_expr(token),
                Expr::Assign(token, expr) => visitor.visit_assign_expr(token, expr),
                Expr::Logical(left, operator, right) => {
                    visitor.visit_logical_expr(left, operator, right)
                }
                Expr::Call(callee, paren, arguments) => {
                    visitor.visit_call_expr(callee, paren, arguments)
                }
                Expr::Get(obj, name) => visitor.visit_get_expr(obj, name),
                Expr::Set(obj, name, value) => visitor.visit_set_expr(obj, name, value),
                Expr::This(keyword) => visitor.visit_this_expr(keyword),
                Expr::Super(keyword, method) => visitor.visit_super_expr(keyword, method),
            }
        }
    }

    pub trait Visitor<T> {
        fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
        fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
        fn visit_literal_expr(&mut self, value: &LiteralValue) -> T;
        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
        fn visit_variable_expr(&mut self, token: &Token) -> T;
        fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> T;
        fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
        fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> T;
        fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> T;
        fn visit_set_expr(&mut self, object: &Expr, name: &Token, value: &Expr) -> T;
        fn visit_this_expr(&mut self, keyword: &Token) -> T;
        fn visit_super_expr(&mut self, keyword: &Token, method: &Token) -> T;
    }

    impl Expr {
        pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
            match self {
                Expr::Binary(left, operator, right) => {
                    visitor.visit_binary_expr(left, operator, right)
                }
                Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
                Expr::Literal(value) => visitor.visit_literal_expr(value),
                Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
                Expr::Variable(token) => visitor.visit_variable_expr(token),
                Expr::Assign(token, expr) => visitor.visit_assign_expr(token, expr),
                Expr::Logical(left, operator, right) => {
                    visitor.visit_logical_expr(left, operator, right)
                }
                Expr::Call(callee, paren, arguments) => {
                    visitor.visit_call_expr(callee, paren, arguments)
                }
                Expr::Get(object, name) => visitor.visit_get_expr(object, name),
                Expr::Set(object, name, value) => visitor.visit_set_expr(object, name, value),
                Expr::This(keyword) => visitor.visit_this_expr(keyword),
                Expr::Super(keyword, method) => visitor.visit_super_expr(keyword, method),
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

        fn visit_literal_expr(&mut self, value: &LiteralValue) -> String {
            format!("{:?}", value)
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

        fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
            let left = left.accept(self);
            let right = right.accept(self);
            format!("({} {} {})", operator.token_type_value(), left, right)
        }

        fn visit_call_expr(&mut self, callee: &Expr, _: &Token, arguments: &Vec<Expr>) -> String {
            let mut args_str = String::new();
            for arg in arguments {
                args_str.push_str(&format!("{}, ", arg.accept(self)));
            }
            format!("{}({})", callee.accept(self), args_str)
        }

        fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> String {
            format!("{}.{}", object, name.get_token_type())
        }

        fn visit_set_expr(&mut self, object: &Expr, name: &Token, value: &Expr) -> String {
            format!("{}.{} = {}", object, name.get_token_type(), value)
        }

        fn visit_this_expr(&mut self, keyword: &Token) -> String {
            format!("{}", keyword.get_token_type())
        }

        fn visit_super_expr(&mut self, keyword: &Token, method: &Token) -> String {
            format!("{} {}", keyword.get_token_type(), method.get_token_type())
        }
    }
}
