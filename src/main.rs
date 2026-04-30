
pub mod data_models;

pub mod engine;
 
use crate::engine::loader::scan_folder;

use crate::engine::audio::play_song;
 

fn main(){


// 1. اقرأ الأغاني
let songs = scan_folder("data/").unwrap();

// 2. اعرض القائمة
for (i, song) in songs.iter().enumerate() {
    println!("{}: {}", i + 1, song.title);
}

// 3. اقرأ اختيار المستخدم
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();

let index: usize = input.trim().parse().unwrap();
match play_song(songs[index - 1].clone()) {
    Ok((sink, _stream)) => {
        println!("🎵 يشغل — اكتب رقم الصوت (0.0 - 1.0):");
        let mut vol = String::new();
        std::io::stdin().read_line(&mut vol).unwrap();
        let volume: f32 = vol.trim().parse().unwrap();
        sink.set_volume(volume);
        sink.sleep_until_end();
    }
    Err(msg) => println!("خطأ: {}", msg),
}
}
 







