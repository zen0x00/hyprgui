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

    /* ───────── Border Size ───────── */

    let border_adj = Adjustment::new(1.0, 0.0, 50.0, 1.0, 1.0, 0.0);
    let border_spin = SpinButton::new(Some(&border_adj), 1.0, 0);
    border_spin.set_numeric(true);
    border_spin.set_value(state.borrow().border_size as f64);

    {
        let state = state.clone();
        border_spin.connect_value_changed(move |s| {
            state.borrow_mut().border_size = s.value() as i32;
        });
    }

    let border_row = ActionRow::new();
    border_row.set_title("Border Size");
    border_row.set_subtitle("Size of the border around windows");
    border_row.add_suffix(&border_spin);
    border_row.set_activatable(false);
    group.add(&border_row);

    /* ───────── Gaps In ───────── */

    let gaps_in_adj = Adjustment::new(5.0, 0.0, 100.0, 1.0, 1.0, 0.0);
    let gaps_in_spin = SpinButton::new(Some(&gaps_in_adj), 1.0, 0);
    gaps_in_spin.set_numeric(true);
    gaps_in_spin.set_value(state.borrow().gaps_in as f64);

    {
        let state = state.clone();
        gaps_in_spin.connect_value_changed(move |s| {
            state.borrow_mut().gaps_in = s.value() as i32;
        });
    }

    let gaps_in_row = ActionRow::new();
    gaps_in_row.set_title("Gaps In");
    gaps_in_row.set_subtitle("Gaps between windows");
    gaps_in_row.add_suffix(&gaps_in_spin);
    gaps_in_row.set_activatable(false);
    group.add(&gaps_in_row);

    /* ───────── Gaps Out ───────── */

    let gaps_out_adj = Adjustment::new(20.0, 0.0, 100.0, 1.0, 1.0, 0.0);
    let gaps_out_spin = SpinButton::new(Some(&gaps_out_adj), 1.0, 0);
    gaps_out_spin.set_numeric(true);
    gaps_out_spin.set_value(state.borrow().gaps_out as f64);

    {
        let state = state.clone();
        gaps_out_spin.connect_value_changed(move |s| {
            state.borrow_mut().gaps_out = s.value() as i32;
        });
    }

    let gaps_out_row = ActionRow::new();
    gaps_out_row.set_title("Gaps Out");
    gaps_out_row.set_subtitle("Gaps between windows and screen edges");
    gaps_out_row.add_suffix(&gaps_out_spin);
    gaps_out_row.set_activatable(false);
    group.add(&gaps_out_row);

    page.add(&group);

    /* ───────── Refresh UI from STATE ───────── */

    let refresh_ui = {
        let state = state.clone();
        let border_spin = border_spin.clone();
        let gaps_in_spin = gaps_in_spin.clone();
        let gaps_out_spin = gaps_out_spin.clone();

        Rc::new(move || {
            let s = state.borrow();
            border_spin.set_value(s.border_size as f64);
            gaps_in_spin.set_value(s.gaps_in as f64);
            gaps_out_spin.set_value(s.gaps_out as f64);
        })
    };

    (page, refresh_ui)
}
