pub mod scan {
    #![allow(dead_code)]

    use std::fmt;
    use std::collections::HashMap;
    use crate::error_handling::error_handling::error;
    
    #[derive(Clone)] #[derive(PartialEq)] #[derive(Debug)]
    pub enum TokenType {
        // Single-character tokens.
        LeftParen, RightParen, LeftBrace, RightBrace,
        Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

        // One or two character tokens.
        Bang, BangEqual,
        Equal, EqualEqual,
        Greater, GreaterEqual,
        Less, LessEqual,

        Identifier(String), String(String), Number(f64),

        // Keywords.
        And, Class, Else, False, Fun, For, If, Nil, Or,
        Print, Return, Super, This, True, Var, While,

        EOF
    }

    macro_rules! create_map {
        ($($token:ident),*) => {{
            let mut m = HashMap::new();
            $(
                m.insert(format!("{}", TokenType::$token), TokenType::$token);
            )*
            m
        }}
    }
    
    fn keywords() -> HashMap<String, TokenType> {
        create_map!(
            And,
            Class,
            Else,
            False,
            Fun,
            For,
            If,
            Nil,
            Or,
            Print,
            Return,
            Super,
            This,
            True,
            Var,
            While
        )
    }

    impl fmt::Display for TokenType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                TokenType::LeftParen => write!(f, "("),
                TokenType::RightParen => write!(f, ")"),
                TokenType::LeftBrace => write!(f, "{{"),
                TokenType::RightBrace => write!(f, "}}"),
                TokenType::Comma => write!(f, ","),
                TokenType::Dot => write!(f, "."),
                TokenType::Minus => write!(f, "-"),
                TokenType::Plus => write!(f, "+"),
                TokenType::Semicolon => write!(f, ";"),
                TokenType::Slash => write!(f, "/"),
                TokenType::Star => write!(f, "*"),
                TokenType::Bang => write!(f, "!"),
                TokenType::BangEqual => write!(f, "!="),
                TokenType::Greater => write!(f, ">"),
                TokenType::GreaterEqual => write!(f, ">="),
                TokenType::Less => write!(f, "<"),
                TokenType::LessEqual => write!(f, "<="),
                TokenType::Equal => write!(f, "="),
                TokenType::EqualEqual => write!(f, "=="),
                TokenType::Identifier(ident) => write!(f, "{}", ident),
                TokenType::String(str) => write!(f, "{}", str),
                TokenType::Number(num) => write!(f, "{}", num),
                TokenType::And => write!(f, "and"),
                TokenType::Class => write!(f, "class"),
                TokenType::Else => write!(f, "else"),
                TokenType::False => write!(f, "false"),
                TokenType::True => write!(f, "true"),
                TokenType::Fun => write!(f, "fun"),
                TokenType::For => write!(f, "for"),
                TokenType::If => write!(f, "if"),
                TokenType::Nil => write!(f, "nil"),
                TokenType::Or => write!(f, "or"),
                TokenType::Print => write!(f, "print"),
                TokenType::Return => write!(f, "return"),
                TokenType::Super => write!(f, "super"),
                TokenType::This => write!(f, "this"),
                TokenType::Var => write!(f, "var"),
                TokenType::While => write!(f, "while"),
                TokenType::EOF => write!(f, "EOF"),
            }
        }
    }

    #[derive(Clone)]
    pub struct Token {
        t_type: TokenType,
        lexeme: String,
        line: u32,
        column: u32,
        length: u32,
    }

    impl Token {
        pub fn new(t_type: TokenType, lexeme: String, line: u32, column: u32, length: u32) -> Token {
            Token {
                t_type,
                lexeme,
                line,
                column,
                length
            }
        }

        pub fn to_string(&self) -> String {
            format!("[Tok]: {} lex:[{}]  line:[{}] col:[{}] len:[{}]",
            self.t_type, self.lexeme, self.line, self.column, self.length)
        }

        pub fn token_type_value(&self) -> String {
            self.t_type.to_string()
        }
    }

    pub struct Scanner {
        source: String,
        tokens: Vec<Token>,
        start: u32,
        current: u32,
        line: u32,
    }

    impl  Scanner {
        pub fn new(source: String) -> Scanner {
            if source.len() > u32::MAX as usize {
                panic!("Given code's length goes past U32 max value!");
            }

            Scanner {
                source,
                tokens: Vec::new(),
                start: 0,
                current: 0,
                line: 0,
            }
        }

        fn is_last(&self) -> bool {
            self.current >= self.source.len() as u32
        }

        pub fn scan_tokens(&mut self) -> Vec<Token> {
            while !self.is_last() {
                self.start = self.current;
                self.scan_token();
            }

            self.tokens.push(Token::new(TokenType::EOF, String::new(), self.line, self.current, 0));
            self.tokens.clone()
        }

        fn scan_token(&mut self) {
            let c: char = self.advance_token();
            match c {
                c if c == '(' => self.add_token(TokenType::LeftParen),
                c if c == ')' => self.add_token(TokenType::RightParen),
                c if c == '{' => self.add_token(TokenType::LeftBrace),
                c if c == '}' => self.add_token(TokenType::RightBrace),
                c if c == ',' => self.add_token(TokenType::Comma),
                c if c == '.' => self.add_token(TokenType::Dot),
                c if c == '-' => {
                    if self.peek().is_digit(10) {
                        self.number();
                    } else {
                        self.add_token(TokenType::Minus)
                    }
                },
                c if c == '+' => self.add_token(TokenType::Plus),
                c if c == ';' => self.add_token(TokenType::Semicolon),
                c if c == '*' => self.add_token(TokenType::Star),
                c if c == '!' => {
                    let t_type: TokenType = if self.match_token('=') { TokenType::BangEqual } else { TokenType::Bang };
                    self.add_token(t_type);
                },
                c if c == '=' => {
                    let t_type: TokenType = if self.match_token('=') { TokenType::EqualEqual } else { TokenType::Equal };
                    self.add_token(t_type);
                },
                c if c == '<' => {
                    let t_type: TokenType = if self.match_token('=') { TokenType::LessEqual } else { TokenType::Less };
                    self.add_token(t_type);
                },
                c if c == '>' => {
                    let t_type: TokenType = if self.match_token('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                    self.add_token(t_type);
                },
                // TODO: add support for C-style comments.
                c if c == '/' => {
                    if self.match_token('/') {
                        while self.peek() != '\n' && !self.is_last() {
                            self.advance_token();
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                },
                // skip all types of whitespaces.
                c if c == ' ' || c == '\r' || c == '\t' => (),
                c if c == '\n' => self.line += 1,
                c if c == '"' => self.string(),
                _ => {
                    if c.is_digit(10) {
                        self.number();
                    } else if c.is_alphabetic() {
                        self.identifier();
                    } else {
                        error(self.line, self.current, format!("Unexpected character: {}", c));
                    }
                }
            }
        }

        fn advance_token(&mut self) -> char {
            assert_eq!(self.current <= self.source.len() as u32, true, "[advance_token] Current index is out of bounds!");

            self.current += 1;
            self.source.chars().nth(self.current as usize - 1).unwrap()
        }

        fn identifier(&mut self) {
            while self.peek().is_alphanumeric() {
                self.advance_token();
            }

            let text: String = match self.source.get(self.start as usize..self.current as usize) {
                Some(text) => text.to_string(),
                None => panic!("[identifier] Could not get text from source!"),
            };
            let clone_text = text.clone();

            let token_type = match keywords().get(&clone_text) {
                Some(token_type) => token_type.clone(),
                None => TokenType::Identifier(text),
            };

            self.add_token(token_type);
        }

        fn peek(&self) -> char {
            if self.is_last() {
                return '\0';
            }
            assert_eq!(self.current < self.source.len() as u32, true, "[peek] Current index is out of bounds!");
            self.source.chars().nth(self.current as usize).unwrap()
        }

        // write a peak function that has a lookahead of n characters
        fn peek_n(&self, lookahead: u32) -> char {
            if self.current + lookahead >= self.source.len() as u32 {
                return '\0';
            }
            assert_eq!(self.current + lookahead < self.source.len() as u32, true, "[peek_n] Current index is out of bounds!");
            self.source.chars().nth(self.current as usize + 1).unwrap()
        }

        fn string(&mut self) {
            while self.peek() != '"' && !self.is_last() {
                if self.peek() == '\n' {
                    self.line += 1;
                }
                self.advance_token();
            }

            if self.is_last() {
                error(self.line, self.current, String::from("Unterminated string!"));
                return;
            }

            self.advance_token();

            let value: String = match self.source.get(self.start as usize + 1..self.current as usize - 1) {
                Some(value) => value.to_string(),
                None => panic!("[string] Could not get value from source!"),
            };
            self.add_token(TokenType::String(value));
        }

        fn match_token(&mut self, expected: char) -> bool {
            if self.is_last() {
                return false;
            }

            if self.source.chars().nth(self.current as usize).unwrap() != expected {
                return false;
            }

            self.current += 1;
            true
        }

        fn number(&mut self) {
            while self.peek().is_digit(10) {
                self.advance_token();
            }

            if self.peek() == '.' && self.peek_n(2).is_digit(10) {
                self.advance_token();

                while self.peek().is_digit(10) {
                    self.advance_token();
                }
            }

            let value: f64 = match self.source.get(self.start as usize..self.current as usize) {
                Some(value) => value.parse::<f64>().unwrap_or(-1.00),
                None => panic!("[number] Could not get value from source!"),
            };
            self.add_token(TokenType::Number(value));
        }

        fn add_token(&mut self, t_type: TokenType) {
            // TODO: add functionality for columns of tokens and also the length of the token
            let text: String = match self.source.get(self.start as usize..self.current as usize) {
                Some(text) => text.to_string(),
                None => panic!("[add_token] Could not get text from source!"),
            };
            self.tokens.push(Token { t_type: t_type, lexeme: text, line: self.line, column: 0, length: 0 });
        }
    }

    #[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_comment_ok() {
        let mut scanner = Scanner::new(String::from("// this is a comment"));
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].t_type, TokenType::EOF);
    }

    #[test]
    fn read_comment_multiple_lines_ok() {
        let mut scanner = Scanner::new(String::from("// this is a comment\n// this is another comment"));
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].t_type, TokenType::EOF);
    }

    // generate a test for each token type
    macro_rules! test_token {
        ($name:ident, $token:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let mut scanner = Scanner::new(String::from($token));
                let tokens = scanner.scan_tokens();
                assert_eq!(tokens.len(), 2);
                assert_eq!(tokens[0].t_type, $expected);
                assert_eq!(tokens[1].t_type, TokenType::EOF);
            }
        };
    }

    test_token!(read_left_paren_ok, "(", TokenType::LeftParen);
    test_token!(read_right_paren_ok, ")", TokenType::RightParen);
    test_token!(read_left_brace_ok, "{", TokenType::LeftBrace);
    test_token!(read_right_brace_ok, "}", TokenType::RightBrace);
    test_token!(read_comma_ok, ",", TokenType::Comma);
    test_token!(read_dot_ok, ".", TokenType::Dot);
    test_token!(read_minus_ok, "-", TokenType::Minus);
    test_token!(read_plus_ok, "+", TokenType::Plus);
    test_token!(read_semicolon_ok, ";", TokenType::Semicolon);
    test_token!(read_slash_ok, "/", TokenType::Slash);
    test_token!(read_star_ok, "*", TokenType::Star);
    test_token!(read_bang_ok, "!", TokenType::Bang);
    test_token!(read_bang_equal_ok, "!=", TokenType::BangEqual);
    test_token!(read_equal_ok, "=", TokenType::Equal);
    test_token!(read_equal_equal_ok, "==", TokenType::EqualEqual);
    test_token!(read_greater_ok, ">", TokenType::Greater);
    test_token!(read_greater_equal_ok, ">=", TokenType::GreaterEqual);
    test_token!(read_less_ok, "<", TokenType::Less);    
    test_token!(read_less_equal_ok, "<=", TokenType::LessEqual);
    // test keywords and identifiers
    test_token!(read_and_ok, "and", TokenType::And);
    test_token!(read_class_ok, "class", TokenType::Class);
    test_token!(read_else_ok, "else", TokenType::Else);
    test_token!(read_false_ok, "false", TokenType::False);
    test_token!(read_fun_ok, "fun", TokenType::Fun);
    test_token!(read_for_ok, "for", TokenType::For);
    test_token!(read_if_ok, "if", TokenType::If);
    test_token!(read_nil_ok, "nil", TokenType::Nil);
    test_token!(read_or_ok, "or", TokenType::Or);
    test_token!(read_print_ok, "print", TokenType::Print);
    test_token!(read_return_ok, "return", TokenType::Return);
    test_token!(read_super_ok, "super", TokenType::Super);
    test_token!(read_this_ok, "this", TokenType::This);
    test_token!(read_true_ok, "true", TokenType::True);
    test_token!(read_var_ok, "var", TokenType::Var);
    test_token!(read_while_ok, "while", TokenType::While);
    test_token!(read_identifier_ok, "identifier", TokenType::Identifier(String::from("identifier")));
    test_token!(read_string_ok, "\"string\"", TokenType::String(String::from("string")));
    test_token!(read_multiple_line_string_ok, "\"first\nsecond\nthird\nfourth\"", TokenType::String(String::from("first\nsecond\nthird\nfourth")));
    test_token!(read_number_ok, "123", TokenType::Number(123.00));
    test_token!(read_number_with_decimal_ok, "123.456", TokenType::Number(123.456));
    test_token!(read_negative_with_decimal_ok, "-123.456", TokenType::Number(-123.456));
    test_token!(read_negative_number_ok, "-123", TokenType::Number(-123.00));

    // try to read a token that is not supported
    #[test]
    fn read_unsupported_token_nok() {
        let mut scanner = Scanner::new(String::from("@"));
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].t_type, TokenType::EOF);
    }

    #[test]
    fn read_multiple_tokens_ok() {
        let mut scanner = Scanner::new(String::from("(){}"));
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].t_type, TokenType::LeftParen);
        assert_eq!(tokens[1].t_type, TokenType::RightParen);
        assert_eq!(tokens[2].t_type, TokenType::LeftBrace);
        assert_eq!(tokens[3].t_type, TokenType::RightBrace);
        assert_eq!(tokens[4].t_type, TokenType::EOF);
    }

    #[test]
    fn read_multiple_tokens_with_keywords_and_identifiers_ok() {
        let mut scanner = Scanner::new(String::from("var identifier = 123.47;"));
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].t_type, TokenType::Var);
        assert_eq!(tokens[1].t_type, TokenType::Identifier(String::from("identifier")));
        assert_eq!(tokens[2].t_type, TokenType::Equal);
        assert_eq!(tokens[3].t_type, TokenType::Number(123.47));
        assert_eq!(tokens[4].t_type, TokenType::Semicolon);
        assert_eq!(tokens[5].t_type, TokenType::EOF);
    }

}
}

