use std::process::Command;
use crate::systems::System;
use crate::systems::arch::Arch;
use crate::systems::fedora::Fedora;
use crate::systems::macos::MacOS;
use crate::systems::ubuntu::Ubuntu;
use crate::systems::void::Void;

pub fn which(command: &str) -> String {
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