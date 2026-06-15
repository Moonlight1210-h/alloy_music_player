use crate::ui::app_state::{App, PlayerState, Screen};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
};

pub fn ui(f: &mut Frame, app: &mut App) {
    match app.screen {
        Screen::SongList => draw_song_list(f, app),
        Screen::Player => draw_player(f, app),
    }
}

fn draw_song_list(f: &mut Frame, app: &mut App) {
    let area = f.size();

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

    // Header
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

    // Song List
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

    // Footer
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
    let area = f.size();

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

    // Header
    let playing_icon = if app.player_state == PlayerState::Playing {
        "▶"
    } else {
        "⏸"
    };
    let header = Paragraph::new(Line::from(vec![Span::styled(
        format!("  {}  A L L O Y   P L A Y E R  ", playing_icon),
        Style::default()
            .fg(Color::Rgb(255, 200, 60))
            .add_modifier(Modifier::BOLD),
    )]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Rgb(50, 50, 80))),
    );
    f.render_widget(header, chunks[0]);

    // Song Title
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

    // Waveform Animation
    let bars = [
        "▁", "▁", "▂", "▂", "▃", "▃", "▄", "▄", "▅", "▅", "▆", "▆", "▇", "▇", "█", "▇", "▆", "▅",
        "▄", "▃", "▂", "▁",
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

    let wave_widget = Paragraph::new(vec![Line::from(""), Line::from(wave_spans)])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(30, 40, 70))),
        )
        .alignment(Alignment::Center);
    f.render_widget(wave_widget, chunks[2]);

    // Volume Gauge
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

    // Volume Input Mode
    if app.show_volume_input {
        let input_widget = Paragraph::new(Line::from(vec![
            Span::styled(
                "  Enter volume (0-100): ",
                Style::default().fg(Color::Rgb(200, 200, 100)),
            ),
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

    // Status
    if !app.status_msg.is_empty() {
        let status = Paragraph::new(Span::styled(
            format!("  {}", app.status_msg),
            Style::default().fg(Color::Rgb(150, 255, 150)),
        ));
        f.render_widget(status, chunks[5]);
    }

    // Controls
    let controls = if app.show_volume_input {
        Paragraph::new(Line::from(vec![
            Span::styled(
                "  Type number + ",
                Style::default().fg(Color::Rgb(100, 100, 140)),
            ),
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
