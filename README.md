# mu-console-theme

Design tokens, Nerd icon table, font metrics, and `WireframeSkin` SVG renderer for MuConsole.

## Architecture

- **Color & Layout Tokens**: Dark-slate canvas, wireframe surfaces, semantic status tones, and `:root` CSS generation via `mu_console_theme::css::tokens_css()`.
- **Nerd Icon Table**: Vector Nerd icon codepoints for semantic indicators (`mu_console_theme::glyphs`).
- **Font Metrics**: Measured typography bounding boxes and advance widths for `Hurmit` and `OpenDyslexicM` CRT layout calculations.
- **WireframeSkin Renderer**: Dependency-free pure SVG renderer (`mu_console_theme::wireframe::WireframeSkin`) with crisp geometry and semantic hit targets.
- **CLI Utilities**:
  - `mu-console-tokens-css`: Emits authoritative `tokens.css`.
  - `mu-console-subset-unicodes`: Generates `pyftsubset` unicode ranges for the `crt` font profile.

## Testing & Verification

```bash
cargo test --all-targets
cargo check --target wasm32-unknown-unknown
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## License

`mu-console-theme` is licensed under the GNU Lesser General Public License version 3.0 with the LGPL-3.0 Linking Exception (`LGPL-3.0-only WITH LGPL-3.0-linking-exception`). See [LICENSE](LICENSE), [COPYING](COPYING), and [LINKING-EXCEPTION](LINKING-EXCEPTION).
