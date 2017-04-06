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
    plugins: Vec<(&'a Plugin, Vec<Message>)>,
}

impl<'a> Shell<'a> {
    fn execute_command(&self, command: parser::Command, return_type: ValueType) -> Value {
        match command {
            parser::Command::Literal(literal) => match Value::new(&literal, ValueType) { Ok(value) => value, Err(message) => Value::new(message, ValueType::Error) }
            parser::Command::Expression { terms } => {
                let resolved_terms = vec![];
                let first = self.execute_command(first);

                for term in terms[1..] {
                    let resolved = self.execute_command(term);
                    resolved_terms.push(resolved);
                }

                return Value::new("");
            }
        }
    }

    fn load_plugins(&self, plugins: Vec<&'a Plugin>) {
        for plugin in plugins {
            print!("loading {}...", plugin.name());
            if plugin.load() {
                let messages = plugin.messages();
                self.plugins.push((plugin, messages));
                println!("done");
            } else {
                println!("error!");
            }
        }
    }
}
