
pub mod data_models;

pub mod engine;
 
use crate::engine::loader::scan_folder;

use crate::engine::audio::play_song;

use std::sync::{Arc, Mutex};
 

fn main(){
println!("Wellcome to Alloy music Player 🎵");

// 1. اقرأ الأغاني
let songs = scan_folder("data/").unwrap();

// 2. اعرض القائمة
for (i, song) in songs.iter().enumerate() {
    println!("{}: {}", i + 1, song.title);
}

// 3. اقرأ اختيار المستخدم
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();

let index: usize = match input.trim().parse(){
    Ok(n) if n > 0 && n <= songs.len()=>n,
    _=>{
        println!("invalid input. please enter a numbeer between 1 and {}",songs.len());
        return;
    }
};

let song = songs[index - 1].clone();

// شغّل الأغنية واحصل على sink
let (sink, _stream) = play_song(song).unwrap();
let sink_clone = Arc::clone(&sink);

// thread يشغل الأغنية
std::thread::spawn(move || {
    sink_clone.lock().unwrap().sleep_until_end();
});

// loop يتحكم
loop {
    let mut cmd = String::new();
    std::io::stdin().read_line(&mut cmd).unwrap();
    match cmd.trim() {
        "p" => sink.lock().unwrap().pause(),
        "r" => sink.lock().unwrap().play(),
        "q" => {println!("Bye!");break},
        _   => println!("أمر غير معروف"),
    }
}
 
}
 







