mod resolver {

    use crate::stmt::stmt::StmtVisitor;
    use crate::expr::expr::Visitor;
    use crate::interpreter::interpreter::Interpreter;
    use std::collections::HashMap;
    use crate::expr::expr::Expr;
    use crate::stmt::stmt::Stmt;
    use crate::scanner::scan::Token;


    pub struct Resolver<'a> {
        interpreter: &'a mut Interpreter,
        scopes: Vec<HashMap<String, bool>>,
    }


    impl<'a> Resolver<'a> {
        pub fn new(interpreter: &'a mut Interpreter) -> Self {
            Self {
                interpreter,
                scopes: Vec::new(),
            }
        }

        fn begin_scope(&mut self) {
            self.scopes.push(HashMap::new());
        }

        fn end_scope(&mut self) {
            self.scopes.pop();
        }

        fn resolve_expr(&mut self, expr: &Expr) {
            expr.accept(self);
        }

        fn resolve_stmt(&mut self, stmt: &Stmt) {
            stmt.accept(self);
        }

        fn resolve(&mut self, stmts: &Vec<Stmt>) {
            for stmt in stmts {
                self.resolve_stmt(stmt);
            }
        }

        fn declare(&mut self, name: &Token) {
            if self.scopes.is_empty() {
                return;
            }

            if let Some(scope) = self.scopes.last_mut() {
                scope.insert(name.get_token_type().to_string(), false);
            }
        }

        fn define(&mut self, name: &Token) {
            if self.scopes.is_empty() {
                return;
            }

            if let Some(scope) = self.scopes.last_mut() {
                scope.insert(name.get_token_type().to_string(), true);
            }
        }

        fn resolve_local(&mut self, expr: &Expr, name: &Token) {
            for (i, scope) in self.scopes.iter().enumerate().rev() {
                if scope.contains_key(&name.get_token_type().to_string()) {
                    self.interpreter.resolve(expr, scope.len() - 1 - i);
                    return;
                }
            }
        }
    }

    impl Visitor<()> for Resolver<'_> {
        fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> () {
            unimplemented!()
        }

        fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> () {
            unimplemented!()
        }

        fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> () {
            unimplemented!()
        }

        fn visit_grouping_expr(&mut self, expression: &Expr) -> () {
            unimplemented!()
        }

        fn visit_literal_expr(&mut self, value: &Token) -> () {
            unimplemented!()
        }

        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> () {
            unimplemented!()
        }

        fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> () {
            unimplemented!()
        }

        fn visit_variable_expr(&mut self, token: &Token) -> () {
            if !self.scopes.is_empty() {
                if let Some(scope) = self.scopes.last() {
                    if let Some(defined) = scope.get(&token.get_token_type().to_string()) {
                        if !defined {
                            println!("Cannot read local variable in its own initializer.");
                        }
                    }
                }
            }

            self.resolve_local(token, token.get_token_type().to_string());
        }
    }

    impl StmtVisitor<()> for Resolver<'_> {
        fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> () {
            self.begin_scope();
            self.resolve(stmts);
            self.end_scope();
        }

        fn visit_expr_stmt(&mut self, expr: &Expr) -> () {
            unimplemented!()
        }

        fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> () {
            unimplemented!()
        }

        fn visit_if_stmt(&mut self, expr: &Expr, stmt: &Stmt, else_stmt: &Option<Box<Stmt>>) -> () {
            unimplemented!()
        }

        fn visit_return_stmt(&mut self, keyword: &Token, expr: &Expr) -> () {
            unimplemented!()
        }

        fn visit_print_stmt(&mut self, expr: &Expr) -> () {
            unimplemented!()
        }

        /**'
         * Resolve a variable declaration statement.
         * Split into two cases:
         *    1. Variable declaration. --> We put false in the hashmap.
         *    2. Variable definition.  --> We put true in the hashmap.
         */
        fn visit_var_stmt(&mut self, token: &Token, expr: &Expr) -> () {
            self.declare(token);
            match expr {
                Expr::Call(_, _, _) => (),
                _ => self.resolve_expr(expr),
            }
            self.define(token);
        }

        fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> () {
            unimplemented!()
        }
    }


}