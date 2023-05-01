use crate::systems::System;
use std::process::{Command, Stdio};

impl Default for Ubuntu {
    fn default() -> Self {
        Self {
            show_output: false,
            apt: String::from("/usr/bin/apt"),
            refresh: Vec::from([String::from("update")]),
            upgrade: Vec::from([String::from("upgrade"), String::from("-y")])
        }
    }
}

pub struct Ubuntu {
    pub show_output: bool,
    pub apt: String,
    pub refresh: Vec<String>,
    pub upgrade: Vec<String>
}

// TODO: Use "which" command to get location of apt
impl System for Ubuntu {

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

        let mut apt = Command::new(&self.apt);
        let mut upgrade = apt.args(&self.upgrade);

        if self.show_output {
            upgrade = upgrade
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }

        let refresh_result = upgrade.output();

        match refresh_result {
            Ok(_) => {
                println!("Upgrade Complete!");
            }
            Err(why) => {
                panic!("There was an issue running the upgrade process!\n\n{:?}", why);
            }
        }
    }
}