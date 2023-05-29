use std::process::{Command, ExitCode};
use crate::systems::fedora::Fedora;
use crate::systems::macos::MacOS;
use crate::systems::ubuntu::Ubuntu;

pub mod macos;
pub mod ubuntu;
pub mod fedora;

pub trait System {
    fn refresh(&self) -> ExitCode;
    fn upgrade(&self) -> ExitCode;
    fn package_search(&self, pkg_name: String) -> ExitCode;
    fn package_info(&self, pkg_name: String) -> ExitCode;
    fn package_install(&self, pkg_name: String) -> ExitCode;
    fn package_remove(&self, pkg_name: String) -> ExitCode;
}

pub fn detect_system(show_output: bool, force: bool) ->  Box<dyn System> {

    let uname_result = Command::new("uname")
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
                            return Box::new(MacOS
                            {
                                show_output,
                                force,
                                ..Default::default()
                            })
                        },
                        x if x.contains("ubuntu") => {
                            println!("Ubuntu Detected.");
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
                    panic!("There was an error getting the command output!\n\n{}", why)
                }
            }
        }
        Err(why) => {
            panic!("There was an error running the uname command!\n\n{}", why)
        }
    }

}