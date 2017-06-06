use std;
use std::error::Error;
use std::io::BufReader;

use super::gtk;
use super::gtk::prelude::*;
use super::gtk::{Builder, Button, Entry, MenuItem, RadioButton, Window};

extern crate rodio;
use self::rodio::Source;

use state;

/** The global state of the program */
static mut STUFF: Option<Stuff> = None;

fn radio_dummy(button: &RadioButton) {
    if button.get_active() {
        /* safe because i ensure all buttons have label */
        println!("pressionado {}", button.get_label().unwrap());
    }
}

fn control_dummy(button: &Button) {
    /* safe because i ensure all buttons have label */
    println!("pressionado {}", button.get_label().unwrap());
}

/** Open and play an audio file */
fn play_sound(fname: &str) -> Result<(), Box<Error>> {
    let endpoint: Result<rodio::Endpoint, Box<Error>> =
        rodio::get_default_endpoint()
            .ok_or(From::from("Não foi possível abrir dispositivo de som."));
    let file: std::fs::File = std::fs::File::open(fname)?;
    let decoder = rodio::Decoder::new(BufReader::new(file))?;
    rodio::play_raw(&endpoint?, decoder.convert_samples());
    Ok(())
}
/** A struct to contain all the stuff */
#[derive(Debug, Clone)]
struct Stuff {
    /** The main window */
    main_window: Window,
    /** The open option on the file menu */
    file_menu_open: MenuItem,
    /** The save option on the file menu */
    file_menu_save: MenuItem,
    /** The quit option on the file menu */
    file_menu_quit: MenuItem,
    /** The text entry */
    text_entry_name: Entry,
    /** Phoneme button 1 */
    fonema1: RadioButton,
    /** Phoneme button 2 */
    fonema2: RadioButton,
    /** Phoneme button 3 */
    fonema3: RadioButton,
    /** Phoneme button 4 */
    fonema4: RadioButton,
    /** Phoneme button 5 */
    fonema5: RadioButton,
    /** Quality button 1 */
    qualidade1: RadioButton,
    /** Quality button 2 */
    qualidade2: RadioButton,
    /** Quality button 3 */
    qualidade3: RadioButton,
    /** Quality button 4 */
    qualidade4: RadioButton,
    /** Quality button 5 */
    qualidade5: RadioButton,
    /** Quality button 6 */
    qualidade6: RadioButton,
    /** Quality button 7 */
    qualidade7: RadioButton,
    /** The back button on the main window */
    control_button_back: Button,
    /** The play button on the main window */
    control_button_play: Button,
    /** The next button on the main window */
    control_button_next: Button,
    /** The file loader window */
    file_loader_window: Window,
    /** The cancel button on the file loader window */
    file_loader_cancel_button: Button,
    /** The open button on the file loader window */
    file_loader_open_button: Button,
    /** The file saver window */
    file_saver_window: Window,
    /** The cancel button on the file saver window */
    file_saver_cancel_button: Button,
    /** The save button on the file saver window */
    file_saver_save_button: Button,
}

impl Stuff {
    /** Creates a new `Stuff` with default values */
    fn new() -> Stuff {
        let button: Button = Button::new();
        let entry: Entry = Entry::new();
        let menu_item: MenuItem = MenuItem::new();
        let popup_window: Window = Window::new(gtk::WindowType::Popup);
        let radio_button: RadioButton = RadioButton::new(&[]);
        let top_window: Window = Window::new(gtk::WindowType::Toplevel);

        Stuff {
            main_window: top_window.clone(),
            file_menu_open: menu_item.clone(),
            file_menu_save: menu_item.clone(),
            file_menu_quit: menu_item.clone(),
            text_entry_name: entry.clone(),
            fonema1: radio_button.clone(),
            fonema2: radio_button.clone(),
            fonema3: radio_button.clone(),
            fonema4: radio_button.clone(),
            fonema5: radio_button.clone(),
            qualidade1: radio_button.clone(),
            qualidade2: radio_button.clone(),
            qualidade3: radio_button.clone(),
            qualidade4: radio_button.clone(),
            qualidade5: radio_button.clone(),
            qualidade6: radio_button.clone(),
            qualidade7: radio_button.clone(),
            control_button_back: button.clone(),
            control_button_play: button.clone(),
            control_button_next: button.clone(),
            file_loader_window: popup_window.clone(),
            file_loader_cancel_button: button.clone(),
            file_loader_open_button: button.clone(),
            file_saver_window: popup_window.clone(),
            file_saver_cancel_button: button.clone(),
            file_saver_save_button: button.clone(),
        }
    }

    /** Initializes a `Stuff` */
    fn init() -> Option<Stuff> {
        fn init_radio_button<F>(builder: &Builder, id: &str, f: F) -> Option<RadioButton>
            where F: Fn(&RadioButton) + 'static
        {
            let ret: RadioButton = try_opt!(builder.get_object(id));
            ret.connect_clicked(f);
            Some(ret)
        }

        /* Load builder from glade file */
        let builder: Builder = Builder::new_from_string(include_str!("../stuff.glade"));
        let builder: &Builder = &builder;

        let mut ret: Stuff = Stuff::new();

        /* Main window */
        {
            ret.main_window = try_opt!(builder.get_object("main_window"));
            ret.main_window
                .connect_delete_event(|_, _| {
                                          /* Stop the main loop. */
                                          gtk::main_quit();
                                          /* Let the default handler destroy the window. */
                                          Inhibit(false)
                                      });
        }

        /* Phoneme buttons */
        {
            fn fonema_button_active(button: &RadioButton) {
                radio_dummy(button);
            }
            ret.fonema1 = try_opt!(init_radio_button(builder, "fonema1", fonema_button_active));
            ret.fonema2 = try_opt!(init_radio_button(builder, "fonema2", fonema_button_active));
            ret.fonema3 = try_opt!(init_radio_button(builder, "fonema3", fonema_button_active));
            ret.fonema4 = try_opt!(init_radio_button(builder, "fonema4", fonema_button_active));
            ret.fonema5 = try_opt!(init_radio_button(builder, "fonema5", fonema_button_active));
        }

        /* Quality buttons */
        {
            fn qualidade_button_active(button: &RadioButton) {
                radio_dummy(button);
            }
            ret.qualidade1 =
                try_opt!(init_radio_button(builder, "qualidade1", qualidade_button_active));
            ret.qualidade2 =
                try_opt!(init_radio_button(builder, "qualidade2", qualidade_button_active));
            ret.qualidade3 =
                try_opt!(init_radio_button(builder, "qualidade3", qualidade_button_active));
            ret.qualidade4 =
                try_opt!(init_radio_button(builder, "qualidade4", qualidade_button_active));
            ret.qualidade5 =
                try_opt!(init_radio_button(builder, "qualidade5", qualidade_button_active));
            ret.qualidade6 =
                try_opt!(init_radio_button(builder, "qualidade6", qualidade_button_active));
            ret.qualidade7 =
                try_opt!(init_radio_button(builder, "qualidade7", qualidade_button_active));
        }

        /* Control buttons */
        {
            fn init_button<F>(builder: &Builder, id: &str, f: F) -> Option<Button>
                where F: Fn(&Button) + 'static
            {
                let ret: Button = try_opt!(builder.get_object(id));
                ret.connect_clicked(f);
                Some(ret)
            }
            fn back(button: &Button) {
                control_dummy(button);
                println!("{}: {}", state::dec_index(), state::get_index());
            }
            fn play(button: &Button) {
                control_dummy(button);
                if let Some(ref track) = state::get_current_track() {
                    println!("{}", track);
                    let _: Result<(), Box<Error>> = play_sound(track);
                }
            }
            fn next(button: &Button) {
                control_dummy(button);
                println!("{}: {}", state::inc_index(), state::get_index());
            }
            ret.control_button_back = try_opt!(init_button(builder, "control_button_back", back));
            ret.control_button_play = try_opt!(init_button(builder, "control_button_play", play));
            ret.control_button_next = try_opt!(init_button(builder, "control_button_next", next));
        }

        /* File menu buttons */
        {
            fn init_menu_item<F>(builder: &Builder, id: &str, f: F) -> Option<MenuItem>
                where F: Fn(&MenuItem) + 'static
            {
                let ret: MenuItem = try_opt!(builder.get_object(id));
                ret.connect_activate(f);
                Some(ret)
            }
            fn open(_button: &MenuItem) {
                println!("open");
                unsafe {
                    assert!(STUFF.is_some());
                    STUFF.as_ref().unwrap().file_loader_window.present();
                }
            }
            fn save(_button: &MenuItem) {
                println!("save");
                unsafe {
                    assert!(STUFF.is_some());
                    STUFF.as_ref().unwrap().file_saver_window.present();
                }
            }
            fn quit(_button: &MenuItem) {
                println!("quit");
                gtk::main_quit();
            }
            ret.file_menu_open = try_opt!(init_menu_item(builder, "file_menu_open", open));
            ret.file_menu_save = try_opt!(init_menu_item(builder, "file_menu_save", save));
            ret.file_menu_quit = try_opt!(init_menu_item(builder, "file_menu_quit", quit));
        }

        /* Text entry box */
        {
            fn activate(te: &Entry) {
                if let Some(text) = te.get_text() {
                    println!("{}", text);
                }
            }
            ret.text_entry_name = try_opt!(builder.get_object("text_entry_name"));
            ret.text_entry_name.connect_activate(activate);
        }

        /* File chooser windows */
        {
            /* File loader */
            ret.file_loader_window = try_opt!(builder.get_object("file_loader_window"));
            ret.file_loader_cancel_button =
                try_opt!(builder.get_object("file_loader_cancel_button"));
            ret.file_loader_cancel_button.connect_clicked(|_| {});
            ret.file_loader_open_button = try_opt!(builder.get_object("file_loader_open_button"));
            /* File saver */
            ret.file_saver_window = try_opt!(builder.get_object("file_saver_window"));
            ret.file_saver_cancel_button = try_opt!(builder.get_object("file_saver_cancel_button"));
            ret.file_saver_save_button = try_opt!(builder.get_object("file_saver_save_button"));
        }

        Some(ret)
    }
}

pub fn init() -> Result<&'static Window, Box<Error>> {
    unsafe {
        /* Make sure `STUFF` is initialized so it can be used later */
        STUFF = Stuff::init();
        if let Some(ref stuff) = STUFF {
            Ok(&stuff.main_window)
        } else {
            Err(From::from("Não foi possível abrir a janela."))
        }
    }
}
