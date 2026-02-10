use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{ Box, Orientation, Button, MessageDialog, ButtonsType, ResponseType };

use crate::state::GeneralState;
use crate::backend::hyprland;

pub fn build(state: Rc<RefCell<GeneralState>>) -> Box {
    let footer = Box::new(Orientation::Horizontal, 12);
    footer.set_margin_top(8);
    footer.set_margin_bottom(8);
    footer.set_margin_start(12);
    footer.set_margin_end(12);
    footer.set_hexpand(true);
    footer.set_halign(gtk4::Align::End);
    footer.add_css_class("footer-bar");

    let reset_btn = Button::with_label("Reset");
    let apply_btn = Button::with_label("Apply");

    reset_btn.add_css_class("destructive-action");
    apply_btn.add_css_class("suggested-action");

    // ---- Reset confirmation ----
    {
        let state = state.clone();
        reset_btn.connect_clicked(move |_| {
            let dialog = MessageDialog::new(
                None::<&gtk4::Window>,
                gtk4::DialogFlags::MODAL,
                gtk4::MessageType::Warning,
                ButtonsType::None,
                "Reset settings to defaults?"
            );

            dialog.add_button("Cancel", ResponseType::Cancel);
            dialog.add_button("Reset", ResponseType::Accept);

            // 🔑 CLONE AGAIN for the inner closure
            let state_inner = state.clone();
            dialog.connect_response(move |d, resp| {
                if resp == ResponseType::Accept {
                    *state_inner.borrow_mut() = GeneralState::default();
                }
                d.close();
            });

            dialog.show();
        });
    }
    // ---- Apply confirmation ----
    {
        let state = state.clone();
        apply_btn.connect_clicked(move |_| {
            let dialog = MessageDialog::new(
                None::<&gtk4::Window>,
                gtk4::DialogFlags::MODAL,
                gtk4::MessageType::Question,
                ButtonsType::None,
                "Apply changes to Hyprland?"
            );

            dialog.add_button("Cancel", ResponseType::Cancel);
            dialog.add_button("Apply", ResponseType::Accept);

            let state_inner = state.clone();
            dialog.connect_response(move |d, resp| {
                if resp == ResponseType::Accept {
                    let _ = hyprland::apply_general(&state_inner.borrow());
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
