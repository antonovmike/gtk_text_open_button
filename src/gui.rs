use gtk::glib;
use gtk::prelude::*;

pub fn build_ui(application: &gtk::Application) {
    // Create a new window, set it's title and default size
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Text viewer");
    window.set_default_size(200, 200);
    
    let margin = 6;

    // Construct the grid that is going contain buttons
    let grid = gtk::Grid::builder()
        .margin_start(margin)
        .margin_end(margin)
        .margin_top(margin)
        .margin_bottom(margin)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(margin)
        .column_spacing(margin)
        .build();

    // Add the grid in the window
    window.set_child(Some(&grid));

    // Create Button and attach it to grid
    let open_button = gtk::Button::with_label("Open button");
    grid.attach(&open_button, 0, 1, 1, 1);

    open_button.connect_clicked(glib::clone!(@weak window => move |_| {
        // Create file-opener
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            gtk::FileChooserAction::Open,
        );
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);
        file_chooser.show_all();
    }));

    window.show_all();
}
