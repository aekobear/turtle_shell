use std::env;

pub trait Turbin {
    fn name(&self) -> String;
    fn load(&self) -> bool;
    fn messages(&self) -> Vec<&str>;
    fn receive(&self, message: &str, params: Vec<String>) -> String;
}

pub struct TurbinHandler<'a> {
    turbin: &'a Turbin,
}

impl<'a> TurbinHandler<'a> {
    pub fn new(turbin: &'a Turbin) -> TurbinHandler {
        TurbinHandler { turbin: turbin }
    }

    pub fn run(&self) {
        let args = env::args().collect::<Vec<String>>();
        if args.len() > 1 {
            let mc: &str = &args[1];
            match mc {
                "name" => println!("{}", self.turbin.name()),
                "load" => println!("{}", self.turbin.load()),
                "messages" => println!("{}", self.turbin.messages().join(" ")),
                "receive" => println!("{}", self.turbin.receive(&args[2], args[3..].to_vec())),
                mc => {
                    println!("this version of turbin does not support the \"{}\" meta-command!",
                             mc)
                }
            }
        } else {
            println!("{} turbin loadable: {}",
                     self.turbin.name(),
                     self.turbin.load());
        }
    }
}
