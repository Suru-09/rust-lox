pub mod error_handling {
    use chrono;
    use colored::{Colorize, CustomColor};
    use log::error;
    use log::{Level, Metadata, Record};

    pub static LOGGER: SimpleLogger = SimpleLogger;
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

    pub fn error(line: u32, column: u32, message: String, location: Option<String>) {
        RLoxError {
            line,
            column,
            message,
            location: location.unwrap_or(String::from("UNKNOWN LOCATION")),
        }
        .report();
    }

    struct RLoxError {
        line: u32,
        column: u32,
        message: String,
        location: String,
    }

    impl RLoxError {
        fn report(&self) {
            error!(
                "<{}> [line: {} & col: {}] msg: {}",
                self.location, self.line, self.column, self.message
            );
        }
    }
}
