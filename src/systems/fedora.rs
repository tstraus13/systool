use std::process::{Command, ExitCode, Stdio};
use crate::systems::System;

impl Default for Fedora {
    fn default() -> Self {
        Self {
            show_output: false,
        }
    }
}

pub struct Fedora {
    pub show_output: bool,
}

impl System for Fedora {
    fn refresh(&self) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();

        args.push("/usr/bin/dnf");
        args.push("check-update");
        args.push("--refresh");

        let mut refresh = Command::new(&args[0]);
        refresh.args(&args[1..]);

        if self.show_output {
            refresh
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }

        let refresh_result = refresh.output();

        match refresh_result {
            Ok(output) => {
                println!("Refresh Complete!");
                return match output.status.code() {
                    Some(code) => ExitCode::from(code as u8),
                    None => ExitCode::FAILURE
                }
            }
            Err(why) => {
                panic!("There was an issue running the refresh process!\n\n{:?}", why);
            }
        }
    }

    fn upgrade(&self) -> ExitCode {
        todo!()
    }

    fn package_search(&self, pkg_name: String) -> ExitCode {
        todo!()
    }

    fn package_info(&self, pkg_name: String) -> ExitCode {
        todo!()
    }

    fn package_install(&self, pkg_name: String) -> ExitCode {
        todo!()
    }

    fn package_remove(&self, pkg_name: String) -> ExitCode {
        todo!()
    }
}