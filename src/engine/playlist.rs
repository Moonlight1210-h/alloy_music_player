 
 #![allow(dead_code)]
 pub struct Song {
    pub title: String,
    pub artist: String,
    pub duration_secs: u32,
   pub  path: String,
}

pub struct Playlist {
    name: String,
    songs: Vec<Song>,
}

pub fn add_song_to_playlist(playlist: &mut Playlist, song: Song) {
    playlist.songs.push(song);
}

pub fn print_playlist(playlist: & Playlist) {
    println!("Playlist: {}",playlist.name);
}


pub fn add_seconds(song: &mut Song,duration:u32) {
    song.duration_secs+=duration;

}

pub fn playing_now(playlist: &Playlist) {
    // استخدمنا .first() لأنها ترجع Option (إما قيمة أو لا شيء)
    // وهذا يغنيك عن فحص .is_empty() يدوياً
    match playlist.songs.first() {
        Some(song) => println!("Now playing: {} by {}", song.title, song.artist),
        None => println!("Playlist is empty!"),
    }
}
    


pub fn main() {
    let mut my_playlist = Playlist {
        name: String::from("My Favorites"),
        songs: vec![],
    };


    let mut song1 = Song {
        title: String::from("As it was"),
        artist: String::from("Harry Styles"),
        duration_secs: 250,
     path: String::from("data/test_sample.wav"),
    };
    add_seconds(&mut song1, 5);

    let  song2 = Song {
        title: String::from("Blinding Lights"),
        artist: String::from("The Weeknd"),
        duration_secs: 200,
        path: String::from("data/test_sample.wav"),
    };

    // إضافة أكثر من أغنية
    add_song_to_playlist(&mut my_playlist, song1);
    add_song_to_playlist(&mut my_playlist, song2);

    // طباعة قائمة التشغيل

    print_playlist(&mut my_playlist);

    // طباعة القائمة
    for (i, s) in my_playlist.songs.iter().enumerate() {
        println!("{}. {} by {}", i + 1, s.title, s.artist);
    }

    playing_now(&my_playlist);
}