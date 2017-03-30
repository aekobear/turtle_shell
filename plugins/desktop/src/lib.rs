extern crate plugin;

use std::process::Command;
use plugin::*;

pub struct Wallpaper;

impl Plugin for Wallpaper {
    fn messages(&self) -> Vec<Message> {
        vec![Message {
                 name: "set_wallpaper".to_string(),
                 params: vec![Param("filepath".to_string(), Value::Text(String::new()))],
             },
             Message {
                 name: "get_wallpaper".to_string(),
                 params: vec![],
             }]
    }

    fn send(&self, m: Message) -> Value {
        match m.name.as_ref() {
            "get_wallpaper" => Value::Text(get_wallpaper()),
            _ => Value::Boolean(false),
        }
    }
}

fn get_wallpaper() -> String {
    let output = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.background")
        .arg("picture-uri")
        .output()
        .expect("failed to execute gsettings");
    if let Some(code) = output.status.code() {
        //println!("exit status: {}", code);
    }
    if let Ok(error) = String::from_utf8(output.stderr) {
        //println!("error: {}", error);
    }
    if let Ok(message) = String::from_utf8(output.stdout) {
        return message;
    } else {
        return String::new();
    }
}

fn set_wallpaper(filepath: &str) {
    let output = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.background")
        .arg("picture-uri")
        .arg(filepath)
        .output()
        .expect("failed to execute gsettings");
    if let Some(code) = output.status.code() {
        //println!("exit status: {}", code);
    }
    if let Ok(error) = String::from_utf8(output.stderr) {
        //println!("error: {}", error);
    }
    if let Ok(message) = String::from_utf8(output.stdout) {
        //println!("message: {}", message);
    }
}
