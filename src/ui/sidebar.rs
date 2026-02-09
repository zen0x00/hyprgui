use gtk4::GestureClick;
use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, ListBox, ListBoxRow, ScrolledWindow, Align};


pub struct Sidebar {
    pub root: Box,
    pub list: ListBox,
}

pub fn build() -> Sidebar {
    let root = Box::new(Orientation::Vertical, 0);
    root.add_css_class("sidebar");
    root.set_size_request(240, -1);

    // ---- List ----
    let list = build_list();

    let scroller = ScrolledWindow::new();
    scroller.set_vexpand(true);
    scroller.set_policy(
        gtk4::PolicyType::Never,
        gtk4::PolicyType::Automatic,
    );
    scroller.set_child(Some(&list));

    root.append(&scroller);

    Sidebar { root, list }
}

fn build_list() -> ListBox {
    let list = ListBox::new();
    list.add_css_class("sidebar-list");
    list.set_selection_mode(gtk4::SelectionMode::Single);

    // -------- Hyprland section --------
    let hyprland_children = hyprland_children();

    let list_clone = list.clone();
    let children_clone = hyprland_children.clone();

    let hyprland_header = section_header("Hyprland", move || {
        collapse_all(&list_clone);
        set_section_visible(&children_clone, true);
        list_clone.select_row(children_clone.first());
    });

    list.append(&hyprland_header);
    for row in &hyprland_children {
        list.append(row);
    }

    // -------- Initial state --------
    set_section_visible(&hyprland_children, true);
    list.select_row(hyprland_children.first());

    list
}

fn hyprland_children() -> Vec<ListBoxRow> {
    vec![
        child_row("General", "general"),
        child_row("Decoration", "decoration"),
        child_row("Animations", "animations"),
        child_row("Input", "input"),
        child_row("Gestures", "gestures"),
        child_row("Group", "group"),
        child_row("Groupbar", "groupbar"),
        child_row("Misc", "misc"),
        child_row("Binds", "binds"),
        child_row("XWayland", "xwayland"),
        child_row("OpenGL", "opengl"),
        child_row("Render", "render"),
        child_row("Cursor", "cursor"),
        child_row("Ecosystem", "ecosystem"),
        child_row("Quirks", "quirks"),
    ]
}

fn hyprlock_children() -> Vec<ListBoxRow> {
    vec![
        child_row("General", "general"),
        child_row("Appearance", "appearance"),
        child_row("Background", "background"),
        child_row("Grub", "grub"),
        child_row("Misc", "misc"),
    ]
}

fn section_header(
    title: &str,
    on_click: impl Fn() + 'static,
) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_selectable(false);
    row.set_activatable(false);

    let label = Label::new(Some(title));
    label.set_xalign(0.0);
    label.set_margin_top(16);
    label.set_margin_bottom(6);
    label.set_margin_start(12);
    label.set_margin_end(12);
    label.add_css_class("heading");

    let gesture = GestureClick::new();
    gesture.connect_pressed(move |_, _, _, _| {
        on_click();
    });

    row.add_controller(gesture);
    row.set_child(Some(&label));
    row
}

fn child_row(label: &str, name: &str) -> ListBoxRow {
    let row = ListBoxRow::new();

    let box_ = Box::new(Orientation::Horizontal, 8);
    box_.set_margin_top(6);
    box_.set_margin_bottom(6);
    box_.set_margin_start(28);
    box_.set_margin_end(12);

    let text = Label::new(Some(label));
    text.set_xalign(0.0);
    text.set_halign(Align::Start);

    row.set_widget_name(name);
    box_.append(&text);
    row.set_child(Some(&box_));

    row
}

fn set_section_visible(rows: &[ListBoxRow], visible: bool) {
    for row in rows {
        row.set_visible(visible);
    }
}

fn collapse_all(list: &ListBox) {
    let mut child = list.first_child();

    while let Some(widget) = child {
        // grab next sibling BEFORE moving widget
        child = widget.next_sibling();

        if let Ok(row) = widget.downcast::<ListBoxRow>() {
            if row.is_selectable() {
                row.set_visible(false);
            }
        }
    }
}

