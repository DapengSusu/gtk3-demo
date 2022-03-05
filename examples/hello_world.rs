use gtk::prelude::*;
use gtk::{ Application, ApplicationWindow };

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            // .resizable(false)
            // .window_position(gtk::WindowPosition::Center)
            // .halign(gtk::Align::Baseline)
            // .valign(gtk::Align::Center)
            .build();

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
}
