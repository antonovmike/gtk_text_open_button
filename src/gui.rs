use std::borrow::BorrowMut;
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

    // Create Open sButton and attach it to grid
    let open_button = gtk::Button::with_label("Open button");
    grid.attach(&open_button, 0, 0, 1, 1);
    
    // Create Save Button and attach it to grid
    let save_button = gtk::Button::with_label("Save button");
    grid.attach(&save_button, 1, 0, 1, 1);

    let quit_button = gtk::Button::with_label("Quit");
    quit_button.connect_clicked(clone!(@weak window => move |_| 
        unsafe {
            window.destroy()
        }
    ));
    grid.attach(&quit_button, 2, 0, 1, 1);

    // Create Text Viewer and attach it to grid
    // Height has to be at least 4
    let text_view = gtk::TextView::new();
    grid.attach(&text_view, 0, 1, 3, 4);

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

    // SAVE BUTTON
    //v1 does not work
    // save_button.connect_clicked(move |_| {
        //
    // });
    //v2 does not work
    // save_button.connect_clicked( 
        // clone!( // clone the references with a macro
            // @strong text_view => move |_| {
            // @weak text_view => move |_| {
            // move |_| {
            // invoke handler with mutable references
            // handle_save(text_view.borrow_mut())
            // text_view.handle_save();
        // })
    //   );
    
    window.show_all();
}