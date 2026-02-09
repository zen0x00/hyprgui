# HyprGUI

A native **GTK4 + libadwaita** graphical configuration tool for **Hyprland**, focused on
system theming, correctness, and beginner-friendly UX.

HyprGUI aims to provide a clean, GNOME-style settings experience while staying true to
Hyprland’s philosophy and configuration model.

---

## Goals

- Native GTK4 application (no WebView)
- Full libadwaita theming support (light/dark, accents, fonts)
- Beginner-friendly UI for Hyprland configuration
- Schema-driven settings (safe, structured, extensible)
- No manual config editing required
- Clean separation between UI and backend logic

---

## Current Status

🚧 **Early development**

Implemented so far:

- GTK4 + libadwaita application shell
- Sidebar navigation using `ListBox`
- Content switching via `Stack`
- Preferences layout using `PreferencesPage` and `PreferencesGroup`
- Theme-aware styling using GTK CSS

Planned next:

- Toggle / slider / dropdown setting rows
- Apply & Reset workflow
- Backend integration with `hyprctl`
- Live config preview
- Error handling and validation

---

## Tech Stack

- **Language:** Rust
- **UI Toolkit:** GTK4
- **Design System:** libadwaita
- **Target Platform:** Linux (Wayland / Hyprland)

---

## Why GTK4 + libadwaita?

HyprGUI is meant to feel like a **real system settings app**, not a web app pretending to
be native.

Using GTK4 + libadwaita provides:

- Automatic system theme integration
- Consistent spacing, typography, and layout
- Native accessibility support
- Wayland-first behavior
- Long-term maintainability

---

## Building & Running

### Dependencies (Arch Linux)

```bash
sudo pacman -S gtk4 libadwaita
```
