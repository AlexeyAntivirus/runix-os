use std::fmt::{Formatter, Display, Result};
use std::env::*;

struct OSInformation {
    name: String,
    code_name: String,
    version: String,
}

impl Display for OSInformation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} v{}", self.name, self.code_name, self.version)
    }
}

struct OSInformationBuilder {
    name: Option<String>,
    code_name: Option<String>,
    version: Option<String>,
}

impl OSInformationBuilder {
    pub fn new() -> Self {
        OSInformationBuilder {
            name: None,
            code_name: None,
            version: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn code_name(mut self, code_name: String) -> Self {
        self.code_name = Some(code_name);
        self
    }

    pub fn version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    pub fn build(self) -> OSInformation {
        if self.name == None || self.code_name == None || self.version == None {

            let mut panic_message: String = String::from("\n");

            if self.name == None {
                panic_message += "OS name is not initialize!\n";
            }
            if self.code_name == None {
                panic_message += "OS code name is not initialize!\n";
            }
            if self.version == None {
                panic_message += "OS version is not initialize!\n";
            }
            panic!(panic_message);
        } else {
            OSInformation {
                name: self.name.unwrap(),
                code_name: self.code_name.unwrap(),
                version: self.version.unwrap(),
            }
        }
    }
}

fn main() {
    /*println!("Hello, {}!", OSInformationBuilder::new()
        .name(String::from("RunixOS"))
        .code_name(String::from("New"))
        .version(String::from("0.1.0-alpha"))
        .build());*/
    println!("{:?}", String::new());
}
