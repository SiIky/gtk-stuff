/** Files to play */
static mut STATE: Option<State> = None;

#[derive(Clone, Debug)]
pub struct State {
    tracks: Vec<&'static str>,
    index: usize,
}

impl State {
    fn new(tracks: Vec<&'static str>, index: usize) -> State {
        State {
            tracks: tracks,
            index: index,
        }
    }

    fn len(&self) -> usize {
        self.tracks.len()
    }

    fn get_tracks(&self) -> &Vec<&'static str> {
        &self.tracks
    }

    fn get_current_track(&self) -> Option<String> {
        self.get_tracks()
            .get(self.get_index())
            .map(|x| x.to_string())
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) -> Option<usize> {
        match index < self.len() {
            false => None,
            true => {
                let ret = self.index;
                self.index = index;
                Some(ret)
            }
        }
    }

    fn inc_index(&mut self) -> Option<usize> {
        let ret = self.get_index() < (self.tracks.len() - 1);
        let index = self.index +
                    match ret {
                        true => 1,
                        false => 0,
                    };
        self.set_index(index)
    }

    fn dec_index(&mut self) -> Option<usize> {
        let ret = self.get_index() > 0;
        let index = self.index -
                    match ret {
                        true => 1,
                        false => 0,
                    };
        self.set_index(index)
    }
}

pub fn init() {
    unsafe {
        if STATE.is_none() {
            STATE =
                Some(State::new(vec!["/home/silky/Music/rush/2112/01-2112_.flac",
                                     "/home/silky/Music/rush/2112/02-a_passage_to_bangkok.flac",
                                     "/home/silky/Music/rush/2112/03-the_twilight_zone.flac",
                                     "/home/silky/Music/rush/2112/04-lessons.flac",
                                     "/home/silky/Music/rush/2112/05-tears.flac",
                                     "/home/silky/Music/rush/2112/06-something_for_nothing.flac"],
                                0));
        }
    }
}

pub fn get_current_track() -> Option<String> {
    init();
    unsafe { STATE.as_mut().unwrap().get_current_track() }
}

pub fn inc_index() -> Option<usize> {
    init();
    unsafe { STATE.as_mut().unwrap().inc_index() }
}

pub fn dec_index() -> Option<usize> {
    init();
    unsafe { STATE.as_mut().unwrap().dec_index() }
}

pub fn get_index() -> usize {
    init();
    unsafe { STATE.as_mut().unwrap().get_index() }
}

pub fn get_tracks() -> &'static Vec<&'static str> {
    init();
    unsafe { &STATE.as_mut().unwrap().get_tracks() }
}
