mod filesystem;
mod cli;
mod config;
mod version;



fn main() {

    let mut current_directory_files: Vec<filesystem::FileDirInfo> = Vec::new();

    let app_info = version::APP_INFO;
    println!("\n\n{} Ver {} {}", app_info.0, app_info.1, app_info.2);
    
    if config::DEBUG {
        config::display_parameters();
    }
    
    filesystem::filesystem_init();
    let current_dir_path:String = filesystem::get_current_path();
    
    println!("Current Dir: {}", current_dir_path);
    
    let files_and_directories = filesystem::list_files_and_directories(current_dir_path);


    current_directory_files.push(filesystem::build_file_dir_info(1, "/home/user/".to_string(), "test.txt".to_string(), 1, 10, 12345678));
    println!("\n\nFiles and Directories ->");

    let filedirinfo: Option<&filesystem::FileDirInfo> = current_directory_files.get(0);
    match filedirinfo {
        Some(filedirinfo) => cli::display_files_and_directories(filedirinfo),
        None => println!("No File or Directory to Display"),
    }

    println!("\n");


}

