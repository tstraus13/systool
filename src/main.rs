mod systems;
mod command_args;
mod commands;
mod file;
mod functions;
mod directory;
mod text;
mod archive;

use std::process::ExitCode;
use clap::Parser;
use crate::commands::*;
use crate::functions::*;
use crate::file::*;
use crate::directory::*;
use crate::text::find_text;


fn main() -> ExitCode {

    let cli = SysTool::parse();

    //dbg!(&cli);

    let system = detect_system();

    return match &cli.command {
        Commands::Refresh(args) => {
            system.refresh(args)
        }
        Commands::Upgrade(args) => {
            system.upgrade(args)
        }
        Commands::Package(args) => {
            match &args.package_commands {
                PackageCommands::Info(args) => {
                    system.package_info(args)
                }
                PackageCommands::Search(args) => {
                    system.package_search(args)
                }
                PackageCommands::Install(args) => {
                    system.package_install(args)
                }
                PackageCommands::Remove(args) => {
                    system.package_remove(args)
                }
            }
        }
        Commands::Find(args) => {
            return match &args.find_commands {
                FindCommands::File(args) => {
                    find_file(args)
                },
                FindCommands::Directory(args) => {
                    find_dir(args)
                },
                FindCommands::Text(args) => {
                    find_text(args)
                }
            }
        }
    }
}