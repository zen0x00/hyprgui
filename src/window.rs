use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use adw::ApplicationWindow;
use gtk4::{ Box, Orientation, Align };

use crate::ui::{ sidebar, content, footer };
use crate::state::GeneralState;

pub fn build(app: &adw::Application) -> ApplicationWindow {
    // ✅ state lives INSIDE the function
    let general_state = Rc::new(RefCell::new(GeneralState::default()));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("HyprGUI")
        .default_width(900)
        .default_height(600)
        .build();

    let root = Box::new(Orientation::Vertical, 0);

    // ---- Top bar ----
    let top_bar = Box::new(Orientation::Horizontal, 0);
    top_bar.set_height_request(56);

    let title = gtk4::Label::new(Some("HyprGUI"));
    title.set_hexpand(true);
    title.set_halign(Align::Center);
    title.set_valign(Align::Center);
    title.add_css_class("app-title");

    top_bar.append(&title);
    root.append(&top_bar);

    let main = Box::new(Orientation::Horizontal, 0);
    let sidebar = sidebar::build();
    let stack = content::build(general_state.clone());

    sidebar.list.select_row(sidebar.list.row_at_index(0).as_ref());

    let stack_clone = stack.clone();
    sidebar.list.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            let name = row.widget_name();
            stack_clone.set_visible_child_name(&name);
        }
    });

    main.append(&sidebar.root);
    main.append(&stack);

    // ---- Footer ----
    let footer = footer::build(general_state.clone());

    root.append(&main);
    root.append(&footer);

    main.append(&sidebar.root);
    main.append(&stack);

    root.append(&main);
    window.set_content(Some(&root));

    window
}
