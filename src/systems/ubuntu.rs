use crate::systems::System;
use std::process::{Command, ExitCode, Stdio};
use crate::command_args::RefreshCommandArgs;

pub struct Ubuntu;

// TODO: Use "which" command to get location of apt
impl System for Ubuntu {

    fn refresh(command_args: RefreshCommandArgs) -> ExitCode {

        let mut args: Vec<&str> = Vec::new();

        args.push("update");

        let mut refresh = Command::new("/usr/bin/apt-get");
        refresh.args(&args);

        if command_args.show_output {
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

        let mut args: Vec<&str> = Vec::new();

        args.push("upgrade");

        if self.force {
            args.push("-y");
        }

        let mut upgrade = Command::new("/usr/bin/apt-get");
        upgrade.args(&args);

        if self.show_output {
            upgrade
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }

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

    fn package_search(&self, pkg_name: String) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();

        args.push("search");
        args.push(&pkg_name);

        let mut search = Command::new("/usr/bin/apt-cache");
        search.args(&args);
        search
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let search_result = search.output();

        match search_result {
            Ok(output) => {
                return match output.status.code() {
                    Some(code) => ExitCode::from(code as u8),
                    None => ExitCode::FAILURE
                }
            }
            Err(why) => {
                panic!("There was an issue running the package search process!\n\n{:?}", why);
            }
        }
    }

    fn package_info(&self, pkg_name: String) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();
        args.push("show");
        args.push(&pkg_name);

        let mut info = Command::new("/usr/bin/apt-cache");
        info.args(&args);
        info
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let info_result = info.output();

        match info_result {
            Ok(output) => {
                return match output.status.code() {
                    Some(code) => ExitCode::from(code as u8),
                    None => ExitCode::FAILURE
                }
            }
            Err(why) => {
                panic!("There was an issue running the package info process!\n\n{:?}", why);
            }
        }
    }

    fn package_install(&self, pkg_name: String) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();
        args.push("install");

        if self.force {
            args.push("-y");
        }

        args.push(&pkg_name);

        let mut install = Command::new("/usr/bin/apt-get");
        install.args(&args);
        install
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let install_result = install.output();

        match install_result {
            Ok(output) => {
                return match output.status.code() {
                    Some(code) => ExitCode::from(code as u8),
                    None => ExitCode::FAILURE
                }
            }
            Err(why) => {
                panic!("There was an issue running the package install process!\n\n{:?}", why);
            }
        }
    }

    fn package_remove(&self, pkg_name: String) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();
        args.push("purge");

        if self.force {
            args.push("-y");
        }

        args.push(&pkg_name);

        let mut remove = Command::new("/usr/bin/apt-get");
        remove.args(&args);
        remove
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let remove_result = remove.output();

        match remove_result {
            Ok(output) => {
                return match output.status.code() {
                    Some(code) => ExitCode::from(code as u8),
                    None => ExitCode::FAILURE
                }
            }
            Err(why) => {
                panic!("There was an issue running the package remove process!\n\n{:?}", why);
            }
        }
    }
}