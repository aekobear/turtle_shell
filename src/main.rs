#![feature(box_syntax, box_patterns)]

use std::io;

mod parser;

extern crate plugin;
use plugin::*;

extern crate desktop;
use desktop::*;

fn main() {
    let mut shell = Shell { plugins: vec![] };
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("welcome to turtle shell v.{}!", VERSION);
    let w = Wallpaper {};
    let plugins: Vec<&Plugin> = vec![&w];
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
    fn execute_command(&self, command: &parser::Command, return_type: ValueType) -> Value {
        match command {
            &parser::Command::Literal(ref literal) => {
                match Value::new(&literal, return_type) {
                    Ok(value) => value,
                    Err(message) => Value::new(&message, ValueType::Error).unwrap(),
                }
            }
            &parser::Command::Expression { ref terms } => {
                let first = self.execute_command(&terms[0], return_type);
                if let Some(mut blueprint) = self.find_blueprint(&first.to_string()) {
                    for (e_term, mut b_term) in terms[1..].iter().zip(blueprint.terms.iter_mut()) {
                        let vt = b_term.value_type;
                        b_term.set(self.execute_command(e_term, vt));
                    }
                    return blueprint.send();
                }
                return Value::new(&format!("command not found: {}", first.to_string()),
                                  ValueType::Error)
                               .unwrap();
            }
        }
    }

    fn load_plugins(&mut self, plugins: Vec<&'a Plugin>) {
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

    fn find_blueprint(&self, name: &str) -> Option<&Blueprint> {
        for &(ref plugin, ref blueprints) in &self.plugins {
            //let blueprints: Vec<Blueprint<'a>>;
            for blueprint in blueprints {
                if blueprint.name.to_string() == name {
                    return Some(blueprint);
                }
            }
        }
        None
    }
}
