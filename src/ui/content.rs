use adw::prelude::*;
use adw::{ ActionRow, EntryRow, PreferencesGroup, PreferencesPage };
use gtk4::{ Adjustment, Button, SpinButton, Stack };
use std::cell::{ Cell, RefCell };
use std::rc::Rc;

use crate::backend::hyprland;
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
        "general:border_size",
        0,
        10,
        state.clone(),
        |s| s.border_size,
        |s, v| { s.border_size = v; },
        &mut refreshers
    );

    add_spin(
        &group,
        "Gaps In",
        "Gaps between windows",
        "general:gaps_in",
        0,
        50,
        state.clone(),
        |s| s.gaps_in,
        |s, v| { s.gaps_in = v; },
        &mut refreshers
    );

    add_spin(
        &group,
        "Gaps Out",
        "Gaps between windows and edges",
        "general:gaps_out",
        0,
        50,
        state.clone(),
        |s| s.gaps_out,
        |s, v| { s.gaps_out = v; },
        &mut refreshers
    );

    add_entry(
        &group,
        "Active Border Color",
        "general:col.active_border",
        state.clone(),
        |s| s.active_border.clone(),
        |s, v| s.active_border = v,
        &mut refreshers
    );

    add_entry(
        &group,
        "Inactive Border Color",
        "general:col.inactive_border",
        state.clone(),
        |s| s.inactive_border.clone(),
        |s, v| s.inactive_border = v,
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

fn add_entry(
    group: &PreferencesGroup,
    title: &str,
    hyprctl_key: &str,
    state: Rc<RefCell<GeneralState>>,
    getter: fn(&GeneralState) -> String,
    setter: fn(&mut GeneralState, String),
    refreshers: &mut Vec<Rc<dyn Fn()>>
) {
    let entry = EntryRow::new();
    entry.set_title(title);
    entry.set_text(&getter(&state.borrow()));

    let original = Rc::new(RefCell::new(getter(&state.borrow())));

    let revert_btn = Button::from_icon_name("edit-undo-symbolic");
    revert_btn.set_valign(gtk4::Align::Center);
    revert_btn.add_css_class("flat");
    revert_btn.set_visible(false);
    entry.add_suffix(&revert_btn);

    // On text change: update state, apply live, toggle revert
    {
        let state_clone = state.clone();
        let original = original.clone();
        let revert_btn = revert_btn.clone();
        let key = hyprctl_key.to_string();
        entry.connect_changed(move |e: &adw::EntryRow| {
            let text = e.text().to_string();
            setter(&mut state_clone.borrow_mut(), text.clone());
            hyprland::apply_keyword(&key, &text);
            revert_btn.set_visible(text != *original.borrow());
        });
    }

    // On revert click: restore original value
    {
        let entry = entry.clone();
        let original = original.clone();
        revert_btn.connect_clicked(move |_| {
            let val = original.borrow().clone();
            entry.set_text(&val);
        });
    }

    // Refresh: update widget and original value, hide revert
    let refresh = {
        let entry = entry.clone();
        let state = state.clone();
        let revert_btn = revert_btn.clone();
        let original = original.clone();
        Rc::new(move || {
            let val = getter(&state.borrow());
            *original.borrow_mut() = val.clone();
            revert_btn.set_visible(false);
            entry.set_text(&val);
        })
    };

    refreshers.push(refresh);
    group.add(&entry);
}

fn add_spin(
    group: &PreferencesGroup,
    title: &str,
    subtitle: &str,
    hyprctl_key: &str,
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

    let original = Rc::new(Cell::new(getter(&state.borrow())));

    let revert_btn = Button::from_icon_name("edit-undo-symbolic");
    revert_btn.set_valign(gtk4::Align::Center);
    revert_btn.add_css_class("flat");
    revert_btn.set_visible(false);

    let row = ActionRow::new();
    row.set_title(title);
    row.set_subtitle(subtitle);
    row.add_suffix(&revert_btn);
    row.add_suffix(&spin);
    row.set_activatable(false);

    // On value change: update state, apply live, toggle revert
    {
        let state_clone = state.clone();
        let original = original.clone();
        let revert_btn = revert_btn.clone();
        let key = hyprctl_key.to_string();
        spin.connect_value_changed(move |s: &SpinButton| {
            let v = s.value() as i32;
            setter(&mut state_clone.borrow_mut(), v);
            hyprland::apply_keyword(&key, &v.to_string());
            revert_btn.set_visible(v != original.get());
        });
    }

    // On revert click: restore original value
    {
        let spin = spin.clone();
        let original = original.clone();
        revert_btn.connect_clicked(move |_| {
            spin.set_value(original.get() as f64);
        });
    }

    // Refresh: update widget and original value, hide revert
    let refresh = {
        let spin = spin.clone();
        let state = state.clone();
        let revert_btn = revert_btn.clone();
        let original = original.clone();
        Rc::new(move || {
            let val = getter(&state.borrow());
            original.set(val);
            revert_btn.set_visible(false);
            spin.set_value(val as f64);
        })
    };

    refreshers.push(refresh);
    group.add(&row);
}
