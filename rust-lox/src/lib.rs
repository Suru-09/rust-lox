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
use error_handling::error_handling::{IS_WASM, LOGGER, WASM_ERRORS, WASM_OUTPUT};
use interpreter::interpreter::Interpreter;
use log::{info, LevelFilter};

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

    let mut write = IS_WASM.write().unwrap();
    *write = true;
}

pub fn execute_file(source: String) -> (String, String) {
    let mut scanner = scanner::scan::Scanner::new(source.clone());
    let tokens = scanner.scan_tokens();

    info!("Excuting file: {}", source);

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

    let output = WASM_OUTPUT
        .read()
        .unwrap()
        .iter()
        .fold("".to_string(), |cur: String, next: &String| {
            cur + next + "\n"
        });

    let errors = WASM_ERRORS
        .read()
        .unwrap()
        .iter()
        .fold("".to_string(), |cur: String, next: &String| {
            cur + next + "\n"
        });

    WASM_OUTPUT.write().unwrap().clear();
    WASM_ERRORS.write().unwrap().clear();

    (output, errors)
}
