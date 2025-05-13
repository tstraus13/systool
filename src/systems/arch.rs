use std::process::{Command, ExitCode, Stdio};
use crate::systems::*;
use crate::functions::*;

pub struct Arch;

impl System for Arch {

    fn refresh(&self, command_args:&RefreshCommandArgs) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();

        args.push("-Syy");

        let refresh_command_path = get_pkg_manager();
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

    fn upgrade(&self, upgrade_args: &UpgradeCommandArgs) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();

        args.push("-Syu");

        if upgrade_args.force {
            args.push("--noconfirm");
        }

        let upgrade_command_path = get_pkg_manager();
        let mut upgrade = Command::new(upgrade_command_path);
        upgrade.args(&args);
        upgrade.stdin(Stdio::inherit());

        if upgrade_args.output || !upgrade_args.force {
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

    fn package_search(&self, pkg_search_args: &PackageSearchCommandArgs) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();

        args.push("-Ssy");
        args.push(&pkg_search_args.package_name);

        let search_command_path = get_pkg_manager();
        let mut search = Command::new(search_command_path);
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

    fn package_info(&self, pkg_info_args: &PackageInfoCommandArgs) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();

        args.push("-Siy");
        args.push(&pkg_info_args.package_name);

        let info_command_path = get_pkg_manager();
        let mut info = Command::new(info_command_path);
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

    fn package_install(&self, pkg_install_args: &PackageInstallCommandArgs) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();
        args.push("-Sy");

        if pkg_install_args.force {
            args.push("-y");
        }

        for package in pkg_install_args.packages.iter() {
            args.push(package);
        }

        let install_command_path = get_pkg_manager();
        let mut install = Command::new(install_command_path);
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

    fn package_remove(&self, pkg_remove_args: &PackageRemoveCommandArgs) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();
        args.push("-R");

        if pkg_remove_args.force {
            args.push("-y");
        }

        for package in pkg_remove_args.packages.iter() {
            args.push(package);
        }

        let remove_command_path = get_pkg_manager();
        let mut remove = Command::new(remove_command_path);
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
                panic!("There was an issue running the package install process!\n\n{:?}", why);
            }
        }
    }
}

fn get_pkg_manager() -> String {

    let yay_path = which("yay");

    if !String::is_empty(&yay_path) {
        println!("yay found!");
        return yay_path;
    }
    
    let paru_path = which("paru");
    
    if !String::is_empty(&paru_path) {
        println!("paru found!");
        return paru_path;
    }

    which("pacman")
}