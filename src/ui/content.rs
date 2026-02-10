use adw::prelude::*;
use adw::{ ActionRow, PreferencesGroup, PreferencesPage };
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
    stack.set_hexpand(true);
    stack.set_vexpand(true);

    let (general_page, refresh_general) = general_page(state);
    stack.add_named(&general_page, Some("general"));

    Content {
        stack,
        refresh_ui: refresh_general,
    }
}

fn general_page(state: Rc<RefCell<GeneralState>>) -> (PreferencesPage, Rc<dyn Fn()>) {
    let page = PreferencesPage::new();
    page.set_title("General");

    let group = PreferencesGroup::new();
    group.set_title("General Settings");

    let mut refreshers: Vec<Rc<dyn Fn()>> = Vec::new();

    add_spin(
        &group,
        "Border Size",
        "Size of the border around windows",
        0,
        10,
        state.clone(),
        |s| s.border_size,
        |s, v| {
            s.border_size = v;
        },
        &mut refreshers
    );

    add_spin(
        &group,
        "Gaps In",
        "Gaps between windows",
        0,
        50,
        state.clone(),
        |s| s.gaps_in,
        |s, v| {
            s.gaps_in = v;
        },
        &mut refreshers
    );

    add_spin(
        &group,
        "Gaps Out",
        "Gaps between windows and edges",
        0,
        50,
        state.clone(),
        |s| s.gaps_out,
        |s, v| {
            s.gaps_out = v;
        },
        &mut refreshers
    );

    page.add(&group);

    let refresh_all = Rc::new(move || {
        for r in &refreshers {
            r();
        }
    });

    (page, refresh_all)
}

fn add_spin(
    group: &PreferencesGroup,
    title: &str,
    subtitle: &str,
    min: i32,
    max: i32,
    state: Rc<RefCell<GeneralState>>,
    getter: fn(&GeneralState) -> i32,
    setter: fn(&mut GeneralState, i32),
    refreshers: &mut Vec<Rc<dyn Fn()>>
) {
    let adjustment = Adjustment::new(
        getter(&state.borrow()) as f64,
        min as f64,
        max as f64,
        1.0,
        1.0,
        0.0
    );

    let spin = SpinButton::new(Some(&adjustment), 1.0, 0);

    let row = ActionRow::new();
    row.set_title(title);
    row.set_subtitle(subtitle);
    row.add_suffix(&spin);
    row.set_activatable(false);

    let state_clone = state.clone();
    spin.connect_value_changed(move |s: &SpinButton| {
        setter(&mut state_clone.borrow_mut(), s.value() as i32);
    });

    let refresh = {
        let spin = spin.clone();
        let state = state.clone();
        Rc::new(move || {
            spin.set_value(getter(&state.borrow()) as f64);
        })
    };

    refreshers.push(refresh);
    group.add(&row);
}
