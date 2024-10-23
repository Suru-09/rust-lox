pub mod common {

    use std::ffi::OsStr;
    use walkdir::WalkDir;

    pub const INTEGRATION_TESTS_PATH: &str = "tests/resources/integration_tests";

    pub fn print_all_files(path: &str) {
        for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
            if let Ok(metadata) = file.metadata() {
                if metadata.is_file() {
                    let path = file.path();
                    if let Some(extension) = path.extension().and_then(OsStr::to_str) {
                        match extension {
                            "lox" => println!("{}", path.display()),
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}
