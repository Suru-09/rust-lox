pub mod expr {

    use crate::scanner::scan::{Token, TokenType};
    use graphviz_rust::dot_generator::*;
    use graphviz_rust::dot_structures::*;
    use graphviz_rust::{
        attributes::*,
        cmd::{CommandArg, Format},
        exec, parse,
        printer::{DotPrinter, PrinterContext},
    };

    pub trait Visitable {
        fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T;
    }

    pub enum Expr {
        Binary(Box<Expr>, Token, Box<Expr>),
        Grouping(Box<Expr>),
        Literal(Token),
        Unary(Token, Box<Expr>),
    }

    impl  Visitable for Expr {
        fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
            match self {
                Expr::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
                Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
                Expr::Literal(value) => visitor.visit_literal_expr(value),
                Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
            }
        }
    }

    pub trait Visitor<T> {
        fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> T;
        fn visit_grouping_expr(&self, expression: &Expr) -> T;
        fn visit_literal_expr(&self, value: &Token) -> T;
        fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> T;
    }

    impl Expr {
        pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
            match self {
                Expr::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
                Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
                Expr::Literal(value) => visitor.visit_literal_expr(value),
                Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
            }
        }
    }

    pub struct AstBuilder;

    impl Visitor<Expr> for AstBuilder {
        fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Expr {
            Expr::Binary(
                Box::new(left.accept(self)),
                operator.clone(),
                Box::new(right.accept(self)),
            )
        }

        fn visit_grouping_expr(&self, expression: &Expr) -> Expr {
            Expr::Grouping(Box::new(expression.accept(self)))
        }

        fn visit_literal_expr(&self, value: &Token) -> Expr {
            Expr::Literal(value.clone())
        }

        fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Expr {
            Expr::Unary(operator.clone(), Box::new(right.accept(self)))
        }
    }

    pub fn build_test_ast() -> Expr {
        let expression = Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, String::from("-"), 1, 1, 0),
                Box::new(Expr::Literal(Token::new(
                    TokenType::Number(123.0),
                    String::from("123"),
                    123,
                    1,
                    2
                ))),
            )),
            Token::new(TokenType::Star, String::from("*"), 1, 1, 0),
            Box::new(Expr::Grouping(Box::new(Expr::Literal(Token::new(
                TokenType::Number(45.67),
                String::from("45.67"),
                45,
                1,
                2
            ))))),
        );
        expression
    }

    pub struct GraphVizPrinter;

    impl Visitor<String> for GraphVizPrinter {
        fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
            let string = String::from("Binary");
            string
        }

        fn visit_grouping_expr(&self, expression: &Expr) -> String {
            let string = String::from("Grouping");
            string
        }

        fn visit_literal_expr(&self, value: &Token) -> String {
            let string = String::from("Literal");
            string
        }

        fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
            let string = String::from("Unary exp");
            string
        }
    }


}