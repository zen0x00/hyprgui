use adw::prelude::*;
use adw::Application;

mod app;
mod window;
mod ui;

fn main() {
    let app = Application::builder()
        .application_id("hyprgui")
        .build();

    app.connect_activate(|app| {
        app::build(app);
    });

    app.run();
}
