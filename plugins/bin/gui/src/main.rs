extern crate turbin;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Entry, Box, Orientation, Window, WindowType};

fn main() {
    let mut gui = Gui(String::new());
    turbin::TurbinHandler::new(&mut gui).run();
}

struct Gui(String);

impl turbin::Turbin for Gui {
    fn name(&self) -> String {
        "gui".to_string()
    }

    fn load(&self) -> bool {
        true
    }

    fn messages(&self) -> Vec<&str> {
        vec!["gui"]
    }

    fn receive(&mut self, message: &str, params: Vec<String>) -> String {
        match message {
            "gui" => self.display(),
            _ => format!("gui plugin does not support the \"{}\" message", message),
        }
    }
}

impl Gui {
    fn set(&mut self, value: &str) {
        self.0 = value.to_owned();
    }
    fn display(&self) -> String {

        if gtk::init().is_err() {
            return "failed to initialize gtk.".to_owned();
        }

        let window = Window::new(WindowType::Toplevel);
        //window.set_decorated(false);
        window.set_title("turtle gui");
        window.set_default_size(350, 70);
        let b = Box::new(Orientation::Vertical, 5);
        let top = Button::new_with_label("top");
        let bottom = Button::new_with_label("bottom");
        let entry = Entry::new();
        b.add(&top);
        b.add(&entry);
        b.add(&bottom);
        window.add(&b);
        window.show_all();

        window.connect_delete_event(|_, _| {
                                        gtk::main_quit();
                                        Inhibit(false)
                                    });

        top.connect_clicked(|_| {
                                println!("Clicked!");
                            });


        entry.connect_activate(move |_| {
                                   gtk::main_quit();
                                   Inhibit(false);
                               });

        gtk::main();

        return if let Some(t) = entry.get_text() {
                   t
               } else {
                   String::new()
               };

    }
}
