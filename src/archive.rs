use std::process::{Command, ExitCode, Stdio};
use serde::Serialize;
use crate::command_args::{ArchiveCreateCommandArgs, ArchiveExtractCommandArgs};
use crate::functions::which;

fn create_zip(args: &ArchiveCreateCommandArgs) -> ExitCode {
    let zip_path = which("zip");
    
    if zip_path.is_empty() {
        panic!("zip not found! Please install before attempting to create a zip archive.");
    }

    let mut zip_args: Vec<&str> = Vec::new();

    let dst = format!("{}/{}", args.dst_path, args.file_name);

    zip_args.push("-r");
    zip_args.push(&*dst);
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

fn create_rar() {
    let uz_path = which("rar");

    if uz_path.is_empty() {
        panic!("rar not found! Please install before attempting to extract a zip archive.");
    }
}

fn extract_rar() {
    let uz_path = which("unrar");

    if uz_path.is_empty() {
        panic!("unrar not found! Please install before attempting to extract a zip archive.");
    }
}

fn create_7z() {
    
}

fn extract_7z() {

}

fn create_tar() {
    
}

fn extract_tar() {

}

pub fn extract(args: &ArchiveExtractCommandArgs) -> ExitCode {

    extract_zip(args)

    //return ExitCode::SUCCESS
}

pub fn create(args: &ArchiveCreateCommandArgs) -> ExitCode {

    match args.archive_type {
        ArchiveType::Zip => {
            create_zip(args)
        },
        _ => ExitCode::SUCCESS
    }

    //return ExitCode::SUCCESS;
}

#[derive(Debug, clap::ValueEnum, Clone, Serialize)]
pub enum ArchiveType {
    Zip,
    Rar,
    P7Zip,
    Tar,
    TarGz,
    TarBz2
}