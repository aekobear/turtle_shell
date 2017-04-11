use plugin::*;
use std;

pub enum Command {
    Literal(String),
    Expression { terms: Vec<Command> },
}


pub fn parse_command(text: &str) -> Command {
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
