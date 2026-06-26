use crate::data_models::song::Song;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn play_song(song: Song, start_from: Duration) -> Result<(Arc<Mutex<Sink>>, OutputStream, Duration), String> {
    let (stream, handle) = OutputStream::try_default().map_err(|e| e.to_string())?;
    let file = File::open(&song.path).map_err(|e| e.to_string())?;
    let buf = BufReader::new(file);

    let sink = Sink::try_new(&handle).map_err(|e| e.to_string())?;
    let source = Decoder::new(buf).map_err(|e| e.to_string())?;
    
    // جلب المدة الإجمالية للأغنية (ووضع 0 كاحتياط لو لم ينجح في قراءتها)
    let total_duration = source.total_duration().unwrap_or(Duration::from_secs(0));

    if start_from > Duration::from_secs(0) {
        sink.append(source.skip_duration(start_from));
    } else {
        sink.append(source);
    }

    let sink = Arc::new(Mutex::new(sink));
    // نرجع الـ duration الإضافي مع الـ sink والـ stream
    Ok((sink, stream, total_duration))
}