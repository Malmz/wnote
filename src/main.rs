mod drawing;

use gtk4::prelude::*;

fn main() {
    let application = gtk4::Application::new(
        Some("com.github.gtk-rs.examples.widget_subclass"),
        Default::default(),
    );
    application.connect_activate(|app| {
        let window = gtk4::ApplicationWindow::new(app);
        let button = drawing::Drawing::new();
        button.set_margin_top(18);
        button.set_margin_bottom(18);
        button.set_margin_start(18);
        button.set_margin_end(18);
        window.set_child(Some(&button));
        window.show();
    });

    application.run();
}
