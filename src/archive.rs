use std::process::{Command, ExitCode, Stdio};
use std::path::Path;
use crate::command_args::{ArchiveCreateCommandArgs, ArchiveExtractCommandArgs};
use crate::functions::which;

fn create_zip(args: &ArchiveCreateCommandArgs) -> ExitCode {
    let zip_path = which("zip");
    
    if zip_path.is_empty() {
        panic!("zip not found! Please install before attempting to create a zip archive.");
    }

    let mut zip_args: Vec<&str> = Vec::new();

    zip_args.push("-r");
    zip_args.push(&*args.dst_path);
    zip_args.push(&*args.src_path);

    let mut zip = Command::new(zip_path);
    zip.args(&zip_args);

    zip
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let zip_result = zip.output();

    match zip_result {
        Ok(output) => {
            match output.status.code() {
                Some(code) => ExitCode::from(code as u8),
                None => ExitCode::FAILURE
            }
        }
        Err(why) => {
            panic!("There was an issue running the zip command!\n\n{:?}", why);
        }
    }
    
}

fn extract_zip(args: &ArchiveExtractCommandArgs) -> ExitCode {
    let uz_path = which("unzip");
    
    if uz_path.is_empty() {
        panic!("unzip not found! Please install before attempting to extract a zip archive.");
    }

    let mut uz_args: Vec<&str> = Vec::new();

    uz_args.push(&*args.src_path);
    uz_args.push("-d");
    uz_args.push(&*args.dst_path);

    let mut unzip = Command::new(uz_path);
    unzip.args(&uz_args);

    unzip
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let unzip_result = unzip.output();

    match unzip_result {
        Ok(output) => {
            match output.status.code() {
                Some(code) => ExitCode::from(code as u8),
                None => ExitCode::FAILURE
            }
        }
        Err(why) => {
            panic!("There was an issue running the zip command!\n\n{:?}", why);
        }
    }
}

fn create_rar(args: &ArchiveCreateCommandArgs) -> ExitCode {
    let rar_path = which("rar");

    if rar_path.is_empty() {
        panic!("rar not found! Please install before attempting to extract a rar archive.");
    }

    let mut rar_args: Vec<&str> = Vec::new();

    rar_args.push("a");
    rar_args.push(&*args.dst_path);
    rar_args.push(&*args.src_path);

    let mut rar = Command::new(rar_path);
    rar.args(&rar_args);

    rar
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let rar_result = rar.output();

    match rar_result {
        Ok(output) => {
            match output.status.code() {
                Some(code) => ExitCode::from(code as u8),
                None => ExitCode::FAILURE
            }
        }
        Err(why) => {
            panic!("There was an issue running the rar command!\n\n{:?}", why);
        }
    }
}

fn extract_rar(args: &ArchiveExtractCommandArgs) -> ExitCode {
    let unrar_path = which("unrar");

    if unrar_path.is_empty() {
        panic!("unrar not found! Please install before attempting to extract a rar archive.");
    }

    let mut unrar_args: Vec<&str> = Vec::new();

    unrar_args.push("x");
    unrar_args.push(&*args.src_path);
    unrar_args.push(&*args.dst_path);

    let mut unrar = Command::new(unrar_path);
    unrar.args(&unrar_args);

    unrar
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let unrar_result = unrar.output();

    match unrar_result {
        Ok(output) => {
            match output.status.code() {
                Some(code) => ExitCode::from(code as u8),
                None => ExitCode::FAILURE
            }
        }
        Err(why) => {
            panic!("There was an issue running the rar command!\n\n{:?}", why);
        }
    }
}

fn create_7z(args: &ArchiveCreateCommandArgs) -> ExitCode {
    let z7_path = which("7zip");

    if z7_path.is_empty() {
        panic!("7zip not found! Please install before attempting to extract a 7zip archive.");
    }

    let mut z7_args: Vec<&str> = Vec::new();

    z7_args.push("a");
    z7_args.push(&*args.dst_path);
    z7_args.push(&*args.src_path);

    let mut z7 = Command::new(z7_path);
    z7.args(&z7_args);

    z7
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let z7_result = z7.output();

    match z7_result {
        Ok(output) => {
            match output.status.code() {
                Some(code) => ExitCode::from(code as u8),
                None => ExitCode::FAILURE
            }
        }
        Err(why) => {
            panic!("There was an issue running the 7zip command!\n\n{:?}", why);
        }
    }
}

fn extract_7z(args: &ArchiveExtractCommandArgs) -> ExitCode {

    let z7_path = which("7zip");

    if z7_path.is_empty() {
        panic!("7zip not found! Please install before attempting to extract a 7zip archive.");
    }

    let mut z7_args: Vec<&str> = Vec::new();
    let dst_path = &*["-o", &*args.dst_path].join("");

    z7_args.push("x");
    z7_args.push(&*args.src_path);
    z7_args.push(dst_path);


    let mut z7 = Command::new(z7_path);
    z7.args(&z7_args);

    z7
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let z7_result = z7.output();

    match z7_result {
        Ok(output) => {
            match output.status.code() {
                Some(code) => ExitCode::from(code as u8),
                None => ExitCode::FAILURE
            }
        }
        Err(why) => {
            panic!("There was an issue running the 7zip command!\n\n{:?}", why);
        }
    }
}

fn create_tar(args: &ArchiveCreateCommandArgs, compress_bz2: bool, compress_gz: bool) -> ExitCode {

    ExitCode::SUCCESS
}

fn extract_tar(args: &ArchiveExtractCommandArgs) -> ExitCode {

    ExitCode::SUCCESS
}

pub fn extract(args: &ArchiveExtractCommandArgs) -> ExitCode {

    let src_path = Path::new(&args.src_path);

    let filename_opt = src_path.file_name();

    let filename = match filename_opt {
        None => panic!("Unable to get filename from source path!"),
        Some(filename) => {
            match filename.to_str() {
                None => panic!("Unable to convert filename to string!"),
                Some(filename) => filename
            }
        }
    };

    if filename.to_lowercase().contains(".zip") {
        extract_zip(args)
    }
    else if filename.to_lowercase().contains(".rar") {
        extract_rar(args)
    }
    else if filename.to_lowercase().contains(".7z") {
        extract_7z(args)
    }
    else if filename.to_lowercase().contains(".tar") {
        extract_tar(args)
    }
    else {
        ExitCode::FAILURE
    }
}

pub fn create(args: &ArchiveCreateCommandArgs) -> ExitCode {

    let dst_path = Path::new(&args.dst_path);

    let filename_opt = dst_path.file_name();

    let filename = match filename_opt {
        None => panic!("Unable to get filename from destination path!"),
        Some(filename) => {
            match filename.to_str() {
                None => panic!("Unable to convert filename to string!"),
                Some(filename) => filename
            }
        }
    };

    if filename.to_lowercase().contains(".zip") {
        create_zip(args)
    }
    else if filename.to_lowercase().contains(".rar") {
        create_rar(args)
    }
    else if filename.to_lowercase().contains(".7z") {
        create_7z(args)
    }
    else if filename.to_lowercase().contains(".tar") {
        create_tar(args, false, false)
    }
    else if filename.to_lowercase().contains(".tar.gz") {
        create_tar(args, false, true)
    }
    else if filename.to_lowercase().contains(".tar.bz2") {
        create_tar(args, true, false)
    }
    else {
        ExitCode::FAILURE
    }
}