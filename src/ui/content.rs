use adw::prelude::*;
use adw::{PreferencesPage, PreferencesGroup};
use gtk4::Stack;

pub fn build() -> Stack {
    let stack = Stack::new();

    stack.set_hexpand(true);
    stack.set_vexpand(true);

    stack.add_named(&general_page(), Some("general"));
    stack.add_named(&decoration_page(), Some("decoration"));
    stack.add_named(&input_page(), Some("input"));

    stack
}
fn general_page() -> PreferencesPage {
    let page = PreferencesPage::new();
    page.set_title("General");

    let group = PreferencesGroup::new();
    group.set_title("Layout & Borders");

    page.add(&group);
    page
}

fn decoration_page() -> PreferencesPage {
    let page = PreferencesPage::new();
    page.set_title("Decoration");

    let group = PreferencesGroup::new();
    group.set_title("Appearance");

    page.add(&group);
    page
}

fn input_page() -> PreferencesPage {
    let page = PreferencesPage::new();
    page.set_title("Input");

    let group = PreferencesGroup::new();
    group.set_title("Keyboard & Mouse");

    page.add(&group);
    page
}
