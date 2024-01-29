use clap::{Parser, Subcommand};
use crate::command_args::{PackageInfoCommandArgs, PackageInstallCommandArgs, PackageRemoveCommandArgs, PackageSearchCommandArgs, RefreshCommandArgs, UpgradeCommandArgs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SysTool {
    /// The Command you wish to execute
    #[command(subcommand)]
    pub command: Commands,
    // Force / Accept Any User Input
    // #[arg(short, long)]
    // force: String,
    // #[arg(short, long)]
    // output: String
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Refresh the system repositories
    Refresh(RefreshCommandArgs),
    /// Upgrade the system packages
    Upgrade(UpgradeCommandArgs),
    /// Repository package commands
    Package(PackageCommands)
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