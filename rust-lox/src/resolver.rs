pub mod resolver {

    use crate::stmt::stmt::{LiteralValue, StmtVisitor};
    use crate::expr::expr::Visitor;
    use crate::interpreter::interpreter::Interpreter;
    use crate::expr::expr::Expr;
    use crate::stmt::stmt::Stmt;
    use crate::scanner::scan::Token;


    pub struct Resolver<'a> {
        pub interpreter: &'a mut Interpreter,
        scopes: Vec<Vec<(String, bool)>>,
    }


    impl<'a> Resolver<'a> {
        pub fn new(interpreter: &'a mut Interpreter) -> Self {
            let mut scopes_local = Vec::new();
            let clock_fun = (String::from("clock"), true);
            let unix_clock_fun = (String::from("unixClock"), true);
            scopes_local.push(Vec::new());
            scopes_local.last_mut().unwrap().push(clock_fun);
            scopes_local.last_mut().unwrap().push(unix_clock_fun);
            
            Self {
                interpreter,
                scopes: scopes_local,
            }
        }

        fn begin_scope(&mut self) {
            self.scopes.push(Vec::new());
        }

        fn end_scope(&mut self) {
            self.scopes.pop();
        }

        fn resolve_expr(&mut self, expr: &Expr) -> Result<(), String> {
            expr.accept(self) 
        }

        fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
            stmt.accept(self)
        }

        pub fn resolve(&mut self, stmts: &Vec<Stmt>) -> Result<(), String> {
            for stmt in stmts {
                self.resolve_stmt(stmt)?;
            }
            Ok(())
        }

        fn declare(&mut self, name: &Token) -> Result<(), String> {
            if self.scopes.is_empty() {
                return Ok(());
            }

            // check if the variable is already declared in the current scope.
            if let Some(scope) = self.scopes.last() {
                if self.contains_key(name, scope) {
                    return Err(format!("Variable '{}' has already been declared in this scope.", name.get_token_type().to_string()));
                }
            }

            if let Some(scope) = self.scopes.last_mut() {
                scope.push((name.get_token_type().to_string(), false));
            }

            Ok(())
        }

        fn define(&mut self, name: &Token) {
            if self.scopes.is_empty() {
                return;
            }

            // find the old variable in the scope and set it to true.
            if let Some(scope) = self.scopes.last_mut() {
                // find index of old variable
                let mut idx = -1;
                for (i, (key, _)) in scope.iter().enumerate() {
                    if key == &name.get_token_type().to_string() {
                        idx = i as i64;
                    }
                }

                if idx != -1 {
                    scope[idx as usize].1 = true;
                }
            }
        }

        fn contains_key(&self, name: &Token, scope: &Vec<(String, bool)>) -> bool {
            for (key, _) in scope {
                if key == &name.get_token_type().to_string() {
                    return true;
                }
            }
            false
        }

     
        fn _get_scope_after_string(&self, name: &Token, scope: &Vec<(String, bool)>) -> Option<(String, bool)> {
            for (i, (key, _)) in scope.iter().enumerate() {
                if key == &name.get_token_type().to_string() {
                    return Some(scope[i].clone());
                }
            }
            None
        }

        fn resolve_local(&mut self, token: &Token, _: Expr) {
            for (i, scope) in self.scopes.iter().enumerate().rev() {
                if self.contains_key(token, scope) {
                    self.interpreter.resolve(Expr::Variable(token.clone()), i);       
                    return;
                }
            }
        }

        fn resolve_function(&mut self, _name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> Result<(), String> {
            self.begin_scope();
            for param in params {
                self.declare(param)?;
                self.define(param);
            }
            self.resolve(body)?;
            self.end_scope();

            Ok(())
        }
    }

    impl Visitor<Result<(), String>> for Resolver<'_> {
        fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> Result<(), String> {
            self.resolve_expr(expr)?;
            self.resolve_local(token, expr.clone());
            Ok(())

        }

        fn visit_binary_expr(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> Result<(), String> {
            self.resolve_expr(left)?;
            self.resolve_expr(right)?;
            Ok(())
        }

        fn visit_call_expr(&mut self, callee: &Expr, _paren: &Token, arguments: &Vec<Expr>) -> Result<(), String> {
            self.resolve_expr(callee)?;

            for arg in arguments {
                self.resolve_expr(arg)?;
            }

            Ok(())
        }

        fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<(), String> {
            self.resolve_expr(expression)?;
            Ok(())
        }

        fn visit_literal_expr(&mut self, _: &LiteralValue) -> Result<(), String> {
            Ok(())
        }

        fn visit_unary_expr(&mut self, _operator: &Token, right: &Expr) -> Result<(), String> {
            self.resolve_expr(right)?;
            Ok(())
        }

        fn visit_logical_expr(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> Result<(), String> {
            self.resolve_expr(left)?;
            self.resolve_expr(right)?;
            Ok(())
        }

        fn visit_variable_expr(&mut self, token: &Token) -> Result<(), String> {
            // if !self.scopes.is_empty() {
            //     if let Some(scope) = self.scopes.last() {
            //         if let Some((_, is_defined)) = self.get_scope_after_string(token, scope) {
            //             // if !is_defined {
            //             //     panic!("Cannot read local variable in its own initializer.");
            //             // }
            //         }
            //     }
            // }

            let expr = Expr::Variable(token.clone());
            self.resolve_local(token, expr);
            Ok(())
        }
    }

    impl StmtVisitor<Result<(), String>> for Resolver<'_> {
        fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), String> {
            self.begin_scope();
            self.resolve(stmts)?;
            self.end_scope();
            Ok(())
        }

        fn visit_class_stmt(&mut self, name: &Token, _: &Vec<Stmt>) -> Result<(), String> {
            self.declare(name)?;
            self.define(name);
            Ok(())
        }

        fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), String> {
            self.resolve_expr(expr)?;
            Ok(())
        }

        fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> Result<(), String> {
            self.declare(name)?;
            self.define(name);
            self.resolve_function(name, params, body)?;
            Ok(())
        }

        fn visit_if_stmt(&mut self, expr: &Expr, stmt: &Stmt, else_stmt: &Option<Box<Stmt>>) -> Result<(), String> {
            self.resolve_expr(expr)?;
            self.resolve_stmt(stmt)?;
            if let Some(else_stmt) = else_stmt {
                self.resolve_stmt(else_stmt)?;
            }
            Ok(())
        }

        fn visit_return_stmt(&mut self, _keyword: &Token, expr: &Expr) -> Result<(), String> {
            self.resolve_expr(expr)?;
            Ok(())
        }

        fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), String> {
            self.resolve_expr(expr)?;
            Ok(())
        }

        /**'
         * Resolve a variable declaration statement.
         * Split into two cases:
         *    1. Variable declaration. --> We put false in the hashmap.
         *    2. Variable definition.  --> We put true in the hashmap.""
         */
        fn visit_var_stmt(&mut self, token: &Token, expr: &Expr) -> Result<(), String> {
            self.declare(token)?;
            match expr {
                Expr::Call(_, _, _) => {
                    self.resolve_expr(expr)?;
                },
                _ => self.resolve_expr(expr)?,
            }
            self.define(token);
            Ok(())
        }

        fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> Result<(), String> {
            self.resolve_expr(expr)?;
            self.resolve_stmt(stmt)?;

            Ok(())
        }
    }


}