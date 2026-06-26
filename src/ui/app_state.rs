use crate::data_models::song::Song;
use crate::engine::audio::play_song;
use ratatui::widgets::ListState;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(PartialEq)]
pub enum Screen {
    SongList,
    Player,
}

#[derive(PartialEq)]
pub enum PlayerState {
    Playing,
    Paused,
}

// الـ Enum الجديد الخاص بالثيمات
#[derive(PartialEq, Clone, Copy)]
pub enum ThemeMode {
    Dark,
    Light,
}

pub struct App {
    pub screen: Screen,
    pub songs: Vec<Song>,
    pub list_state: ListState,
    pub player_state: PlayerState,
    pub current_title: String,
    pub volume: f32,
    pub sink: Option<Arc<Mutex<rodio::Sink>>>,
    pub _stream: Option<rodio::OutputStream>,
    pub wave_tick: usize,
    pub show_volume_input: bool,
    pub volume_input_buf: String,
    pub status_msg: String,
    pub status_until: Option<Instant>,
    pub current_pos: Duration,
    pub current_duration: Duration, // مدة الأغنية الإجمالية
    pub theme: ThemeMode,           // الوضع الحالي (داكن أو فاتح)
}

impl App {
    pub fn new(songs: Vec<Song>) -> Self {
        let mut list_state = ListState::default();
        if !songs.is_empty() {
            list_state.select(Some(0));
        }
        App {
            screen: Screen::SongList,
            songs,
            list_state,
            player_state: PlayerState::Paused,
            current_title: String::new(),
            volume: 0.5,
            sink: None,
            _stream: None,
            wave_tick: 0,
            show_volume_input: false,
            volume_input_buf: String::new(),
            status_msg: String::new(),
            status_until: None,
            current_pos: Duration::from_secs(0),
            current_duration: Duration::from_secs(0),
            theme: ThemeMode::Dark, // الافتراضي دارك مود
        }
    }

    pub fn selected_index(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    pub fn play_selected(&mut self) {
        let idx = self.selected_index();
        let song = self.songs[idx].clone();
        self.current_title = song.title.clone();
        self.current_pos = Duration::from_secs(0);

        match play_song(song, Duration::from_secs(0)) {
            Ok((sink, stream, total_dur)) => {
                {
                    let s = sink.lock().unwrap();
                    s.set_volume(self.volume);
                }
                self.sink = Some(sink);
                self._stream = Some(stream);
                self.current_duration = total_dur; // حفظ المدة الإجمالية
                self.player_state = PlayerState::Playing;
                self.screen = Screen::Player;
                self.set_status("▶  Now Playing");
            }
            Err(e) => {
                self.set_status(&format!("Error: {}", e));
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        let is_playing = self.player_state == PlayerState::Playing;

        if let Some(ref sink) = self.sink {
            let s = sink.lock().unwrap();
            if is_playing {
                s.pause();
                drop(s);
                self.player_state = PlayerState::Paused;
                self.set_status("⏸  Paused");
            } else {
                s.play();
                drop(s);
                self.player_state = PlayerState::Playing;
                self.set_status("▶  Resumed");
            }
        }
    }

    // دالة التبديل بين الدارك واللايت مود
    pub fn toggle_theme(&mut self) {
        if self.theme == ThemeMode::Dark {
            self.theme = ThemeMode::Light;
            self.set_status("☀️  Light Mode Enabled");
        } else {
            self.theme = ThemeMode::Dark;
            self.set_status("🌙  Dark Mode Enabled");
        }
    }

    fn apply_volume(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.lock().unwrap().set_volume(self.volume);
        }
    }

    pub fn volume_up(&mut self) {
        self.volume = (self.volume + 0.05).min(1.0);
        self.apply_volume();
        self.set_status(&format!("VOL {:>3.0}%", self.volume * 100.0));
    }

    pub fn volume_down(&mut self) {
        self.volume = (self.volume - 0.05).max(0.0);
        self.apply_volume();
        self.set_status(&format!("VOL {:>3.0}%", self.volume * 100.0));
    }

    pub fn set_status(&mut self, msg: &str) {
        self.status_msg = msg.to_string();
        self.status_until = Some(Instant::now() + Duration::from_secs(3));
    }

    pub fn tick(&mut self) {
        self.wave_tick = self.wave_tick.wrapping_add(1);

        if self.player_state == PlayerState::Playing {
            self.current_pos += Duration::from_millis(80);
            
            // حماية اختيارية: لو الوقت الحالي تخطى الإجمالي، نوقفه
            if self.current_duration > Duration::from_secs(0) && self.current_pos >= self.current_duration {
                self.current_pos = self.current_duration;
                self.player_state = PlayerState::Paused;
            }
        }

        if let Some(until) = self.status_until {
            if Instant::now() > until {
                self.status_msg.clear();
                self.status_until = None;
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        if let Some(ref sink) = self.sink {
            sink.lock().unwrap().empty()
        } else {
            false
        }
    }

    pub fn list_next(&mut self) {
        let len = self.songs.len();
        if len == 0 {
            return;
        }
        let next = (self.selected_index() + 1) % len;
        self.list_state.select(Some(next));
    }

    pub fn list_prev(&mut self) {
        let len = self.songs.len();
        if len == 0 {
            return;
        }
        let cur = self.selected_index();
        let prev = if cur == 0 { len - 1 } else { cur - 1 };
        self.list_state.select(Some(prev));
    }

    fn seek_to(&mut self, new_pos: Duration) {
        // حماية لكي لا نقدم أبعد من نهاية الأغنية
        let mut target_pos = new_pos;
        if self.current_duration > Duration::from_secs(0) && target_pos > self.current_duration {
            target_pos = self.current_duration;
        }

        let idx = self.selected_index();
        let song = self.songs[idx].clone();

        self.sink = None;
        self._stream = None;

        match play_song(song, target_pos) {
            Ok((sink, stream, _)) => {
                {
                    let s = sink.lock().unwrap();
                    s.set_volume(self.volume);
                    if self.player_state == PlayerState::Paused {
                        s.pause();
                    }
                }
                self.sink = Some(sink);
                self._stream = Some(stream);
                self.current_pos = target_pos;
                self.set_status(&format!("🎯 Seeked to {}:{:02}", target_pos.as_secs() / 60, target_pos.as_secs() % 60));
            }
            Err(e) => {
                self.set_status(&format!("Error seeking: {}", e));
            }
        }
    }

    pub fn seek_forward(&mut self) {
        if self.sink.is_some() {
            let new_pos = self.current_pos + Duration::from_secs(5);
            self.seek_to(new_pos);
        }
    }

    pub fn seek_backward(&mut self) {
        if self.sink.is_some() {
            let new_pos = self.current_pos.saturating_sub(Duration::from_secs(5));
            self.seek_to(new_pos);
        }
    }
}