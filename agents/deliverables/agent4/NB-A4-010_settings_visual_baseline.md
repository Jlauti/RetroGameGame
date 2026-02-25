# Visual Baseline: Settings Panel (NB-A4-010)

This document defines the visual standards for the **RetroGameGame** Settings Panel, specifically optimized for the **Era 4 (Dark Synthwave)** aesthetic.

## 1. Layout & Hierarchy

The panel should use a centered vertical stack with clear sectioning.

- **Header**: Large title "SETTINGS" centered at the top.
- **Body**: Three categorized groups of controls.
    1. **Audio**: Music Volume (Slider).
    2. **Display**: Resolution (Dropdown/Cycle), Mode (Toggle).
    3. **Game**: Quit Behavior (Cycle).
- **Footer**: Horizontal action bar.
    - `[ BACK ]` (Left)
    - `[ APPLY ]` (Right, Primary Focus)

### Spacing Grid
- **Base Unit**: 8px.
- **Outer Padding**: 48px.
- **Row Gap**: 16px.
- **Section Margin**: 32px.

---

## 2. Styling (Era 4: Dark Synthwave)

### Color Palette (from `src/ui/mod.rs`)
- **Panel Background**: `PANEL_BG` (`#0d0d1e` @ 92% opacity).
- **Border**: `PANEL_BORDER` (`#404073`).
- **Text**: `TEXT_PRIMARY` (`#e6e6f2`).
- **Labels**: `TEXT_SECONDARY` (`#9999b3`).
- **Accents**: `TEXT_ACCENT` (`#66ccff`).

### Interaction States
- **Normal**: `BUTTON_NORMAL` with `PANEL_BORDER`.
- **Hover**: `BUTTON_HOVER` with `EGA_BRIGHT_CYAN` border.
- **Selected/Focused**: Add 4px neon cyan (`#00ffff`) outer glow.
- **Destructive (Quit)**: `EGA_MAGENTA` text when highlighted.

---

## 3. Component Specifications

### Controls
- **Sliders**: Track in `EGA_DARK_GRAY`, handle in `TEXT_ACCENT`.
- **Buttons**: Beveled edges (2px) to match main menu style.
- **Toggles**: [ ON ] / [ off ] - Highlighted state uses `TEXT_ACCENT`.

### In-Game Overlay
- When active, the background game world must be dimmed using a full-screen semi-transparent overlay: `srgba(0.0, 0.0, 0.0, 0.65)`.

---

## 4. Reusable Asset Inventory

| Asset Path | Usage | Status |
| :--- | :--- | :--- |
| `assets/ui/main_menu_bg_v2.png` | Underlying background for Hub mode. | Found |
| N/A | Settings Header Font (Inter/Outfit) | System Default |
| **Missing** | Neon border sprite for glow effects | CTO Callout |

> [!IMPORTANT]
> **Readability First**: Ensure high contrast between text (`TEXT_PRIMARY`) and background (`PANEL_BG`) even when overlays are stacked.
