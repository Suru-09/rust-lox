pub mod error_handling;
pub mod scanner;


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
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("[run_prompt] Failed to read line");
        run(input);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(args[1].clone()),
        _ => println!("Usage: rust-lox [script_path]"),
    }
}
