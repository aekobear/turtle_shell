extern crate plugin;
extern crate desktop;

use std::process::Command;

use plugin::*;
use desktop::*;

fn main() {
    let w = Wallpaper {};
    for message in w.messages() {
        println!("message: {}", message);
    }
    if let Value::Text(response) = w.send(Message::new("get_wallpaper".to_string())) {
        println!("get_wallpaper: {}", response);
    }
}
