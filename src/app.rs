use adw::prelude::*;
use adw::Application;
use gtk4::{CssProvider, StyleContext};
use gtk4::gdk::Display;

use crate::window;

pub fn build(app: &Application) {
    load_css();

    let win = window::build(app);
    win.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("assets/style.css");

    StyleContext::add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
