use crate::systems::System;
use std::process::{Command, Stdio};

pub struct Ubuntu {
    show_output: bool,
    apt: String,
    refresh: Vec<String>,
    upgrade: Vec<String>
}

impl System for Ubuntu {
    fn new(show_output: bool) -> Self {
        Ubuntu {
            show_output,
            apt: String::from("/usr/bin/apt"),
            refresh: Vec::from([String::from("update")]),
            upgrade: Vec::from([String::from("upgrade"), String::from("-y")])
        }
        // TODO: Use "which" command to get location of apt
    }

    fn refresh(&self) {
        let mut apt = Command::new(&self.apt);
        let mut refresh = apt.args(&self.refresh);

        if self.show_output {
            refresh = refresh
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }

        let refresh_result = refresh.output();

        match refresh_result {
            Ok(_) => {
                println!("Refresh Complete!");
            }
            Err(why) => {
                panic!("There was an issue running the upgrade process!\n\n{:?}", why);
            }
        }
    }

    fn upgrade(&self) {
        todo!()
    }
}