pub mod parser {

    use crate::error_handling::error_handling::error;
    use crate::expr::expr::Expr;
    use crate::function_name;
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
    use crate::stmt::stmt::{Stmt, LiteralValue};
    use log::debug;

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

        fn binary_expr_loop(
            &mut self,
            operators: Vec<TokenType>,
            next_rule: fn(&mut Self) -> Result<Expr, String>,
        ) -> Result<Expr, String> {
            let mut expr = next_rule(self)?;

            while self.match_token(operators.clone()) {
                let operator = self.previous();
                let right = next_rule(self)?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            }
            Ok(expr)
        }

        fn logical_expr_loop(
            &mut self,
            operators: Vec<TokenType>,
            next_rule: fn(&mut Self) -> Result<Expr, String>,
        ) -> Result<Expr, String> {
            let mut expr = next_rule(self)?;

            while self.match_token(operators.clone()) {
                let operator = self.previous();
                let right = next_rule(self)?;
                expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
            }
            Ok(expr)
        }

        fn expression(&mut self) -> Result<Expr, String> {
            self.assignment()
        }

        fn or(&mut self) -> Result<Expr, String> {
            self.logical_expr_loop(vec![TokenType::Or], Self::and)
        }

        fn and(&mut self) -> Result<Expr, String> {
            self.logical_expr_loop(vec![TokenType::And], Self::equality)
        }

        fn equality(&mut self) -> Result<Expr, String> {
            self.binary_expr_loop(
                vec![TokenType::BangEqual, TokenType::EqualEqual],
                Self::comparison,
            )
        }

        fn comparison(&mut self) -> Result<Expr, String> {
            self.binary_expr_loop(
                vec![
                    TokenType::Greater,
                    TokenType::GreaterEqual,
                    TokenType::Less,
                    TokenType::LessEqual,
                ],
                Self::term,
            )
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
            self.call()
        }

        fn call(&mut self) -> Result<Expr, String> {
            let mut expr = self.primary()?;

            loop {
                if self.match_token(vec![TokenType::LeftParen]) {
                    expr = self.finish_call(expr)?;
                } else {
                    break;
                }
            }
            Ok(expr)
        }

        fn finish_call(&mut self, expr: Expr) -> Result<Expr, String> {
            let mut arguments: Vec<Expr> = Vec::new();
            if !self.check(TokenType::RightParen) {
                loop {
                    if arguments.len() >= 255 {
                        error(
                            self.peek().get_line(),
                            self.peek().get_column(),
                            "Can't have more than 255 arguments.".to_string(),
                            function_name!(),
                        );
                    }
                    arguments.push(self.expression()?);
                    if !self.match_token(vec![TokenType::Comma]) {
                        break;
                    }
                }
            }
            let paren = self.consume(
                TokenType::RightParen,
                "Expect ')' after arguments.".to_string(),
            );
            Ok(Expr::Call(Box::new(expr), paren, arguments))
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

        fn match_any_identifier(&mut self) -> bool {
            let is_identifier = match self.peek().get_token_type() {
                TokenType::Identifier(_) => true,
                _ => false,
            };
            if is_identifier {
                self.advance();
            }
            is_identifier
        }

        fn consume_any_identifier(&mut self, kind: String) -> Token {
            if self.match_any_identifier() {
                return self.previous();
            }
            error(
                self.peek().get_line(),
                self.peek().get_column(),
                format!("Expect {} name.", kind),
                function_name!(),
            );
            self.peek()
        }

        fn primary(&mut self) -> Result<Expr, String> {
            debug!("{}", self.peek().get_token_type());

            if self.match_token(vec![TokenType::False]) {
                return Ok(Expr::Literal(LiteralValue::Bool(false)));
            }

            if self.match_token(vec![TokenType::True]) {
                return Ok(Expr::Literal(LiteralValue::Bool(true)));
            }

            if self.match_token(vec![TokenType::Nil]) {
                return Ok(Expr::Literal(LiteralValue::Nil));
            }

            if self.match_any_number_or_string() {
                match self.previous().get_token_type() {
                    TokenType::String(str) => {return Ok(Expr::Literal(LiteralValue::String(str)));},
                    TokenType::Number(num) => {return Ok(Expr::Literal(LiteralValue::Number(num)));},
                    _ => {
                        error(
                            self.previous().get_line(),
                            self.previous().get_column(), 
                            "It has to be either a string or a number at this point".to_string(),
                            function_name!());
                        return Err("It has to be either a string or a number at this point".to_string());
                    }
                }
            }

            if self.match_any_identifier() {
                return Ok(Expr::Variable(self.previous()));
            }

            if self.match_token(vec![TokenType::LeftParen]) {
                let expr = match self.expression() {
                    Ok(expr_val) => expr_val,
                    Err(e) => return Err(e),
                };
                self.consume(
                    TokenType::RightParen,
                    "Expect ')' after expression.".to_string(),
                );
                return Ok(Expr::Grouping(Box::new(expr)));
            }

            error(
                self.peek().get_line(),
                self.peek().get_column(),
                "Expect expression.".to_string(),
                function_name!(),
            );
            Err("Expect Expression?".to_string())
        }

        fn consume(&mut self, token_type: TokenType, message: String) -> Token {
            if self.check(token_type) {
                self.advance();
                return self.previous();
            }
            let current_token = self.tokens.get(self.current as usize).unwrap();
            error(
                current_token.get_line(),
                current_token.get_column(),
                message,
                function_name!(),
            );
            self.peek()
        }

        pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
            let mut expressions: Vec<Stmt> = Vec::new();
            while !self.is_at_end() {
                expressions.push(self.declaration()?);
            }
            Ok(expressions)
        }

        pub fn statement(&mut self) -> Result<Stmt, String> {
            match self.peek().get_token_type() {
                TokenType::Print => self.print_statement(),
                TokenType::Return => {
                    self.advance();
                    self.return_statement()
                }
                TokenType::While => self.while_statement(),
                TokenType::For => self.for_statement(),
                TokenType::If => self.if_statement(),
                TokenType::LeftBrace => {
                    self.advance();
                    Ok(Stmt::BlockStmt(self.block_statement()?))
                }
                _ => self.expression_statement(),
            }
        }

        fn block_statement(&mut self) -> Result<Vec<Stmt>, String> {
            let mut statements: Vec<Stmt> = Vec::new();
            while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                statements.push(self.declaration()?);
            }
            self.consume(TokenType::RightBrace, "Expect '}' after block.".to_string());
            Ok(statements)
        }

        pub fn print_statement(&mut self) -> Result<Stmt, String> {
            self.advance();
            let value = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
            Ok(Stmt::PrintStmt(value))
        }

        pub fn return_statement(&mut self) -> Result<Stmt, String> {
            let keyword = self.previous();
            let mut value = Expr::Literal(LiteralValue::Nil);
            if !self.check(TokenType::Semicolon) {
                value = self.expression()?;
            }
            self.consume(
                TokenType::Semicolon,
                "Expect ';' after return value.".to_string(),
            );
            Ok(Stmt::ReturnStmt(keyword, value))
        }

        fn while_statement(&mut self) -> Result<Stmt, String> {
            self.consume(
                TokenType::While,
                "Expect 'while' after 'while'.".to_string(),
            );
            self.consume(
                TokenType::LeftParen,
                "Expect '(' after 'while'.".to_string(),
            );
            let condition = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expect ')' after condition.".to_string(),
            );
            let body = self.statement()?;
            Ok(Stmt::WhileStmt(condition, Box::new(body)))
        }

        fn for_statement(&mut self) -> Result<Stmt, String> {
            self.consume(TokenType::For, "Expect 'for' after 'for'.".to_string());
            self.consume(TokenType::LeftParen, "Expect '(' after 'for'.".to_string());

            let initializer = if self.match_token(vec![TokenType::Semicolon]) {
                None
            } else if self.match_token(vec![TokenType::Var]) {
                Some(self.var_declaration()?)
            } else {
                Some(self.expression_statement()?)
            };

            let mut condition = None;
            if !self.check(TokenType::RightParen) {
                condition = Some(self.expression()?);
            }
            self.consume(
                TokenType::Semicolon,
                "Expect ';' after loop condition.".to_string(),
            );

            let mut increment = None;
            if !self.check(TokenType::RightParen) {
                increment = Some(self.expression()?);
            }
            self.consume(
                TokenType::RightParen,
                "Expect ')' after for clauses.".to_string(),
            );

            let mut body = self.statement()?;

            if let Some(increment_val) = increment {
                body = Stmt::BlockStmt(vec![body, Stmt::ExprStmt(increment_val)]);
            }

            if let None = condition {
                condition = Some(Expr::Literal(LiteralValue::Bool(true)));
            }

            match condition {
                Some(condition_val) => body = Stmt::WhileStmt(condition_val, Box::new(body)),
                None => (),
            }

            if let Some(initializer_val) = initializer {
                body = Stmt::BlockStmt(vec![initializer_val, body]);
            }

            Ok(body)
        }

        fn if_statement(&mut self) -> Result<Stmt, String> {
            self.consume(TokenType::If, "Expect 'if' after 'if'.".to_string());
            self.consume(TokenType::LeftParen, "Expect '(' after 'if'.".to_string());
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expect ')' after if condition.".to_string(),
            );
            let then_branch = self.statement()?;

            let else_branch = if self.match_token(vec![TokenType::Else]) {
                match self.statement() {
                    Ok(else_branch_val) => Some(Box::new(else_branch_val)),
                    Err(e) => return Err(e),
                }
            } else {
                None
            };

            return Ok(Stmt::IfStmt(expr, Box::new(then_branch), else_branch));
        }

        pub fn expression_statement(&mut self) -> Result<Stmt, String> {
            let expr = self.expression()?;
            self.consume(
                TokenType::Semicolon,
                "Expect ';' after expression.".to_string(),
            );
            Ok(Stmt::ExprStmt(expr))
        }

        fn declaration(&mut self) -> Result<Stmt, String> {
            if self.match_token(vec![TokenType::Class]) {
                return self.class_declaration();
            }

            if self.match_token(vec![TokenType::Fun]) {
                return self.function("function".to_string());
            }

            if self.match_token(vec![TokenType::Var]) {
                return self.var_declaration();
            }
            self.statement()
        }

        fn class_declaration(&mut self) -> Result<Stmt, String> {
            let name: Token = self.consume_any_identifier("class".to_string());
            self.consume(
                TokenType::LeftBrace,
                "Expect '{' before class body.".to_string(),
            );

            let mut methods: Vec<Stmt> = Vec::new();
            while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                methods.push(self.function("method".to_string())?);
            }

            self.consume(
                TokenType::RightBrace,
                "Expect '}' after class body.".to_string(),
            );

            Ok(Stmt::ClassStmt(name, methods))
        }

        fn function(&mut self, kind: String) -> Result<Stmt, String> {
            let name = self.consume_any_identifier(kind.clone());
            self.consume(
                TokenType::LeftParen,
                format!("Expect '(' after {} name.", kind.clone()),
            );
            let mut parameters: Vec<Token> = Vec::new();
            if !self.check(TokenType::RightParen) {
                loop {
                    if parameters.len() >= 255 {
                        error(
                            self.peek().get_line(),
                            self.peek().get_column(),
                            "Can't have more than 255 parameters.".to_string(),
                            function_name!(),
                        );
                    }
                    parameters.push(self.consume_any_identifier("parameter".to_string()));

                    if !self.match_token(vec![TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(
                TokenType::RightParen,
                "Expect ')' after parameters.".to_string(),
            );
            self.consume(
                TokenType::LeftBrace,
                format!("Expect '{{' before {} body.", kind.clone()),
            );
            let body = self.block_statement()?;
            Ok(Stmt::Function(name, parameters, body))
        }

        fn var_declaration(&mut self) -> Result<Stmt, String> {
            if !self.match_any_identifier() {
                error(
                    self.peek().get_line(),
                    self.peek().get_column(),
                    "Expect variable name.".to_string(),
                    function_name!(),
                );
                return Err("".to_string());
            }
            let name = self.previous();
            let mut initializer = Expr::Literal(LiteralValue::Nil);
            if self.match_token(vec![TokenType::Equal]) {
                initializer = self.expression()?;
            };

            self.consume(
                TokenType::Semicolon,
                "Expect ';' after variable declaration.".to_string(),
            );
            Ok(Stmt::VarStmt(name, initializer))
        }

        fn assignment(&mut self) -> Result<Expr, String> {
            let expr = self.or()?;

            /*
             * * * Note: we enter the if statement only if we have an assignment,
             * * * otherwise we just return the expression, therefore for variable
             * * * declaration we return directly the value of expr.
             */
            if self.match_token(vec![TokenType::Equal]) {
                let equals = self.previous();
                let value = self.assignment()?;

                match expr {
                    Expr::Variable(name) | Expr::Assign(name, _) => {
                        return Ok(Expr::Assign(name, Box::new(value)))
                    },
                    _ => {
                        error(
                            equals.get_line(),
                            equals.get_column(),
                            "Invalid assignment target.".to_string(),
                            function_name!(),
                        );
                        return Err("Invalid assignment target.".to_string());
                    }
                }
            }
            Ok(expr)
        }
    }
}
