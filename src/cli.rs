use std::io;
use std::io::Write;

use crate::filesystem;
use crate::errors;

pub const SUPPORTED_COMMANDS: [&str; 5]  = [ "help", "exit", "clear", "ls", "cd" ]; //, "cd", "rm", "mv", "cp" ];


struct CLI_ {
   pub current_path: String,
}


impl CLI_ {

    pub fn set_current_path(current_path: String) -> Self {
        Self {
            current_path: current_path,
        }
    }

    pub fn get_current_path(&self) -> String {
        return self.current_path.clone();
    }

}


pub fn display_files_and_directories(filedirinfo: &filesystem::FileDirInfo){

    println!("index: {}, base path: {}, name: {}, type: {}, size: {}, last modified: {}", 
        filedirinfo.index, filedirinfo.base_path, filedirinfo.name, filedirinfo.file_dir_type,
        filedirinfo.size_in_bytes, filedirinfo.last_modified_time);

}

pub fn change_directory(cmd_: Vec<&str>) -> u8{

        return errors::ERROR_SUCCESS;

}

pub fn display_folder_content(cmd_: Vec<&str>) -> u8{
    
    let mut dir_path: String = String::new();

    if (cmd_.len() == 1) || ( (cmd_.len() == 2) && (cmd_[1].trim() == ".".to_string()) )  {
    
        dir_path = filesystem::get_current_path();
        let files_and_directories = filesystem::get_list_of_files_and_directories(dir_path);
        for each in files_and_directories {
            let filedirinfo: filesystem::FileDirInfo = each;
            display_files_and_directories(&filedirinfo)
        }

        return errors::ERROR_SUCCESS;

    } else {

        let path_separator: &str = filesystem::get_path_seperator();
        let path_ = cmd_[1]; 
        let folder_path: Vec<&str> = path_.trim().split(path_separator).collect();
        println!("{:?} {:?}", folder_path, path_);
        
        if folder_path[0] == "" { 

            let files_and_directories = filesystem::get_list_of_files_and_directories(filesystem::get_path_seperator().to_string());
            for each in files_and_directories {
                let filedirinfo: filesystem::FileDirInfo = each;
                display_files_and_directories(&filedirinfo)
            }
        
        } else {

            dir_path = filesystem::get_current_path();
            let mut final_path: String = dir_path.clone();
            final_path.push_str(path_separator);
            final_path.push_str(path_);
            println!("Path: {:?}", final_path);
            let files_and_directories = filesystem::get_list_of_files_and_directories(final_path);
            for each in files_and_directories {
                let filedirinfo: filesystem::FileDirInfo = each;
                display_files_and_directories(&filedirinfo)
            }

        }

        return errors::ERROR_SUCCESS;

    }

}

pub fn display_help(){
    println!("\nSuppported Operations:\n\t{:?}\n",SUPPORTED_COMMANDS);
}

pub fn error_handle(_err: u8){

    match _err {
        errors::ERROR_UNSUPPORTED=> {
            println!("Unsupported Operation");
        },
        errors::ERROR_FAILED=> {
             println!("Failed to Execute");
        },
        errors::ERROR_SUCCESS=> {
        },
        _=> {
            println!("Unknown Error");
        },
    };

}


pub fn process_cmd(cmd_to_process_: String) -> u8 {

    if cmd_to_process_.trim() == "".to_string() {
        return errors::ERROR_SUCCESS;
    }

    const HELP_CMD:&str = SUPPORTED_COMMANDS[0];
    const EXIT_CMD:&str = SUPPORTED_COMMANDS[1];
    const CLEAR_CMD:&str = SUPPORTED_COMMANDS[2];
    const LS_CMD:&str = SUPPORTED_COMMANDS[3];
    const CD_CMD:&str = SUPPORTED_COMMANDS[4];

    let cmd_to_process: Vec<&str> = cmd_to_process_.split_whitespace().collect();

    match cmd_to_process[0].trim().as_ref() {
        LS_CMD=>{
            return display_folder_content(cmd_to_process);
        },
        CD_CMD=>{
            return change_directory(cmd_to_process);
        },
        CLEAR_CMD=>{
        },
        HELP_CMD=>{
            display_help();
        },
        EXIT_CMD=>{
        },
        _=> {
            return errors::ERROR_UNSUPPORTED;
        },
    };

    return errors::ERROR_SUCCESS;
}


pub fn cli_init() {

    println!("CLI Init\r\n");
    println!("Type \"help\" for options.");
    
    let cli_ = CLI_{
      current_path: filesystem::get_current_path(),  
    };

    //CLI_::set_current_path(filesystem::get_current_path().clone());
    println!("Current Path: {}", cli_.get_current_path());

}


pub fn cli_run() {
    let mut cmd = String::new();

    loop {
        
        print!("FM$ ");
        io::stdout().flush().unwrap();
        
        let _ = io::stdin().read_line(&mut cmd);
        error_handle(process_cmd(cmd.clone()));
        
        cmd.clear();
    }

}


