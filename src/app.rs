use adw::prelude::*;
use adw::Application;
use gtk4::CssProvider;

use crate::window;

const CSS: &str = include_str!("../assets/style.css");

pub fn build(app: &Application) {
    load_css();

    let win = window::build(app);
    win.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(CSS);
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not get default display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
