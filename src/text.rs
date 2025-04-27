use crate::command_args::FindTextCommandArgs;
use colored::*;
use std::fs;
use std::fs::DirEntry;
use std::process::ExitCode;
use std::time::Instant;

pub fn find_text(command_args: &FindTextCommandArgs) -> ExitCode {
    let found_items = &mut Vec::new();

    traverse(command_args, found_items);

    if found_items.is_empty() {
        return ExitCode::FAILURE;
    }
    
    return ExitCode::SUCCESS;
}

fn traverse(command_args: &FindTextCommandArgs, found_items: &mut Vec<String>) {
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
                                                        proc_file(item, &command_args, found_items);
                                                    }
                                                    // Item type is a directory
                                                    else if file_type.is_dir() {
                                                        proc_dir(item, &command_args, found_items);
                                                    }
                                                    // Item type is a symlink
                                                    else if file_type.is_symlink() && command_args.follow_symlinks {
                                                        if item.path().is_file() {
                                                            proc_file(item, &command_args, found_items);
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

fn proc_file(entry: DirEntry, args: &FindTextCommandArgs, found_items: &mut Vec<String>) {
    let path_bind = entry.path();
    let path = path_bind.to_str();
    let text_lower = args.text.to_lowercase();
    let text_len = text_lower.len();

    match path {
        Some(path) => {
            let is_binary = binaryornot::is_binary(path).unwrap_or(true);

            if !is_binary {
                let contents = fs::read_to_string(path)
                    .unwrap_or_else(|_| String::from(""));

                let found = contents.to_lowercase().contains(&text_lower);

                if found {

                    println!("{} {}", "FOUND!".bold(), entry.path().to_str().unwrap().to_string());

                    found_items.push(entry.path().to_str().unwrap().to_string());

                    for (i, line) in contents.lines().enumerate() {
                        let line_lower = line.to_lowercase();

                        if line_lower.contains(&text_lower) {

                            let text_start = line_lower.find(&text_lower).unwrap();

                            let start = &line[..text_start];
                            let matched = &line[text_start..(text_start + text_len)]
                                .bold().yellow();
                            let end = &line[(text_start + text_len)..];

                            println!("\t{},{}: {}{}{}", (i + 1).to_string().bold(), (text_start + 1).to_string().bold(), start, matched, end);
                        }
                    }
                    
                    println!();
                }
            }
        }
        None => {}
    }
}

fn proc_dir(entry: DirEntry, current_args: &FindTextCommandArgs, found_items: &mut Vec<String>) {
    let new_args = FindTextCommandArgs {
        text: current_args.text.to_string(),
        path: entry.path().to_str().unwrap().to_string(),
        hidden: current_args.hidden,
        follow_symlinks: current_args.follow_symlinks,
    };
    traverse(&new_args, found_items);
}
