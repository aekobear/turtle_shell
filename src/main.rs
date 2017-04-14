use std::io::{self, Write};
use std::fs;
use std::path::Path;

mod parser;

mod plugin;
use plugin::Plugin;

mod turbin;
use turbin::Turbin;

mod core;

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
                        let t = Turbin::new(entry.path()
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
        let c = parser::Command::parse(input);
        self.execute(&c)
    }

    fn execute(&mut self, command: &parser::Command) -> String {
        match command {
            &parser::Command::Literal(ref s) => s.to_string(),
            &parser::Command::Expression { ref terms } => {
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
