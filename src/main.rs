#![feature(box_syntax, box_patterns)]

use std::io;

mod parser;

extern crate plugin;
use plugin::*;

extern crate desktop;
use desktop::*;

fn main() {
    let shell = Shell { plugins: vec![] };
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("welcome to turtle shell v.{}!", VERSION);
    let plugins: Vec<&Plugin> = vec![&Wallpaper {}];
    shell.load_plugins(plugins);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let command = parser::parse_command(&input);

    }
}

struct Shell<'a> {
    plugins: Vec<(&'a Plugin, Vec<Blueprint<'a>>)>,
}

impl<'a> Shell<'a> {
    fn execute_command(&self, command: parser::Command, return_type: ValueType) -> Value {
        match command {
            parser::Command::Literal(literal) => {
                match Value::new(&literal, return_type) {
                    Ok(value) => value,
                    Err(message) => Value::new(&message, ValueType::Error).unwrap(),
                }
            }
            parser::Command::Expression { terms } => {
                let resolved_terms = vec![];
                let first = self.execute_command(terms[0], return_type);
                if let Some(blueprint) = self.find_blueprint(first) {
                    //TODO: iterate through each term and set it
                    match blueprint.build(self, resolved_terms) {
                        Ok(message) => return message.send(),
                        Err(error) => return error,
                    }
                }
                return Value::new(format!("command not found: {}", first), ValueType::Error);
            }
        }
    }

    fn load_plugins(&self, plugins: Vec<&'a Plugin>) {
        for plugin in plugins {
            print!("loading {}...", plugin.name());
            if plugin.load() {
                let blueprints = plugin.blueprints();
                self.plugins.push((plugin, blueprints));
                println!("done");
            } else {
                println!("error!");
            }
        }
    }

    fn find_blueprint(&self, name: Value) -> Option<Blueprint> {
        for plugin in self.plugins {
            for blueprint in plugin.1 {
                if blueprint.name() == name {
                    return Some(blueprint);
                }
            }
        }
        None;
    }
}
