pub mod resolver {

    use crate::error_handling::error_handling::{error, RLoxErrorType};
    use crate::expr::expr::Expr;
    use crate::expr::expr::Visitor;
    use crate::function_name;
    use crate::interpreter::interpreter::{Error, Interpreter};
    use crate::scanner::scan::Token;
    use crate::stmt::stmt::Stmt;
    use crate::stmt::stmt::{LiteralValue, StmtVisitor};

    #[derive(Clone, Debug, PartialEq)]
    pub enum ClassType {
        None,
        Class,
    }

    pub struct Resolver<'a> {
        pub interpreter: &'a mut Interpreter,
        scopes: Vec<Vec<(String, bool)>>,
        current_class: ClassType,
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
                current_class: ClassType::None,
            }
        }

        fn begin_scope(&mut self) {
            self.scopes.push(Vec::new());
        }

        fn end_scope(&mut self) {
            self.scopes.pop();
        }

        fn resolve_expr(&mut self, expr: &Expr) -> Result<(), Error> {
            expr.accept(self)
        }

        fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
            stmt.accept(self)
        }

        pub fn resolve(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
            for stmt in stmts {
                self.resolve_stmt(stmt)?;
            }
            Ok(())
        }

        fn declare(&mut self, name: &Token) -> Result<(), Error> {
            if self.scopes.is_empty() {
                return Ok(());
            }

            // check if the variable is already declared in the current scope.
            if let Some(scope) = self.scopes.last() {
                if self.contains_key(name, scope) && self.scopes.last() != self.scopes.first() {
                    error(
                        name.get_line(),
                        name.get_column(),
                        format!(
                            "Error at '{}': Already a variable with this name in this scope.",
                            name.get_token_type()
                        ),
                        function_name!(),
                        Some(RLoxErrorType::RuntimeError),
                    );
                    return Err(Error::LoxRuntimeError);
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

        fn get_scope_after_string(
            &self,
            name: &Token,
            scope: &Vec<(String, bool)>,
        ) -> Option<(String, bool)> {
            for (i, (key, _)) in scope.iter().enumerate() {
                if key == &name.get_token_type().to_string() {
                    return Some(scope[i].clone());
                }
            }
            None
        }

        fn resolve_local(&mut self, token: &Token) {
            for (i, scope) in self.scopes.iter().enumerate().rev() {
                if self.contains_key(token, scope) {
                    self.interpreter
                        .resolve(Expr::Variable(token.clone()), self.scopes.len() - 1 - i);
                    return;
                }
            }
        }

        fn resolve_function(
            &mut self,
            _name: &Token,
            params: &Vec<Token>,
            body: &Vec<Stmt>,
        ) -> Result<(), Error> {
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

    impl Visitor<Result<(), Error>> for Resolver<'_> {
        fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> Result<(), Error> {
            self.resolve_expr(expr)?;
            self.resolve_local(token);
            Ok(())
        }

        fn visit_binary_expr(
            &mut self,
            left: &Expr,
            _operator: &Token,
            right: &Expr,
        ) -> Result<(), Error> {
            self.resolve_expr(left)?;
            self.resolve_expr(right)?;
            Ok(())
        }

        fn visit_call_expr(
            &mut self,
            callee: &Expr,
            _paren: &Token,
            arguments: &Vec<Expr>,
        ) -> Result<(), Error> {
            self.resolve_expr(callee)?;

            for arg in arguments {
                self.resolve_expr(arg)?;
            }

            Ok(())
        }

        fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<(), Error> {
            self.resolve_expr(expression)?;
            Ok(())
        }

        fn visit_literal_expr(&mut self, _: &LiteralValue) -> Result<(), Error> {
            Ok(())
        }

        fn visit_unary_expr(&mut self, _operator: &Token, right: &Expr) -> Result<(), Error> {
            self.resolve_expr(right)?;
            Ok(())
        }

        fn visit_get_expr(&mut self, object: &Expr, _name: &Token) -> Result<(), Error> {
            self.resolve_expr(object)?;
            Ok(())
        }

        fn visit_set_expr(
            &mut self,
            object: &Expr,
            _name: &Token,
            value: &Expr,
        ) -> Result<(), Error> {
            self.resolve_expr(value)?;
            self.resolve_expr(object)?;
            Ok(())
        }

        fn visit_this_expr(&mut self, keyword: &Token) -> Result<(), Error> {
            if self.current_class == ClassType::None {
                error(
                    keyword.get_line(),
                    keyword.get_column(),
                    format!(
                        "Error at '{}': Can't use 'this' outside of a class.",
                        keyword.get_token_type()
                    ),
                    function_name!(),
                    Some(RLoxErrorType::RuntimeError),
                );
                return Err(Error::LoxRuntimeError);
            }
            self.resolve_local(keyword);
            Ok(())
        }

        fn visit_logical_expr(
            &mut self,
            left: &Expr,
            _operator: &Token,
            right: &Expr,
        ) -> Result<(), Error> {
            self.resolve_expr(left)?;
            self.resolve_expr(right)?;
            Ok(())
        }

        fn visit_variable_expr(&mut self, token: &Token) -> Result<(), Error> {
            if !self.scopes.is_empty() {
                if let Some(scope) = self.scopes.last() {
                    if let Some((_, is_defined)) = self.get_scope_after_string(token, scope) {
                        if !is_defined {
                            error(
                                token.get_line(),
                                token.get_column(),
                                format!("Error at '{}': Can't read local variable in its own initializer.", token.get_token_type()),
                                function_name!(),
                                Some(RLoxErrorType::RuntimeError)
                            );
                        }
                    }
                }
            }

            self.resolve_local(token);
            Ok(())
        }
    }

    impl StmtVisitor<Result<(), Error>> for Resolver<'_> {
        fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
            self.begin_scope();
            self.resolve(stmts)?;
            self.end_scope();
            Ok(())
        }

        fn visit_class_stmt(&mut self, name: &Token, methods: &Vec<Stmt>) -> Result<(), Error> {
            let enclosing_class = self.current_class.clone();
            self.current_class = ClassType::Class;
            self.declare(name)?;
            self.define(name);

            self.begin_scope();
            self.scopes
                .last_mut()
                .unwrap()
                .push((String::from("this"), true));

            for method in methods {
                match method {
                    Stmt::Function(fn_name, fn_params, fn_body) => {
                        self.resolve_function(fn_name, fn_params, fn_body)?;
                    }
                    _ => (),
                }
            }

            self.end_scope();
            self.current_class = enclosing_class;

            Ok(())
        }

        fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), Error> {
            self.resolve_expr(expr)?;
            Ok(())
        }

        fn visit_function_stmt(
            &mut self,
            name: &Token,
            params: &Vec<Token>,
            body: &Vec<Stmt>,
        ) -> Result<(), Error> {
            self.declare(name)?;
            self.define(name);
            self.resolve_function(name, params, body)?;
            Ok(())
        }

        fn visit_if_stmt(
            &mut self,
            expr: &Expr,
            stmt: &Stmt,
            else_stmt: &Option<Box<Stmt>>,
        ) -> Result<(), Error> {
            self.resolve_expr(expr)?;
            self.resolve_stmt(stmt)?;
            if let Some(else_stmt) = else_stmt {
                self.resolve_stmt(else_stmt)?;
            }
            Ok(())
        }

        fn visit_return_stmt(&mut self, _keyword: &Token, expr: &Expr) -> Result<(), Error> {
            self.resolve_expr(expr)?;
            Ok(())
        }

        fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), Error> {
            self.resolve_expr(expr)?;
            Ok(())
        }

        /**'
         * Resolve a variable declaration statement.
         * Split into two cases:
         *    1. Variable declaration. --> We put false in the hashmap.
         *    2. Variable definition.  --> We put true in the hashmap.""
         */
        fn visit_var_stmt(&mut self, token: &Token, expr: &Expr) -> Result<(), Error> {
            self.declare(token)?;
            match expr {
                Expr::Call(_, _, _) => {
                    self.resolve_expr(expr)?;
                }
                _ => self.resolve_expr(expr)?,
            }
            self.define(token);
            Ok(())
        }

        fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> Result<(), Error> {
            self.resolve_expr(expr)?;
            self.resolve_stmt(stmt)?;

            Ok(())
        }
    }
}
