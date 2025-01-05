use crate::filesystem;


pub fn display_files_and_directories(filedirinfo: &filesystem::FileDirInfo){

    println!("index: {}, base path: {}, name: {}, type: {}, size: {}, last modified: {}", 
        filedirinfo.index, filedirinfo.base_path, filedirinfo.name, filedirinfo.file_dir_type,
        filedirinfo.size_in_bytes, filedirinfo.last_modified_time);

}



