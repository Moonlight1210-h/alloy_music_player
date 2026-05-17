pub mod data_models;
pub mod engine;

use crate::data_models::song::Song;
use crate::engine::loader::scan_folder;
use crate::engine::audio::play_song;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

// ─── حالة التطبيق ────────────────────────────────────────────────────────────

#[derive(PartialEq)]
enum Screen {
    SongList,
    Player,
}

#[derive(PartialEq)]
enum PlayerState {
    Playing,
    Paused,
}

struct App {
    screen: Screen,
    songs: Vec<Song>,
    list_state: ListState,
    player_state: PlayerState,
    current_title: String,
    volume: f32,
    sink: Option<Arc<Mutex<rodio::Sink>>>,
    _stream: Option<rodio::OutputStream>,
    wave_tick: usize,
    show_volume_input: bool,
    volume_input_buf: String,
    status_msg: String,
    status_until: Option<Instant>,
}

impl App {
    fn new(songs: Vec<Song>) -> Self {
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

    fn selected_index(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    fn play_selected(&mut self) {
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

    fn toggle_pause(&mut self) {
        // نسخ الحالة الحالية أولاً قبل ما نعمل borrow على sink
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

    fn volume_up(&mut self) {
        self.volume = (self.volume + 0.05).min(1.0);
        self.apply_volume();
        self.set_status(&format!("VOL {:>3.0}%", self.volume * 100.0));
    }

    fn volume_down(&mut self) {
        self.volume = (self.volume - 0.05).max(0.0);
        self.apply_volume();
        self.set_status(&format!("VOL {:>3.0}%", self.volume * 100.0));
    }

    fn set_status(&mut self, msg: &str) {
        self.status_msg = msg.to_string();
        self.status_until = Some(Instant::now() + Duration::from_secs(3));
    }

    fn tick(&mut self) {
        self.wave_tick = self.wave_tick.wrapping_add(1);
        if let Some(until) = self.status_until {
            if Instant::now() > until {
                self.status_msg.clear();
                self.status_until = None;
            }
        }
    }

    fn is_finished(&self) -> bool {
        if let Some(ref sink) = self.sink {
            sink.lock().unwrap().empty()
        } else {
            false
        }
    }

    fn list_next(&mut self) {
        let len = self.songs.len();
        if len == 0 { return; }
        let next = (self.selected_index() + 1) % len;
        self.list_state.select(Some(next));
    }

    fn list_prev(&mut self) {
        let len = self.songs.len();
        if len == 0 { return; }
        let cur = self.selected_index();
        let prev = if cur == 0 { len - 1 } else { cur - 1 };
        self.list_state.select(Some(prev));
    }
}

// ─── الواجهة ──────────────────────────────────────────────────────────────────

fn ui(f: &mut Frame, app: &mut App) {
    match app.screen {
        Screen::SongList => draw_song_list(f, app),
        Screen::Player   => draw_player(f, app),
    }
}

fn draw_song_list(f: &mut Frame, app: &mut App) {
    let area = f.size(); // ← size() بدل area()

    f.render_widget(
        Block::default().style(Style::default().bg(Color::Rgb(10, 10, 20))),
        area,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    // ── Header ──
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("  ♬  ", Style::default().fg(Color::Rgb(255, 180, 50))),
            Span::styled(
                "A L L O Y   P L A Y E R",
                Style::default()
                    .fg(Color::Rgb(255, 220, 100))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            "  ─────────────────────────────",
            Style::default().fg(Color::Rgb(60, 60, 80)),
        )),
    ])
    .alignment(Alignment::Left);
    f.render_widget(header, chunks[0]);

    // ── Song List ──
    let items: Vec<ListItem> = app
        .songs
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let num = Span::styled(
                format!(" {:>2}. ", i + 1),
                Style::default().fg(Color::Rgb(100, 100, 130)),
            );
            let title = Span::styled(
                s.title.clone(),
                Style::default().fg(Color::Rgb(200, 200, 220)),
            );
            ListItem::new(Line::from(vec![num, title]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(60, 60, 100)))
                .title(Span::styled(
                    " 📂 Library ",
                    Style::default().fg(Color::Rgb(150, 150, 200)),
                )),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(40, 40, 80))
                .fg(Color::Rgb(255, 200, 80))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, chunks[1], &mut app.list_state);

    // ── Footer ──
    let help = Paragraph::new(Line::from(vec![
        styled_key("↑↓"),
        Span::raw(" navigate   "),
        styled_key("Enter"),
        Span::raw(" play   "),
        styled_key("q"),
        Span::raw(" quit"),
    ]))
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Rgb(40, 40, 60))),
    )
    .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_player(f: &mut Frame, app: &mut App) {
    let area = f.size(); // ← size() بدل area()

    f.render_widget(
        Block::default().style(Style::default().bg(Color::Rgb(8, 8, 18))),
        area,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

    // ── Header ──
    let playing_icon = if app.player_state == PlayerState::Playing { "▶" } else { "⏸" };
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("  {}  A L L O Y   P L A Y E R  ", playing_icon),
            Style::default()
                .fg(Color::Rgb(255, 200, 60))
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Rgb(50, 50, 80))),
    );
    f.render_widget(header, chunks[0]);

    // ── Song Title ──
    let title_widget = Paragraph::new(Line::from(vec![
        Span::styled("  ♪  ", Style::default().fg(Color::Rgb(255, 150, 50))),
        Span::styled(
            &app.current_title,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC),
        ),
    ]))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });
    f.render_widget(title_widget, chunks[1]);

    // ── Waveform Animation ──
    let bars = [
        "▁","▁","▂","▂","▃","▃","▄","▄","▅","▅","▆","▆","▇","▇","█",
        "▇","▆","▅","▄","▃","▂","▁",
    ];
    let wave_len = bars.len();
    let offset = app.wave_tick % wave_len;

    let wave_spans: Vec<Span> = (0..36)
        .map(|i| {
            let idx = (i + offset) % wave_len;
            let height = match bars[idx] {
                "▁" | "▂" => 0,
                "▃" | "▄" => 1,
                "▅" | "▆" => 2,
                _ => 3,
            };
            let color = match height {
                0 => Color::Rgb(60, 80, 120),
                1 => Color::Rgb(80, 140, 200),
                2 => Color::Rgb(100, 200, 255),
                _ => Color::Rgb(180, 230, 255),
            };
            let bar = if app.player_state == PlayerState::Paused {
                "▂"
            } else {
                bars[idx]
            };
            Span::styled(format!("{} ", bar), Style::default().fg(color))
        })
        .collect();

    let wave_widget = Paragraph::new(vec![
        Line::from(""),
        Line::from(wave_spans),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(30, 40, 70))),
    )
    .alignment(Alignment::Center);
    f.render_widget(wave_widget, chunks[2]);

    // ── Volume Gauge ──
    let vol_pct = (app.volume * 100.0) as u16;
    let vol_color = if app.volume > 0.7 {
        Color::Rgb(255, 100, 80)
    } else if app.volume > 0.4 {
        Color::Rgb(100, 220, 120)
    } else {
        Color::Rgb(80, 160, 255)
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(40, 40, 70)))
                .title(Span::styled(
                    " 🔊 Volume ",
                    Style::default().fg(Color::Rgb(150, 150, 200)),
                )),
        )
        .gauge_style(Style::default().fg(vol_color).bg(Color::Rgb(20, 20, 40)))
        .percent(vol_pct)
        .label(Span::styled(
            format!(" {}% ", vol_pct),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));
    f.render_widget(gauge, chunks[3]);

    // ── Volume Input Mode ──
    if app.show_volume_input {
        let input_widget = Paragraph::new(Line::from(vec![
            Span::styled("  Enter volume (0-100): ", Style::default().fg(Color::Rgb(200, 200, 100))),
            Span::styled(
                &app.volume_input_buf,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("█", Style::default().fg(Color::Rgb(255, 200, 0))),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(200, 200, 0))),
        );
        f.render_widget(input_widget, chunks[4]);
    }

    // ── Status ──
    if !app.status_msg.is_empty() {
        let status = Paragraph::new(Span::styled(
            format!("  {}", app.status_msg),
            Style::default().fg(Color::Rgb(150, 255, 150)),
        ));
        f.render_widget(status, chunks[5]);
    }

    // ── Controls ──
    let controls = if app.show_volume_input {
        Paragraph::new(Line::from(vec![
            Span::styled("  Type number + ", Style::default().fg(Color::Rgb(100, 100, 140))),
            styled_key("Enter"),
            Span::raw("  "),
            styled_key("Esc"),
            Span::styled(" cancel", Style::default().fg(Color::Rgb(100, 100, 140))),
        ]))
    } else {
        Paragraph::new(Line::from(vec![
            styled_key("p"),
            Span::raw(" pause/play  "),
            styled_key("↑↓"),
            Span::raw(" volume  "),
            styled_key("v"),
            Span::raw(" type vol  "),
            styled_key("b"),
            Span::raw(" back  "),
            styled_key("q"),
            Span::raw(" quit"),
        ]))
    }
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Rgb(40, 40, 60))),
    )
    .alignment(Alignment::Center);
    f.render_widget(controls, chunks[7]);
}

fn styled_key(k: &str) -> Span<'_> {
    Span::styled(
        format!("[{}]", k),
        Style::default()
            .fg(Color::Rgb(255, 200, 60))
            .add_modifier(Modifier::BOLD),
    )
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let songs = scan_folder("data/").unwrap_or_default();
    let mut app = App::new(songs);

    if app.songs.is_empty() {
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        println!("No songs found in data/ folder.");
        return Ok(());
    }

    let tick_rate = Duration::from_millis(80);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_default();

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // ── Volume Input Mode ──
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
                                app.apply_volume();
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

                // ── Normal Mode ──
                match app.screen {
                    Screen::SongList => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Down | KeyCode::Char('j') => app.list_next(),
                        KeyCode::Up   | KeyCode::Char('k') => app.list_prev(),
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
                        KeyCode::Up   | KeyCode::Char('+') => app.volume_up(),
                        KeyCode::Down | KeyCode::Char('-') => app.volume_down(),
                        _ => {}
                    },
                }

                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('c')
                {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            if app.screen == Screen::Player && app.is_finished() {
                app.player_state = PlayerState::Paused;
                app.set_status("✔  Finished — press [b] to go back");
            }
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    println!("Bye! 🎵");
    Ok(())
}
