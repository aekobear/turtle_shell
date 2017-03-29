use std::fmt::{self, Formatter, Display};

pub mod wallpaper;

pub trait Plugin {
    fn messages(&self) -> Vec<Message>;
    fn send(&self, Message) -> Value;
}

pub enum Value {
    Number(i64),
    Decimal(f64),
    Boolean(bool),
    Text(String),
}


pub struct Param(String, Value);

impl Param {
    fn new(text: &str) -> Param {
        if let Ok(n) = text.parse::<i64>() {
            return Param(String::new(), Value::Number(n));
        } else if let Ok(d) = text.parse::<f64>() {
            return Param(String::new(), Value::Decimal(d));
        } else if let Ok(b) = text.parse::<bool>() {
            return Param(String::new(), Value::Boolean(b));
        }
        Param(String::new(), Value::Text(text.to_string()))
    }
}

pub struct Message {
    name: String,
    params: Vec<Param>,
}

impl Message {
    pub fn new(text: String) -> Message {
        let mut split = text.split_whitespace();
        let name = match split.next() {
            Some(name) => name.to_string(),
            None => String::new(),
        };
        let mut v = vec![];
        while let Some(param) = split.next() {
            v.push(Param::new(&text));
        }
        Message {
            name: name,
            params: v,
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        if let Err(error) = write!(f, " ( {}", &self.name) {
            return Err(error);
        }

        for param in &self.params {

            if let Err(error) = write!(f, " {}", param.0) {
                return Err(error);
            }

            if let Value::Number(x) = param.1 {
                if let Err(error) = write!(f, "={}", x) {
                    return Err(error);
                }
            }

        }

        if let Err(error) = write!(f, " )") {
            return Err(error);
        }

        Ok(())
    }
}
