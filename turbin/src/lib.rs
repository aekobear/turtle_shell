use std::env;

pub trait Turbin {
    fn name(&self) -> String;
    fn load(&self) -> bool;
    fn messages(&self) -> Vec<&str>;
    fn receive(&self, message: &str, params: Vec<String>) -> String;
}

pub struct TurbinHandler<'a> { turbin: &'a Turbin }

impl <'a> TurbinHandler<'a> {
    pub fn new(turbin: &'a Turbin) -> TurbinHandler {
        TurbinHandler { turbin: turbin }
    }
    
    pub fn run(&self) {
        let args = env::args().collect::<Vec<String>>();
        if args.len() > 1 {
            let result = self.turbin.receive(&args[1], args[2..].to_vec());
            println!("{}", result);
        }
        else {
            println!("{} turbin loadable: {}", self.turbin.name(), self.turbin.load());
        }
    }
}
