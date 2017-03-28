use std::process::Command;

mod plugins;

use plugins::wallpaper;

fn main() {
    println!("Hello, world!");
    println!("{}", wallpaper::get_wallpaper());
}
