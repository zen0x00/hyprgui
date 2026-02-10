use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use adw::ApplicationWindow;
use gtk4::{ Box, Orientation, Align };

use crate::ui::{ sidebar, content, footer };
use crate::state::GeneralState;
use crate::backend::hyprland;

pub fn build(app: &adw::Application) -> ApplicationWindow {
    // ✅ READ FROM HYPRLAND ON STARTUP
    let initial_state = hyprland::read_general().unwrap_or_else(|_| GeneralState::default());

    let general_state = Rc::new(RefCell::new(initial_state));

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

    // ---- Main ----
    let main = Box::new(Orientation::Horizontal, 0);

    let sidebar = sidebar::build();
    let content = content::build(general_state.clone());

    sidebar.list.select_row(sidebar.list.row_at_index(0).as_ref());

    let stack = content.stack.clone();
    sidebar.list.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            stack.set_visible_child_name(&row.widget_name());
        }
    });

    main.append(&sidebar.root);
    main.append(&content.stack);

    // ---- Footer ----
    let footer = footer::build(
        window.clone().upcast::<gtk4::Window>(),
        general_state.clone(),
        content.refresh_ui.clone()
    );

    root.append(&main);
    root.append(&footer);

    window.set_content(Some(&root));

    // 🔑 FORCE INITIAL UI SYNC
    (content.refresh_ui)();

    window
}
