use adw::prelude::*;
use adw::Application;
use gtk4::{ CssProvider };

use crate::window;

pub fn build(app: &Application) {
    load_css();

    let win = window::build(app);
    win.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("assets/style.css");
}
