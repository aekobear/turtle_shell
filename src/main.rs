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
