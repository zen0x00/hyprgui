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

    // ---- gaps_in ----
    let gaps_in_adjustment = Adjustment::new(5.0, 0.0, 50.0, 1.0, 5.0, 0.0);
    let gaps_in_spin = SpinButton::new(Some(&gaps_in_adjustment), 1.0, 0);
    gaps_in_spin.set_numeric(true);

    // init from state
    gaps_in_spin.set_value(state.borrow().gaps_in as f64);

    // write back to state
    let state_clone = state.clone();
    gaps_in_spin.connect_value_changed(move |s| {
        state_clone.borrow_mut().gaps_in = s.value() as i32;
    });

    let gaps_in_row = ActionRow::new();
    gaps_in_row.set_title("Gaps In");
    gaps_in_row.set_subtitle("Gaps between windows");
    gaps_in_row.add_suffix(&gaps_in_spin);
    gaps_in_row.set_activatable(false);

    group.add(&gaps_in_row);

    // ---- gaps_out----
    let gaps_out_adjustment = Adjustment::new(5.0, 0.0, 50.0, 1.0, 5.0, 0.0);
    let gaps_out_spin = SpinButton::new(Some(&gaps_out_adjustment), 1.0, 0);
    gaps_out_spin.set_numeric(true);

    // init from state
    gaps_out_spin.set_value(state.borrow().gaps_in as f64);

    // write back to state
    let state_clone = state.clone();
    gaps_out_spin.connect_value_changed(move |s| {
        state_clone.borrow_mut().gaps_out = s.value() as i32;
    });

    let gaps_out_row = ActionRow::new();
    gaps_out_row.set_title("Gaps Out");
    gaps_out_row.set_subtitle("Gaps between windows");
    gaps_out_row.add_suffix(&gaps_out_spin);
    gaps_out_row.set_activatable(false);

    group.add(&gaps_out_row);

    // ✅ reset closure lives where `spin` lives
    let reset_ui: Rc<dyn Fn()> = {
        let border_spin = SpinButton::new(Some(&adjustment), 1.0, 0);
        border_spin.set_numeric(true);
        let gaps_in_spin = gaps_in_spin.clone();
        let gaps_out_spin = gaps_out_spin.clone();

        Rc::new(move || {
            let defaults = GeneralState::default();
            border_spin.set_value(defaults.border_size as f64);
            gaps_in_spin.set_value(defaults.gaps_in as f64);
            gaps_out_spin.set_value(defaults.gaps_out as f64);
        })
    };

    (page, reset_ui)
}
