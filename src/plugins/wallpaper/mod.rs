
use std::process::Command;

pub fn get_wallpaper() -> String {
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

pub fn set_wallpaper(filepath: &str) {
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
