use std::fmt::{self, Formatter, Display};

pub trait Plugin {
    fn load(&self) -> bool;
    fn name(&self) -> String;
    fn blueprints(&self) -> Vec<Blueprint>;
    fn receive(&self, &Blueprint) -> Value;
}

#[derive(Clone, Copy)]
pub enum ValueType {
    Number,
    Decimal,
    Boolean,
    Text,
    Error,
    None,
}

pub enum Value {
    Number(i64),
    Decimal(f64),
    Boolean(bool),
    Text(String),
    Error(i64, String),
}

impl Value {
    pub fn new(value: &str, value_type: ValueType) -> Result<Value, String> {
        match value_type {
            Number => {
                match value.parse::<i64>() {
                    Ok(value) => Ok(Value::Number(value)),
                    Err(_) => Err(format!("this value cannot be made into a Number: {}", value)),
                }
            }
            Decimal => {
                match value.parse::<f64>() {
                    Ok(value) => Ok(Value::Decimal(value)),
                    Err(_) => Err(format!("this value cannot be made into a Decimal: {}", value)),
                }
            }
            Boolean => {
                match value.parse::<bool>() {
                    Ok(value) => Ok(Value::Boolean(value)),
                    Err(_) => Err(format!("this value cannot be made into a Boolean: {}", value)),
                }
            }
            Text => Ok(Value::Text(value.to_string())),
            Error => Ok(Value::Error(1, value.to_string())),
        }
    }

    pub fn is_a(&self, value_type: ValueType) -> bool {
        match self {
            &Value::Number(_) => {
                match value_type {
                    ValueType::Number => true,
                    _ => false,
                }
            }
            &Value::Decimal(_) => {
                match value_type {
                    ValueType::Decimal => true,
                    _ => false,
                }
            }
            &Value::Boolean(_) => {
                match value_type {
                    ValueType::Boolean => true,
                    _ => false,
                }
            }
            &Value::Text(_) => {
                match value_type {
                    ValueType::Text => true,
                    _ => false,
                }
            }
            &Value::Error(_, _) => {
                match value_type {
                    ValueType::Error => true,
                    _ => false,
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            &Value::Number(n) => n.to_string(),
            &Value::Decimal(d) => d.to_string(),
            &Value::Boolean(b) => b.to_string(),
            &Value::Text(ref t) => t.to_string(),
            &Value::Error(c, ref t) => format!("({}): {}", c.to_string(), t.to_string()),
        }
    }
}

pub struct Term {
    name: String,
    pub value_type: ValueType,
    value: Option<Value>,
    optional: bool,
}

impl Term {
    pub fn new(name: &str, value_type: ValueType, optional: bool) -> Term {
        Term {
            name: name.to_string(),
            value_type: value_type,
            optional: optional,
            value: None,
        }
    }

    pub fn set(&mut self, value: Value) -> Result<(), String> {
        if value.is_a(self.value_type) {
            self.value = Some(value);
            return Ok(());
        }
        return Err("set term to wrong type".to_string());
    }
}

pub struct Blueprint<'a> {
    plugin: &'a Plugin,
    pub name: Value,
    return_type: ValueType,
    pub terms: Vec<Term>,
}

impl<'a> Blueprint<'a> {
    pub fn new(plugin: &'a Plugin,
               name: &str,
               return_type: ValueType,
               terms: Vec<Term>)
               -> Blueprint<'a> {
        Blueprint {
            name: Value::Text(name.to_string()),
            return_type: return_type,
            terms: terms,
            plugin: plugin,
        }

    }

    pub fn send(&self) -> Value {
        self.plugin.receive(self)
    }
}

pub struct Param(pub String, pub Value);

impl Param {
    pub fn new(text: &str) -> Param {
        Param(String::new(), Value::new(&text, ValueType::Text).unwrap())
    }
}


pub struct Message {
    pub name: String,
    pub params: Vec<Param>,
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
            v.push(Param::new(&param));
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
