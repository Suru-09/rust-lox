pub mod error_handling {

    use log::error;

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
        fn report(&self) {
            error!("[line {} and column {}] Error {}: {}", self.line, self.column, self.location, self.message);
        }
    }
}
