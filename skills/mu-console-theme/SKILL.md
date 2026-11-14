---
name: mu-console-theme
description: "Skin-agnostic ConsoleScene/DialogModel descriptors, HtmlCoreSkin, WireframeSkin, CSS tokens, Nerd glyphs, and font metrics for MuConsole."
---

# `mu-console-theme` Design Tokens, `HtmlCoreSkin` & `WireframeSkin`

Use this skill when developing or testing skin-agnostic scene descriptors, semantic HTML5 rendering, Wireframe SVG rendering, typography metrics, and CSS tokens across MuConsole.

## 1. Design System & Styling Invariants

- **Scene & Modal Descriptors**: Define views via `mu_console_theme::scene::{ConsoleScene, DialogModel, SceneRenderer}` across both `HtmlCoreSkin` and `WireframeSkin` so action routing, disabled controls, and ARIA semantics stay identical.
- **HtmlCoreSkin (`data-skin="html"`)**: Emit semantic HTML5 via void-aware `mu_console_theme::html::{HtmlWriter, HtmlCoreSkin}` with `<dialog>` modals, natural scrolling (`overflow-y: auto`), responsive grids, and `font-size: 16px` mobile inputs.
- **WireframeSkin (`data-skin="wireframe"`)**: Emit bounded, deterministic flat-fill SVG via `mu_console_theme::wireframe::WireframeSkin` and `mu_console_theme::svg::SvgWriter` (fixed `100dvh`, zero filters, strict viewport containment).
- **Color & Surface Tokens**: Dark-slate canvas, cyan accent, wireframe panels, disabled tokens, and semantic status tones live in `mu_console_theme::tokens` / `mu_console_theme::css::tokens_css()` (synchronized to `assets/tokens.css` and `dist/tokens.css`).
- **Nerd Font Icons**: Use `mu_console_theme::glyphs` for CRT icon codepoints and font subsetting (`crt` profile for `Hurmit` and `OpenDyslexicM`).

## 2. Command-Line Utilities

- **`mu-console-tokens-css`**: Emits the authoritative `:root` CSS variables and wireframe layout styles.
- **`mu-console-subset-unicodes`**: Calculates minimal unicode coverage for font subsetting (`crt` profile).

## 3. Verification Commands

```bash
cargo test --all-targets
cargo check --target wasm32-unknown-unknown
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```
