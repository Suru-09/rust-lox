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
use args_parser::args_parser::Args;
use clap::Parser;
use error_handling::error_handling::LOGGER;
use log::error;
use log::LevelFilter;
use std::fs;
use std::path::Path;
use stmt::stmt::StmtGraphvizPrinter;

fn run(source: String, args: &Args) {
    let mut scanner = scanner::scan::Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = parser::parser::Parser::new(tokens);
    let ast = parser
        .parse()
        .expect("Expected to be able to parse the source file!");

    if args.graphviz == true {
        StmtGraphvizPrinter::generate(&ast);
    }

    if args.cli_graph == true {
        // TO BE IMPLEMENTED
    }

    let mut interpreter = interpreter::interpreter::Interpreter::new();
    let mut resolver = Resolver::new(&mut interpreter);
    resolver
        .resolve(&ast)
        .expect("Expected to be able to resolve stuff without errors!");

    let _ = resolver.interpreter.interpret(&ast);
}

fn run_file(args: &Args) {
    if Path::new(&args.src_path.clone()).exists() {
        run(
            fs::read_to_string(&args.src_path).expect("Given path does not contain an OK file!!"),
            args,
        );
    } else {
        panic!("Given path: {}, does not exist!!", &args.src_path);
    }
}

fn run_prompt(args: &Args) {
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("[run_prompt] Failed to read line");
        run(input, args);
    }
}

fn main() {
    let log_level = if cfg!(debug_assertions) {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    };

    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log_level))
        .unwrap();

    // delete old generated files
    if !utils::utils::clean_folder(utils::utils::GENERATED_FOLDER_PATH) {
        error!("Could not clean generated folder");
    }

    let args = Args::parse();
    if args.src_path.is_empty() {
        run_prompt(&args);
    } else {
        run_file(&args);
    }
}
