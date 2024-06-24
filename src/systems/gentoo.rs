use std::process::{Command, ExitCode, Stdio};
use crate::command_args::{PackageInfoCommandArgs, PackageInstallCommandArgs, PackageRemoveCommandArgs, PackageSearchCommandArgs, RefreshCommandArgs, UpgradeCommandArgs};
use crate::functions::*;
use crate::systems::System;

pub struct Gentoo;

impl System for Gentoo {
    fn refresh(&self, refresh_args: &RefreshCommandArgs) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();

        args.push("--sync");

        let refresh_command_path = which("emerge");
        let mut refresh = Command::new(refresh_command_path);
        refresh.args(&args);

        if refresh_args.output {
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

    fn upgrade(&self, upgrade_args: &UpgradeCommandArgs) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();

        let upgrade_command_path = which("emerge");
        let mut upgrade = Command::new(upgrade_command_path);
        upgrade.stdin(Stdio::inherit());
        
        if !upgrade_args.force {
            args.push("--ask");
            args.push("--verbose");
            upgrade
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }

        else if upgrade_args.output {
            args.push("--verbose");
            upgrade
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }

        args.push("--update");
        args.push("--deep");
        args.push("--newuse");
        args.push("@world");

        upgrade.args(&args);

        let upgrade_result = upgrade.output();

        match upgrade_result {
            Ok(output) => {
                println!("Upgrade Complete!");
                return match output.status.code() {
                    Some(code) => ExitCode::from(code as u8),
                    None => ExitCode::FAILURE
                }
            }
            Err(why) => {
                panic!("There was an issue running the upgrade process!\n\n{:?}", why);
            }
        }
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
