# 🀄️ Taiwan Mahjong Solver (16-Tile)

A high-performance, standalone desktop application built with **Tauri v2**, **Rust**, and **React**. This tool calculates the optimal discard, Shanten (moves to win), and Ukeire (effective tiles) for Taiwanese 16-tile Mahjong.

## 🚀 Features

- **High-Performance Core**: Recursive backtracking (DFS) algorithm written in Rust for sub-millisecond Shanten calculations.
- **Interactive Tile Selector**: Build your hand and track visible discards via a modern, clickable UI—no manual string entry required.
- **Taiwanese 16-Tile Logic**: Specifically tuned for the "5 Melds + 1 Pair" (17 tiles in hand after draw) winning condition.
- **Real-time Analysis**: Provides a sorted list of discard recommendations based on:
  - **Shanten**: Minimum moves remaining to reach a winning state.
  - **Outs (Ukeire)**: The exact number of remaining tiles in the wall that will improve your hand.
- **Native macOS Support**: Bundled as a standalone `.app` and `.dmg`.

## 🛠 Tech Stack

- **Backend**: Rust (utilizing `serde` for serialization and `HashMap` memoization).
- **Frontend**: React + TypeScript (Vite).
- **Framework**: Tauri v2 (Native system WebView).
- **Package Manager**: Bun.

## 📦 Installation & Build

### Prerequisites
- [Rust](https://www.rust-lang.org/)
- [Bun](https://bun.sh/)

### Development
```bash
# Install dependencies
bun install

# Start development server
bun run tauri dev
```

### Build Standalone App (macOS)
```bash
# Build production bundle (.app and .dmg)
bun run tauri build
```
The output will be located in `src-tauri/target/release/bundle/macos/`.

## 🧠 Algorithm Logic

The solver uses a **Frequency Table** approach (mapping 34 tile types to their counts) to ensure $O(1)$ lookups. The Shanten calculation uses a recursive backtracking search optimized with **memoization** to avoid redundant branch exploration. 

Unlike 13-tile Riichi solvers, this engine is built from scratch to handle the **16-tile rule set**, where a complete hand consists of 5 sets (melds) and 1 pair.

## 📝 Usage

1. **Input Hand**: Click tiles in the grid to add exactly 17 tiles to your hand.
2. **Track Discards**: Toggle "Visible Discards" mode to mark tiles already played on the table.
3. **Analyze**: Hit "Analyze Hand" to view the recommendation table.
4. **Iterate**: Click a tile in your hand to "discard" it, then add a new tile to see the next optimal move.

---
*Created with ❤️ for Mahjong enthusiasts.*
