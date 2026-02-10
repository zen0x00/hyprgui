use adw::prelude::*;
use adw::{ PreferencesPage, PreferencesGroup, ActionRow };
use gtk4::{ Adjustment, SpinButton };
use gtk4::Stack;
use std::rc::Rc;
use std::cell::RefCell;

use crate::state::GeneralState;

pub struct Content {
    pub stack: Stack,
    pub reset_ui: Rc<dyn Fn()>,
}

pub fn build(state: Rc<RefCell<GeneralState>>) -> Content {
    let stack = Stack::new();

    let (general_page, reset_general_ui) = general_page(state.clone());
    stack.add_named(&general_page, Some("general"));

    stack.set_hexpand(true);
    stack.set_vexpand(true);

    Content {
        stack,
        reset_ui: reset_general_ui, // ✅ just forward it
    }
}

pub fn general_page(state: Rc<RefCell<GeneralState>>) -> (PreferencesPage, Rc<dyn Fn()>) {
    let page = PreferencesPage::new();
    page.set_title("General");

    let group = PreferencesGroup::new();
    group.set_title("General Settings");

    let adjustment = Adjustment::new(1.0, 0.0, 10.0, 1.0, 1.0, 0.0);
    let spin = SpinButton::new(Some(&adjustment), 1.0, 0);
    spin.set_numeric(true);

    // init from state
    spin.set_value(state.borrow().border_size as f64);

    // write back to state
    let state_clone = state.clone();
    spin.connect_value_changed(move |s| {
        state_clone.borrow_mut().border_size = s.value() as i32;
    });

    let row = ActionRow::new();
    row.set_title("Border Size");
    row.set_subtitle("Size of the border around windows");
    row.add_suffix(&spin);
    row.set_activatable(false);

    group.add(&row);
    page.add(&group);

    // ✅ reset closure lives where `spin` lives
    let reset_ui: Rc<dyn Fn()> = {
        let spin = spin.clone();
        Rc::new(move || {
            spin.set_value(GeneralState::default().border_size as f64);
        })
    };

    (page, reset_ui)
}
