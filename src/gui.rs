// use std::borrow::BorrowMut;
use gtk::Window;
use gtk::WindowType;
use gtk::FileChooserAction;
use gtk::ResponseType;
use std::path::PathBuf;
use gtk::FileChooserDialog;
use std::io::BufWriter;
use std::path::Path;
use gtk::glib;
use glib::clone;
use gtk::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{
    // fs::{self, File},
    io::{Result, Write},
    // path::Path,
};

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
    // https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.FileChooserDialog.html
    // https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.FileChooserDialog.html
    // https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.FileChooserNative.html
    save_button.connect_clicked(glib::clone!(@weak window => move |_| {
    	let file_saver = gtk::FileChooserDialog::new(
    		Some("Save File"),
    		Some(&window),
    		gtk::FileChooserAction::Save,
    	);
        file_saver.add_buttons(&[
            ("Save", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);
        // Save function
        // file_saver.connect_response(glib::clone!(@weak text_view => move |file_chooser, response| {
        //     // implement
        //     file_saver.close();
        // }));

        file_saver.show_all();
    }));
    
    window.show_all();
}

// CHECK
// https://developer-old.gnome.org/gtk3/unstable/GtkFileChooser.html

pub struct SaveDialog(FileChooserDialog);

impl SaveDialog {
    pub fn new(path: Option<PathBuf>) -> SaveDialog {
// New popup menu dialogue
        let save_dialog = FileChooserDialog::new(
            Some("Save As"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
        );

// Add cancel, save buttons to dialogue
        save_dialog.add_button("Cancel", ResponseType::Cancel.into());
        save_dialog.add_button("Save", ResponseType::Ok.into());

// Default open file path
        path.map(|p| save_dialog.set_current_folder(p));

        SaveDialog(save_dialog)
    }
}