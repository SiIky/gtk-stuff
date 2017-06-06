use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

extern crate csv;
extern crate serde;

type Answer = (Phoneme, Quality);
type Name = String;
type Phoneme = String;
type Quality = String;

/** The `Entry` type. */
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Entry {
    #[serde(rename = "Nome")]
    name: Name,
    #[serde(rename = "Ficheiro")]
    path: PathBuf,
    #[serde(rename = "Fonema")]
    phoneme: Phoneme,
    #[serde(rename = "Qualidade")]
    quality: Quality,
}

impl Entry {
    fn new(name: Name, path: PathBuf, phoneme: Phoneme, quality: Quality) -> Entry {
        Entry {
            name: name,
            path: path,
            phoneme: phoneme,
            quality: quality,
        }
    }
}

/** The `Person` type. */
#[derive(Clone, Debug)]
pub struct Person {
    pub name: Name,
    pub answers: HashMap<PathBuf, Answer>,
}

/* Private Methods */
impl Person {
    /** Convert a `Person` to a `Vec<Entry>`, returning error if name is empty */
    fn to_entries(&self) -> Result<Vec<Entry>, Box<Error>> {
        let mut ret = if self.name.is_empty() {
            Err(From::from("Nome está vazio."))
        } else if self.answers.is_empty() {
            Err(From::from("Não existem respostas"))
        } else {
            Ok(Vec::new())
        };

        if let Ok(ref mut vec) = ret {
            for (key, val) in self.answers.iter() {
                vec.push(Entry::new(self.name.clone(),
                                    key.to_owned(),
                                    val.0.to_owned(),
                                    val.1.to_owned()));
            }
        }

        ret
    }

    fn read_file(&self, fname: &PathBuf) -> Result<Vec<Entry>, Box<Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(&fname)?;

        let mut ret = Vec::new();

        for res in rdr.deserialize() {
            ret.push(res?);
        }

        Ok(ret)
    }
}

/* Public methods */
impl Person {
    /** Create a new `Person` */
    pub fn new(name: String) -> Person {
        Person {
            name: name,
            answers: HashMap::with_capacity(10),
        }
    }

    /** Update one of the answers */
    pub fn update_answer(&mut self, path: PathBuf, ph: Phoneme, q: Quality) -> Option<Answer> {
        self.answers.insert(path, (ph, q))
    }

    /** Write a person to file */
    pub fn write_to_file(&self, fname: PathBuf) -> Result<(), Box<Error>> {
        let entries = {
            let mut tmp = Vec::new();

            let read_entries = self.read_file(&fname);
            if read_entries.is_ok() {
                let mut read = read_entries.unwrap();
                tmp.append(&mut read);
            } else {
                println!("Não foi possível ler o ficheiro: {}", read_entries.err().unwrap());
            }

            let mut new_entries = self.to_entries()?;
            tmp.append(&mut new_entries);

            tmp
        };

        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .quote_style(csv::QuoteStyle::Always)
            .from_path(&fname)?;

        for entry in entries {
            wtr.serialize(entry)?;
        }

        wtr.flush()?;

        Ok(())
    }
}
