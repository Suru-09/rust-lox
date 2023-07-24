use expr::expr::GraphVizPrinter;

pub mod error_handling;
pub mod scanner;
pub mod expr;


fn run(source: String) {
    let mut scanner = scanner::scan::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{}", token.to_string());
    }
}

fn run_file(path: String) {
    let source = std::fs::read_to_string(path).expect("[run_file] Something went wrong reading the file");
    run(source);
}

fn run_prompt() {
    // loop {
    //     print!("> ");
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).expect("[run_prompt] Failed to read line");
    //     run(input);
    // }
    let ast = expr::expr::build_test_ast();
    let mut graph_printer = GraphVizPrinter::new( String::from("test"));
    let graph = ast.accept(&mut graph_printer);
    graph_printer.close_graph();

    println!("{}", graph_printer.to_string());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(args[1].clone()),
        _ => println!("Usage: rust-lox [script_path]"),
    }
}
