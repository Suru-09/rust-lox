pub mod utils {

    use std::env::current_dir;
    use std::fs::remove_dir_all;

    pub const GENERATED_FOLDER_PATH: &str = "/src/resources/generated/ast/";

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
            Some(type_name_of(f).to_string())
        }};
    }
}
