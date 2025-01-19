mod filesystem;
mod cli;
mod config;
mod version;
mod errors;


fn main() {

    let app_info = version::APP_INFO;
    println!("\n\n{} Ver {} {}", app_info.0, app_info.1, app_info.2);
    
    if config::DEBUG {
        config::display_parameters();
    }

    
    filesystem::filesystem_init();
    cli::cli_init();


    cli::cli_run();

    let current_dir_path:String = filesystem::get_current_path();
    let files_and_directories = filesystem::get_list_of_files_and_directories(current_dir_path);


    println!("\n\nFiles and Directories ->");
    for each in files_and_directories {
        let filedirinfo: filesystem::FileDirInfo = each;
        cli::display_files_and_directories(&filedirinfo)
    }

    println!("\n");


}

