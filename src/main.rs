use std::process::Command;

mod plugins;

use plugins::*;
use plugins::wallpaper::*;

fn main() {
    let w = Wallpaper {};
    for message in w.messages() {
        println!("message: {}", message);
    }
    if let Value::Text(response) = w.send(Message::new("get_wallpaper".to_string())) {
        println!("get_wallpaper: {}", response);
    }
}
