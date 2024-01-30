mod systems;
mod command_args;
mod commands;

use std::process::ExitCode;
use clap::Parser;
use crate::commands::*;
use crate::systems::detect_system;



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
    }
}

// fn main() -> ExitCode {
//
//     let mut program_args: Vec<String> = env::args().collect();
//     let switches = build_switches();
//     let commands = build_commands();
//
//     let show_output: bool = program_args.contains(&switches.long_show_output) ||
//         program_args.contains(&switches.short_show_output);
//     let force: bool = program_args.contains(&switches.long_force) ||
//         program_args.contains(&switches.short_force);
//
//     // Check for show output
//     // Shows the output of underlying command
//     if show_output {
//         for (index, value) in program_args.clone().iter().enumerate() {
//             if value == &switches.short_show_output {
//                 program_args.remove(index);
//             }
//             else if value == &switches.long_show_output {
//                 program_args.remove(index);
//             }
//         }
//     }
//
//     // Check for force command
//     // Will accept prompts from various system commands
//     if force {
//         for (index, value) in program_args.clone().iter().enumerate() {
//             if value == &switches.short_force {
//                 program_args.remove(index);
//             }
//             else if value == &switches.long_force {
//                 program_args.remove(index);
//             }
//         }
//     }
//
//     // Detect system type
//     let system = detect_system(show_output, force);
//
//     // Check for package command
//     if program_args[1] == commands.package {
//
//         if program_args[2] == commands.info {
//             return system.package_info(program_args[3..].join(" "));
//         }
//
//         if program_args[2] == commands.search {
//             return system.package_search(program_args[3..].join(" "));
//         }
//
//         if program_args[2] == commands.install {
//             return system.package_install(program_args[3..].join(" "));
//         }
//
//         if program_args[2] == commands.remove {
//             return system.package_remove(program_args[3..].join(" "));
//         }
//
//         println!("Command {} is not valid! Exiting...", program_args[2]);
//         return ExitCode::FAILURE;
//     }
//     // Check for update/refresh command
//     else if program_args[1] == commands.refresh {
//         return system.refresh();
//     }
//     // Check for upgrade
//     else if program_args[1] == commands.upgrade {
//         return system.upgrade();
//     }
//
//     println!("Command {} is not valid! Exiting...", program_args[1]);
//     return ExitCode::FAILURE;
// }
//
// fn build_commands() -> Commands {
//     Commands {
//         refresh: String::from("refresh"),
//         upgrade: String::from("upgrade"),
//         package: String::from("pkg"),
//         info: String::from("info"),
//         search: String::from("search"),
//         install: String::from("install"),
//         remove: String::from("remove")
//     }
// }
//
// fn build_switches() -> Switches {
//     Switches{
//         short_show_output: String::from("-o"),
//         long_show_output: String::from("--output"),
//         short_force: String::from("-f"),
//         long_force: String::from("--force")
//     }
// }
//
// struct Commands {
//     refresh: String,
//     upgrade: String,
//     package: String,
//     info: String,
//     search: String,
//     install: String,
//     remove: String
// }
//
// struct Switches {
//     short_show_output: String,
//     long_show_output: String,
//     short_force: String,
//     long_force: String
// }