use std::fs;
use std::fs::DirEntry;
use std::process::ExitCode;
use colored::Colorize;
use crate::command_args::{FindDirectoryCommandArgs};

pub fn find_dir(command_args: &FindDirectoryCommandArgs) -> ExitCode {

    let found_items = &mut Vec::new();

    traverse(command_args, found_items);

    if found_items.is_empty() {
        return ExitCode::FAILURE;
    }

    return ExitCode::SUCCESS;

}

fn traverse(command_args: &FindDirectoryCommandArgs, found_items: &mut Vec<String>) {

    rayon::scope(|scope| {
        scope.spawn(|_| {

            let contents = fs::read_dir(&command_args.path).into_iter();

            for dir_content in contents {
                rayon::scope(|dcs| {
                    dcs.spawn(|_| {

                        let dir_items = dir_content.into_iter();

                        for item in dir_items {
                            rayon::scope(|is| {
                                is.spawn(|_| {

                                    match item {
                                        Ok(item) => {

                                            if !command_args.hidden {
                                                match item.file_name().to_str() {
                                                    Some(name) => {
                                                        if name.starts_with(".") {
                                                            return;
                                                        }
                                                    }
                                                    None => {

                                                    }
                                                }
                                            }

                                            match item.file_type() {
                                                Ok(file_type) => {
                                                    // Item type is a file
                                                    if file_type.is_file() {
                                                        return;
                                                    }
                                                    // Item type is a directory
                                                    else if file_type.is_dir() {
                                                        proc_dir(item, &command_args, found_items);
                                                    }
                                                    // Item type is a symlink
                                                    else if file_type.is_symlink() && command_args.follow_symlinks {
                                                        if item.path().is_file() {
                                                            return;
                                                        }
                                                        else if item.path().is_dir() {
                                                            proc_dir(item, &command_args, found_items);
                                                        }
                                                    }
                                                }
                                                Err(why) => {
                                                    panic!("There was an error finding the file!\n\n{}", why)
                                                }
                                            }
                                        }
                                        Err(why) => {
                                            panic!("There was an error finding the file!\n\n{}", why)
                                        }
                                    }

                                });
                            });
                        }

                    });
                });
            }
        });
    });
}

fn proc_dir(entry: DirEntry, current_args: &FindDirectoryCommandArgs, found_items: &mut Vec<String>) {
    if dir_name_lowercase(&entry).contains(&current_args.directory_name.to_lowercase()) {
        println!("{} {}", "FOUND!".bold().yellow(), entry.path().to_str().unwrap().to_string());
        found_items.push(entry.path().to_str().unwrap().to_string())
    }

    let new_args = FindDirectoryCommandArgs {
        directory_name: current_args.directory_name.to_string(),
        path: entry.path().to_str().unwrap().to_string(),
        hidden: current_args.hidden,
        follow_symlinks: current_args.follow_symlinks
    };
    traverse(&new_args, found_items);
}

fn dir_name_lowercase(entry: &DirEntry) -> String {
    match entry.path().to_str()
    {
        Some(name) => {
            return name.to_string().to_lowercase();
        }
        None => {
            panic!("There was an error parsing the file name!")
        }
    }
}