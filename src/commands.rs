use clap::{Parser, Subcommand};
use crate::command_args::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SysTool {
    /// The Command you wish to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Refresh the system repositories
    Refresh(RefreshCommandArgs),
    /// Upgrade the system packages
    Upgrade(UpgradeCommandArgs),
    #[clap(name = "pkg")]
    /// System repositories package commands
    Package(PackageCommandArgs)
}

#[derive(Subcommand, Debug)]
pub enum PackageCommands {
    /// Get the information on a particular package
    Info(PackageInfoCommandArgs),
    /// Search for a particular package in the system repositories
    Search(PackageSearchCommandArgs),
    /// Install package(s) from the system repositories
    Install(PackageInstallCommandArgs),
    /// Remove package(s) from the system repositories
    Remove(PackageRemoveCommandArgs)
}