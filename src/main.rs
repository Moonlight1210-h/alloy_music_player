mod data_models;
mod engine;
mod ui;

use crate::ui::app_state::{App, Screen};
use crate::ui::widgets::ui;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, BufReader};
use std::time::{Duration, Instant};
use std::fs::{self, File}; 
use rodio::Source;         

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. إعداد الطرفية
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. 📂 قراءة ملفات الـ MP3 ديناميكياً وحساب مدتها الحقيقية
    let mut real_songs = Vec::new();
    let data_dir = "data";

    if let Ok(entries) = fs::read_dir(data_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "mp3") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    
                    // ── ⏱️ تم الإصلاح هنا: إزالة المسافة وتحديد النوع كـ u32 ──
                    let mut duration_sec: u32 = 0; 
                    if let Ok(file) = File::open(&path) {
                        let reader = BufReader::new(file);
                        if let Ok(decoder) = rodio::Decoder::new(reader) {
                            if let Some(duration) = decoder.total_duration() {
                                 
                                duration_sec = duration.as_secs() as u32; 
                            }
                        }
                    }

                    real_songs.push(data_models::song::Song {
                        title: file_name.replace('_', " ").to_string(),
                        artist: "Local Audio".to_string(),
                        path: path.to_string_lossy().to_string(),
                        duration_sec, 
                    });
                }
            }
        }
    }

    if real_songs.is_empty() {
        real_songs.push(data_models::song::Song {
            title: "⚠️ No MP3 files found in ./data folder".to_string(),
            artist: "System".to_string(),
            path: "".to_string(),
            duration_sec: 0,
        });
    }

    // 3. بناء حالة التطبيق المركزية
    let mut app = App::new(real_songs);
    if !app.songs.is_empty() {
        app.list_state.select(Some(0));
    }

    let tick_rate = Duration::from_millis(80);
    let mut last_tick = Instant::now();

    // 4. حلقة الأحداث الرئيسية
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    
                    if app.show_volume_input {
                        match key.code {
                            KeyCode::Char(c) if c.is_digit(10) => {
                                if app.volume_input_buf.len() < 3 {
                                    app.volume_input_buf.push(c);
                                }
                            }
                            KeyCode::Backspace => {
                                app.volume_input_buf.pop();
                            }
                            KeyCode::Esc => {
                                app.show_volume_input = false;
                                app.volume_input_buf.clear();
                            }
                            KeyCode::Enter => {
                                if let Ok(val) = app.volume_input_buf.parse::<u32>() {
                                    let clamped = val.min(100);
                                    app.volume = clamped as f32 / 100.0;
                                    if let Some(ref sink) = app.sink {
                                        if let Ok(lock) = sink.lock() {
                                            lock.set_volume(app.volume);
                                        }
                                    }
                                    app.set_status(&format!("VOL SET TO {}%", clamped));
                                }
                                app.show_volume_input = false;
                                app.volume_input_buf.clear();
                            }
                            _ => {}
                        }
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            app.toggle_theme();
                            continue;
                        }
                        _ => {}
                    }

                    match app.screen {
                        Screen::SongList => match key.code {
                            KeyCode::Char('q') | KeyCode::Char('Q') => break,
                            KeyCode::Up => app.list_prev(),
                            KeyCode::Down => app.list_next(),
                            KeyCode::Enter => {
                                app.play_selected();
                                app.screen = Screen::Player;
                            }
                            _ => {}
                        },
                        Screen::Player => match key.code {
                            KeyCode::Char('q') | KeyCode::Char('Q') => break,
                            KeyCode::Char('p') | KeyCode::Char('P') => app.toggle_pause(),
                            KeyCode::Char('b') | KeyCode::Char('B') => app.screen = Screen::SongList,
                            KeyCode::Char('v') | KeyCode::Char('V') => {
                                app.show_volume_input = true;
                                app.volume_input_buf.clear();
                            }
                            KeyCode::Up | KeyCode::Char('+') => app.volume_up(),
                            KeyCode::Down | KeyCode::Char('-') => app.volume_down(),
                            KeyCode::Right => app.seek_forward(),
                            KeyCode::Left => app.seek_backward(),
                            _ => {}
                        },
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}