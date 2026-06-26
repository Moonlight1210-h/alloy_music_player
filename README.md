# Alloy 🦀

Alloy is a high-performance, data-driven audio engine and music player built from the ground up using **Rust**. This project is an open-source initiative focused on raw systems control, efficient memory management, and a polished terminal experience.

## 🚀 Vision

The goal of Alloy is to move away from "pre-packaged" solutions and build a specialized audio system that gives developers and users deep control over their music library. By combining **Systems Programming** principles with practical audio analysis tools, Alloy aims to be both lightweight and powerful.

## ✅ Implemented Features

- **Custom Audio Engine** — Built in Rust via `rodio` for near-zero latency playback
- **Interactive TUI** — Real-time terminal interface powered by `ratatui` + `crossterm`
- **Live Waveform Animation** — Animated bar visualizer that reacts to playback state; freezes on pause
- **Dark / Light Theme Toggle** — Switch between dark and light modes at any time with `T`
- **Real Progress Bar** — Displays current position and total duration as `MM:SS / MM:SS`
- **Seek Forward / Backward** — Jump ±5 seconds with the arrow keys
- **Real Volume Control** — Adjust with `↑↓` or `+/-`, or type an exact value with `V`
- **Pause / Resume** — Instant response with waveform freeze on pause
- **Song Library Browser** — Navigate your music folder dynamically with keyboard
- **ID3 Tag Reading** — Extracts title and artist metadata from MP3 files automatically
- **Dynamic MP3 Scanning** — Auto-discovers all `.mp3` files in the `data/` folder at startup
- **Real Duration Detection** — Calculates accurate track lengths from audio data at load time
- **CI / CD Pipeline** — Automated build and test checks via GitHub Actions on every push
- **Low Memory Footprint** — ~10 MB RAM vs ~300 MB for Electron-based players

## 🎮 Keyboard Controls

### Song List Screen

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate library |
| `Enter` | Play selected song |
| `T` | Toggle Dark / Light theme |
| `Q` | Quit |

### Player Screen

| Key | Action |
|-----|--------|
| `P` | Pause / Resume |
| `↑` / `+` | Volume up +5% |
| `↓` / `-` | Volume down -5% |
| `V` | Type exact volume (0–100) |
| `→` | Seek forward 5 seconds |
| `←` | Seek backward 5 seconds |
| `T` | Toggle Dark / Light theme |
| `B` | Back to library |
| `Q` | Quit |

### Volume Input Mode

| Key | Action |
|-----|--------|
| `0–9` | Type volume value |
| `Enter` | Confirm volume |
| `Esc` | Cancel |

## 📁 Project Structure

```
alloy/
├── src/
│   ├── main.rs                  # Entry point — TUI setup, MP3 scanning, event loop
│   ├── engine.rs                # Engine module declaration
│   ├── engine/
│   │   ├── audio.rs             # Playback engine — rodio sink, seek, duration
│   │   ├── loader.rs            # ID3 tag reader and folder scanner
│   │   └── playlist.rs          # Playlist data structure (add / find)
│   ├── data_models.rs           # Data models module declaration
│   ├── data_models/
│   │   └── song.rs              # Song struct (title, artist, path, duration_sec)
│   ├── ui.rs                    # UI module declaration
│   └── ui/
│       ├── app_state.rs         # Central app state, all logic (seek, volume, theme)
│       └── widgets.rs           # TUI rendering — song list + player screens
├── .github/
│   └── workflows/
│       └── rust.yml             # GitHub Actions CI (build + test)
├── data/                        # Drop your .mp3 files here
├── scripts/                     # Python audio analysis tools (planned)
└── Cargo.toml
```

## 💻 Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust 🦀 (Edition 2024) |
| Audio | `rodio 0.17` |
| TUI | `ratatui 0.26` + `crossterm 0.27` |
| Metadata | `id3 1.0` |
| Error handling | `anyhow 1` |
| OS | Linux (Optimized for Linux Mint XFCE) |
| Tools | Git, Cargo, VSCodium |
| CI/CD | GitHub Actions |
| Data Science (Planned) | Python — Librosa / Essentia |

## 🚀 Getting Started

```bash
# Clone the repository
git clone https://github.com/Moonlight1210-h/alloy_music_player
cd alloy_music_player

# Add your MP3 files to the data/ folder
cp ~/Music/*.mp3 data/

# Build and run
cargo run
```

> **Note:** If no `.mp3` files are found in `data/`, the player will display a warning and show an empty library.

## ⚡ Performance

Alloy is intentionally lightweight:

| | Alloy | Spotify (Electron) |
|--|-------|-------------------|
| RAM (idle) | ~10 MB | ~300 MB |
| Startup time | instant | 3–5 sec |
| CPU (idle) | < 1% | 5–15% |

## 🗺 Roadmap

### v0.1 — Foundation ✅ Complete

- [x] Audio playback engine (rodio)
- [x] Song data models
- [x] Dynamic MP3 folder scanning
- [x] Real duration detection at load time
- [x] Interactive TUI (ratatui)
- [x] Song library browser
- [x] Live waveform animation
- [x] Pause / Resume
- [x] Real-time volume control (scroll, hotkeys, direct input)
- [x] Progress bar with real `MM:SS / MM:SS` display
- [x] Seek forward / backward ±5 seconds
- [x] Dark / Light theme toggle
- [x] ID3 tag reader
- [x] CI/CD pipeline (GitHub Actions)

### v0.2 — Core Features (Next)

- [ ] Playlist management (save / load)
- [ ] Shuffle mode
- [ ] Repeat / Loop modes
- [ ] Auto-play next song when track ends
- [ ] Support for additional audio formats (WAV, FLAC, OGG)

### v0.3 — Data & Analysis

- [ ] BPM detection (Python / Librosa)
- [ ] Mood analysis
- [ ] Auto-tagging from audio features
- [ ] JSON metadata export

### v0.4 — Polish

- [ ] Config file support (`~/.config/alloy`)
- [ ] Performance benchmarks
- [ ] Mouse support in TUI

### Future Vision

- Music recommendation engine
- Network / streaming support

## 🤝 Collaboration

Alloy is an open-source project. We believe in *"Code speaks louder than bureaucracy."*
Feel free to open a **Pull Request** or an **Issue**.

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

*Built with passion by a single developer* 🦀