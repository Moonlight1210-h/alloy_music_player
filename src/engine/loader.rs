 

use id3::{Tag, TagLike};
use crate::data_models::song::Song;


pub fn load_song(path: String) -> Result<Song, String> {
    let tag = Tag::read_from_path(&path);
    match tag {
        Ok(t) => {
            return Ok(Song {
                title:        t.title().unwrap_or("unknown").to_string(),
                artist:       t.artist().unwrap_or("unknown").to_string(),
                duration_sec: 0,
                path:         path,
            });
        }
        Err(e) => {
            return Err(e.to_string()); 
        }
    }
}

pub fn scan_folder(path: &str) -> Result<Vec<Song>, String> {
    let mut songs = Vec::new();
    
    for file in std::fs::read_dir(path).map_err(|e| e.to_string())? {
        // كل file هو Result أيضاً
        let file = file.map_err(|e| e.to_string())?;
        let path = file.path();
       let song = load_song(path.to_string_lossy().to_string());

       match song {
        Ok(s)=> songs.push(s),
        Err(_)=>{}
       }
    }
    
    Ok(songs)
}