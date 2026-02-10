use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use adw::ApplicationWindow;
use gtk4::{ Box, Orientation, Align };

use crate::backend::hyprland;
use crate::state::GeneralState;
use crate::ui::{ sidebar, content, footer };

pub fn build(app: &adw::Application) -> ApplicationWindow {
    let initial_state = hyprland::read_general().unwrap_or_else(|_| GeneralState::default());

    let state = Rc::new(RefCell::new(initial_state));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("HyprGUI")
        .default_width(900)
        .default_height(600)
        .build();

    let root = Box::new(Orientation::Vertical, 0);

    let main = Box::new(Orientation::Horizontal, 0);

    let sidebar = sidebar::build();
    let content = content::build(state.clone());

    (content.refresh_ui)(); // 🔑 initial sync

    sidebar.list.select_row(sidebar.list.row_at_index(0).as_ref());

    let stack = content.stack.clone();
    sidebar.list.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            stack.set_visible_child_name(&row.widget_name());
        }
    });

    main.append(&sidebar.root);
    main.append(&content.stack);

    let footer = footer::build(window.clone().upcast::<gtk4::Window>(), state, content.refresh_ui);

    root.append(&main);
    root.append(&footer);

    window.set_content(Some(&root));
    window
}
