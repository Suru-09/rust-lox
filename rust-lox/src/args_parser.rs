pub mod args_parser {
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
    pub struct Args {
        /// Path to the source you want to be interpreted
        #[clap(short, long, default_value_t = String::new())]
        pub src_path: String,

        /// Flag to enable generation of graphviz images with the AST.
        #[clap(short, long, default_value_t = false)]
        pub graphviz: bool,

        /// Similar flag to the graphviz AST images flag, however it is in the CLI.
        #[clap(short, long, default_value_t = false)]
        pub cli_graph: bool,
    }
}
