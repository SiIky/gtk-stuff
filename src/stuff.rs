use std;
use std::error::Error;
use std::io::BufReader;
use std::path::PathBuf;

use super::gtk;
use super::gtk::prelude::*;
use super::gtk::{Builder, Button, Entry, MenuItem, RadioButton, Window};

extern crate rodio;
use self::rodio::Source;

use super::person;

#[doc = "The global state of the program"]
static mut STUFF: Option<Stuff> = None;

fn get_stuff() -> &'static mut Stuff {
    unsafe { STUFF.as_mut().unwrap() }
}

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

#[doc = "Open and play an audio file"]
fn play_sound(fname: &PathBuf) -> Result<(), Box<Error>> {
    let endpoint = get_stuff().snd_dev.as_ref().unwrap();
    let file: std::fs::File = std::fs::File::open(fname)?;
    let decoder = rodio::Decoder::new(BufReader::new(file))?;
    rodio::play_raw(&endpoint, decoder.convert_samples());
    Ok(())
}

#[doc = "A struct to contain all the stuff"]
#[derive(Clone)]
pub struct Stuff {
    #[doc = "The name of the user"]
    pub name: String,
    #[doc = "The sound device"]
    pub snd_dev: Option<rodio::Endpoint>,
    #[doc = "Tracks to play"]
    pub tracks: Vec<person::Entry>,
    #[doc = "Track number"]
    pub index: usize,
    #[doc = "The main window"]
    pub main_window: Window,
    #[doc = "The open option on the file menu"]
    pub file_menu_open: MenuItem,
    #[doc = "The save option on the file menu"]
    pub file_menu_save: MenuItem,
    #[doc = "The quit option on the file menu"]
    pub file_menu_quit: MenuItem,
    #[doc = "The text entry"]
    pub text_entry_name: Entry,
    #[doc = "Phoneme button 1"]
    pub fonema1: RadioButton,
    #[doc = "Phoneme button 2"]
    pub fonema2: RadioButton,
    #[doc = "Phoneme button 3"]
    pub fonema3: RadioButton,
    #[doc = "Phoneme button 4"]
    pub fonema4: RadioButton,
    #[doc = "Phoneme button 5"]
    pub fonema5: RadioButton,
    #[doc = "Quality button 1"]
    pub qualidade1: RadioButton,
    #[doc = "Quality button 2"]
    pub qualidade2: RadioButton,
    #[doc = "Quality button 3"]
    pub qualidade3: RadioButton,
    #[doc = "Quality button 4"]
    pub qualidade4: RadioButton,
    #[doc = "Quality button 5"]
    pub qualidade5: RadioButton,
    #[doc = "Quality button 6"]
    pub qualidade6: RadioButton,
    #[doc = "Quality button 7"]
    pub qualidade7: RadioButton,
    #[doc = "The back button on the main window"]
    pub control_button_back: Button,
    #[doc = "The play button on the main window"]
    pub control_button_play: Button,
    #[doc = "The next button on the main window"]
    pub control_button_next: Button,
    #[doc = "The file loader window"]
    pub file_loader_window: Window,
    #[doc = "The cancel button on the file loader window"]
    pub file_loader_cancel_button: Button,
    #[doc = "The open button on the file loader window"]
    pub file_loader_open_button: Button,
    #[doc = "The file saver window"]
    pub file_saver_window: Window,
    #[doc = "The cancel button on the file saver window"]
    pub file_saver_cancel_button: Button,
    #[doc = "The save button on the file saver window"]
    pub file_saver_save_button: Button,
    #[doc = "The button to enter"]
    pub name_button: Button,
}

impl Stuff {
    #[doc = "Creates a new `Stuff` with default values"]
    fn new() -> Stuff {
        let button: Button = Button::new();
        let entry: Entry = Entry::new();
        let index: usize = 0;
        let menu_item: MenuItem = MenuItem::new();
        let popup_window: Window = Window::new(gtk::WindowType::Popup);
        let radio_button: RadioButton = RadioButton::new(&[]);
        let top_window: Window = Window::new(gtk::WindowType::Toplevel);
        let tracks: Vec<person::Entry> = Vec::new();

        Stuff {
            name: String::new(),
            snd_dev: None,
            tracks: tracks,
            index: index,
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
            name_button: button.clone(),
        }
    }

    #[doc = "Initializes a `Stuff`"]
    fn init() -> Option<Stuff> {
        fn init_button<F>(builder: &Builder, id: &str, f: F) -> Option<Button>
        where
            F: Fn(&Button) + 'static,
        {
            let ret: Button = try_opt!(builder.get_object(id));
            ret.connect_clicked(f);
            Some(ret)
        }
        fn init_menu_item<F>(builder: &Builder, id: &str, f: F) -> Option<MenuItem>
        where
            F: Fn(&MenuItem) + 'static,
        {
            let ret: MenuItem = try_opt!(builder.get_object(id));
            ret.connect_activate(f);
            Some(ret)
        }
        fn init_radio_button<F>(builder: &Builder, id: &str, f: F) -> Option<RadioButton>
        where
            F: Fn(&RadioButton) + 'static,
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
            ret.main_window.connect_delete_event(|_, _| {
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
            ret.qualidade1 = try_opt!(init_radio_button(
                builder,
                "qualidade1",
                qualidade_button_active,
            ));
            ret.qualidade2 = try_opt!(init_radio_button(
                builder,
                "qualidade2",
                qualidade_button_active,
            ));
            ret.qualidade3 = try_opt!(init_radio_button(
                builder,
                "qualidade3",
                qualidade_button_active,
            ));
            ret.qualidade4 = try_opt!(init_radio_button(
                builder,
                "qualidade4",
                qualidade_button_active,
            ));
            ret.qualidade5 = try_opt!(init_radio_button(
                builder,
                "qualidade5",
                qualidade_button_active,
            ));
            ret.qualidade6 = try_opt!(init_radio_button(
                builder,
                "qualidade6",
                qualidade_button_active,
            ));
            ret.qualidade7 = try_opt!(init_radio_button(
                builder,
                "qualidade7",
                qualidade_button_active,
            ));
        }

        /* Control buttons */
        {
            fn back(button: &Button) {
                control_dummy(button);
                println!("{:?}: {}", dec_index(), get_index());
            }
            fn play(button: &Button) {
                control_dummy(button);
                if let Some(ref track) = get_current_track() {
                    println!("{:?}", track);
                    let _: Result<(), Box<Error>> = play_sound(track);
                }
            }
            fn next(button: &Button) {
                control_dummy(button);
                println!("{:?}: {}", inc_index(), get_index());
            }
            ret.control_button_back = try_opt!(init_button(builder, "control_button_back", back));
            ret.control_button_play = try_opt!(init_button(builder, "control_button_play", play));
            ret.control_button_next = try_opt!(init_button(builder, "control_button_next", next));
        }

        /* File menu buttons */
        {
            fn open(_button: &MenuItem) {
                println!("open");
                get_stuff().file_loader_window.present();
            }
            fn save(_button: &MenuItem) {
                println!("save");
                get_stuff().file_saver_window.present();
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
            fn update_name(te: &Entry) {
                if let Some(text) = te.get_text() {
                    get_stuff().name = text.clone();
                    println!("{}", text);
                }
            }
            ret.text_entry_name = try_opt!(builder.get_object("text_entry_name"));
            ret.text_entry_name.connect_activate(update_name);
            ret.text_entry_name.connect_changed(update_name);
        }

        /* File chooser windows */
        {
            /* File loader */
            ret.file_loader_window = try_opt!(builder.get_object("file_loader_window"));
            ret.file_loader_cancel_button =
                try_opt!(builder.get_object("file_loader_cancel_button"));
            ret.file_loader_cancel_button.connect_clicked(
                file_load_cancel_action,
            );
            ret.file_loader_open_button = try_opt!(builder.get_object("file_loader_open_button"));
            /* File saver */
            ret.file_saver_window = try_opt!(builder.get_object("file_saver_window"));
            ret.file_saver_cancel_button = try_opt!(builder.get_object("file_saver_cancel_button"));
            ret.file_saver_cancel_button.connect_clicked(
                file_save_cancel_action,
            );
            ret.file_saver_save_button = try_opt!(builder.get_object("file_saver_save_button"));
        }

        /* Tracks & index */
        {
            let aux = |path| {
                person::Entry::new(
                    String::new(),
                    From::from(path),
                    String::new(),
                    String::new(),
                )
            };
            ret.tracks = vec![
                aux("test"),
                aux("/home/silky/Music/rush/2112/01-2112_.flac"),
                aux("/home/silky/Music/rush/2112/02-a_passage_to_bangkok.flac"),
                aux("/home/silky/Music/rush/2112/03-the_twilight_zone.flac"),
                aux("/home/silky/Music/rush/2112/04-lessons.flac"),
                aux("/home/silky/Music/rush/2112/05-tears.flac"),
                aux("/home/silky/Music/rush/2112/06-something_for_nothing.flac"),
            ];
            ret.index = 0;
        }

        /* The sound stuff */
        {
            let snd_dev = try_opt!(rodio::get_default_endpoint());
            ret.snd_dev = Some(snd_dev);
        }

        Some(ret)
    }

    fn tracks_len(&self) -> usize {
        self.tracks.len()
    }

    fn get_tracks(&self) -> Vec<PathBuf> {
        self.tracks.iter().map(|entry| entry.path.clone()).collect()
    }

    fn get_current_track(&self) -> Option<PathBuf> {
        self.get_tracks().get(self.get_index()).map(|x| x.clone())
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) -> Option<usize> {
        match index < self.tracks_len() {
            false => None,
            true => {
                let ret = self.index;
                self.index = index;
                Some(ret)
            }
        }
    }

    fn inc_index(&mut self) -> Option<usize> {
        let ret = self.get_index() < (self.tracks_len() - 1);
        let index = self.get_index() + if ret { 1 } else { 0 };
        self.set_index(index)
    }

    fn dec_index(&mut self) -> Option<usize> {
        let cond = self.get_index() > 0;
        let index = self.get_index() - if cond { 1 } else { 0 };
        self.set_index(index)
    }
}

fn file_save_cancel_action(_ignore: &Button) {
    init().expect("save_window fucked");
    get_stuff().file_saver_window.hide();
}

fn file_load_cancel_action(_ignore: &Button) {
    init().expect("load_window fucked");
    get_stuff().file_loader_window.hide();
}

pub fn get_current_track() -> Option<PathBuf> {
    get_stuff().get_current_track()
}

pub fn inc_index() -> Option<usize> {
    get_stuff().inc_index()
}

pub fn dec_index() -> Option<usize> {
    get_stuff().dec_index()
}

pub fn get_index() -> usize {
    get_stuff().get_index()
}

pub fn get_tracks() -> Vec<PathBuf> {
    get_stuff().get_tracks()
}

pub fn set_fonema(entry: &mut person::Entry, txt: String) {}

pub fn update_fonema(txt: String) -> Option<()> {
    let _ = try_opt!(get_current_track());
    Some(())
}

pub fn init() -> Result<&'static Stuff, Box<Error>> {
    unsafe {
        if STUFF.is_none() {
            /* Make sure `STUFF` is initialized so it can be used later */
            STUFF = Stuff::init();
        }
        if let Some(ref stuff) = STUFF {
            Ok(&stuff)
        } else {
            Err(From::from("Não foi possível abrir a janela."))
        }
    }
}
