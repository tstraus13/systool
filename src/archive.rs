use crate::functions::which;

pub fn create_zip() {
    let zip_path = which("zip");
    
    if zip_path.is_empty() {
        panic!("zip not found! Please install before attempting to create a zip archive.");
    }
    
    
}

fn extract_zip() {
    let uz_path = which("unzip");
    
    if uz_path.is_empty() {
        panic!("unzip not found! Please install before attempting to extract a zip archive.");
    }
}

pub fn create_rar() {

}

pub fn create_7z() {
    
}

pub fn create_tar() {
    
}

pub fn extract() {

}