extern crate turbin;

use std::process::Command;

fn main() {
    let d = Desktop {};
    turbin::TurbinHandler::new(&d).run();
}

struct Desktop {}

impl turbin::Turbin for Desktop {
    fn name(&self) -> String {
        "desktop".to_string()
    }

    fn load(&self) -> bool {
        true
    }

    fn messages(&self) -> Vec<&str> {
        vec!["get_wallpaper", "set_wallpaper"]
    }

    fn receive(&self, message: &str, params: Vec<String>) -> String {
        match message {
            "get_wallpaper" => Desktop::get_wallpaper(),
            "set_wallpaper" => {
                if let Some(first) = params.first() {
                    println!("setting: {}", first);
                    Desktop::set_wallpaper(first);
                };
                String::new()
            }
            _ => {
                format!("desktop plugin does not support the \"{}\" message",
                        message)
            }
        }
    }
}

impl Desktop {
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
        if let Ok(mut message) = String::from_utf8(output.stdout) {
            message.pop();
            return message;
        } else {
            return String::new();
        }
    }

    fn set_wallpaper(filepath: &str) {
        let output = Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri")
            .arg(filepath)
            .output()
            .expect("failed to execute gsettings");
        if let Some(code) = output.status.code() {
            //println!("exit status: {}", code);
        }
        if let Ok(error) = String::from_utf8(output.stderr) {
            println!("error: {}", error);
        }
        if let Ok(message) = String::from_utf8(output.stdout) {
            //println!("message: {}", message);
        }
    }
}
