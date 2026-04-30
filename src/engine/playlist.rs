
use crate::data_models::song::Song;


pub struct PlayList{
    pub name: String,
   pub songs: Vec<Song>,
    pub path: String,
// داله تضيف اغنيه الى قائمه التشغيل

}


impl PlayList{

    // داله تضيف اغنيه

    pub fn add_song(&mut self,song:  Song){
        self.songs.push(song);
    }

    // داله تبحث عن اغنيه

   pub fn find_song(& self,title:String) -> Option<String> {
    for s in &self.songs{
        if s.title == title {
           return Some(s.title.clone());
        }
    }None

   }

}

 


