use crate::data_models::song::Song;

pub struct PlayList {
    pub name: String,
    pub songs: Vec<Song>,
    pub path: String,
}

impl PlayList {
    // داله تضيف اغنيه

    pub fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }

    // داله تبحث عن اغنيه

    pub fn find_song(&self, title: &str) -> Option<&Song> {
        self.songs.iter().find(|s| s.title == title)
    }
}
