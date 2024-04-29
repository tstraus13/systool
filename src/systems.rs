use std::cell::RefCell;
use std::fs;
use std::fs::DirEntry;
use std::process::{Command, ExitCode};
use std::rc::Rc;
use std::sync::mpsc::channel;
use rayon::prelude::*;
use crate::command_args::*;
use crate::systems::fedora::Fedora;
use crate::systems::macos::MacOS;
use crate::systems::ubuntu::Ubuntu;
use crate::systems::arch::Arch;
use crate::systems::void::Void;

pub mod macos;
pub mod ubuntu;
pub mod fedora;
pub mod void;
pub mod arch;

pub trait System {
    fn refresh(&self, command_args:&RefreshCommandArgs) -> ExitCode;
    fn upgrade(&self, command_args:&UpgradeCommandArgs) -> ExitCode;
    fn package_search(&self, command_args:&PackageSearchCommandArgs) -> ExitCode;
    fn package_info(&self, command_args:&PackageInfoCommandArgs) -> ExitCode;
    fn package_install(&self, command_args:&PackageInstallCommandArgs) -> ExitCode;
    fn package_remove(&self, command_args:&PackageRemoveCommandArgs) -> ExitCode;
}

fn which(command: &str) -> String {
    let which_result = Command::new("which")
        .arg(command)
        .output();

    match which_result {
        Ok(output) => {
            let path_result = String::from_utf8(output.stdout);

            match path_result {
                Ok(path) => {
                    return path.trim().to_string();
                }
                Err(why) => {
                    panic!("There was an error parsing the output of the which command!\n\n{}", why);
                }
            }
        }
        Err(why) => {
            panic!("There was an error running the which command!\n\n{}", why);
        }
    }
}

pub fn find_file(command_args: &FindFileCommandArgs) -> ExitCode {

    fn file_name_lowercase(entry: &DirEntry) -> String {
        match entry.file_name().to_str()
        {
            Some(name) => {
                return name.to_string().to_lowercase();
            }
            None => {
                panic!("There was an error parsing the file name!")
            }
        }
    }

    fn proc_file(entry: DirEntry, args: &FindFileCommandArgs, found_items: &mut Vec<String>) {
        if file_name_lowercase(&entry).contains(&args.file_name.to_lowercase()) {
            println!("FOUND! {}", entry.path().to_str().unwrap().to_string());
            found_items.push(entry.path().to_str().unwrap().to_string())
        }
    }

    fn proc_dir(entry: DirEntry, current_args: &FindFileCommandArgs, found_items: &mut Vec<String>) {
        let new_args = FindFileCommandArgs {
            file_name: current_args.file_name.to_string(),
            path: entry.path().to_str().unwrap().to_string(),
            hidden: current_args.hidden,
            follow_symlinks: current_args.follow_symlinks
        };
        find(&new_args, found_items);
    }

    fn find(command_args: &FindFileCommandArgs, found_items: &mut Vec<String>) {

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

        /*rayon::scope(|z| {
            z.spawn(|| {
                fs::read_dir(&command_args.path).into_iter().for_each(|dir_content| {
                    rayon::scope(|s| {
                        s.spawn(|| {
                            dir_content.into_iter().for_each(|dir_item| {
                                rayon::scope(|r| {
                                    r.spawn(|| {
                                        match dir_item {
                                            Ok(item) => {
                                                if item.path().is_dir() {
                                                    let new_args = FindFileCommandArgs {
                                                        file_name: command_args.file_name.to_string(),
                                                        path: item.path().to_str().unwrap().to_string(),
                                                        hidden: command_args.hidden,
                                                        follow_symlinks: command_args.follow_symlinks
                                                    };
                                                    find(&new_args, found_items);
                                                }
                                                else {
                                                    match item.file_name().to_str()
                                                    {
                                                        Some(name) => {
                                                            if name.to_string().to_lowercase().contains(&command_args.file_name.to_lowercase()) {
                                                                println!("FOUND! {}", item.path().to_str().unwrap());
                                                                found_items.push(item.path().to_str().unwrap().to_string())
                                                            }
                                                        }
                                                        None => {
                                                            panic!("There was an error finding the file!")
                                                        }
                                                    }
                                                }
                                            }
                                            Err(why) => {
                                                panic!("There was an error finding the file!\n\n{}", why)
                                            }
                                        }
                                    });
                                });
                            });
                        });
                    });
                });
            });
        });*/
    }

    let found_items = &mut Vec::new();

    find(command_args, found_items);

    if found_items.is_empty() {
        return ExitCode::FAILURE;
    }

    return ExitCode::SUCCESS;
}

pub fn detect_system() ->  Box<dyn System> {

    // Check for system type by lsb_release command
    let lsb_release_path = which("lsb_release");

    if !String::is_empty(&lsb_release_path) {
        let lsb_release_result = Command::new(lsb_release_path)
            .arg("-a")
            .output();

        match lsb_release_result {
            Ok(output) => {

                let output_text_result = String::from_utf8(output.stdout.to_ascii_lowercase());

                match output_text_result {
                    Ok(lowercase_text) => {
                        match lowercase_text {
                            x if x.contains("darwin") => {
                                println!("macOS Detected.");
                                return Box::new(MacOS)
                            },
                            x if x.contains("ubuntu") || x.contains("debian") || x.contains("devuan") => {
                                println!("Ubuntu/Debian/Devuan Detected.");
                                return Box::new(Ubuntu)
                            },
                            x if x.contains("fedora") => {
                                println!("Fedora Detected!");
                                return Box::new(Fedora)
                            },
                            x if x.contains("voidlinux") => {
                                println!("Void Detected!");
                                return Box::new(Void)
                            }
                            ,
                            x if x.contains("arch") => {
                                println!("Void Detected!");
                                return Box::new(Arch)
                            }
                            _ => {}
                        }
                    }
                    Err(why) => {
                        panic!("There was an error getting the lsb_release command output!\n\n{}", why)
                    }
                }
            }
            Err(why) => {
                panic!("There was an error running the lsb_release command!\n\n{}", why)
            }
        }
    }

    // Check for system type by uname command
    let uname_path = which("uname");

    if !String::is_empty(&uname_path) {
        let uname_result = Command::new(uname_path)
            .arg("-a")
            .output();

        match uname_result {
            Ok(output) => {

                let output_text_result = String::from_utf8(output.stdout.to_ascii_lowercase());

                match output_text_result {
                    Ok(lowercase_text) => {
                        match lowercase_text {
                            x if x.contains("darwin") => {
                                println!("macOS Detected.");
                                return Box::new(MacOS)
                            },
                            x if x.contains("ubuntu") || x.contains("debian") => {
                                println!("Ubuntu/Debian Detected.");
                                return Box::new(Ubuntu)
                            },
                            x if x.contains("fc38") || x.contains("fc37") => {
                                println!("Fedora Detected!");
                                return Box::new(Fedora)
                            }
                            _ => panic!("Could not detect system! Exiting...")
                        }
                    }
                    Err(why) => {
                        panic!("There was an error getting the uname command output!\n\n{}", why)
                    }
                }
            }
            Err(why) => {
                panic!("There was an error running the uname command!\n\n{}", why)
            }
        }
    }

    panic!("Unable to detect system! Exiting...")
}