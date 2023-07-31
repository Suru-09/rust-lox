pub mod stmt {

    use crate::expr::expr::Expr;
    use std::fmt;

    pub enum Stmt {
        ExprStmt(Expr),
        PrintStmt(Expr),
    }
    
    impl fmt::Display for Stmt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Stmt::ExprStmt(expr) => write!(f, "{}", expr),
                Stmt::PrintStmt(expr) => write!(f, "(print {})", expr),
            }
        }
    }

    pub trait StmtVisitable {
        fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T;
    }

    pub trait StmtVisitor<T> {
        fn visit_expr_stmt(&mut self, expr: &Expr) -> T;
        fn visit_print_stmt(&mut self, expr: &Expr) -> T;
    }

    impl StmtVisitable for Stmt {
        fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
            match self {
                Stmt::ExprStmt(expr) => visitor.visit_expr_stmt(expr),
                Stmt::PrintStmt(expr) => visitor.visit_print_stmt(expr),
            }
        }
    }

    impl Stmt {
        pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
            match self {
                Stmt::ExprStmt(expr) => visitor.visit_expr_stmt(expr),
                Stmt::PrintStmt(expr) => visitor.visit_print_stmt(expr),
            }
        }
    }

}