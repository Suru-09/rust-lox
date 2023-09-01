pub mod utils {


pub const GENERATED_FOLDER_PATH: &str = "/src/resources/generated/ast/";

pub fn clean_folder(path: &str) -> bool {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(why) => panic!("couldn't get current dir: {}", why),
    };

    let path_string = format!("{}{}", current_dir.display(), path);
    let path = std::path::Path::new(&path_string);
    if path.exists() {
        match std::fs::remove_dir_all(path) {
            Ok(_) => {},
            Err(_) => return false
        }
    }
    else {
        return false;
    }
    
    return true;
}


}