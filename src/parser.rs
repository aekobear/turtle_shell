
pub enum Command {
    Literal(String),
    Expression { terms: Vec<Command> },
}

impl Command {
    pub fn parse(text: &str) -> Command {
        Command::_parse(&mut text.chars())
    }

    fn _parse(chars: &mut ::std::str::Chars) -> Command {

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
