use clap::Args;
use crate::commands::*;

#[derive(Args, Debug)]
pub struct RefreshCommandArgs {
    /// Show output of the command
    #[arg(short, long)]
    pub output: bool
}

#[derive(Args, Debug)]
pub struct UpgradeCommandArgs {
    /// Show output of the command
    #[arg(short, long)]
    pub output: bool,
    /// Accept any/all user input
    #[arg(short, long)]
    pub force: bool
}

#[derive(Args, Debug)]
pub struct PackageCommandArgs{
    #[command(subcommand)]
    pub package_commands: PackageCommands
}

#[derive(Args, Debug)]
pub struct PackageInfoCommandArgs {
    /// Package name to get info on
    //#[arg(short, long)]
    pub package_name: String
}

#[derive(Args, Debug)]
pub struct PackageSearchCommandArgs {
    /// Package name to search for
    //#[arg(short, long)]
    pub package_name: String
}

#[derive(Args, Debug)]
pub struct PackageInstallCommandArgs {
    /// Package(s) to install
    //#[arg(short, long)]
    pub packages: Vec<String>,
    /// Accept any/all user input
    #[arg(short, long)]
    pub force: bool
}

#[derive(Args, Debug)]
pub struct PackageRemoveCommandArgs {
    /// Package(s) to remove
    //#[arg(short, long)]
    pub packages: Vec<String>,
    /// Accept any/all user input
    #[arg(short, long)]
    pub force: bool
}