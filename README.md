# Alloy 🦀

Alloy is a high-performance, data-driven audio engine and music player built from the ground up using **Rust**. This project is an open-source initiative focused on raw systems control, efficient memory management, and intelligent audio analysis.

## 🚀 Vision

The goal of Alloy is to move away from "pre-packaged" solutions and build a specialized audio system that gives developers and users deep control over their music library. By combining **Systems Programming (Rust)** with **Data Science**, Alloy aims to provide an intelligent listening experience through advanced audio feature extraction.

## ✅ Features

- **Custom Audio Engine** — Built in Rust via `rodio` for near-zero latency
- **Interactive TUI** — Real-time terminal interface powered by `ratatui` + `crossterm`
- **Live Waveform Animation** — Animated bar visualizer that reacts to playback state
- **Real Volume Control** — Scroll with `↑↓`, fine-tune with `+/-`, or type exact value with `v`
- **Pause / Resume** — Instant response, waveform freezes on pause
- **Song Library Browser** — Navigate your music folder with keyboard
- **Low Memory Footprint** — ~10 MB RAM vs ~300 MB for Electron-based players
- **CLI Focused** — Designed for developers who love the terminal

## 🎮 Controls

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate list / Adjust volume ±5% |
| `Enter` | Play selected song |
| `p` | Pause / Resume |
| `v` | Type exact volume (0-100) |
| `+` / `-` | Volume up / down |
| `b` | Back to library |
| `q` | Quit |

## 📁 Project Structure

```
alloy/
├── src/
│   ├── main.rs              # TUI event loop & UI rendering
│   ├── engine/
│   │   ├── audio.rs         # Playback engine (rodio)
│   │   └── loader.rs        # Folder scanner
│   └── data_models/
│       └── song.rs          # Song struct
├── data/                    # Audio files (.mp3, .wav, .flac)
├── scripts/                 # Python audio analysis tools
└── Cargo.toml
```

## 💻 Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust 🦀 |
| Audio | `rodio` |
| TUI | `ratatui` + `crossterm` |
| OS | Linux (Optimized for Linux Mint XFCE) |
| Tools | Git, Cargo, VSCodium |
| Data Science | Python (Librosa / Essentia) |

## 🚀 Usage

```bash
git clone https://github.com/Moonlight1210-h/alloy_music_player
cd alloy_music_player

# Add your audio files to data/
cargo run
```

## 🗺 Roadmap

### v0.1 — Foundation ✅ Complete

- [x] Basic audio playback engine
- [x] Data models for songs / metadata
- [x] CLI interface
- [x] Interactive TUI (ratatui)
- [x] Live waveform animation
- [x] Real-time volume control
- [x] Pause / Resume

### v0.2 — Core Features (Current)

- [ ] Playlist management
- [ ] Shuffle / Repeat modes
- [ ] Song progress bar with seek
- [ ] Auto-play next song

### v0.3 — Data & Analysis

- [ ] BPM detection (Python / Librosa)
- [ ] Mood analysis
- [ ] Auto-tagging from audio features

### v0.4 — Polish

- [ ] Config file support (`~/.config/alloy`)
- [ ] Multiple audio format support
- [ ] Performance benchmarks

### Future Vision

- Music recommendation engine
- Network streaming support

## ⚡ Performance

Alloy is intentionally lightweight:

| | Alloy | Spotify (Electron) |
|--|-------|--------------------|
| RAM (idle) | ~10 MB | ~300 MB |
| Startup | instant | 3-5 sec |
| CPU (idle) | <1% | 5-15% |

## 🤝 Collaboration

Alloy is an open-source project. We believe in *"Code speaks louder than bureaucracy."*
Feel free to open a **Pull Request** or an **Issue**.

---

*Built with passion by a single developer* 🦀
