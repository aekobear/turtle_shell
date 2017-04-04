#![feature(box_syntax, box_patterns)]

use std::io;

extern crate plugin;
use plugin::*;

extern crate desktop;
use desktop::*;

fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("welcome to turtle shell v.{}!", VERSION);
    let w = Wallpaper {};
    for message in w.messages() {
        println!("message: {}", message);
    }
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("failed to read line");

    }
    if let Value::Text(response) = w.send(Message::new("get_wallpaper".to_string())) {
        println!("get_wallpaper: {}", response);
    }
}


enum Command {
    Literal(String),
    Expression { params: Vec<Command> },
}

fn parse_command(text: &str) -> Command {
    _parse_command(&mut text.chars())
}

fn _parse_command(chars: &mut std::str::Chars) -> Command {

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
            params.push(_parse_command(chars));
        } else if c == ')' {
            if !word.is_empty() {
                params.push(Command::Literal(word));
            }
            return Command::Expression { params: params };
        } else {
            word.push(c);
        }
    }

    if !word.is_empty() {
        params.push(Command::Literal(word));
    }
    Command::Expression { params: params }
}
