 use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use crate::data_models::song::Song;

pub fn play_song(song: Song) -> Result<(Sink, rodio::OutputStream), String> {
    // 1. شغّل جهاز الصوت
    let (_stream, handle) = OutputStream::try_default()
        .map_err(|e| e.to_string())?;
    
    // 2. افتح الملف
    let file = File::open(&song.path).map_err(|e| e.to_string())?;
    let buf  = BufReader::new(file);

    // 3. شغّل الملف
let sink = Sink::try_new(&handle).map_err(|e| e.to_string())?;
let source = Decoder::new(buf).map_err(|e| e.to_string())?;
sink.append(source);
 

Ok((sink, _stream))
    
    
}