use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, ListBox, ListBoxRow};

pub fn build() -> ListBox {
    let list = ListBox::new();

    list.add_css_class("sidebar");
    list.set_selection_mode(gtk4::SelectionMode::Single);

    list.append(&row("General", "general"));
    list.append(&row("Decoration", "decoration"));
    list.append(&row("Input", "input"));

    list
}

fn row(label: &str, name: &str) -> ListBoxRow {
    let row = ListBoxRow::new();

    let box_ = Box::new(Orientation::Horizontal, 12);
    box_.set_margin_top(12);
    box_.set_margin_bottom(12);
    box_.set_margin_start(12);
    box_.set_margin_end(12);


    let text = Label::new(Some(label));
    text.set_xalign(0.0);

    row.set_widget_name(name);
    box_.append(&text);
    row.set_child(Some(&box_));

    row
}
