


// App Config
const APP_INFO: (&str, f32, &str) = ("FileManager", 0.1, "Alpha");
const DEBUG: bool = true;

// Parameters
const CACHE_FOLDER: &str = ".FileManagerCache";
const MAX_FILES_DIRS_PER_DIR: u16 = 65000;
const DIR_LIST_HEADER_LIST: [&str; 3] = ["Type", "Name", "Size"];

const MAX_APP_CACHE_SIZE_IN_MB: u128 = 1000;


struct FileDirInfo {
    index: u16,
    base_path: String,
    name: String,
    file_dir_type: u8,
    size_in_bytes: u128,
    last_modified_time: u128,
}


fn build_file_dir_info(index: u16, base_path: String, name: String, file_dir_type: u8, size_in_bytes: u128, last_modified: u128) -> FileDirInfo {
    FileDirInfo {
        index : index,
        base_path : base_path.to_string(),
        name : name.to_string(),
        file_dir_type : file_dir_type,
        size_in_bytes : size_in_bytes,
        last_modified_time : last_modified,
    }
}


fn display_files_and_directories(filedirinfo: &FileDirInfo){

    println!("index: {}, base path: {}, name: {}, type: {}, size: {}, last modified: {}", 
        filedirinfo.index, filedirinfo.base_path, filedirinfo.name, filedirinfo.file_dir_type,
        filedirinfo.size_in_bytes, filedirinfo.last_modified_time);

}

fn main() {

    let mut currentfilesanddirspath: Vec<FileDirInfo> = Vec::new();

    println!("\n\n{} Ver {} {}", APP_INFO.0, APP_INFO.1, APP_INFO.2);


    if DEBUG {
        println!("\nParameters:");
        println!("\tMax Cache Size: {} MB", MAX_APP_CACHE_SIZE_IN_MB);
        println!("\tCache File: {}", CACHE_FOLDER);
        println!("\tMax Files/Dirs Per Dir: {}", MAX_FILES_DIRS_PER_DIR);
        println!("\tHeader List: {:?}", DIR_LIST_HEADER_LIST);
    }

    currentfilesanddirspath.push(build_file_dir_info(1, "/home/user/".to_string(), "test.txt".to_string(), 1, 10, 12345678));

    println!("\n\nFiles and Directories ->");

    let filedirinfo: Option<&FileDirInfo> = currentfilesanddirspath.get(0);
    match filedirinfo {
        Some(filedirinfo) => display_files_and_directories(filedirinfo),
        None => println!("No File or Directory to Display"),
    }

    println!("\n");


}

