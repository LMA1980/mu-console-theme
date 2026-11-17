# mu-console-theme

Design tokens, Nerd icon table, font metrics, skin-agnostic `ConsoleScene`/`DialogModel` descriptors, `HtmlCoreSkin` HTML5+CSS renderer, and `WireframeSkin` SVG renderer for [`mu-console`](https://github.com/LMA1980/mu-console) (Protocol v8).

## Architecture

- **Scene & Modal Descriptors (`mu_console_theme::scene`)**: Skin-agnostic `ConsoleScene` components (`StatusMatrix`, `TableGrid`, `LogViewer`, `TrainwayGraph`, `GateChecklist`, `SparklineCard`, `KeyValueList`, `PlatformPanel`) and `DialogModel` variants (`PasskeyEnrollmentGate`, `UserProfile`, `SessionRoles`, `RiskConfirm`, `InviteCreatedOnce`) rendered uniformly via `SceneRenderer` with [`mu-console-protocol`](https://github.com/LMA1980/mu-console-protocol) models and [`mu-console-layout`](https://github.com/LMA1980/mu-console-layout) primitives.
- **HtmlCoreSkin Renderer (`mu_console_theme::html`)**: Void-aware HTML5 `HtmlWriter` and semantic HTML+CSS `HtmlCoreSkin` supporting natural vertical scrolling (`html[data-skin="html"]`), `<dialog>` modals (`SessionRolesDialog` with inert disabled role rows via `.mu-role-row[data-disabled="true"]`), disabled `StatusMatrix` tone preservation (retaining `data-tone` border and metric status colors when `disabled` / `aria-disabled="true"`), and mobile viewport rules (`font-size: 16px` inputs).
- **WireframeSkin Renderer (`mu_console_theme::wireframe`)**: Dependency-free pure SVG renderer (`WireframeSkin`) with crisp flat-fill geometry, bounded `data-bbox-*` hit targets, fixed `100dvh` framing (`html[data-skin="wireframe"]`), and ARIA/action-attribute/tone parity with `HtmlCoreSkin`.
- **Color & Layout Tokens**: Dark-slate canvas, wireframe surfaces, semantic status tones, disabled control tokens, and `:root` CSS generation via `mu_console_theme::css::tokens_css()`.
- **Nerd Icon Table & Font Metrics**: Vector Nerd icon codepoints (`mu_console_theme::glyphs`) and measured typography metrics for `Hurmit` and `OpenDyslexicM` (`mu_console_theme::metrics`).
- **CLI Utilities**:
  - `mu-console-tokens-css`: Emits authoritative `tokens.css` (kept in sync with `assets/tokens.css` and `dist/tokens.css`).
  - `mu-console-subset-unicodes`: Generates `pyftsubset` unicode ranges for the `crt` font profile.

## Ecosystem Integration

- **Parent Workspace**: [`mu-console`](https://github.com/LMA1980/mu-console)
- **Companion Crates**: [`mu-console-protocol`](https://github.com/LMA1980/mu-console-protocol), [`mu-console-layout`](https://github.com/LMA1980/mu-console-layout), [`mu-console-ui`](https://github.com/LMA1980/mu-console-ui)

## Testing & Verification

```bash
cargo fmt --all -- --check
./scripts/format_docs.sh --check
cargo clippy --all-targets -- -D warnings
cargo clippy --target wasm32-unknown-unknown --lib -- -D warnings
cargo test --all-targets
cargo build --target wasm32-unknown-unknown
MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test --lib
```

## License

`mu-console-theme` is licensed under the GNU Lesser General Public License version 3.0 with the LGPL-3.0 Linking Exception (`LGPL-3.0-only WITH LGPL-3.0-linking-exception`). See [LICENSE](LICENSE), [COPYING](COPYING), and [LINKING-EXCEPTION](LINKING-EXCEPTION).
