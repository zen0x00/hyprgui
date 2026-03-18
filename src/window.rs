use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use adw::{ ApplicationWindow, HeaderBar };
use gtk4::{ Box, Orientation, Separator };

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

    // Header bar
    let header = HeaderBar::new();
    header.set_title_widget(Some(&gtk4::Label::builder()
        .label("HyprGUI")
        .css_classes(["title"])
        .build()
    ));

    // Sidebar + separator + content
    let sidebar = sidebar::build();
    let content = content::build(state.clone());

    (content.refresh_ui)();

    sidebar.list.select_row(sidebar.list.row_at_index(0).as_ref());

    let stack = content.stack.clone();
    sidebar.list.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            stack.set_visible_child_name(&row.widget_name());
        }
    });

    let main_box = Box::new(Orientation::Horizontal, 0);
    main_box.append(&sidebar.root);
    main_box.append(&Separator::new(Orientation::Vertical));
    main_box.append(&content.stack);
    main_box.set_vexpand(true);

    // Footer
    let footer = footer::build(window.clone().upcast::<gtk4::Window>(), state, content.refresh_ui);

    // Root layout: header → content → footer
    let root = Box::new(Orientation::Vertical, 0);
    root.append(&header);
    root.append(&main_box);
    root.append(&Separator::new(Orientation::Horizontal));
    root.append(&footer);

    window.set_content(Some(&root));
    window
}
