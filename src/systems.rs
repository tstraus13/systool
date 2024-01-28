use std::process::{Command, ExitCode};
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
    fn refresh(&self) -> ExitCode;
    fn upgrade(&self) -> ExitCode;
    fn package_search(&self, pkg_name: String) -> ExitCode;
    fn package_info(&self, pkg_name: String) -> ExitCode;
    fn package_install(&self, pkg_name: String) -> ExitCode;
    fn package_remove(&self, pkg_name: String) -> ExitCode;
}

pub fn detect_system(show_output: bool, force: bool) ->  Box<dyn System> {

    // Use lsb_release to get system info
    let lsb_release_result = Command::new("lsb_release")
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
                            return Box::new(MacOS
                            {
                                show_output,
                                force,
                                ..Default::default()
                            })
                        },
                        x if x.contains("ubuntu") || x.contains("debian") => {
                            println!("Ubuntu/Debian Detected.");
                            return Box::new(Ubuntu
                            {
                                show_output,
                                force,
                                ..Default::default()
                            })
                        },
                        x if x.contains("fedora") => {
                            println!("Fedora Detected!");
                            return Box::new(Fedora
                            {
                                show_output,
                                force,
                                ..Default::default()
                            })
                        },
                        x if x.contains("voidlinux") => {
                            println!("Void Detected!");
                            return Box::new(Void
                            {
                                show_output,
                                force,
                                ..Default::default()
                            })
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

    // Check for system type by uname command
    let uname_result = Command::new("uname")
        .arg("-a")
        .output()
        ;

    match uname_result {
        Ok(output) => {

            let output_text_result = String::from_utf8(output.stdout.to_ascii_lowercase());

            match output_text_result {
                Ok(lowercase_text) => {
                    match lowercase_text {
                        x if x.contains("darwin") => {
                            println!("macOS Detected.");
                            return Box::new(MacOS
                            {
                                show_output,
                                force,
                                ..Default::default()
                            })
                        },
                        x if x.contains("ubuntu") || x.contains("debian") => {
                            println!("Ubuntu/Debian Detected.");
                            return Box::new(Ubuntu
                            {
                                show_output,
                                force,
                                ..Default::default()
                            })
                        },
                        x if x.contains("fc38") || x.contains("fc37") => {
                            println!("Fedora Detected!");
                            return Box::new(Fedora
                            {
                                show_output,
                                force,
                                ..Default::default()
                            })
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