use std::env;
use std::fs;
use std::path::Path;
use std::io::ErrorKind;
use std::time::UNIX_EPOCH;
use std::path::MAIN_SEPARATOR_STR;

pub const TYPE_FILE:&str = "file";
pub const TYPE_DIR:&str = "dir";
//pub const TYPE_UNKNOWN:&str = "Unknown";

pub struct FileDirInfo {
    pub index: u16,
    pub base_path: String,
    pub name: String,
    pub file_dir_type: String,
    pub size_in_bytes: u128,
    pub last_modified_time: u128,
    pub is_symlink: bool,
//    pub symlink_file_exists: bool,
}


pub fn filesystem_init(){
    println!("FileSystem Module Init");
}


pub fn get_path_seperator() -> &'static str{
    return MAIN_SEPARATOR_STR;
}


pub fn get_current_path() -> String {

    let current_dir = env::current_dir();
    match current_dir{
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}


pub fn get_list_of_files_and_directories(required_path: String) -> Vec<FileDirInfo>{
    
    let mut file_dir_list_struct: Vec<FileDirInfo> = Vec::new();
    let mut index: u16 = 0;   
    
    let path = Path::new(required_path.as_str());

    if path.try_exists().unwrap() == false {
        println!("Path Does not Exist");
        return file_dir_list_struct; 
    }

    let is_dir = path.is_dir();
    
    if is_dir {
        
        let base_path = path.canonicalize().unwrap();
        println!("{:?}", base_path);
     
        let cur_dir_wrapped = fs::read_dir(required_path);
        
        match cur_dir_wrapped {
           
            Ok(_) => {

                let cur_dir = cur_dir_wrapped.unwrap();

                for each_path in cur_dir {

                    let full_path: String = each_path.unwrap().path().display().to_string();

                    let symlink_metadata = fs::symlink_metadata(full_path.clone()).unwrap();
                    let metadata = fs::metadata(full_path.clone()).unwrap();
                    
                    let mut is_symlink = false;
                    if symlink_metadata.file_type().is_symlink() {
                        is_symlink = true;
                    }
                    
                    let splited_path: Vec<&str> = full_path.split("/").collect();
                    let name: &str = splited_path[splited_path.len() - 1];
                    
                    let file_type:&str;

                    if metadata.file_type().is_dir(){
                        file_type = TYPE_DIR;
                    } else {
                        file_type = TYPE_FILE;
                    }
                    
                    let file_dir_size = metadata.len();
                    let last_modified = metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();

                    file_dir_list_struct.push(build_file_dir_info(index, base_path.display().to_string(), name.to_string(), file_type.to_string(), is_symlink, file_dir_size.into(), last_modified.into()));
                    /*
                    println!("-> {} Path: {}, Name: {}, Type: {}, size: {}, Last Modified: {:?}", 
                                        file_dir_list_struct.index, 
                                        file_dir_list_struct.base_path, 
                                        file_dir_list_struct.name, 
                                        file_dir_list_struct.file_dir_type,
                                        file_dir_list_struct.size_in_bytes, 
                                        file_dir_list_struct.last_modified_time,
                            );
                    */
                    index = index + 1;
                 }

            },

            Err(ref error) => {
                println!("{:?}", error)
            },

        }

    } else {

        let full_path = path.canonicalize().unwrap().display().to_string();
        let base_path = Path::new(&full_path).parent().unwrap().display();
       
        let metadata = fs::metadata(full_path.clone()).unwrap();
        
        let mut is_symlink = false;
        if path.is_symlink() {
            is_symlink = true;
        }
        
        let splited_path: Vec<&str> = full_path.split("/").collect();
        let name: &str = splited_path[splited_path.len() - 1];
        let file_type: &str = TYPE_FILE;
        
        let file_dir_size = metadata.len();
        let last_modified = metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();

        file_dir_list_struct.push(build_file_dir_info(index, base_path.to_string(), name.to_string(), file_type.to_string(), is_symlink, file_dir_size.into(), last_modified.into()));
        index = index + 1;

    }

    return file_dir_list_struct;

}


pub fn build_file_dir_info(index: u16, base_path: String, name: String, file_dir_type: String, is_symlink_: bool, size_in_bytes: u128, last_modified: u128) -> FileDirInfo {
    FileDirInfo {
        index : index,
        base_path : base_path.to_string(),
        name : name.to_string(),
        file_dir_type : file_dir_type,
        size_in_bytes : size_in_bytes,
        last_modified_time : last_modified,
        is_symlink: is_symlink_,
    }
}


