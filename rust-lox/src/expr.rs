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
        fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T;
    }

    pub enum Expr {
        Binary(Box<Expr>, Token, Box<Expr>),
        Grouping(Box<Expr>),
        Literal(Token),
        Unary(Token, Box<Expr>),
    }

    impl  Visitable for Expr {
        fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
            match self {
                Expr::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
                Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
                Expr::Literal(value) => visitor.visit_literal_expr(value),
                Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
            }
        }
    }

    pub trait Visitor<T> {
        fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
        fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
        fn visit_literal_expr(&mut self, value: &Token) -> T;
        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
    }

    impl Expr {
        pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
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
    
        fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Expr {
            Expr::Binary(
                Box::new(left.accept(self)),
                operator.clone(),
                Box::new(right.accept(self)),
            )
        }

        fn visit_grouping_expr(&mut self, expression: &Expr) -> Expr {
            Expr::Grouping(Box::new(expression.accept(self)))
        }

        fn visit_literal_expr(&mut self, value: &Token) -> Expr {
            Expr::Literal(value.clone())
        }

        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Expr {
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

    pub struct GraphVizPrinter  {
        graph: String,
        node_count: u64,
    }

    impl GraphVizPrinter {
        pub fn new(graph_name: String) -> GraphVizPrinter {
            GraphVizPrinter { 
                graph: format!("digraph {} {{\n", graph_name),
                node_count: 0
            }
        }

        pub fn close_graph(&mut self) {
            self.graph.push_str("\n}")
        }

        pub fn increase_node_count(&mut self) {
            self.node_count += 1;
        }

        pub fn add_node(&mut self, label: String, related_nodes: Vec<String>) {
            self.increase_node_count();
            self.graph.push_str(format!("\tnode_{} [label=\"{}\"];\n", self.node_count, label).as_str());
            let mut count: u64 = 0;
            for node in related_nodes {
                count += 1;
                self.add_edge(self.node_count, self.node_count - count);
            }
        }

        pub fn add_edge(&mut self, node1: u64, node2: u64) {
            self.graph.push_str(format!("\tnode_{} -> node_{};\n", node1, node2).as_str());
        }

        pub fn to_string(&self) -> String {
            self.graph.clone()
        }
    }

    impl Visitor<String> for GraphVizPrinter {
        fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
            let left_graph = left.accept(self);
            let right_graph = right.accept(self);
            self.add_node(operator.token_type_value(), vec![left_graph.clone(), right_graph.clone()]);
            left_graph + operator.token_type_value().as_str() + right_graph.as_str()
        }

        fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
            let expr = expression.accept(self);
            self.add_node(format!("({})", expr), vec![expr.clone()]);
            format!("({})", expr)
        }

        fn visit_literal_expr(&mut self, value: &Token) -> String {
            let literal = value.token_type_value();
            self.add_node(literal.clone(), vec![]);
            literal
        }

        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String {
            let right_graph = right.accept(self);
            self.add_node(operator.token_type_value(), vec![right_graph.clone()]);
            operator.token_type_value() + right_graph.as_str()
        }
    }


}