pub mod utils {
    use std::env::current_dir;
    use std::fs::remove_dir_all;

    pub const GENERATED_FOLDER_PATH: &str = "rust-lox/src/resources/generated/ast/";

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
                    if (*last == "f" || *last == "{{closure}}") && parts.len() > 2 {
                        // ignore "f" which is artifially added.
                        let last_two_before: Vec<&str> =
                            parts.iter().rev().skip(1).take(2).rev().copied().collect();
                        Some(last_two_before.join("::"))
                    } else if parts.len() == 2 {
                        // take exactly the last 2
                        let last_two: Vec<&str> =
                            parts.iter().rev().take(2).rev().copied().collect();
                        Some(last_two.join("::"))
                    } else {
                        None
                    }
                }
                None => None,
            }
        }};
    }
}
