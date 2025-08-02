# 🐠 asciiquarium-rs

> A faithful Rust port of the classic ASCII aquarium animation

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Dive into a mesmerizing underwater world rendered entirely in ASCII art. This is a faithful Rust implementation of the beloved `asciiquarium` by Kirk Baucom, featuring swimming fish, floating seaweed, majestic whales, and other sea creatures—all in your terminal.

## ✨ Features

- 🎣 **Multiple fish species** with authentic ASCII art
- 🐋 **Large sea creatures** (whales, ships, sea monsters, sharks)
- 🌊 **Animated water surface** with realistic wave patterns
- 🏰 **Underwater castle** as a scenic backdrop
- 🪸 **Swaying seaweed** that lives and dies naturally
- ⚡ **Death callback system** - authentic population management
- 📺 **Dynamic screen adaptation** - scales beautifully to any terminal size
- 🎨 **Colorful animations** with randomized fish colors

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/asciiquarium-rs.git
cd asciiquarium-rs

# Run the aquarium
cargo run --release
```

## 🎮 Controls

| Key | Action |
|-----|--------|
| `q` | Quit the aquarium |
| `r` | Redraw (recreate all entities) |
| `p` | Toggle pause/unpause |

## 🛠️ Installation

### From Source

```bash
cargo install --git https://github.com/yourusername/asciiquarium-rs.git
```

### Requirements

- Rust 1.70 or later
- A terminal that supports ANSI colors

## 🎯 Design Philosophy

This implementation stays true to the original while embracing modern Rust practices:

- **Authentic Behavior**: Matches the original's entity spawning and death callback system
- **Single Large Creature**: Only one whale/ship/monster at a time, just like the original
- **Clean Architecture**: Simple functions instead of over-engineered managers
- **Performance**: Efficient entity management with proper depth layering

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐
│ Death Callbacks │───▶│ Entity Spawning  │
└─────────────────┘    └──────────────────┘
         │                       │
         ▼                       ▼
┌─────────────────┐    ┌──────────────────┐
│ Population      │◀───│ Screen Adaptation│
│ Management      │    │                  │
└─────────────────┘    └──────────────────┘
```

## 🐟 Entity Types

- **Fish**: Multiple species with horizontal movement and bubble generation
- **Seaweed**: Bottom-anchored plants with 2-frame sway animation
- **Water Surface**: 4-layer animated wave system
- **Large Creatures**: Whales (with spouts), ships, sea monsters, sharks
- **Castle**: Static background decoration

## 🔧 Development

```bash
# Build
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Lint
cargo clippy
```

## 📊 Project Stats

- **~4,000 lines** of clean Rust code
- **54 tests** ensuring correctness
- **Death callback system** for authentic behavior
- **Single large creature constraint** like the original

## 🙏 Credits

- **Original Asciiquarium**: Kirk Baucom (kbaucom@schizoid.com)
- **ASCII Art**: Joan Stark and contributors
- **Rust Port**: Built with [Ratatui](https://ratatui.rs)

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*"In the depths of your terminal, life finds a way..."* 🌊