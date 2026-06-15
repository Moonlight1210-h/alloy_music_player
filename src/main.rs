pub mod data_models;
pub mod engine;
pub mod ui; 
use crate::engine::loader::scan_folder;
use crate::ui::app_state::{App, PlayerState, Screen};
use crate::ui::widgets::ui;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::time::{Duration, Instant};

fn main() -> anyhow::Result<()> {
    // 1. تهيئة التيرمنال والمحيط الخام
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. البحث عن الموسيقى وشحن التطبيق
    let songs = scan_folder("data/").unwrap_or_default();
    let mut app = App::new(songs);

    if app.songs.is_empty() {
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture
        )?;
        println!("No songs found in data/ folder.");
        return Ok(());
    }

    let tick_rate = Duration::from_millis(80);
    let mut last_tick = Instant::now();

    // 3. محرك تشغيل الأحداث والواجهة (Event Loop)
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_default();

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // ── وضع إدخال مستوى الصوت يدوياً ──
                if app.show_volume_input {
                    match key.code {
                        KeyCode::Esc => {
                            app.show_volume_input = false;
                            app.volume_input_buf.clear();
                        }
                        KeyCode::Enter => {
                            if let Ok(v) = app.volume_input_buf.trim().parse::<f32>() {
                                let v = v.clamp(0.0, 100.0) / 100.0;
                                app.volume = v;
                                app.volume_up(); // تطبيق وتحديث الصوت
                                app.set_status(&format!("VOL set to {:.0}%", v * 100.0));
                            } else {
                                app.set_status("Invalid number");
                            }
                            app.show_volume_input = false;
                            app.volume_input_buf.clear();
                        }
                        KeyCode::Backspace => {
                            app.volume_input_buf.pop();
                        }
                        KeyCode::Char(c) if c.is_ascii_digit() || c == '.' => {
                            if app.volume_input_buf.len() < 5 {
                                app.volume_input_buf.push(c);
                            }
                        }
                        _ => {}
                    }
                    continue;
                }

                // ── الوضع العادي للتنقل بين الأغاني والمشغل ──
                match app.screen {
                    Screen::SongList => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Down | KeyCode::Char('j') => app.list_next(),
                        KeyCode::Up | KeyCode::Char('k') => app.list_prev(),
                        KeyCode::Enter => app.play_selected(),
                        _ => {}
                    },
                    Screen::Player => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Char('p') => app.toggle_pause(),
                        KeyCode::Char('b') => app.screen = Screen::SongList,
                        KeyCode::Char('v') => {
                            app.show_volume_input = true;
                            app.volume_input_buf.clear();
                        }
                        KeyCode::Up | KeyCode::Char('+') => app.volume_up(),
                        KeyCode::Down | KeyCode::Char('-') => app.volume_down(),
                        _ => {}
                    },
                }

                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    break;
                }
            }
        }

        // 4. تحديث العدادات الداخلية عند انتهاء الوقت الـ Tick
        if last_tick.elapsed() >= tick_rate {
            app.tick();
            if app.screen == Screen::Player && app.is_finished() {
                app.player_state = PlayerState::Paused;
                app.set_status("✔  Finished — press [b] to go back");
            }
            last_tick = Instant::now();
        }
    }

    // 5. عند الخروج: تنظيف شاشة التيرمنال وإرجاعها لوضعها الطبيعي للينكس
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    println!("Bye! 🎵");
    Ok(())
}
