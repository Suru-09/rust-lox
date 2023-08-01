pub mod stmt {

    use crate::expr::expr::{Expr, Visitor};
    use crate::scanner::scan::Token;
    use std::fmt;
    use std::fs::File;
    use std::io::prelude::*;
    use std::process::Command;


    #[derive(Clone)]
    pub enum Stmt {
        ExprStmt(Expr),
        PrintStmt(Expr),
        VarStmt(Token, Expr),
    }
    
    impl fmt::Display for Stmt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Stmt::ExprStmt(expr) => write!(f, "{}", expr),
                Stmt::PrintStmt(expr) => write!(f, "(print {})", expr),
                Stmt::VarStmt(token, expr) => write!(f, "(var {} {})", token.get_token_type(), expr),
            }
        }
    }

    pub trait StmtVisitable {
        fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T;
    }

    pub trait StmtVisitor<T> {
        fn visit_expr_stmt(&mut self, expr: &Expr) -> T;
        fn visit_print_stmt(&mut self, expr: &Expr) -> T;
        fn visit_var_stmt(&mut self, token: &Token, expr: &Expr) -> T;
    }

    impl StmtVisitable for Stmt {
        fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
            match self {
                Stmt::ExprStmt(expr) => visitor.visit_expr_stmt(expr),
                Stmt::PrintStmt(expr) => visitor.visit_print_stmt(expr),
                Stmt::VarStmt(token, expr) => visitor.visit_var_stmt(token, expr),
            }
        }
    }

    impl Stmt {
        pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
            match self {
                Stmt::ExprStmt(expr) => visitor.visit_expr_stmt(expr),
                Stmt::PrintStmt(expr) => visitor.visit_print_stmt(expr),
                Stmt::VarStmt(token, expr) => visitor.visit_var_stmt(token, expr),
            }
        }
    }

    pub struct StmtGraphvizPrinter {
        pub graph_name: String,
        pub graph: String,
        pub node_count: u64,
    }

    impl StmtGraphvizPrinter {
        pub fn new(name: String) -> StmtGraphvizPrinter {
            StmtGraphvizPrinter {
                graph_name: name,
                graph: String::from("digraph G { \n rankdir=\"LR\" \n"),
                node_count: 0,
            }
        }

        pub fn close_graph(&mut self) {
            self.graph.push_str("}");
        }

        pub fn increase_node_count(&mut self) {
            self.node_count += 1;
        }

        pub fn add_node(&mut self, label: String) -> u64 {
            self.increase_node_count();
            // special rule for escaping strings which contain " as first and last character.
            let formated_label = if label.starts_with("\"") && label.ends_with("\"") {
                format!("\"{}\"", "\\".to_string() + (&label.clone()[..label.len() - 1]) + "\\\"")
            } else {
                format!("\"{}\"", label)
            };

            self.graph.push_str(format!("\tnode_{} [label={}];\n", self.node_count, formated_label).as_str());
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

    impl StmtVisitor<u64> for StmtGraphvizPrinter {
        fn visit_expr_stmt(&mut self, expr: &Expr) -> u64 {
            let expr_node_id = expr.accept(self);
            let node_id = self.add_node(String::from("expr"));
            self.add_edge(node_id, expr_node_id);
            node_id
        }

        fn visit_print_stmt(&mut self, expr: &Expr) -> u64 {
            let node_id = self.add_node(String::from("print"));
            let expr_node_id = expr.accept(self);
            self.add_edge(node_id, expr_node_id);
            node_id
        }

        fn visit_var_stmt(&mut self, token: &Token, expr: &Expr) -> u64 {
            let expr_node_id = expr.accept(self);
            let token_node_id = self.add_node(token.token_type_value());
            self.add_edge(token_node_id, expr_node_id);
            token_node_id
        }
    }

    impl Visitor<u64> for StmtGraphvizPrinter {
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

}