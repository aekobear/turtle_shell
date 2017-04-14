use std::io::{self, Write};
use std::fs;
use std::path::Path;

fn main() {
    let mut ts = TurtleShell::new();
    ts.load_turbins();
    while ts.running() {
        print!("\u{1f422} ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        println!("{}", &ts.parse(&input));
    }

}

struct TurtleShell<'a> {
    running: bool,
    turbins: Vec<Turbin>,
    plugins: Vec<&'a Plugin>,
}


impl<'a> TurtleShell<'a> {
    fn new() -> TurtleShell<'a> {
        TurtleShell {
            running: true,
            turbins: vec![],
            plugins: vec![],
        }
    }

    fn running(&self) -> bool {
        self.running
    }

    fn load_plugin(&mut self, plugin: &'a Plugin) -> bool {
        self.plugins.push(plugin);
        plugin.load()
    }

    fn load_turbins(&mut self) -> bool {
        if let Ok(dirs) = fs::read_dir("./plugins/bin") {
            for entry in dirs {
                let entry = entry.unwrap();
                if let Some(ext) = entry.path().extension() {
                    if ext == "turbin" {
                        let t = Turbin(entry.path()
                                           .to_str()
                                           .unwrap()
                                           .to_string());
                        println!("found: {}", t.name());
                        self.turbins.push(t);
                    }
                }
            }
            return true;
        }
        false
    }

    fn parse(&mut self, input: &str) -> String {
        let c = Command::parse(input);
        self.execute(&c)
    }

    fn execute(&mut self, command: &Command) -> String {
        match command {
            &Command::Literal(ref s) => s.to_string(),
            &Command::Expression { ref terms } => {
                let message = self.execute(&terms[0]);
                let mut params = vec![];
                for term in &terms[1..] {
                    params.push(self.execute(term));
                }
                self.send(&message, params)
            }
        }
    }

    fn send(&mut self, message: &str, params: Vec<String>) -> String {
        for m in self.messages() {
            if message == m {
                return self.receive(message, params);
            }
        }

        for plugin in &self.plugins {
            for m in plugin.messages() {
                if message == m {
                    return plugin.receive(message, params);
                }
            }
        }
        for turbin in &self.turbins {
            for m in turbin.messages() {
                if message == m {
                    return turbin.receive(message, params);
                }
            }
        }
        return "message not found D:".to_string();
    }
}

// ~~~~~ plugin ~~~~~:

pub trait Plugin {
    fn name(&self) -> String;
    fn load(&self) -> bool;
    fn messages(&self) -> Vec<String>;
    fn receive(&self, message: &str, params: Vec<String>) -> String;
}

impl<'a> TurtleShell<'a> {
    fn name(&self) -> String {
        "core".to_string()
    }
    fn load(&self) -> bool {
        true
    }
    fn messages(&self) -> Vec<String> {
        vec!["+", "-", "echo", "goodbye"].iter().map(|&s| s.to_owned()).collect()
    }
    fn receive(&mut self, message: &str, params: Vec<String>) -> String {
        match message {
            "+" => {
                match self.add(params) {

                    Ok(s) => s.to_string(),
                    Err(err) => err,
                }
            }
            "-" => {
                match self.subtract(params) {
                    Ok(d) => d.to_string(),
                    Err(err) => err,
                }
            }
            "echo" => params.join(" "),
            "goodbye" => {
                self.running = false;
                "oki bai!".to_string()
            }
            _ => format!("message \"{}\" not found :c", message),
        }
    }
}
impl<'a> TurtleShell<'a> {
    fn add(&self, params: Vec<String>) -> Result<f64, String> {
        let mut x = 0.0;
        for param in params {
            match param.parse::<f64>() {
                Ok(v) => x += v,
                Err(_) => {
                    return Err(format!("the value \"{}\" is invalid and cannot be added", param))
                }
            }
        }
        Ok(x)
    }
    fn subtract(&self, params: Vec<String>) -> Result<f64, String> {
        if let Ok(mut first) = params[0].parse::<f64>() {
            for param in &params[1..] {
                match param.parse::<f64>() {
                    Ok(v) => first -= v,
                    Err(_) => {
                        return Err(format!("the value \"{}\" is invalid and can not subtract",
                                           param))
                    }
                }
            }
            return Ok(first);
        }
        return Err(format!("the value \"{}\" is invalid and cannot be subtracted",
                           params[0]));
    }
}

// ~~~~~ filesystem ~~~~~:



// ~~~~~ parser ~~~~~:

pub enum Command {
    Literal(String),
    Expression { terms: Vec<Command> },
}

impl Command {
    pub fn parse(text: &str) -> Command {
        Command::_parse(&mut text.chars())
    }

    fn _parse(chars: &mut std::str::Chars) -> Command {

        let mut params = vec![];

        let mut word = String::new();

        let mut stringmode = false;

        while let Some(c) = chars.next() {
            if c == '"' {
                stringmode = !stringmode;
            } else if stringmode {
                word.push(c);
            } else if c.is_whitespace() {
                if !word.is_empty() {
                    params.push(Command::Literal(word));
                }
                word = String::new();
            } else if c == '(' {
                if !word.is_empty() {
                    params.push(Command::Literal(word));
                }
                word = String::new();
                params.push(Command::_parse(chars));
            } else if c == ')' {
                if !word.is_empty() {
                    params.push(Command::Literal(word));
                }
                return Command::Expression { terms: params };
            } else {
                word.push(c);
            }
        }

        if !word.is_empty() {
            params.push(Command::Literal(word));
        }
        Command::Expression { terms: params }
    }
}

// ~~~~~ turbin ~~~~~:

use std::process::Command as Com;

struct Turbin(String);

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
