# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build           # Debug build
cargo build --release # Release build
cargo run             # Run the application
cargo clippy          # Lint
cargo fmt             # Format code
cargo test            # Run tests
```

Runtime dependency: `hyprctl` must be available (requires Hyprland running). GTK4 and libadwaita are needed at build time (`gtk4`, `libadwaita` packages on Arch).

## Architecture

HyprGUI is a native GTK4 + libadwaita settings GUI for Hyprland. It reads and writes Hyprland configuration via `hyprctl` commands.

**Data flow:**
1. On launch: `backend/hyprland.rs::read_general()` calls `hyprctl getoption -j` → parses JSON → populates `GeneralState`
2. UI widgets are initialized from `GeneralState` values
3. Widget change signals update `GeneralState` via `Rc<RefCell<GeneralState>>`
4. On Apply: `hyprland::apply_general()` sends each setting via `hyprctl keyword`, then re-reads state to refresh UI
5. On Reset: state reverts to defaults, UI refreshes

**Module layout:**
- `src/main.rs` — creates `libadwaita::Application`, connects activate signal
- `src/app.rs` — loads CSS from `assets/style.css`, builds the window
- `src/window.rs` — assembles the 3-panel layout (sidebar + content stack + footer), wires sidebar navigation to the content stack
- `src/state.rs` — `GeneralState` struct holding all current Hyprland settings
- `src/backend/hyprland.rs` — all `hyprctl` I/O: `read_general()`, `apply_general()`, and parsers (`read_int`, `read_gradient`, `read_gap`)
- `src/ui/sidebar.rs` — collapsible navigation sidebar
- `src/ui/content.rs` — settings pages (currently just the General page with spin buttons and color entries)
- `src/ui/footer.rs` — Reset/Apply buttons with confirmation dialogs

**State sharing pattern:** `GeneralState` is wrapped in `Rc<RefCell<>>` and passed by clone into GTK signal closures. Each widget's change callback mutates the shared state, and a "refresh" closure reads it back to sync all widgets.

**Adding a new setting:** Add field to `GeneralState` → parse it in `read_general()` → write it in `apply_general()` → add UI widget in `content.rs` with change/refresh callbacks.
