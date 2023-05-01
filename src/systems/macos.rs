use crate::systems::System;
use std::process::{Command, Stdio};

impl Default for MacOS {
    fn default() -> Self {
        Self {
            show_output: false,
            brew: String::from("/opt/homebrew/bin/brew"),
            refresh: Vec::from([String::from("update"), String::from("--force")]),
            upgrade: Vec::from([String::from("upgrade")])
        }
    }
}

pub struct MacOS {
    pub show_output: bool,
    pub brew: String,
    pub refresh: Vec<String>,
    pub upgrade: Vec<String>
}

/*impl MacOS {

    pub fn new() -> Self {
        let mut mac = MacOS {
            version: String::from(""),
            build: String::from("") 
        };

        mac.init();

        return mac;
    }

    fn get_version(&mut self) {

        let version_result = Command::new("sw_vers")
            .arg("--productVersion")
            .output();

        match version_result {
            Ok(output) => {
                let parse_output = String::from_utf8(output.stdout);

                match parse_output {
                    Ok(version) => {
                        self.version = version.replace("\n", "");
                    }
                    Err(why) => {
                        panic!("There was an issue parsing the output from sw_vers!\n\n{:?}", why);
                    }
                }
            }
            Err(why) => {
                panic!("There was an issue calling sw_vers to get the version of macOS!\n\n{:?}", why);
            }
        }

        let build_result = Command::new("sw_vers")
            .arg("--buildVersion")
            .output();

        match build_result {
            Ok(output) => {
                let parse_output = String::from_utf8(output.stdout);

                match parse_output {
                    Ok(build) => {
                        self.build = build.replace("\n", "");
                    }
                    Err(why) => {
                        panic!("There was an issue parsing the output from sw_vers!\n\n{:?}", why);
                    }
                }
            }
            Err(why) => {
                panic!("There was an issue calling sw_vers to get the build of macOS!\n\n{:?}", why);
            }
        }

    }

}*/

// TODO: Use "which" command to get location of brew
impl System for MacOS {

    fn refresh(&self) {

        let mut brew = Command::new(&self.brew);
        let mut refresh = brew.args(&self.refresh);

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
        
        let mut brew = Command::new(&self.brew);
        let mut upgrade = brew.args(&self.upgrade);

        if self.show_output {
            upgrade = upgrade
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }
        
        let upgrade_result = upgrade.output();

        match upgrade_result {
            Ok(_) => {
                println!("Upgrade Complete!");
            }
            Err(why) => {
                panic!("There was an issue running the upgrade process!\n\n{:?}", why);
            }
        }

    }
}