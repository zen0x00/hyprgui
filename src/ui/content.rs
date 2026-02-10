use adw::prelude::*;
use adw::{ PreferencesPage, PreferencesGroup, ActionRow };
use gtk4::{ Adjustment, SpinButton, Stack };
use std::cell::RefCell;
use std::rc::Rc;

use crate::state::GeneralState;

pub struct Content {
    pub stack: Stack,
    pub refresh_ui: Rc<dyn Fn()>,
}

pub fn build(state: Rc<RefCell<GeneralState>>) -> Content {
    let stack = Stack::new();

    let (general_page, refresh_general_ui) = general_page(state.clone());
    stack.add_named(&general_page, Some("general"));

    stack.set_hexpand(true);
    stack.set_vexpand(true);

    Content {
        stack,
        refresh_ui: refresh_general_ui,
    }
}

fn general_page(state: Rc<RefCell<GeneralState>>) -> (PreferencesPage, Rc<dyn Fn()>) {
    let page = PreferencesPage::new();
    page.set_title("General");

    let group = PreferencesGroup::new();
    group.set_title("General Settings");

    let adjustment = Adjustment::new(1.0, 0.0, 50.0, 1.0, 1.0, 0.0);
    let spin = SpinButton::new(Some(&adjustment), 1.0, 0);
    spin.set_numeric(true);

    // 🔑 INIT FROM STATE
    spin.set_value(state.borrow().border_size as f64);

    // 🔑 WRITE BACK TO STATE
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

    // 🔑 REFRESH UI FROM STATE (NOT DEFAULTS)
    let refresh_ui = {
        let spin = spin.clone();
        let state = state.clone();
        Rc::new(move || {
            spin.set_value(state.borrow().border_size as f64);
        })
    };

    (page, refresh_ui)
}
