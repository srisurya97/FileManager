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


}

