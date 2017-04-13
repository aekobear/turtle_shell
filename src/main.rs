use std::io::{self, Write};

fn main() {
    let mut ts = TurtleShell { plugins: vec![] };

    loop {
        print!("\u{1f422} ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        println!("{}", &ts.parse(&input));
    }

}

struct TurtleShell<'a> {
    plugins: Vec<&'a Plugin>,
}

impl<'a> TurtleShell<'a> {
    fn load_plugin(&mut self, plugin: &'a Plugin) -> bool {
        self.plugins.push(plugin);
        plugin.load()
    }

    fn parse(&self, input: &str) -> String {
        let c = Command::parse(input);
        self.execute(&c)
    }

    fn execute(&self, command: &Command) -> String {
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

    fn send(&self, message: &str, params: Vec<String>) -> String {
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
        return "message not found D:".to_string();
    }
}

// ~~~~~ plugin ~~~~~:

pub trait Plugin {
    fn name(&self) -> String;
    fn load(&self) -> bool;
    fn messages(&self) -> Vec<&str>;
    fn receive(&self, message: &str, params: Vec<String>) -> String;
}

impl<'a> Plugin for TurtleShell<'a> {
    fn name(&self) -> String {
        "core".to_string()
    }
    fn load(&self) -> bool {
        true
    }
    fn messages(&self) -> Vec<&str> {
        vec!["+", "-", "\"", "goodbye"]
    }
    fn receive(&self, message: &str, params: Vec<String>) -> String {
        match message {
            "+" => match self.add(params) {
                    Ok(s) => s.to_string(),
                    Err(err) => err,
                },
            "-" => match self.subtract(params) {
                    Ok(d) => d.to_string(),
                    Err(err) => err,
                },
            "\"" => params.join(" "),
            "goodbye" => std::process::exit(0),
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
                    Err(_) => return Err(format!("the value \"{}\" is invalid and can not subtract", param)),
                }
            }
            return Ok(first);
        }
        return Err(format!("the value \"{}\" is invalid and cannot be subtracted", params[0]));
    }
}

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

        while let Some(c) = chars.next() {
            if c.is_whitespace() {
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
