pub mod parser {

    use crate::scanner::scan::{Token, TokenType};
    use crate::expr::expr::Expr;
    use crate::error_handling::error_handling::error;

    pub struct Parser {
        tokens: Vec<Token>,
        current: u64,
    }

    impl Parser {
        pub fn new(tokens_vec: Vec<Token>) -> Parser {
            Parser {
                tokens: tokens_vec,
                current: 0,
            }
        }

        fn previous(&self) -> Token {
            self.tokens[self.current as usize - 1].clone()
        }

        fn peek(&self) -> Token {
            self.tokens[self.current as usize].clone()
        }

        fn is_at_end(&self) -> bool {
            self.peek().get_token_type() == TokenType::EOF
        }

        fn check(&self, token_type: TokenType) -> bool {
            if self.is_at_end() {
                return false;
            }
            self.peek().token_type_value() == token_type.to_string()
        }

        fn advance(&mut self) -> Token {
            if !self.is_at_end() {
                self.current += 1;
            }
            self.previous()
        }


        fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
            for token_type in token_types {
                if self.check(token_type) {
                    self.advance();
                    return true;
                }
            }
            false
        }

        fn binary_expr_loop(&mut self, operators: Vec<TokenType>, next_rule: fn(&mut Self) -> Expr) -> Expr {
            let mut expr = next_rule(self);
            while self.match_token(operators.clone()) {
                let operator = self.previous();
                let right = next_rule(self);
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            }
            expr
        }

        fn expression(&mut self) -> Expr {
            self.equality()
        }

        fn equality(&mut self) -> Expr {
            self.binary_expr_loop(vec![TokenType::BangEqual, TokenType::EqualEqual], Self::comparison)
        }

        fn comparison(&mut self) -> Expr {
            self.binary_expr_loop(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual], Self::term)
        }

        fn term(&mut self) -> Expr {
            self.binary_expr_loop(vec![TokenType::Minus, TokenType::Plus], Self::factor)
        }

        fn factor(&mut self) -> Expr {
            self.binary_expr_loop(vec![TokenType::Slash, TokenType::Star], Self::unary)
        }

        fn unary(&mut self) -> Expr {
            if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
                let operator = self.previous();
                let right = self.unary();
                return Expr::Unary(operator, Box::new(right));
            }
            self.primary()
        }

        fn match_any_number_or_string(&mut self) -> bool {
            match self.peek().get_token_type() {
                TokenType::Number(_) | TokenType::String(_) => true,
                _ => false,
            }
        }

        fn primary(&mut self) -> Expr {
            println!("{}", self.peek().to_string());

            if self.match_token(vec![TokenType::False]) {
                return Expr::Literal(self.previous());
            }

            if self.match_token(vec![TokenType::True]) {
                return Expr::Literal(self.previous());
            }

            if self.match_token(vec![TokenType::Nil]) {
                return Expr::Literal(self.previous());
            }

            if self.match_any_number_or_string() {
                return Expr::Literal(self.previous());
            }

            if self.match_token(vec![TokenType::LeftParen]) {
                let expr = self.expression();
                self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string());
                return Expr::Grouping(Box::new(expr));
            }
            
            panic!("Expect expression.");
        }

        fn consume(&mut self, token_type: TokenType, message: String) {
            if self.check(token_type) {
                self.advance();
                return;
            }
            error(self.current as u32, 0, message);
        }

        pub fn parse(&mut self) -> Expr {
            self.expression()
        }

    }
}