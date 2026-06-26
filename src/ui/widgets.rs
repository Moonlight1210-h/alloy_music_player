use crate::ui::app_state::{App, PlayerState, Screen, ThemeMode};
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
    let is_dark = app.theme == ThemeMode::Dark;

    // ── 🎨 استخراج ألوان الثيم لشاشة قائمة الأغاني (Song List) ──
    let bg_color = if is_dark { Color::Rgb(10, 10, 20) } else { Color::Rgb(240, 242, 245) };
    let accent_color = if is_dark { Color::Rgb(255, 180, 50) } else { Color::Rgb(210, 100, 0) };
    let title_color = if is_dark { Color::Rgb(255, 220, 100) } else { Color::Rgb(150, 70, 0) };
    let border_color = if is_dark { Color::Rgb(60, 60, 100) } else { Color::Rgb(170, 170, 190) };
    let text_color = if is_dark { Color::Rgb(200, 200, 220) } else { Color::Rgb(40, 40, 50) };
    let muted_color = if is_dark { Color::Rgb(100, 100, 130) } else { Color::Rgb(130, 130, 150) };
    let highlight_bg = if is_dark { Color::Rgb(40, 40, 80) } else { Color::Rgb(215, 230, 250) };
    let highlight_fg = if is_dark { Color::Rgb(255, 200, 80) } else { Color::Rgb(180, 60, 0) };
    let line_color = if is_dark { Color::Rgb(60, 60, 80) } else { Color::Rgb(190, 190, 210) };
    let footer_border = if is_dark { Color::Rgb(40, 40, 60) } else { Color::Rgb(200, 200, 210) };

    f.render_widget(
        Block::default().style(Style::default().bg(bg_color)),
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
            Span::styled("  ♬  ", Style::default().fg(accent_color)),
            Span::styled(
                "A L L O Y   P L A Y E R",
                Style::default()
                    .fg(title_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            "  ─────────────────────────────",
            Style::default().fg(line_color),
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
                Style::default().fg(muted_color),
            );
            let title = Span::styled(
                s.title.clone(),
                Style::default().fg(text_color),
            );
            ListItem::new(Line::from(vec![num, title]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(Span::styled(
                    " 📂 Library ",
                    Style::default().fg(if is_dark { Color::Rgb(150, 150, 200) } else { Color::Rgb(100, 100, 150) }),
                )),
        )
        .highlight_style(
            Style::default()
                .bg(highlight_bg)
                .fg(highlight_fg)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, chunks[1], &mut app.list_state);

    // Footer
    let help = Paragraph::new(Line::from(vec![
        styled_key("↑↓", is_dark),
        Span::styled(" navigate   ", Style::default().fg(text_color)),
        styled_key("Enter", is_dark),
        Span::styled(" play   ", Style::default().fg(text_color)),
        styled_key("T", is_dark),
        Span::styled(" toggle theme   ", Style::default().fg(text_color)),
        styled_key("q", is_dark),
        Span::styled(" quit", Style::default().fg(text_color)),
    ]))
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(footer_border)),
    )
    .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_player(f: &mut Frame, app: &mut App) {
    let area = f.size();
    let is_dark = app.theme == ThemeMode::Dark;

    // ── 🎨 استخراج ألوان الثيم لشاشة المشغل الرئيسي (Player) ──
    let bg_color = if is_dark { Color::Rgb(8, 8, 18) } else { Color::Rgb(245, 245, 247) };
    let accent_color = if is_dark { Color::Rgb(255, 200, 60) } else { Color::Rgb(190, 90, 0) };
    let text_main = if is_dark { Color::White } else { Color::Rgb(30, 30, 30) };
    let border_color = if is_dark { Color::Rgb(40, 40, 70) } else { Color::Rgb(180, 180, 200) };
    let wave_border = if is_dark { Color::Rgb(30, 40, 70) } else { Color::Rgb(190, 200, 220) };
    let status_color = if is_dark { Color::Rgb(150, 255, 150) } else { Color::Rgb(0, 130, 50) };
    let gauge_bg = if is_dark { Color::Rgb(20, 20, 40) } else { Color::Rgb(220, 220, 230) };
    let icon_color = if is_dark { Color::Rgb(255, 150, 50) } else { Color::Rgb(230, 90, 0) };
    let footer_border = if is_dark { Color::Rgb(40, 40, 60) } else { Color::Rgb(200, 200, 210) };
    let muted_color = if is_dark { Color::Rgb(100, 100, 130) } else { Color::Rgb(130, 130, 150) };

    f.render_widget(
        Block::default().style(Style::default().bg(bg_color)),
        area,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Header [0]
            Constraint::Length(3), // Song Title [1]
            Constraint::Length(5), // Waveform [2]
            Constraint::Length(3), // Progress Bar [3]
            Constraint::Length(3), // Volume Gauge [4]
            Constraint::Length(3), // Volume Input [5]
            Constraint::Length(3), // Status [6]
            Constraint::Min(1),    // Spacer [7]
            Constraint::Length(3), // Controls [8]
        ])
        .split(area);

    // Header
    let playing_icon = if app.player_state == PlayerState::Playing { "▶" } else { "⏸" };
    let header = Paragraph::new(Line::from(vec![Span::styled(
        format!("  {}  A L L O Y   P L A Y E R  ", playing_icon),
        Style::default()
            .fg(accent_color)
            .add_modifier(Modifier::BOLD),
    )]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(border_color)),
    );
    f.render_widget(header, chunks[0]);

    // Song Title
    let title_widget = Paragraph::new(Line::from(vec![
        Span::styled("  ♪  ", Style::default().fg(icon_color)),
        Span::styled(
            &app.current_title,
            Style::default()
                .fg(text_main)
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
                0 => if is_dark { Color::Rgb(60, 80, 120) } else { Color::Rgb(140, 160, 190) },
                1 => if is_dark { Color::Rgb(80, 140, 200) } else { Color::Rgb(100, 150, 220) },
                2 => if is_dark { Color::Rgb(100, 200, 255) } else { Color::Rgb(50, 120, 240) },
                _ => if is_dark { Color::Rgb(180, 230, 255) } else { Color::Rgb(0, 80, 200) },
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
                .border_style(Style::default().fg(wave_border)),
        )
        .alignment(Alignment::Center);
    f.render_widget(wave_widget, chunks[2]);

    // Progress Bar
    let current_secs = app.current_pos.as_secs();
    let total_secs = app.current_duration.as_secs();
    
    let progress_ratio = if total_secs > 0 {
        (current_secs as f64 / total_secs as f64).min(1.0)
    } else {
        0.0
    };

    let progress_label = format!(
        " {:02}:{:02} / {:02}:{:02} ",
        current_secs / 60, current_secs % 60,
        total_secs / 60, total_secs % 60
    );

    let progress_bar = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(Span::styled(
                    " ⌛ Progress ",
                    Style::default().fg(if is_dark { Color::Rgb(150, 150, 200) } else { Color::Rgb(100, 100, 150) }),
                )),
        )
        .gauge_style(
            Style::default()
                .fg(if is_dark { Color::Rgb(0, 210, 140) } else { Color::Rgb(0, 140, 90) })
                .bg(gauge_bg)
        )
        .ratio(progress_ratio)
        .label(Span::styled(
            progress_label,
            Style::default().fg(text_main).add_modifier(Modifier::BOLD),
        ));
    f.render_widget(progress_bar, chunks[3]);

    // Volume Gauge
    let vol_pct = (app.volume * 100.0) as u16;
    let vol_color = if app.volume > 0.7 {
        Color::Rgb(255, 100, 80)
    } else if app.volume > 0.4 {
        if is_dark { Color::Rgb(100, 220, 120) } else { Color::Rgb(30, 160, 50) }
    } else {
        Color::Rgb(80, 160, 255)
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(Span::styled(
                    " 🔊 Volume ",
                    Style::default().fg(if is_dark { Color::Rgb(150, 150, 200) } else { Color::Rgb(100, 100, 150) }),
                )),
        )
        .gauge_style(Style::default().fg(vol_color).bg(gauge_bg))
        .percent(vol_pct)
        .label(Span::styled(
            format!(" {}% ", vol_pct),
            Style::default()
                .fg(text_main)
                .add_modifier(Modifier::BOLD),
        ));
    f.render_widget(gauge, chunks[4]);

    // Volume Input Mode
    if app.show_volume_input {
        let input_widget = Paragraph::new(Line::from(vec![
            Span::styled(
                "  Enter volume (0-100): ",
                Style::default().fg(if is_dark { Color::Rgb(200, 200, 100) } else { Color::Rgb(140, 120, 0) }),
            ),
            Span::styled(
                &app.volume_input_buf,
                Style::default()
                    .fg(text_main)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("█", Style::default().fg(Color::Rgb(255, 200, 0))),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(200, 200, 0))),
        );
        f.render_widget(input_widget, chunks[5]);
    }

    // Status
    if !app.status_msg.is_empty() {
        let status = Paragraph::new(Span::styled(
            format!("  {}", app.status_msg),
            Style::default().fg(status_color),
        ));
        f.render_widget(status, chunks[6]);
    }

    // Controls
    let controls = if app.show_volume_input {
        Paragraph::new(Line::from(vec![
            Span::styled(
                "  Type number + ",
                Style::default().fg(muted_color),
            ),
            styled_key("Enter", is_dark),
            Span::raw("  "),
            styled_key("Esc", is_dark),
            Span::styled(" cancel", Style::default().fg(muted_color)),
        ]))
    } else {
        Paragraph::new(Line::from(vec![
            styled_key("p", is_dark),
            Span::styled(" pause/play  ", Style::default().fg(text_main)),
            styled_key("←↓", is_dark),
            Span::styled(" vol/seek  ", Style::default().fg(text_main)),
            styled_key("v", is_dark),
            Span::styled(" type vol  ", Style::default().fg(text_main)),
            styled_key("T", is_dark),
            Span::styled(" theme  ", Style::default().fg(text_main)),
            styled_key("b", is_dark),
            Span::styled(" back  ", Style::default().fg(text_main)),
            styled_key("q", is_dark),
            Span::styled(" quit", Style::default().fg(text_main)),
        ]))
    }
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(footer_border)),
    )
    .alignment(Alignment::Center);
    f.render_widget(controls, chunks[8]);
}

fn styled_key(k: &str, is_dark: bool) -> Span<'_> {
    let key_color = if is_dark { Color::Rgb(255, 200, 60) } else { Color::Rgb(190, 90, 0) };
    Span::styled(
        format!("[{}]", k),
        Style::default()
            .fg(key_color)
            .add_modifier(Modifier::BOLD),
    )
}