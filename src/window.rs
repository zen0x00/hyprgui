use adw::prelude::*;
use adw::ApplicationWindow;
use gtk4::{Box, Orientation, Align};

use crate::ui::{sidebar, content};

pub fn build(app: &adw::Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("HyprGUI")
        .default_width(900)
        .default_height(600)
        .build();

    // Root vertical layout
    let root = Box::new(Orientation::Vertical, 0);

    // -------- Top Bar --------
    let top_bar = Box::new(Orientation::Horizontal, 0);
    top_bar.set_height_request(56);
    top_bar.set_halign(Align::Fill);
    top_bar.set_valign(Align::Center);
    top_bar.add_css_class("top-bar");

    let title = gtk4::Label::new(Some("HyprGUI"));
    title.set_hexpand(true);
    title.set_halign(Align::Center);
    title.set_valign(Align::Center);
    title.add_css_class("app-title");

    top_bar.append(&title);
    root.append(&top_bar);

    // -------- Main Area --------
    let main = Box::new(Orientation::Horizontal, 0);

    let sidebar = sidebar::build();
    let stack = content::build();

    // initial selection
    sidebar
        .list
        .select_row(sidebar.list.row_at_index(0).as_ref());

    let stack_clone = stack.clone();
    sidebar.list.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            let name = row.widget_name();
            stack_clone.set_visible_child_name(&name);
        }
    });

    main.append(&sidebar.root);
    main.append(&stack);

    root.append(&main);

    window.set_content(Some(&root));
    window
}
