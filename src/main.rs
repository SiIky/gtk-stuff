#![allow(dead_code)]

extern crate gtk;
use gtk::WidgetExt;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate try_opt;

mod person;
//use person::Person;

mod stuff;

fn main() {
    /*
    fn csv_test() {
        let mut person: Person = Person::new("derperino".to_string());
        for track in state::get_tracks() {
            person.update_answer(PathBuf::from(track),
                                 "fonema1".to_string(),
                                 "qualidade1".to_string());
        }

        println!("{:#?}", person.clone());

        if let Err(ref e) = person.write_to_file(PathBuf::from("test.tsv")) {
            println!("Não foi possível escrever no ficheiro: {}", e);
        }
    }
    */

    #[doc = "Initialize and open the main window"]
    fn run_gtk() {
        gtk::init().expect("Não foi possível inicializar GTK.");
        let s: &stuff::Stuff = stuff::init().unwrap_or_else(|e| panic!("{}", e));
        /* open the main window */
        s.main_window.show_all();
        /* The GTK main loop */
        gtk::main();
    }

    run_gtk();
    //csv_test();
}
