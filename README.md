# mu-console-theme

Design tokens, Nerd icon table, font metrics, skin-agnostic `ConsoleScene`/`DialogModel` descriptors, `HtmlCoreSkin` HTML5+CSS renderer, and `WireframeSkin` SVG renderer for MuConsole.

## Architecture

- **Scene & Modal Descriptors (`mu_console_theme::scene`)**: Skin-agnostic `ConsoleScene` components (`StatusMatrix`, `TableGrid`, `LogViewer`, `TrainwayGraph`, `GateChecklist`, `SparklineCard`, `KeyValueList`, `PlatformPanel`) and `DialogModel` variants (`PasskeyEnrollmentGate`, `UserProfile`, `SessionRoles`, `RiskConfirm`, `InviteCreatedOnce`) rendered uniformly via `SceneRenderer`.
- **HtmlCoreSkin Renderer (`mu_console_theme::html`)**: Void-aware HTML5 `HtmlWriter` and semantic HTML+CSS `HtmlCoreSkin` supporting natural vertical scrolling (`html[data-skin="html"]`), `<dialog>` modals, disabled inert states, and mobile viewport rules (`font-size: 16px` inputs).
- **WireframeSkin Renderer (`mu_console_theme::wireframe`)**: Dependency-free pure SVG renderer (`WireframeSkin`) with crisp flat-fill geometry, bounded `data-bbox-*` hit targets, fixed `100dvh` framing (`html[data-skin="wireframe"]`), and ARIA/action-attribute parity with `HtmlCoreSkin`.
- **Color & Layout Tokens**: Dark-slate canvas, wireframe surfaces, semantic status tones, disabled control tokens, and `:root` CSS generation via `mu_console_theme::css::tokens_css()`.
- **Nerd Icon Table & Font Metrics**: Vector Nerd icon codepoints (`mu_console_theme::glyphs`) and measured typography metrics for `Hurmit` and `OpenDyslexicM` (`mu_console_theme::metrics`).
- **CLI Utilities**:
  - `mu-console-tokens-css`: Emits authoritative `tokens.css` (kept in sync with `assets/tokens.css` and `dist/tokens.css`).
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
