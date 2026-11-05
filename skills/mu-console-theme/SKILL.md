______________________________________________________________________

## name: mu-console-theme description: "Avionics Terminal design tokens, 5x7 LCD glyphs, font metrics, and SVG instruments for MuConsole."

# `mu-console-theme` Avionics Terminal Tokens & SVG Instrument Library

Use this skill when developing or testing visual presentation, material styling, SVG instruments, typography metrics, and CSS tokens across the MuConsole terminal architecture.

## 1. Avionics Design System & Styling Invariants

- **Material & Color Tokens**: Avionics dark slate, cyan glow, CRT phosphor, amber warning, green annunciator, and alarm red tokens are defined in `mu_console_theme::tokens` and generated via `mu_console_theme::css::tokens_css()`.
- **CSS Delivery**: Use `:root` CSS variables defined in `dist/tokens.css` or `assets/tokens.css` for all console views.
- **Parametric Instruments**: Render dynamic annunciator lamps, rotary controls, CRT frames, and STN LCD windows using the pure SVG generator library in `mu_console_theme::instruments`.
- **5x7 LCD Glyphs**: Enforce 5x7 dot-matrix bitmap glyph fallback for STN displays (`mu_console_theme::lcd`) and Nerd Font icon glyphs for status annunciators (`mu_console_theme::glyphs`).

## 2. Command-Line Utilities

- **`mu-console-tokens-css`**: Emits the authoritative `:root` CSS variables and wireframe layout styles.
- **`mu-console-subset-unicodes`**: Calculates minimal unicode coverage for font subsetting (`crt` and `etch` profiles).

## 3. Verification Commands

```bash
cargo test --all-targets
cargo check --target wasm32-unknown-unknown
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```
