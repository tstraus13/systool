pub mod systems;

use std::env;
use std::process::ExitCode;
use crate::systems::detect_system;

fn main() -> ExitCode {

    let mut program_args: Vec<String> = env::args().collect();
    let switches = build_switches();
    let commands = build_commands();

    let show_output: bool = program_args.contains(&switches.long_show_output) ||
        program_args.contains(&switches.short_show_output);

    // Check for show output
    // Shows the output of underlying command
    if show_output {
        for (index, value) in program_args.clone().iter().enumerate() {
            if value == &switches.short_show_output {
                program_args.remove(index);
            }
            else if value == &switches.long_show_output {
                program_args.remove(index);
            }
        }
    }

    // Detect system type
    let system = detect_system(show_output);

    // Check for package command
    if program_args[1] == commands.package {

        if program_args[2] == commands.info {
            return system.package_info(program_args[3..].join(" "));
        }

        if program_args[2] == commands.search {
            return system.package_search(program_args[3..].join(" "));
        }

        println!("Command {} is not valid! Exiting...", program_args[2]);
        return ExitCode::FAILURE;
    }
    // Check for update/refresh command
    else if program_args[1] == commands.refresh {
        return system.refresh();
    }
    // Check for upgrade
    else if program_args[1] == commands.upgrade() {
        return system.upgrade();
    }

    println!("Command {} is not valid! Exiting...", program_args[1]);
    return ExitCode::FAILURE;
}

fn build_commands() -> Commands {
    Commands {
        refresh: String::from("refresh"),
        upgrade: String::from("upgrade"),
        package: String::from("pkg"),
        info: String::from("info"),
        search: String::from("search")
    }
}

fn build_switches() -> Switches {
    Switches{
        short_show_output: String::from("-o"),
        long_show_output: String::from("--output")
    }
}

struct Commands {
    refresh: String,
    upgrade: String,
    package: String,
    info: String,
    search: String
}

struct Switches {
    short_show_output: String,
    long_show_output: String,
}