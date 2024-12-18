pub mod error_handling {
    use chrono;
    use colored::{Colorize, CustomColor};
    use log::error;
    use log::{Level, Metadata, Record};
    use std::fmt;
    use std::sync::{LazyLock, RwLock};
    pub static LOGGER: SimpleLogger = SimpleLogger;

    pub static IS_WASM: LazyLock<RwLock<bool>> = LazyLock::new(|| RwLock::new(false));
    pub static WASM_OUTPUT: LazyLock<RwLock<Vec<String>>> = LazyLock::new(|| RwLock::new(vec![]));
    pub static WASM_ERRORS: LazyLock<RwLock<Vec<String>>> = LazyLock::new(|| RwLock::new(vec![]));
    pub struct SimpleLogger;

    impl log::Log for SimpleLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Info
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                println!(
                    "{} - {} - {}",
                    chrono::offset::Local::now().to_string().bright_green(),
                    match record.level() {
                        Level::Trace => record.level().to_string().blue(),
                        Level::Debug => record.level().to_string().on_custom_color(CustomColor {
                            r: 128,
                            g: 128,
                            b: 128
                        }),
                        Level::Info => record.level().to_string().bright_white(),
                        Level::Warn => record.level().to_string().yellow(),
                        Level::Error => record.level().to_string().red(),
                    },
                    record.args().to_string().white()
                );
            }
        }

        fn flush(&self) {}
    }

    pub fn error(
        line: u32,
        column: u32,
        message: String,
        location: Option<String>,
        error_type: Option<RLoxErrorType>,
    ) {
        RLoxError {
            line,
            column,
            message,
            location: location.unwrap_or(String::from("UNKNOWN LOCATION")),
            error_type: error_type.unwrap_or(RLoxErrorType::RuntimeError),
        }
        .report();
    }

    #[derive(Debug, PartialEq)]
    pub enum RLoxErrorType {
        RuntimeError,
        ParseError,
        ScannerError,
    }

    impl fmt::Display for RLoxErrorType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let str;
            match self {
                RLoxErrorType::RuntimeError => str = "RuntimeErorr",
                RLoxErrorType::ParseError => str = "ParseError",
                RLoxErrorType::ScannerError => str = "ScannerError",
            }
            write!(f, "{}", str)
        }
    }

    struct RLoxError {
        line: u32,
        column: u32,
        message: String,
        location: String,
        error_type: RLoxErrorType,
    }

    impl RLoxError {
        fn report(&self) {
            let error_msg = format!(
                "[{}] <{}> [line: {} & col: {}] msg: {}",
                self.error_type, self.location, self.line, self.column, self.message
            );
            error!("{}", error_msg);

            if *IS_WASM.read().unwrap() == true {
                WASM_ERRORS.write().unwrap().push(error_msg);
            }
        }
    }
}
