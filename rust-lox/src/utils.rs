pub mod utils {

    use chrono;
    use log::{Level, Metadata, Record};
    use std::env::current_dir;
    use std::fs::remove_dir_all;

    pub static LOGGER: SimpleLogger = SimpleLogger;
    pub const GENERATED_FOLDER_PATH: &str = "/src/resources/generated/ast/";

    pub struct SimpleLogger;

    impl log::Log for SimpleLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Info
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                println!(
                    "{} - {} - {}",
                    chrono::offset::Local::now(),
                    record.level(),
                    record.args()
                );
            }
        }

        fn flush(&self) {}
    }

    pub fn clean_folder(path: &str) -> bool {
        let current_directory = match current_dir() {
            Ok(dir) => dir,
            Err(why) => panic!("couldn't get current dir: {}", why),
        };

        let path_string = format!("{}{}", current_directory.display(), path);
        let path_final = std::path::Path::new(&path_string);
        if path_final.exists() {
            match remove_dir_all(path_final) {
                Ok(_) => {}
                Err(_) => return false,
            }
        }
        return true;
    }

    #[macro_export]
    macro_rules! function_name {
        () => {{
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let parts: Vec<&str> = type_name_of(f).split("::").collect();
            match parts.last() {
                Some(last) => {
                    if *last == "f" || *last == "{{closure}}" {
                        // Get the last two components before the last one
                        if parts.len() > 2 {
                            let last_two_before: Vec<&str> =
                                parts.iter().rev().skip(1).take(2).rev().copied().collect();
                            let result = last_two_before.join("::");
                            Some(result)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                None => None,
            }
        }};
    }
}
