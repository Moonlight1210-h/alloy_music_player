mod engine;
mod data_models;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use data_models::song::Song;

fn play_audio(file_path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().expect("Error: No audio output device found");
    let sink = Sink::try_new(&stream_handle).expect("Error: Failed to create audio sink");

    let file = File::open(file_path).expect("Error: Audio file not found in data folder");
    let source = Decoder::new(BufReader::new(file)).expect("Error: Failed to decode audio file");

    sink.append(source);
    
    // This keeps the program alive while the music plays
    sink.sleep_until_end(); 
}

fn main() {
    // Elegant ASCII Header
    println!("========================================");
    println!("      ALLOY ENGINE - v0.1.0 Alpha       ");
    println!("   The Future of High-Fidelity Audio    ");
    println!("========================================");

    let current_song = Song {
        title: String::from("Yeshbahak Galbi"),
        artist: String::from("Aseel Hameem"),
        duration_secs: 304,
        path: String::from("data/assel.mp3"), 
    };

    println!("\n[STATUS] Loading assets...");
    println!("[INFO] Track  : {}", current_song.title);
    println!("[INFO] Artist : {}", current_song.artist);
    println!("[INFO] Path   : {}", current_song.path);
    println!("\nNOW PLAYING 🎶");
    println!("----------------------------------------");

    play_audio(&current_song.path);
    
    println!("\n----------------------------------------");
    println!("[SUCCESS] Playback finished.");
    println!("========================================");
}
