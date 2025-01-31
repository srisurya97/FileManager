use crate::filesystem;


pub const DEBUG: bool = true;


// Parameters
const CACHE_FOLDER: &str = ".FileManagerCache";
const MAX_FILES_DIRS_PER_DIR: u16 = 65000;
const DIR_LIST_HEADER_LIST: [&str; 3] = ["Type", "Name", "Size"];

const MAX_APP_CACHE_SIZE_IN_MB: u128 = 1000;



pub fn display_parameters() {

    println!("\nParameters:");
    println!("\tMax Cache Size: {} MB", MAX_APP_CACHE_SIZE_IN_MB);
    println!("\tCache File: {}", CACHE_FOLDER);
    println!("\tMax Files/Dirs Per Dir: {}", MAX_FILES_DIRS_PER_DIR);
    println!("\tHeader List: {:?}", DIR_LIST_HEADER_LIST);
    println!("\tFile System Path {:?}", filesystem::get_path_seperator());
    println!("\n");

}
