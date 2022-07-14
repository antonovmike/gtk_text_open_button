use gtk::glib;
use glib::clone;
use gtk::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
        // .valign(gtk::Align::Center)
        .row_spacing(margin)
        .column_spacing(margin)
        .build();

    // Add the grid in the window
    window.set_child(Some(&grid));

    // Create Button and attach it to grid
    let open_button = gtk::Button::with_label("Open button");
    grid.attach(&open_button, 0, 0, 1, 1);
    
    let quit_button = gtk::Button::with_label("Quit");
    quit_button.connect_clicked(clone!(@weak window => move |_| 
        unsafe {
            window.destroy()
        }
    ));
    grid.attach(&quit_button, 1, 0, 1, 1);

    let text_view = gtk::TextView::new();
    grid.attach(&text_view, 0, 1, 2, 2);

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
        // Open function
        file_chooser.connect_response(glib::clone!(@weak text_view => move |file_chooser, response| {
            if response == gtk::ResponseType::Ok {
                let filename = file_chooser.filename().expect("Couldn't get filename");
                let file = File::open(&filename).expect("Couldn't open file");

                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);

                text_view
                    .buffer()
                    .expect("Couldn't get window")
                    .set_text(&contents);
            }
            file_chooser.close();
        }));
        
        file_chooser.show_all();
    }));

    window.show_all();
}
