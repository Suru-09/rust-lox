pub mod stmt {

    use crate::expr::expr::{Expr, Visitor};
    use crate::rlox_callable::rlox_callable::Callable;
    use crate::scanner::scan::Token;
    use log::debug;
    use std::fmt;
    use std::fs::File;
    use std::io::prelude::*;
    use std::process::Command;

    #[derive(Clone)]
    pub enum Stmt {
        ExprStmt(Expr),
        PrintStmt(Expr),
        ReturnStmt(Token, Expr),
        VarStmt(Token, Expr),
        BlockStmt(Vec<Stmt>),
        ClassStmt(Token, Vec<Stmt>),
        Function(Token, Vec<Token>, Vec<Stmt>),
        IfStmt(Expr, Box<Stmt>, Option<Box<Stmt>>),
        WhileStmt(Expr, Box<Stmt>),
    }

    #[derive(Clone)]
    pub enum LiteralValue {
        Number(f64),
        Bool(bool),
        String(String),
        Callable(Callable),
        Nil,
    }

    impl fmt::Display for Stmt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Stmt::ExprStmt(expr) => write!(f, "{}", expr),
                Stmt::ReturnStmt(_keyword, value) => write!(f, "(return {})", value),
                Stmt::PrintStmt(expr) => write!(f, "(print {})", expr),
                Stmt::VarStmt(token, expr) => {
                    write!(f, "(var {} {})", token.get_token_type(), expr)
                }
                Stmt::BlockStmt(stmts) => {
                    let mut stmts_str = String::new();
                    for stmt in stmts {
                        stmts_str.push_str(format!("{}", stmt).as_str());
                    }
                    write!(f, "{}", stmts_str)
                }
                Stmt::ClassStmt(name, methods) => {
                    let mut methods_str = String::new();
                    // iterate over methods and add them to the string.
                    for method in methods {
                        methods_str.push_str(format!("{}, ", method).as_str());
                    }
                    write!(
                        f,
                        "(class: <{}> methods: [{}])",
                        name.get_token_type(),
                        methods_str
                    )
                }
                Stmt::Function(name, params, body) => {
                    let mut function_str = String::new();
                    function_str.push_str(format!("(fun {} (", name.get_token_type()).as_str());
                    for param in params {
                        function_str.push_str(format!("{} ", param.get_token_type()).as_str());
                    }
                    function_str.push_str(") ");
                    for stmt in body {
                        function_str.push_str(format!("{}", stmt).as_str());
                    }
                    function_str.push_str(")");
                    write!(f, "{}", function_str)
                }
                Stmt::IfStmt(expr, stmt, else_stmt) => {
                    let mut if_stmt_str = String::new();
                    if_stmt_str.push_str(format!("(if {} {} ", expr, stmt).as_str());
                    if let Some(else_stmt) = else_stmt {
                        if_stmt_str.push_str(format!("{} ", else_stmt).as_str());
                    }
                    if_stmt_str.push_str(")");
                    write!(f, "{}", if_stmt_str)
                }
                Stmt::WhileStmt(expr, stmt) => write!(f, "(while {} {})", expr, stmt),
            }
        }
    }

    pub trait StmtVisitable {
        fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T;
    }

    pub trait StmtVisitor<T> {
        fn visit_expr_stmt(&mut self, expr: &Expr) -> T;
        fn visit_print_stmt(&mut self, expr: &Expr) -> T;
        fn visit_return_stmt(&mut self, keyword: &Token, expr: &Expr) -> T;
        fn visit_var_stmt(&mut self, token: &Token, expr: &Expr) -> T;
        fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> T;
        fn visit_class_stmt(&mut self, name: &Token, methods: &Vec<Stmt>) -> T;
        fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>)
            -> T;
        fn visit_if_stmt(&mut self, expr: &Expr, stmt: &Stmt, else_stmt: &Option<Box<Stmt>>) -> T;
        fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> T;
    }

    impl StmtVisitable for Stmt {
        fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
            match self {
                Stmt::ExprStmt(expr) => visitor.visit_expr_stmt(expr),
                Stmt::PrintStmt(expr) => visitor.visit_print_stmt(expr),
                Stmt::ReturnStmt(keyword, expr) => visitor.visit_return_stmt(keyword, expr),
                Stmt::VarStmt(token, expr) => visitor.visit_var_stmt(token, expr),
                Stmt::BlockStmt(stmts) => visitor.visit_block_stmt(stmts),
                Stmt::ClassStmt(name, methods) => visitor.visit_class_stmt(name, methods),
                Stmt::Function(name, params, body) => {
                    visitor.visit_function_stmt(name, params, body)
                }
                Stmt::IfStmt(expr, stmt, else_stmt) => visitor.visit_if_stmt(expr, stmt, else_stmt),
                Stmt::WhileStmt(expr, stmt) => visitor.visit_while_stmt(expr, stmt),
            }
        }
    }

    impl Stmt {
        pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
            match self {
                Stmt::ExprStmt(expr) => visitor.visit_expr_stmt(expr),
                Stmt::PrintStmt(expr) => visitor.visit_print_stmt(expr),
                Stmt::ReturnStmt(keyword, expr) => visitor.visit_return_stmt(keyword, expr),
                Stmt::VarStmt(token, expr) => visitor.visit_var_stmt(token, expr),
                Stmt::BlockStmt(stmts) => visitor.visit_block_stmt(stmts),
                Stmt::ClassStmt(name, methods) => visitor.visit_class_stmt(name, methods),
                Stmt::Function(name, params, body) => {
                    visitor.visit_function_stmt(name, params, body)
                }
                Stmt::IfStmt(expr, stmt, else_stmt) => visitor.visit_if_stmt(expr, stmt, else_stmt),
                Stmt::WhileStmt(expr, stmt) => visitor.visit_while_stmt(expr, stmt),
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

        pub fn generate(ast_val: &Vec<Stmt>) {
            let mut counter: u32 = 0;
            for expr in ast_val {
                // generate graph from AST both as a dot file and as a png image.
                let graph_name = format!("graph_{}", counter);
                let mut graph_printer = StmtGraphvizPrinter::new(graph_name);
                counter += 1;
                expr.accept(&mut graph_printer);
                graph_printer.close_graph();
                graph_printer.write_to_file();
                graph_printer.generate_image();
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
                format!(
                    "\"{}\"",
                    "\\".to_string() + (&label.clone()[..label.len() - 1]) + "\\\""
                )
            } else {
                format!("\"{}\"", label)
            };

            self.graph.push_str(
                format!("\tnode_{} [label={}];\n", self.node_count, formated_label).as_str(),
            );
            self.node_count
        }

        pub fn add_edge(&mut self, from: u64, to: u64) {
            self.graph
                .push_str(format!("\tnode_{} -> node_{};\n", from, to).as_str());
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
                Ok(_) => {}
                Err(why) => panic!("couldn't write to file: {}", why),
            }
        }

        fn path_to_generated(&self) -> String {
            let current_dir = match std::env::current_dir() {
                Ok(dir) => dir,
                Err(why) => panic!("couldn't get current dir: {}", why),
            };

            // make sure bothh generated and ast directories exist, if not create them.
            let generated_dir = format!("{}/src/resources/generated/", current_dir.display());
            let ast_dir = format!("{}/src/resources/generated/ast/", current_dir.display());

            if !std::path::Path::new(&generated_dir).exists() {
                match std::fs::create_dir(&generated_dir) {
                    Ok(_) => debug!("Successfully created generated directory"),
                    Err(why) => panic!("couldn't create generated directory: {}", why),
                }
            }

            if !std::path::Path::new(&ast_dir).exists() {
                match std::fs::create_dir(&ast_dir) {
                    Ok(_) => debug!("Successfully created ast directory"),
                    Err(why) => panic!("couldn't create ast directory: {}", why),
                }
            }
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
            debug!("Output: {}", String::from_utf8_lossy(&output.stdout));
        }
    }

    impl StmtVisitor<u64> for StmtGraphvizPrinter {
        fn visit_expr_stmt(&mut self, expr: &Expr) -> u64 {
            let expr_node_id = expr.accept(self);
            let node_id = self.add_node(expr.name());
            self.add_edge(node_id, expr_node_id);
            node_id
        }

        fn visit_print_stmt(&mut self, expr: &Expr) -> u64 {
            let node_id = self.add_node(String::from("print"));
            let expr_node_id = expr.accept(self);
            self.add_edge(node_id, expr_node_id);
            node_id
        }

        fn visit_return_stmt(&mut self, keyword: &Token, expr: &Expr) -> u64 {
            let expr_node_id = expr.accept(self);
            let token_node_id = self.add_node(keyword.token_type_value());
            self.add_edge(token_node_id, expr_node_id);
            token_node_id
        }

        fn visit_var_stmt(&mut self, token: &Token, expr: &Expr) -> u64 {
            let expr_node_id = expr.accept(self);
            let token_node_id = self.add_node(token.token_type_value());
            self.add_edge(token_node_id, expr_node_id);
            token_node_id
        }

        fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> u64 {
            let stmts_node_id = self.add_node(String::from("Block"));
            for stmt in stmts {
                let stmt_node_id = stmt.accept(self);
                self.add_edge(stmts_node_id, stmt_node_id);
            }
            stmts_node_id
        }

        fn visit_class_stmt(&mut self, name: &Token, methods: &Vec<Stmt>) -> u64 {
            let class_node_id = self.add_node(String::from("class"));
            let name_node_id = self.add_node(name.token_type_value());
            self.add_edge(class_node_id, name_node_id);
            for method in methods {
                let method_node_id = method.accept(self);
                self.add_edge(class_node_id, method_node_id);
            }
            class_node_id
        }

        fn visit_function_stmt(
            &mut self,
            name: &Token,
            params: &Vec<Token>,
            body: &Vec<Stmt>,
        ) -> u64 {
            let function_node_id = self.add_node(String::from("function"));
            let name_node_id = self.add_node(name.token_type_value());
            self.add_edge(function_node_id, name_node_id);
            for param in params {
                let param_node_id = self.add_node(param.token_type_value());
                self.add_edge(function_node_id, param_node_id);
            }
            for stmt in body {
                let stmt_node_id = stmt.accept(self);
                self.add_edge(function_node_id, stmt_node_id);
            }
            function_node_id
        }

        fn visit_if_stmt(
            &mut self,
            expr: &Expr,
            stmt: &Stmt,
            else_stmt: &Option<Box<Stmt>>,
        ) -> u64 {
            let expr_node_id = expr.accept(self);
            let stmt_node_id = stmt.accept(self);
            let if_node_id = self.add_node(String::from("if"));
            self.add_edge(if_node_id, expr_node_id);
            self.add_edge(if_node_id, stmt_node_id);
            if let Some(else_stmt) = else_stmt {
                let else_node_id = else_stmt.accept(self);
                self.add_edge(if_node_id, else_node_id);
            }
            if_node_id
        }

        fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> u64 {
            let expr_node_id = expr.accept(self);
            let stmt_node_id = stmt.accept(self);
            let while_node_id = self.add_node(String::from("while"));
            self.add_edge(while_node_id, expr_node_id);
            self.add_edge(while_node_id, stmt_node_id);
            while_node_id
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

        fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> u64 {
            let expr_node_id = expr.accept(self);
            let token_node_id = self.add_node(token.token_type_value());
            self.add_edge(token_node_id, expr_node_id);
            token_node_id
        }

        fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> u64 {
            let left_node_index = left.accept(self);
            let right_node_index = right.accept(self);
            let operator_node_index = self.add_node(operator.token_type_value());
            self.add_edge(operator_node_index, left_node_index);
            self.add_edge(operator_node_index, right_node_index);
            operator_node_index
        }

        fn visit_call_expr(&mut self, callee: &Expr, _paren: &Token, arguments: &Vec<Expr>) -> u64 {
            let callee_node_index = callee.accept(self);
            for argument in arguments {
                let argument_node_index = argument.accept(self);
                self.add_edge(callee_node_index, argument_node_index);
            }
            callee_node_index
        }
    }
}
