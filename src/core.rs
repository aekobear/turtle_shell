use TurtleShell;

impl<'a> TurtleShell<'a> {
    pub fn name(&self) -> String {
        "core".to_string()
    }
    pub fn load(&self) -> bool {
        true
    }
    pub fn messages(&self) -> Vec<String> {
        vec!["+", "-", "/", "*", "echo", "s", "exit"].iter().map(|&s| s.to_owned()).collect()
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
            "echo" => params.join(" "),
            "s" => params.join(""),
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
}
