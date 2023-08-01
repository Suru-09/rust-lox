pub mod stmt {

    use crate::expr::expr::Expr;
    use crate::scanner::scan::Token;
    use std::fmt;

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

}