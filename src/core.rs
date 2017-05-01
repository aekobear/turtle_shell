use std::io::{self, Write};

extern crate rand;

use TurtleShell;

impl<'a> TurtleShell<'a> {
    pub fn name(&self) -> String {
        "core".to_string()
    }
    pub fn load(&self) -> bool {
        true
    }
    pub fn messages(&self) -> Vec<String> {
        vec!["+", "-", "/", "*", "format_decimal", "echo", "s", "run", "ask", "random", "exit"]
            .iter()
            .map(|&s| s.to_owned())
            .collect()
    }
    pub fn receive(&mut self, message: &str, params: Vec<String>) -> String {
        match message {
            "+" => {
                match self.add(params) {

                    Ok(s) => s.to_string(),
                    Err(err) => err,
                }
            }
            "-" => {
                match self.subtract(params) {
                    Ok(d) => d.to_string(),
                    Err(err) => err,
                }
            }
            "/" => {
                match self.divide(params) {
                    Ok(q) => q.to_string(),
                    Err(err) => err,
                }
            }
            "*" => {
                match self.multiply(params) {
                    Ok(p) => p.to_string(),
                    Err(err) => err,
                }
            }
            "format_decimal" => {
                match self.format_decimal(params) {
                    Ok(s) => s,
                    Err(err) => err,
                }
            }
            "random" => rand::random::<f64>().to_string(),
            "echo" => params.join(" "),
            "s" => params.join(""),
            "run" => self.run(params).unwrap(),
            "ask" => self.ask(params).unwrap(),
            "exit" => {
                self.running = false;
                "oki bai!".to_string()
            }
            _ => format!("message \"{}\" not found :c", message),
        }
    }
}
impl<'a> TurtleShell<'a> {
    fn add(&self, params: Vec<String>) -> Result<f64, String> {
        let mut x = 0.0;
        for param in params {
            match param.parse::<f64>() {
                Ok(v) => x += v,
                Err(_) => {
                    return Err(format!("the value \"{}\" is invalid and cannot be added", param))
                }
            }
        }
        Ok(x)
    }

    fn subtract(&self, params: Vec<String>) -> Result<f64, String> {
        if let Ok(mut first) = params[0].parse::<f64>() {
            for param in &params[1..] {
                match param.parse::<f64>() {
                    Ok(v) => first -= v,
                    Err(_) => {
                        return Err(format!("the value \"{}\" is invalid and can not subtract",
                                           param))
                    }
                }
            }
            return Ok(first);
        }
        return Err(format!("the value \"{}\" is invalid and cannot be subtracted",
                           params[0]));
    }

    fn multiply(&self, params: Vec<String>) -> Result<f64, String> {
        let mut x = 1.0;
        for param in params {
            if let Ok(f) = param.parse::<f64>() {
                x *= f;
            } else {
                return Err(format!("the value \"{}\" is invalid and can not be multiplied.",
                                   param));
            }
        }
        Ok(x)
    }

    fn divide(&self, params: Vec<String>) -> Result<f64, String> {
        if let Ok(mut first) = params[0].parse::<f64>() {
            for param in &params[1..] {
                match param.parse::<f64>() {
                    Ok(v) => first /= v,
                    Err(_) => {
                        return Err(format!("the value \"{}\" is invalid and can not divide", param))
                    }
                }
            }
            return Ok(first);
        }
        return Err(format!("the value \"{}\" is invalid and cannot be divided",
                           params[0]));
    }

    fn format_decimal(&self, params: Vec<String>) -> Result<String, String> {
        if let Ok(decimal) = params[0].parse::<f64>() {
            if let Ok(digits) = params[1].parse::<usize>() {
                return Ok(format!("{:.*}", digits, decimal));
            }
            return Err(format!("the value \"{}\" is an invalid number of digits", params[1]));
        }
        return Err(format!("the value \"{}\" is an invalid number", params[0]));
    }

    fn run(&mut self, params: Vec<String>) -> Result<String, String> {
        Ok(self.parse(&params[0]))
    }

    fn ask(&self, params: Vec<String>) -> Result<String, String> {
        print!(">");
        io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut i = input.to_owned();
                i.pop();
                Ok(i)
            }
            Err(error) => Err(error.to_string()),
        }
    }
}
