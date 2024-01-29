use clap::Args;

#[derive(Args, Debug)]
pub struct RefreshCommandArgs {
    /// Show output of the command
    #[arg(short, long)]
    pub show_output: bool
}

#[derive(Args, Debug)]
pub struct UpgradeCommandArgs {
    /// Show output of the command
    #[arg(short, long)]
    pub show_output: bool,
    /// Accept any/all user input
    #[arg(shor, long)]
    pub force: bool
}

#[derive(Args, Debug)]
pub struct PackageInfoCommandArgs {
    /// Package name to get info on
    #[arg(short, long)]
    pub package_name: String
}

#[derive(Args, Debug)]
pub struct PackageSearchCommandArgs {
    /// Package name to search for
    #[arg(short, long)]
    pub package_name: String
}

#[derive(Args, Debug)]
pub struct PackageInstallCommandArgs {
    /// Package to install
    #[arg(short, long)]
    pub package_name: String,
    /// Accept any/all user input
    #[arg(shor, long)]
    pub force: bool
}

#[derive(Args, Debug)]
pub struct PackageRemoveCommandArgs {
    /// Package to install
    #[arg(short, long)]
    pub package_name: String,
    /// Accept any/all user input
    #[arg(shor, long)]
    pub force: bool
}