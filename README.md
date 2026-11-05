# mu-console-theme

Avionics Terminal design tokens, 5x7 LCD glyphs, Nerd icon table, font metrics, and a dependency-free SVG instrument library for MuConsole.

## Architecture

- **Material & Color Tokens**: Avionics terminal design tokens, cyan glow, dark slate, CRT phosphor, intensity settings, and `:root` CSS generation.
- **5x7 LCD & Nerd Font Table**: Dot-matrix bitmap glyphs for STN LCD windows and vector Nerd icon codepoints for status annunciators.
- **Font Metrics**: Measured typography bounding boxes and advance widths for terminal layout calculations.
- **Parametric SVG Instruments**: Dependency-free pure SVG component generators for rotary knobs, pistol switches, annunciator lamps, CRT bezels, and textured panels.
- **CLI Utilities**:
  - `mu-console-tokens-css`: Emits authoritative `tokens.css`.
  - `mu-console-subset-unicodes`: Generates `pyftsubset` unicode ranges for CRT and Etched fonts.

## Testing & Verification

```bash
cargo test --all-targets
cargo check --target wasm32-unknown-unknown
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## License

`mu-console-theme` is licensed under the GNU Lesser General Public License version 3.0 with the LGPL-3.0 Linking Exception (`LGPL-3.0-only WITH LGPL-3.0-linking-exception`). See [LICENSE](LICENSE), [COPYING](COPYING), and [LINKING-EXCEPTION](LINKING-EXCEPTION).
