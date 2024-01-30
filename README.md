# systool

*A tool for standardizing system functions across operating systems*

I created this tool because I wanted a single command process for doing basic system tasks (refreshing repos,
 updating/upgrading system, install package, remove package, search package, etc).

Currently, I have some basic tasks working for macOS (homebrew), Ubuntu, Fedora, and Void. I plan to add more systems as time
goes on. Just run the command ```systool``` to get a help in running the command.

```shell
Usage: systool <COMMAND>

Commands:
  refresh  Refresh the system repositories
  upgrade  Upgrade the system packages
  pkg      System repositories package commands
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Yes I know you could easily write a similar tool in whatever shell scripting language of your choice, but I wanted to
learn more Rust.