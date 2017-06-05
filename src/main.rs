use std::error::Error;
use std::io::BufReader;

extern crate gtk;
use gtk::prelude::*;
use gtk::{Builder, Button, Entry, MenuItem, RadioButton, Window};

#[macro_use]
extern crate lazy_static;

extern crate rodio;
use rodio::Source;

#[macro_use]
extern crate try_opt;

/** Index of the file to play */
static mut CURRENT: usize = 0;

lazy_static! {
    /** Files to play */
    static ref TRACKS: Vec<&'static str> = vec![
        "/home/silky/Music/rush/2112/01-2112_.flac",
       "/home/silky/Music/rush/2112/02-a_passage_to_bangkok.flac",
       "/home/silky/Music/rush/2112/03-the_twilight_zone.flac",
       "/home/silky/Music/rush/2112/04-lessons.flac",
       "/home/silky/Music/rush/2112/05-tears.flac",
       "/home/silky/Music/rush/2112/06-something_for_nothing.flac"
    ];
}

#[derive(Debug, Clone)]
struct Stuff {
    main_window: Window,
    file_menu_open: MenuItem,
    file_menu_save: MenuItem,
    file_menu_quit: MenuItem,
    text_entry_name: Entry,
    fonema1: RadioButton,
    fonema2: RadioButton,
    fonema3: RadioButton,
    fonema4: RadioButton,
    fonema5: RadioButton,
    qualidade1: RadioButton,
    qualidade2: RadioButton,
    qualidade3: RadioButton,
    qualidade4: RadioButton,
    qualidade5: RadioButton,
    qualidade6: RadioButton,
    qualidade7: RadioButton,
    control_button_back: Button,
    control_button_play: Button,
    control_button_next: Button,
    file_loader_window: Window,
    file_loader_cancel_button: Button,
    file_loader_open_button: Button,
    file_saver_window: Window,
    file_saver_cancel_button: Button,
    file_saver_save_button: Button,
}

impl Stuff {
    /** Creates a new `Stuff` with default values */
    fn new() -> Stuff {
        let top_window = Window::new(gtk::WindowType::Toplevel);
        let popup_window = Window::new(gtk::WindowType::Popup);
        let menu_item = MenuItem::new();
        let entry = Entry::new();
        let radio_button = RadioButton::new(&[]);
        let button = Button::new();

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

    /** Initializes a `Stuff` with the `GLADE_FILE` */
    fn init(builder: &Builder) -> Option<Stuff> {
        fn init_radio_button<F>(builder: &Builder, id: &str, f: F) -> Option<RadioButton>
            where F: Fn(&RadioButton) + 'static
        {
            let ret: RadioButton = try_opt!(builder.get_object(id));
            ret.connect_clicked(f);
            Some(ret)
        }

        let mut ret = Stuff::new();
        ret.main_window = try_opt!(builder.get_object("main_window"));

        /* init botoes de fonemas */
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

        /* init botoes de qualidade */
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

        /* init botoes de controlo */
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
                unsafe {
                    if CURRENT > 0 {
                        CURRENT -= 1;
                    }
                }
            }

            fn play(button: &Button) {
                control_dummy(button);
                unsafe {
                    let _ = play_sound(TRACKS[CURRENT]);
                }
            }

            fn next(button: &Button) {
                control_dummy(button);
                unsafe {
                    if CURRENT < TRACKS.len() - 1 {
                        CURRENT += 1;
                    }
                }
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
            }
            fn save(_button: &MenuItem) {
                println!("save");
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

        /* open the file chooser windows */
        {
            ret.file_loader_window = try_opt!(builder.get_object("file_loader_window"));
            ret.file_saver_window =  try_opt!(builder.get_object("file_saver_window"));
        }

        Some(ret)
    }
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

fn play_sound(fname: &str) -> Result<(), Box<Error>> {
    let endpoint = rodio::get_default_endpoint();
    let file = std::fs::File::open(fname)?;
    let decoder = rodio::Decoder::new(BufReader::new(file))?;
    if let Some(endpoint) = endpoint {
        rodio::play_raw(&endpoint, decoder.convert_samples());
    }
    Ok(())
}

fn main() {
    /* Beginning of `main()` */
    if gtk::init().is_err() {
        panic!("Não foi possível inicializar GTK.");
    }

    /* Load builder from glade file */
    let builder: Builder = Builder::new_from_string(include_str!("../stuff.glade"));

    /* Initialize and open the main window */
    let stuff = match Stuff::init(&builder) {
        Some(w) => w,
        None => panic!("Não foi possível abrir a janela."),
    };

    /* open the main window */
    {
        stuff.main_window.show_all();
        stuff
            .main_window
            .connect_delete_event(|_, _| {
                                      /* Stop the main loop. */
                                      gtk::main_quit();
                                      /* Let the default handler destroy the window. */
                                      Inhibit(false)
                                  });
    }

    /* The GTK main loop */
    gtk::main();
}
