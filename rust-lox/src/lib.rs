pub mod args_parser;
pub mod environment;
pub mod error_handling;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod resolver;
pub mod rlox_callable;
pub mod scanner;
pub mod stmt;
pub mod utils;

use crate::resolver::resolver::Resolver;
use error_handling::error_handling::LOGGER;
use interpreter::interpreter::Interpreter;
use log::LevelFilter;

pub fn init() {
    let log_level = if cfg!(debug_assertions) {
        LevelFilter::Trace
    } else {
        LevelFilter::Trace
    };

    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log_level))
        .expect(&format!(
            "I should be able to set MAX log level to: {}!",
            log_level.to_string()
        ));
}

pub fn execute_file(source: String) {
    let mut scanner = scanner::scan::Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = parser::parser::Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut interpreter = Interpreter::new();
    let mut resolver = Resolver::new(&mut interpreter);
    match resolver.resolve(&ast) {
        Ok(_) => {}
        Err(_) => {
            std::process::exit(1);
        }
    }

    match resolver.interpreter.interpret(&ast) {
        Ok(_) => {}
        Err(_) => {
            std::process::exit(1);
        }
    }
}
