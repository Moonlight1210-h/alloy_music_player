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
        }
    }

    pub fn selected_index(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    pub fn play_selected(&mut self) {
        let idx = self.selected_index();
        let song = self.songs[idx].clone();
        self.current_title = song.title.clone();

        match play_song(song) {
            Ok((sink, stream)) => {
                {
                    let s = sink.lock().unwrap();
                    s.set_volume(self.volume);
                }
                self.sink = Some(sink);
                self._stream = Some(stream);
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
}
