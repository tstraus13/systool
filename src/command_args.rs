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
pub struct PackageCommandArgs {
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

#[derive(Args, Debug)]
pub struct FindCommandArgs {
    #[command(subcommand)]
    pub find_commands: FindCommands
}

#[derive(Args, Debug)]
pub struct FindFileCommandArgs {
    /// File Name to look for
    pub file_name: String,
    pub path: String,
    #[arg(long)]
    pub hidden: bool,
    #[arg(short, long)]
    #[clap(name = "symlinks")]
    pub follow_symlinks: bool,
}

#[derive(Args, Debug)]
pub struct FindDirectoryCommandArgs {
    /// File Name to look for
    pub directory_name: String,
    pub path: String,
    #[arg(long)]
    pub hidden: bool,
    #[arg(short, long)]
    #[clap(name = "symlinks")]
    pub follow_symlinks: bool,
}

#[derive(Args, Debug)]
pub struct FindTextCommandArgs {
    /// Text to look for
    pub text: String,
    pub path: String,
    #[arg(long)]
    pub hidden: bool,
    #[arg(short, long)]
    #[clap(name = "symlinks")]
    pub follow_symlinks: bool,
}

#[derive(Args, Debug)]
pub struct ArchiveCommandArgs {
    #[command(subcommand)]
    pub archive_commands: ArchiveCommands
}

#[derive(Args, Debug)]
pub struct ArchiveCreateCommandArgs {
    #[arg(short,long)]
    #[clap(name = "source")]
    pub src_path: String,
    #[arg(short,long)]
    #[clap(name = "destination")]
    pub dst_path: String,
    #[arg(short,long)]
    #[clap(name = "name")]
    pub file_name: String,
}

#[derive(Args, Debug)]
pub struct ArchiveExtractCommandArgs {
    #[arg(short,long)]
    #[clap(name = "source")]
    pub src_path: String,
    #[arg(short,long)]
    #[clap(name = "destination")]
    pub dst_path: String,
}