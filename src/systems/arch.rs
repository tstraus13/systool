use std::process::{Command, ExitCode, Stdio};
use crate::systems::System;

impl Default for Arch {
    fn default() -> Self {
        Self {
            show_output: false,
            force: false
        }
    }
}

pub struct Arch {
    pub show_output: bool,
    pub force: bool
}

// TODO: Use "which" command to get location of pacman
impl System for Arch {
    fn refresh(&self) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();

        args.push("-Syy");

        let mut refresh = Command::new("/usr/bin/pacman");
        refresh.args(&args);

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
        let mut args: Vec<&str> = Vec::new();

        args.push("-Syu");

        if self.force {
            args.push("--noconfirm");
        }

        let mut upgrade = Command::new("/usr/bin/pacman");
        upgrade.args(&args);
        upgrade.stdin(Stdio::inherit());

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

        args.push("-Ssy");
        args.push(&pkg_name);

        let mut search = Command::new("/usr/bin/pacman");
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

        args.push("-Siy");
        args.push(&pkg_name);

        let mut search = Command::new("/usr/bin/pacman");
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
                panic!("There was an issue running the package info process!\n\n{:?}", why);
            }
        }
    }

    fn package_install(&self, pkg_name: String) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();
        args.push("-Sy");

        if self.force {
            args.push("--noconfirm");
        }

        args.push(&pkg_name);

        let mut install = Command::new("/usr/bin/pacman");
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
        args.push("-R");

        if self.force {
            args.push("--noconfirm");
        }

        args.push(&pkg_name);

        let mut install = Command::new("/usr/bin/pacman");
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
}