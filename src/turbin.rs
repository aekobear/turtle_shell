use std::process::Command as Com;

use plugin::Plugin;

pub struct Turbin(String);

impl Plugin for Turbin {
    fn name(&self) -> String {
        self.instruct(vec!["name".into()])
    }

    fn load(&self) -> bool {
        match self.instruct(vec!["load".into()]).parse::<bool>() {
            Ok(val) => val,
            Err(_) => false,
        }
    }

    fn messages(&self) -> Vec<String> {
        self.instruct(vec!["messages".into()])
            .split(" ")
            .map(|s| s.to_owned())
            .collect()
    }

    fn receive(&self, message: &str, mut params: Vec<String>) -> String {
        let mut metacommand = vec!["receive".into(), message.into()];
        metacommand.extend(params);
        self.instruct(metacommand)
    }
}

impl Turbin {
    pub fn new(s: String) -> Turbin {
        Turbin(s)
    }

    fn instruct(&self, args: Vec<String>) -> String {
        let mut com = Com::new(&self.0);
        for arg in args {
            com.arg(arg);
        }
        let output = com.output().expect(&format!("failed to execute turbin: {}", self.0));
        if let Ok(mut message) = String::from_utf8(output.stdout) {
            message.pop();
            return message;
        }

        if let Ok(mut error) = String::from_utf8(output.stderr) {
            error.pop();
            return error;
        }

        return String::new();

    }
}
