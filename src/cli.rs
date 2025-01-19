use std::io;
use std::io::Write;

use crate::filesystem;
use crate::errors;

pub const SUPPORTED_COMMANDS: [&str; 3]  = [ "exit", "clear", "ls" ]; //, "cd", "rm", "mv", "cp" ];


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

pub fn display_folder_content(){

    let current_dir_path:String = filesystem::get_current_path();
    let files_and_directories = filesystem::get_list_of_files_and_directories(current_dir_path);
    for each in files_and_directories {
        let filedirinfo: filesystem::FileDirInfo = each;
        display_files_and_directories(&filedirinfo)
    }

}


pub fn error_handle(_err: u8){
}


pub fn process_cmd(cmd_to_process: String) -> u8 {

    if cmd_to_process.trim() == "".to_string() {
        return errors::ERROR_SUCCESS;
    }

    const exit_cmd:&str = SUPPORTED_COMMANDS[0];
    const clear_cmd:&str = SUPPORTED_COMMANDS[1];
    const ls_cmd:&str = SUPPORTED_COMMANDS[2];


    match cmd_to_process.trim().as_ref() {
        ls_cmd=>{
            display_folder_content();
        },
        clear_cmd=>{
        },
        exit_cmd=>{
        },
        _=> {
            return errors::ERROR_FAILED;
        },
    };


    return errors::ERROR_SUCCESS;
}


pub fn cli_init() {

    println!("CLI Init\r\n");
    println!("Supported Commands: {:?}", SUPPORTED_COMMANDS);
    
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
        
        if process_cmd(cmd.clone()) != errors::ERROR_SUCCESS {
            println!("Unsupported Operation");
        }
        
        cmd.clear();
    }

}


