______________________________________________________________________

## name: mu-console-theme description: "Wireframe theme tokens, Nerd icon glyphs, font metrics, and SVG primitives for MuConsole."

# `mu-console-theme` Design Tokens & `WireframeSkin` Library

Use this skill when developing or testing visual presentation, Wireframe SVG rendering, typography metrics, and CSS tokens across MuConsole.

## 1. Design System & Styling Invariants

- **Color & Surface Tokens**: Dark-slate canvas, cyan accent, wireframe panels, and semantic status tones are defined in `mu_console_theme::tokens` and generated via `mu_console_theme::css::tokens_css()`.
- **CSS Delivery**: Use `:root` CSS variables and shell rules defined in `dist/tokens.css` or `assets/tokens.css` for console views.
- **Wireframe SVG Renderer**: Render controls, selectors, rotaries, lamps, readouts, and trainway screens cleanly using `mu_console_theme::wireframe::WireframeSkin` and `mu_console_theme::svg::SvgWriter`.
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
