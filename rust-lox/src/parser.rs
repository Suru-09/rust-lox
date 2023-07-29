pub mod parser {

    /**
     * ! Notes to my self:
     * ! No. 1:
     * * * * Most of the functions used in the parser have side-effects, calling match_token, match_any_number_or_string, etc.
     * * * * if they are returning true it means that the index(current) has been changed and should proceed with caution knowing this
     * * * * at typical bug would be calling match_token and after testing the value calling self.peek() which returns the current value
     * * * * (jokes on you) because it was changed, when match_token found the character it did current += 1.
     * *
     */

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

        fn binary_expr_loop(&mut self, operators: Vec<TokenType>, next_rule: fn(&mut Self) -> Result<Expr, String>) -> Result<Expr, String> {
            let mut expr = match next_rule(self) {
                Ok(expr_val) => expr_val,
                Err(e) => return Err(e),
            };


            while self.match_token(operators.clone()) {
                let operator = self.previous();
                let right = match next_rule(self) {
                    Ok(right_val) => right_val,
                    Err(e) => return Err(e),
                };
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            }
            Ok(expr)
        }

        fn expression(&mut self) -> Result<Expr, String>  {
            self.equality()
        }

        fn equality(&mut self) -> Result<Expr, String> {
            self.binary_expr_loop(vec![TokenType::BangEqual, TokenType::EqualEqual], Self::comparison)
        }

        fn comparison(&mut self) -> Result<Expr, String> {
            self.binary_expr_loop(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual], Self::term)
        }

        fn term(&mut self) -> Result<Expr, String> {
            self.binary_expr_loop(vec![TokenType::Minus, TokenType::Plus], Self::factor)
        }

        fn factor(&mut self) -> Result<Expr, String> {
            self.binary_expr_loop(vec![TokenType::Slash, TokenType::Star], Self::unary)
        }

        fn unary(&mut self) -> Result<Expr, String> {
            if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
                let operator = self.previous();
                let right = match self.unary() {
                    Ok(right_val) => right_val,
                    Err(e) => return Err(e),
                };
                return Ok(Expr::Unary(operator, Box::new(right)));
            }
            self.primary()
        }

        fn match_any_number_or_string(&mut self) -> bool {
            let is_number_or_string = match self.peek().get_token_type() {
                TokenType::Number(_) | TokenType::String(_) => true,
                _ => false,
            };
            if is_number_or_string {
                self.advance();
            }
            is_number_or_string
        }

        fn primary(&mut self) -> Result<Expr, String> {
            println!("{}", self.peek().to_string());

            if self.match_token(vec![TokenType::False]) {
                return Ok(Expr::Literal(Token::new(TokenType::False, String::from("false"), 0, 0, 0)));
            }

            if self.match_token(vec![TokenType::True]) {
                return Ok(Expr::Literal(Token::new(TokenType::True, String::from("true"), 0, 0, 0)));
            }

            if self.match_token(vec![TokenType::Nil]) {
                return Ok(Expr::Literal(Token::new(TokenType::Nil, String::from("nil"), 0, 0, 0)));
            }

            if self.match_any_number_or_string() {
                return Ok(Expr::Literal(self.previous()));
            }

            if self.match_token(vec![TokenType::LeftParen]) {
                let expr = match self.expression() {
                    Ok(expr_val) => expr_val,
                    Err(e) => return Err(e),
                };
                self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string());
                return Ok(Expr::Grouping(Box::new(expr)));
            }

            error(self.peek().get_line(), self.peek().get_column(),"Expect expression.".to_string());
            Err("".to_string())
        }

        fn consume(&mut self, token_type: TokenType, message: String) {
            if self.check(token_type) {
                self.advance();
                return;
            }
            error(self.current as u32, 0, message);
        }

        pub fn parse(&mut self) -> Result<Expr, String> {
            self.expression()
        }

    }
}
