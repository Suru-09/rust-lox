pub mod expr {

    use crate::scanner::scan::{Token, TokenType};
    use std::fs::File;
    use std::process::Command;
    use std::io::prelude::*;
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
    }

    impl fmt::Display for Expr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Expr::Binary(left_expr, operand, right_expr) => write!(f, "({} {} {})", left_expr, operand.token_type_value() , right_expr),
                Expr::Grouping(expression) => write!(f, "(group {})", expression),
                Expr::Literal(value) => write!(f, "{}", value.token_type_value()),
                Expr::Unary(operand, right_expr) => write!(f, "({} {})", operand.token_type_value(), right_expr),
                Expr::Variable(token) => write!(f, "{}", token.token_type_value()),
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
            }
        }
    }

    pub trait Visitor<T> {
        fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
        fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
        fn visit_literal_expr(&mut self, value: &Token) -> T;
        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
        fn visit_variable_expr(&mut self, token: &Token) -> T;
    }

    impl Expr {
        pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
            match self {
                Expr::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
                Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
                Expr::Literal(value) => visitor.visit_literal_expr(value),
                Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
                Expr::Variable(token) => visitor.visit_variable_expr(token),
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
    }

    pub struct GraphVizPrinter  {
        graph: String,
        node_count: u64,
        graph_name: String,
    }

    impl GraphVizPrinter {
        pub fn new(graph_name: String) -> GraphVizPrinter {
            GraphVizPrinter {
                graph: format!("digraph {} {{\n \trankdir=\"LR\";\n", graph_name),
                node_count: 0,
                graph_name: graph_name,
            }
        }

        pub fn close_graph(&mut self) {
            self.graph.push_str("\n}")
        }

        pub fn increase_node_count(&mut self) {
            self.node_count += 1;
        }

        pub fn add_node(&mut self, label: String) -> u64 {
            self.increase_node_count();
            self.graph.push_str(format!("\tnode_{} [label=\"{}\"];\n", self.node_count, label).as_str());
            self.node_count
        }

        pub fn add_edge(&mut self, from: u64, to: u64) {
            self.graph.push_str(format!("\tnode_{} -> node_{};\n", from, to).as_str());
        }

        pub fn to_string(&self) -> String {
            self.graph.clone()
        }

        pub fn write_to_file(&self) {
            let path = self.path_to_generated() + &self.graph_name + ".dot";

            let mut file = match File::create(path) {
                Ok(file) => file,
                Err(why) => panic!("couldn't create file: {}", why),
            };

            match file.write_all(self.graph.as_bytes()) {
                Ok(_) => println!("successfully wrote to file"),
                Err(why) => panic!("couldn't write to file: {}", why),
            }
        }

        fn path_to_generated(&self) -> String {
            let current_dir = match std::env::current_dir() {
                Ok(dir) => dir,
                Err(why) => panic!("couldn't get current dir: {}", why),
            };
            println!("The current directory is {}", current_dir.display());

            format!("{}/src/resources/generated/ast/", current_dir.display())
        }

        pub fn generate_image(&self) {
            let path = self.path_to_generated();
            let dot_path = path.clone() + &self.graph_name + ".dot";
            let output_path = path + &self.graph_name + ".png";
            let output = Command::new("dot")
                .arg("-Tpng")
                .arg(dot_path)
                .arg("-o")
                .arg(output_path)
                .output()
                .expect("failed to execute process");
            println!("output: {}", String::from_utf8_lossy(&output.stdout));
        }


    }

    impl Visitor<u64> for GraphVizPrinter {
        fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> u64 {
            let left_node_index = left.accept(self);
            let right_node_index = right.accept(self);
            let operator_node_index = self.add_node(operator.token_type_value());
            self.add_edge(operator_node_index, left_node_index);
            self.add_edge(operator_node_index, right_node_index);
            operator_node_index
        }

        fn visit_grouping_expr(&mut self, expression: &Expr) -> u64 {
            let expression_node_index = expression.accept(self);
            let grouping_node_index = self.add_node(String::from("(Grouping)"));
            self.add_edge(grouping_node_index, expression_node_index);
            grouping_node_index
        }

        fn visit_literal_expr(&mut self, value: &Token) -> u64 {
            self.add_node(value.token_type_value())
        }

        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> u64 {
            let right_node_index = right.accept(self);
            let operator_node_index = self.add_node(operator.token_type_value());
            self.add_edge(operator_node_index, right_node_index);
            operator_node_index
        }

        fn visit_variable_expr(&mut self, token: &Token) -> u64 {
            self.add_node(token.token_type_value())
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
            ),
        )))),
        );
        expression
    }

    pub fn generate_test_graph() {
        let ast = build_test_ast();
        let mut graph_printer = GraphVizPrinter::new( String::from("test"));
        ast.accept(&mut graph_printer);
        graph_printer.close_graph();
        graph_printer.write_to_file();
        graph_printer.generate_image();
        println!("{}", graph_printer.to_string());
    }


}
