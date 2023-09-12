mod resolver {

    use crate::stmt::stmt::StmtVisitor;
    use crate::expr::expr::Visitor;
    use crate::interpreter::interpreter::Interpreter;


    pub struct Resolver<'a> {
        interpreter: &'a mut Interpreter,
    }


    impl<'a> Resolver<'a> {
        pub fn new(interpreter: &'a mut Interpreter) -> Self {
            Self {
                interpreter,
            }
        }
    }

    impl Visitor<()> for Resolver<'_> {
        fn visit_assign_expr(&mut self, token: &crate::scanner::scan::Token, expr: &crate::expr::expr::Expr) -> () {
            unimplemented!()
        }

        fn visit_binary_expr(&mut self, left: &crate::expr::expr::Expr, operator: &crate::scanner::scan::Token, right: &crate::expr::expr::Expr) -> () {
            unimplemented!()
        }

        fn visit_call_expr(&mut self, callee: &crate::expr::expr::Expr, paren: &crate::scanner::scan::Token, arguments: &Vec<crate::expr::expr::Expr>) -> () {
            unimplemented!()
        }

        fn visit_grouping_expr(&mut self, expression: &crate::expr::expr::Expr) -> () {
            unimplemented!()
        }

        fn visit_literal_expr(&mut self, value: &crate::scanner::scan::Token) -> () {
            unimplemented!()
        }

        fn visit_unary_expr(&mut self, operator: &crate::scanner::scan::Token, right: &crate::expr::expr::Expr) -> () {
            unimplemented!()
        }

        fn visit_logical_expr(&mut self, left: &crate::expr::expr::Expr, operator: &crate::scanner::scan::Token, right: &crate::expr::expr::Expr) -> () {
            unimplemented!()
        }

        fn visit_variable_expr(&mut self, token: &crate::scanner::scan::Token) -> () {
            unimplemented!()
        }
    }

    impl StmtVisitor<()> for Resolver<'_> {
        fn visit_block_stmt(&mut self, stmts: &Vec<crate::stmt::stmt::Stmt>) -> () {
            unimplemented!()
        }

        fn visit_expr_stmt(&mut self, expr: &crate::expr::expr::Expr) -> () {
            unimplemented!()
        }

        fn visit_function_stmt(&mut self, name: &crate::scanner::scan::Token, params: &Vec<crate::scanner::scan::Token>, body: &Vec<crate::stmt::stmt::Stmt>) -> () {
            unimplemented!()
        }

        fn visit_if_stmt(&mut self, expr: &crate::expr::expr::Expr, stmt: &crate::stmt::stmt::Stmt, else_stmt: &Option<Box<crate::stmt::stmt::Stmt>>) -> () {
            unimplemented!()
        }

        fn visit_return_stmt(&mut self, keyword: &crate::scanner::scan::Token, expr: &crate::expr::expr::Expr) -> () {
            unimplemented!()
        }

        fn visit_print_stmt(&mut self, expr: &crate::expr::expr::Expr) -> () {
            unimplemented!()
        }

        fn visit_var_stmt(&mut self, token: &crate::scanner::scan::Token, expr: &crate::expr::expr::Expr) -> () {
            unimplemented!()
        }

        fn visit_while_stmt(&mut self, expr: &crate::expr::expr::Expr, stmt: &crate::stmt::stmt::Stmt) -> () {
            unimplemented!()
        }
    }


}