use std::process::{Command, ExitCode, Stdio};
use colored::Colorize;
use crate::command_args::{PackageInfoCommandArgs, PackageInstallCommandArgs, PackageRemoveCommandArgs, PackageSearchCommandArgs, RefreshCommandArgs, UpgradeCommandArgs};
use crate::functions::*;
use crate::systems::System;

pub struct Gentoo;

impl Gentoo {
    fn depclean(&self, force: bool) {
        let mut args: Vec<&str> = Vec::new();

        args.push("--depclean");

        let depclean_command_path = which("emerge");
        let mut depclean = Command::new(depclean_command_path);

        if !force {
            args.push("--ask");
        }

        depclean.args(&args);

        depclean
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let depclean_result = depclean.output();

        match depclean_result {
            Ok(output) => {
                println!("{}", "Depclean Complete!".bold());
            }
            Err(why) => {
                panic!("There was an issue running the depclean process!\n\n{:?}", why);
            }
        }

    }
}

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
                println!("{}", "Upgrade Complete!".bold());

                // For Gentoo should run a depclean after an Upgrade
                self.depclean(upgrade_args.force);

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

        args.push("--search");
        args.push(&pkg_search_args.package_name);

        let search_command_path = which("emerge");
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

    // Gentoo does not really have a Package Info command.
    // Doing a search should be sufficient. Will update/change
    // if I find something better.
    fn package_info(&self, pkg_info_args: &PackageInfoCommandArgs) -> ExitCode {
        let search_args = PackageSearchCommandArgs {
            package_name: pkg_info_args.package_name.clone()
        };

        return self.package_search(&search_args);
    }

    fn package_install(&self, pkg_install_args: &PackageInstallCommandArgs) -> ExitCode {
        let mut args: Vec<&str> = Vec::new();

        if !pkg_install_args.force {
            args.push("--ask");
        }

        for package in pkg_install_args.packages.iter() {
            args.push(package);
        }

        let install_command_path = which("emerge");
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
        args.push("--deselect");

        if !pkg_remove_args.force {
            args.push("--ask");
        }

        for package in pkg_remove_args.packages.iter() {
            args.push(package);
        }

        let remove_command_path = which("emerge");
        let mut remove = Command::new(remove_command_path);
        remove.args(&args);
        remove
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let remove_result = remove.output();

        match remove_result {
            Ok(output) => {

                // For Gentoo should run a depclean after an Upgrade
                self.depclean(pkg_remove_args.force);

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
