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
use crate::rlox_callable::rlox_callable::{RLoxClass, RLoxFunction};
use log::{error, info};
use std::path::Path;

fn run(source: String) {
    let mut scanner = scanner::scan::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens.clone() {
        info!("{}", token.to_string());
    }

    let mut parser = parser::parser::Parser::new(tokens);
    let ast = parser.parse();
    match ast {
        Ok(ast_val) => {
            let mut counter: u32 = 0;
            for expr in ast_val.clone() {
                // generate graph from AST both as a dot file and as a png image.
                let graph_name = format!("graph_{}", counter);
                let mut graph_printer = stmt::stmt::StmtGraphvizPrinter::new(graph_name);
                counter += 1;
                expr.accept(&mut graph_printer);
                graph_printer.close_graph();
                graph_printer.write_to_file();
                graph_printer.generate_image();
            }

            let mut interpreter = interpreter::interpreter::Interpreter::new();
            let mut resolver = Resolver::new(&mut interpreter);
            match resolver.resolve(&ast_val) {
                Ok(_) => {
                    info!("Resolver finished successfully");
                }
                Err(err) => {
                    error!("{}", err);
                    return;
                }
            }

            let interpreted_vec = resolver.interpreter.interpret(ast_val);

            match interpreted_vec {
                Ok(interpreted_vec_val) => {
                    for interpreted in interpreted_vec_val {
                        let token = interpreted.downcast_ref::<scanner::scan::Token>();
                        match token {
                            Some(token_val) => {
                                println!("{}", token_val.to_string());
                            }
                            None => {
                                let stmt = interpreted.downcast_ref::<stmt::stmt::Stmt>();
                                match stmt {
                                    Some(stmt_val) => {
                                        println!("{}", stmt_val.to_string());
                                    }
                                    None => {
                                        let expr = interpreted.downcast_ref::<expr::expr::Expr>();
                                        match expr {
                                            Some(expr_val) => {
                                                println!("{}", expr_val.to_string());
                                            }
                                            None => {
                                                let string = interpreted.downcast_ref::<String>();
                                                match string {
                                                    Some(rlox_class_val) => {
                                                        println!("{}", rlox_class_val.to_string());
                                                    }
                                                    None => {
                                                        let rlox_class =
                                                            interpreted.downcast_ref::<RLoxClass>();
                                                        match rlox_class {
                                                            Some(rlox_class_val) => {
                                                                println!(
                                                                    "{}",
                                                                    rlox_class_val.to_string()
                                                                );
                                                            }
                                                            None => {
                                                                let rlox_func = interpreted
                                                                    .downcast_ref::<RLoxFunction>(
                                                                );
                                                                match rlox_func {
                                                                    Some(rlox_func_val) => {
                                                                        println!(
                                                                            "{}",
                                                                            rlox_func_val
                                                                                .to_string()
                                                                        );
                                                                    }
                                                                    None => {
                                                                        error!("Could not downcast to any type(Token, Stmt, Expr, String, RLoxClass, RLoxFunction)");
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        }
        Err(err) => {
            error!("{}", err);
        }
    }
}

fn run_file(path: String) {
    if Path::new(&path.clone()).exists() {
        run(path);
    } else {
        panic!("Given path: {}, does not exist!!", path);
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("[run_prompt] Failed to read line");
        run(input);
    }
}

fn main() {
    env_logger::init();

    // delete old generated files
    if !utils::utils::clean_folder(utils::utils::GENERATED_FOLDER_PATH) {
        error!("Could not clean generated folder");
    }

    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(args[1].clone()),
        _ => println!("Usage: rust-lox [script_path]"),
    }
}
