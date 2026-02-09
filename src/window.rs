use adw::prelude::*;
use adw::ApplicationWindow;
use gtk4::{Box, Orientation};

use crate::ui::{sidebar, content};

pub fn build(app: &adw::Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("HyprGUI")
        .default_width(900)
        .default_height(600)
        .build();

    let root = Box::new(Orientation::Horizontal, 0);

    let sidebar = sidebar::build();
    let stack = content::build();

    sidebar.select_row(sidebar.row_at_index(0).as_ref());

    let stack_clone = stack.clone();
    sidebar.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            let name = row.widget_name();
            if !name.is_empty() {
                stack_clone.set_visible_child_name(&name);
            }
        }
    });

    root.append(&sidebar);
    root.append(&stack);

    window.set_content(Some(&root));
    window
}
