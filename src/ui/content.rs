use adw::prelude::*;
use adw::{ PreferencesPage, PreferencesGroup, ActionRow };
use gtk4::{ Adjustment, SpinButton };
use gtk4::Stack;
use std::rc::Rc;
use std::cell::RefCell;
use crate::state::GeneralState;

pub fn build(state: Rc<RefCell<GeneralState>>) -> Stack {
    let stack = Stack::new();

    stack.add_named(&general_page(state.clone()), Some("general"));

    stack.set_hexpand(true);
    stack.set_vexpand(true);

    stack
}

pub fn general_page(state: Rc<RefCell<GeneralState>>) -> PreferencesPage {
    let page = PreferencesPage::new();
    page.set_title("General");

    let group = PreferencesGroup::new();
    group.set_title("General Settings");

    let adjustment = Adjustment::new(1.0, 0.0, 10.0, 1.0, 1.0, 0.0);
    let spin = SpinButton::new(Some(&adjustment), 1.0, 0);
    spin.set_numeric(true);

    // read from state
    spin.set_value(state.borrow().border_size as f64);

    // write to state
    let state_clone = state.clone();
    spin.connect_value_changed(move |spin| {
        state_clone.borrow_mut().border_size = spin.value() as i32;
    });

    let row = ActionRow::new();
    row.set_title("Border Size");
    row.set_subtitle("Size of the border around windows");
    row.add_suffix(&spin);
    row.set_activatable(false);

    group.add(&row);
    page.add(&group);

    page
}
