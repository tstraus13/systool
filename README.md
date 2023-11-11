# systool

*A tool for standardizing system functions across operating systems*

I created this tool because I wanted a single command process for doing basic system tasks (refreshing repos,
 updating/upgrading system, install package, remove package, search package, etc).

Currently, I have some basic tasks working for macOS (homebrew), Ubuntu, Fedora, and Void. I plan to add more systems as time
goes on. As of now you will need to look at the code to figure out what commands are available. I want to soon add a help
command that describes all available functions, but until then look at the code. Here is an example command for
installing a package.

```shell
systool pkg install [PACKAGE]
```

Yes I know you could easily write a similar tool in whatever shell scripting language of your choice, but I wanted to
learn more Rust.