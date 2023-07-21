pub mod error_handling {

    pub fn error(line: u32, column: u32, message: String) {
        RLoxError{
            line,
            column, 
            message, 
            location: String::new()
        }.report();
    }

    struct RLoxError {
        line: u32,
        column: u32,
        message: String,
        location: String,
    }

    impl RLoxError {
        fn new(line: u32, column: u32, message: String, location: String) -> RLoxError {
            RLoxError {
                line,
                column,
                message,
                location,
            }
        }

        fn report(&self) {
            println!("[line {} and column {}] Error {}: {}", self.line, self.column, self.location, self.message);
        }
    }
}