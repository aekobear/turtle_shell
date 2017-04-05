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
    fn execute_command(&self, command: parser::Command) -> Value {
        match command {
            parser::Command::Literal(literal) => Value::new(&literal),
            parser::Command::Expression { terms } => {
                let resolved_terms = vec![];
                for term in terms {
                    let resolved = match term {
                        parser::Command::Literal(literal) => Value::new(&literal),
                        expression => self.execute_command(expression),
                    };
                    resolved_terms.push(resolved);
                }
                let message = Message {
                    name: resolved_terms[0].to_string(),
                    //TODO: fix these annoying Value casting issues.
                    //TODO: put resolved_terms into a message and send it!
                    params: vec![],
                };
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
