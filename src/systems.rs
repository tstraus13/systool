use std::process::{ExitCode};
use crate::command_args::*;

pub mod macos;
pub mod ubuntu;
pub mod fedora;
pub mod void;
pub mod arch;
pub mod gentoo;

pub trait System {
    fn refresh(&self, command_args:&RefreshCommandArgs) -> ExitCode;
    fn upgrade(&self, command_args:&UpgradeCommandArgs) -> ExitCode;
    fn package_search(&self, command_args:&PackageSearchCommandArgs) -> ExitCode;
    fn package_info(&self, command_args:&PackageInfoCommandArgs) -> ExitCode;
    fn package_install(&self, command_args:&PackageInstallCommandArgs) -> ExitCode;
    fn package_remove(&self, command_args:&PackageRemoveCommandArgs) -> ExitCode;
}
