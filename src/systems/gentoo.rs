use std::process::{Command, ExitCode, Stdio};
use crate::command_args::{PackageInfoCommandArgs, PackageInstallCommandArgs, PackageRemoveCommandArgs, PackageSearchCommandArgs, RefreshCommandArgs, UpgradeCommandArgs};
use crate::functions::*;
use crate::systems::System;

pub struct Gentoo;

impl System for Gentoo {
    fn refresh(&self, command_args: &RefreshCommandArgs) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();

        args.push("--sync");

        let refresh_command_path = which("emerge");
        let mut refresh = Command::new(refresh_command_path);
        refresh.args(&args);

        if command_args.output {
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

    fn upgrade(&self, command_args: &UpgradeCommandArgs) -> ExitCode {
        todo!()
    }

    fn package_search(&self, command_args: &PackageSearchCommandArgs) -> ExitCode {
        todo!()
    }

    fn package_info(&self, command_args: &PackageInfoCommandArgs) -> ExitCode {
        todo!()
    }

    fn package_install(&self, command_args: &PackageInstallCommandArgs) -> ExitCode {
        todo!()
    }

    fn package_remove(&self, command_args: &PackageRemoveCommandArgs) -> ExitCode {
        todo!()
    }
}
