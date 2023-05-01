use std::process::Command;
use crate::systems::macos::MacOS;
use crate::systems::ubuntu::Ubuntu;

pub mod macos;
pub mod ubuntu;

pub trait System {
    fn refresh(&self);
    fn upgrade(&self);
}

pub fn detect_system(show_output: bool) ->  Box<dyn System> {

    let uname_result = Command::new("uname")
        .arg("-a")
        .output();

    match uname_result {
        Ok(output) => {

            let output_text_result = String::from_utf8(output.stdout.to_ascii_lowercase());

            match output_text_result {
                Ok(lowercase_text) => {
                    match lowercase_text {
                        x if x.contains("darwin") => return Box::new(MacOS { show_output, ..Default::default() }),
                        x if x.contains("ubuntu") => return Box::new(Ubuntu { show_output, ..Default::default() }),
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