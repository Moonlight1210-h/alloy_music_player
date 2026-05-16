
pub mod data_models;

pub mod engine;
 
use crate::engine::loader::scan_folder;

use crate::engine::audio::play_song;

use std::sync::Arc;
 
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_now_playing(title: &str, volume: f32) {
    clear_screen();
    
    let bars = ["▁","▂","▃","▄","▅","▆","▇","█"];
    let wave: Vec<&str> = vec![
        bars[0], bars[2], bars[4], bars[6], bars[7],
        bars[5], bars[3], bars[1], bars[0], bars[2],
        bars[5], bars[7], bars[6], bars[3], bars[1],
    ];
 
    println!("\n");
    println!("  ╔══════════════════════════════════════╗");
    println!("  ║       🎵  A L L O Y  P L A Y E R    ║");
    println!("  ╠══════════════════════════════════════╣");
    println!("  ║                                      ║");
    println!("  ║   ♪  {:<34}║", title);
    println!("  ║                                      ║");
    println!("  ║   {}  ║", wave.join(" "));
    println!("  ║                                      ║");
    println!("  ║   VOL: [{:<20}] {:.0}%  ║", 
        "█".repeat((volume * 20.0) as usize),
        volume * 100.0
    );
    println!("  ║                                      ║");
    println!("  ╠══════════════════════════════════════╣");
    println!("  ║  p=pause  r=resume  v=vol  q=quit   ║");
    println!("  ╚══════════════════════════════════════╝");
    println!("\n  > ");
}


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
let title = song.title.clone();

// شغّل الأغنية واحصل على sink
let (sink, _stream) = play_song(song).unwrap();
let sink_clone = Arc::clone(&sink);
let mut current_volume = 0.5f32;
print_now_playing(&title, current_volume);
// thread يشغل الأغنية
std::thread::spawn(move || {
    sink_clone.lock().unwrap().sleep_until_end();
});

// loop يتحكم
loop {
    let mut cmd = String::new();
    std::io::stdin().read_line(&mut cmd).unwrap();
    match cmd.trim() {
        "p" =>  {sink.lock().unwrap().pause();
    print_now_playing(&title, current_volume);},

        "r" =>{sink.lock().unwrap().play();  
    print_now_playing(&title, current_volume);},

        "q" => {println!("Bye!");break},

        "v"=>{let mut vol = String::new();
    std::io::stdin().read_line(&mut vol).unwrap();
    if let Ok(v) = vol.trim().parse::<f32>() {
        current_volume = v;
        sink.lock().unwrap().set_volume(v);
        print_now_playing(&title, current_volume);} },

   

        _=> println!("invalid order!"),
    }
}
}
    

 

 







