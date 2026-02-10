use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{ Box, Orientation, Button, MessageDialog, ButtonsType, ResponseType };

use crate::state::GeneralState;
use crate::backend::hyprland;

pub fn build(
    parent: gtk4::Window,
    state: Rc<RefCell<GeneralState>>,
    refresh_ui: Rc<dyn Fn()>
) -> Box {
    let footer = Box::new(Orientation::Horizontal, 12);
    footer.set_margin_top(8);
    footer.set_margin_bottom(8);
    footer.set_margin_start(12);
    footer.set_margin_end(12);
    footer.set_hexpand(true);
    footer.set_halign(gtk4::Align::End);

    let reset_btn = Button::with_label("Reset");
    let apply_btn = Button::with_label("Apply");

    reset_btn.add_css_class("destructive-action");
    apply_btn.add_css_class("suggested-action");

    // ---- RESET ----
    {
        let parent = parent.clone();
        let state = state.clone();
        let refresh_ui = refresh_ui.clone();

        reset_btn.connect_clicked(move |_| {
            let dialog = MessageDialog::new(
                Some(&parent),
                gtk4::DialogFlags::MODAL,
                gtk4::MessageType::Warning,
                ButtonsType::None,
                "Reset settings to defaults?"
            );

            dialog.add_button("Cancel", ResponseType::Cancel);
            dialog.add_button("Reset", ResponseType::Accept);

            let state = state.clone();
            let refresh_ui = refresh_ui.clone();

            dialog.connect_response(move |d, resp| {
                if resp == ResponseType::Accept {
                    *state.borrow_mut() = GeneralState::default();
                    refresh_ui();
                }
                d.close();
            });

            dialog.show();
        });
    }

    // ---- APPLY ----
    {
        let parent = parent.clone();
        let state = state.clone();
        let refresh_ui = refresh_ui.clone();

        apply_btn.connect_clicked(move |_| {
            let dialog = MessageDialog::new(
                Some(&parent),
                gtk4::DialogFlags::MODAL,
                gtk4::MessageType::Question,
                ButtonsType::None,
                "Apply changes to Hyprland?"
            );

            dialog.add_button("Cancel", ResponseType::Cancel);
            dialog.add_button("Apply", ResponseType::Accept);

            let state = state.clone();
            let refresh_ui = refresh_ui.clone();

            dialog.connect_response(move |d, resp| {
                if resp == ResponseType::Accept {
                    if hyprland::apply_general(&state.borrow()).is_ok() {
                        if let Ok(current) = hyprland::read_general() {
                            *state.borrow_mut() = current;
                            refresh_ui();
                        }
                    }
                }
                d.close();
            });

            dialog.show();
        });
    }

    footer.append(&reset_btn);
    footer.append(&apply_btn);

    footer
}
