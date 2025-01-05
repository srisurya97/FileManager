use std::env;
use std::fs;

pub struct FileDirInfo {
    pub index: u16,
    pub base_path: String,
    pub name: String,
    pub file_dir_type: u8,
    pub size_in_bytes: u128,
    pub last_modified_time: u128,
}


pub fn filesystem_init(){
    println!("FileSystem Module Init");
}


pub fn get_current_path() -> String {

    let current_dir = env::current_dir();
    match current_dir{
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

pub fn list_files_and_directories(required_path: String) -> Result<()>{
    
    let paths = fs::read_dir(required_path).unwrap();
    match paths {
        Ok(paths) => paths,
        Err(_) => "FAILED".to_string(),
    };

   // for each_path in paths {
   //     println!("-> {}", each_path.unwrap().path().display());
   // }
}

pub fn build_file_dir_info(index: u16, base_path: String, name: String, file_dir_type: u8, size_in_bytes: u128, last_modified: u128) -> FileDirInfo {
    FileDirInfo {
        index : index,
        base_path : base_path.to_string(),
        name : name.to_string(),
        file_dir_type : file_dir_type,
        size_in_bytes : size_in_bytes,
        last_modified_time : last_modified,
    }
}


