// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! Generated `tokens.css`: colour custom properties and `@font-face` rules.

use crate::metrics::{CrtFontMetrics, CRT_FAMILY_ALIAS};
use crate::tokens::{chrome, wireframe};
use std::fmt::Write as _;

/// Every exported colour token as `(css-name, value)`.
#[must_use]
pub fn color_tokens() -> Vec<(&'static str, &'static str)> {
    vec![
        ("mat-dark-slate", chrome::DARK_SLATE),
        ("accent-neon-cyan", chrome::NEON_CYAN),
        ("color-green-glow", chrome::GREEN_GLOW),
        ("bg-canvas", wireframe::BG_CANVAS),
        ("bg-panel", wireframe::BG_PANEL),
        ("bg-screen", wireframe::BG_SCREEN),
        ("bg-control", wireframe::BG_CONTROL),
        ("bg-disabled", wireframe::BG_DISABLED),
        ("border-wireframe", wireframe::BORDER),
        ("border-disabled", wireframe::BORDER_DISABLED),
        ("text-primary", wireframe::TEXT_PRIMARY),
        ("text-muted", wireframe::TEXT_MUTED),
        ("text-disabled", wireframe::TEXT_DISABLED),
        ("status-pass", wireframe::PASS),
        ("status-warn", wireframe::WARN),
        ("status-fail", wireframe::FAIL),
        ("status-info", wireframe::INFO),
    ]
}

/// `@font-face` binding the CRT alias to `font` (`font-display: block`, so
/// terminals never flash a fallback). `query` is the `?v=<hash>` cache-busting
/// suffix (may be empty).
#[must_use]
pub fn font_faces(font: &CrtFontMetrics, base_url: &str, query: &str) -> String {
    let mut s = String::new();
    let bold = font.bold_file.unwrap_or(font.regular_file);
    let _ = writeln!(
        s,
        "@font-face{{font-family:{CRT_FAMILY_ALIAS};font-weight:400;font-display:block;src:url(\"{base_url}/{}.woff2{query}\") format(\"woff2\")}}",
        font.regular_file
    );
    let _ = writeln!(
        s,
        "@font-face{{font-family:{CRT_FAMILY_ALIAS};font-weight:700;font-display:block;src:url(\"{base_url}/{bold}.woff2{query}\") format(\"woff2\")}}"
    );
    let _ = writeln!(
        s,
        "@font-face{{font-family:\"{}\";font-weight:400;font-display:block;src:url(\"{base_url}/{}.woff2{query}\") format(\"woff2\")}}",
        font.family, font.regular_file
    );
    let _ = writeln!(
        s,
        "@font-face{{font-family:\"{}\";font-weight:700;font-display:block;src:url(\"{base_url}/{bold}.woff2{query}\") format(\"woff2\")}}",
        font.family
    );
    s
}

/// Full `tokens.css` (custom properties, `data-skin` layout framing, and responsive rules).
#[must_use]
pub fn tokens_css() -> String {
    let mut s = String::from(":root{color-scheme:dark;--font-crt:\"MuCRT\",monospace;");
    for (name, value) in color_tokens() {
        let _ = write!(s, "--{name}:{value};");
    }
    s.push_str("}\n");
    s.push_str("html[data-crt-font=\"opendyslexic\"],#muconsole-shell[data-crt-font=\"opendyslexic\"]{--font-crt:\"OpenDyslexicM Nerd Font Mono\",\"MuCRT\",monospace}\n");
    s.push_str("html[data-crt-font=\"opendyslexic\"] svg text,#muconsole-shell[data-crt-font=\"opendyslexic\"] svg text{font-family:var(--font-crt)}\n");
    s.push_str("html[data-skin=\"wireframe\"],html[data-skin=\"wireframe\"] body{margin:0;padding:0;height:100vh;overflow:hidden;height:100dvh;background:var(--mat-dark-slate);color:var(--text-primary)}\n");
    s.push_str("html[data-skin=\"html\"],html[data-skin=\"html\"] body,html:not([data-skin]),html:not([data-skin]) body{margin:0;padding:0;height:auto;min-height:100dvh;overflow-x:hidden;overflow-y:auto;background:var(--mat-dark-slate);color:var(--text-primary);font-family:var(--font-crt)}\n");
    s.push_str("#muconsole-shell{display:flex;flex-direction:column;margin:0;padding:0}\n");
    s.push_str("#muconsole-shell[data-skin=\"wireframe\"],html[data-skin=\"wireframe\"] #muconsole-shell{height:100vh;height:100dvh;overflow:hidden}\n");
    s.push_str("#muconsole-shell[data-skin=\"html\"],html[data-skin=\"html\"] #muconsole-shell{height:auto;min-height:100dvh;overflow:visible}\n");
    s.push_str("#muconsole-header{display:flex;flex-direction:row;align-items:center;justify-content:flex-start;height:48px;background:var(--mat-dark-slate);border-bottom:1px solid var(--accent-neon-cyan);gap:12px;padding:0 16px;box-sizing:border-box;flex-shrink:0}\n");
    s.push_str(".skip-link{position:absolute;top:-9999px;left:-9999px}\n");
    s.push_str(".skip-link:focus{top:8px;left:8px;z-index:1000;padding:8px 12px;background:var(--accent-neon-cyan);color:#000;font-family:var(--font-crt);text-decoration:none;border-radius:2px;font-weight:700}\n");
    s.push_str("#muconsole-filter{font-family:var(--font-crt);font-size:16px;color:var(--accent-neon-cyan);background:rgba(0,0,0,0.45);border:1px solid var(--accent-neon-cyan);border-radius:3px;padding:4px 10px;outline:none;box-sizing:border-box}\n");
    s.push_str("#muconsole-filter::placeholder{color:var(--accent-neon-cyan);opacity:0.5}\n");
    s.push_str("#muconsole-filter:focus{border-color:var(--accent-neon-cyan);box-shadow:0 0 4px var(--accent-neon-cyan)}\n");
    s.push_str("#muconsole-aria-polite,#muconsole-aria-assertive{font-family:var(--font-crt);font-size:12px;color:var(--color-green-glow);padding:2px 8px;border-radius:2px;background:rgba(0,0,0,0.35);white-space:nowrap;overflow:hidden;text-overflow:ellipsis;box-sizing:border-box}\n");
    s.push_str("#muconsole-session-error,#muconsole-session-loading,#muconsole-auth-denied{font-family:var(--font-crt);font-size:12px;white-space:nowrap}\n");
    s.push_str("#muconsole-session-error,#muconsole-auth-denied{color:var(--status-fail)}\n");
    s.push_str(".mu-status-lamps{display:flex;flex-wrap:wrap;align-items:center;gap:8px;margin-left:auto}\n");
    s.push_str(".mu-lamp{display:inline-flex;align-items:center;gap:6px;padding:3px 8px;border-radius:999px;border:1px solid var(--border-wireframe);background:var(--bg-screen);font-size:12px}\n");
    s.push_str(".mu-lamp-val{font-weight:700}\n");
    s.push_str(
        "#muconsole-dashboard{flex:1 1 auto;overflow:hidden;position:relative;display:flex}\n",
    );
    s.push_str("html[data-skin=\"html\"] #muconsole-dashboard,#muconsole-shell[data-skin=\"html\"] #muconsole-dashboard{overflow:visible;display:block}\n");
    s.push_str("#muconsole-dashboard>svg{width:100%;height:100%;display:block}\n");
    s.push_str("#muconsole-auth-link{margin-left:auto;font-family:var(--font-crt);font-size:13px;color:var(--accent-neon-cyan);text-decoration:none;border:1px solid var(--accent-neon-cyan);padding:4px 10px;border-radius:3px;background:rgba(0,0,0,0.45);white-space:nowrap;box-sizing:border-box}\n");
    s.push_str("#muconsole-auth-link:hover,#muconsole-auth-link:focus{background:var(--accent-neon-cyan);color:#000}\n");
    s.push_str("input,select,textarea,.mu-input{font-family:var(--font-crt);font-size:16px;color:var(--text-primary);background:var(--bg-screen);border:1px solid var(--border-wireframe);border-radius:4px;padding:8px 10px;box-sizing:border-box;max-width:100%}\n");
    s.push_str(".mu-console-scene{display:flex;flex-direction:column;gap:16px;padding:12px 16px 24px;min-height:100dvh;background:var(--bg-canvas);color:var(--text-primary);font-family:var(--font-crt);box-sizing:border-box}\n");
    s.push_str(".mu-scene-header{display:flex;flex-wrap:wrap;align-items:center;gap:12px;padding:10px 12px;background:var(--bg-panel);border:1px solid var(--border-wireframe);border-radius:6px}\n");
    s.push_str(".mu-scene-brand{display:flex;align-items:center;gap:8px;font-weight:700}\n");
    s.push_str(".mu-area-pill,.mu-badge,.mu-env-pill,.mu-role-chip{display:inline-flex;align-items:center;padding:2px 8px;border-radius:999px;border:1px solid var(--border-wireframe);background:var(--bg-screen);font-size:12px}\n");
    s.push_str(".mu-area-nav,.mu-subview-nav,.mu-repo-selector{min-width:0;max-width:100%}\n");
    s.push_str(".mu-repo-selector{display:flex;align-items:center;gap:8px}\n");
    s.push_str(".mu-segmented{display:flex;flex-wrap:nowrap;gap:6px;overflow-x:auto;max-width:100%;-webkit-overflow-scrolling:touch;scrollbar-width:thin;padding-bottom:2px}\n");
    s.push_str(".mu-btn,.mu-tab{font-family:var(--font-crt);font-size:13px;line-height:1.2;color:var(--text-primary);background:var(--bg-control);border:1px solid var(--border-wireframe);border-radius:4px;padding:6px 12px;cursor:pointer;white-space:nowrap;flex:0 0 auto;display:inline-flex;align-items:center;gap:6px;box-sizing:border-box}\n");
    s.push_str(".mu-tab[aria-selected=\"true\"],.mu-btn[aria-pressed=\"true\"],.mu-btn-primary{border-color:var(--accent-neon-cyan);background:#1f3452}\n");
    s.push_str(".mu-btn[aria-pressed=\"true\"]{box-shadow:inset 0 0 0 1px var(--accent-neon-cyan);color:var(--accent-neon-cyan);font-weight:700}\n");
    s.push_str(".mu-btn-danger{border-color:var(--status-fail);background:#34181c}\n");
    s.push_str("[disabled],[aria-disabled=\"true\"],.mu-btn[disabled],.mu-tab[disabled]{background:var(--bg-disabled);border-color:var(--border-disabled);color:var(--text-disabled);cursor:not-allowed;opacity:0.75}\n");
    s.push_str(":focus-visible{outline:2px solid var(--accent-neon-cyan);outline-offset:2px}\n");
    s.push_str(".mu-scene-main{display:flex;flex-direction:column;gap:14px;flex:1 1 auto}\n");
    s.push_str(".mu-scene-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,460px),1fr));gap:16px;align-items:start}\n");
    s.push_str(".mu-section,.mu-card{background:var(--bg-panel);border:1px solid var(--border-wireframe);border-radius:6px;padding:14px;display:flex;flex-direction:column;gap:12px;box-sizing:border-box}\n");
    s.push_str(".mu-card{background:var(--bg-screen)}\n");
    s.push_str(".mu-card-bar,.mu-section-header{display:flex;flex-wrap:wrap;align-items:center;justify-content:space-between;gap:8px}\n");
    s.push_str(".mu-card-title,.mu-dialog-title,.mu-section-header h2{margin:0;font-size:14px}\n");
    s.push_str(
        ".mu-trainway-svg,.mu-sparkline-svg{width:100%;height:auto;display:block;max-width:100%}\n",
    );
    s.push_str(".mu-matrix-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(160px,1fr));gap:8px}\n");
    s.push_str(".mu-matrix-cell{display:flex;flex-direction:column;gap:4px;padding:8px 10px;background:var(--bg-panel);border:1px solid var(--border-wireframe);border-radius:4px;text-align:left;color:var(--text-primary)}\n");
    s.push_str(".mu-table-wrap{overflow-x:auto;max-width:100%}\n");
    s.push_str(".mu-table{width:100%;border-collapse:collapse;font-size:13px}\n");
    s.push_str(".mu-table th,.mu-table td{padding:8px 10px;border-bottom:1px solid var(--border-wireframe);text-align:left;vertical-align:middle}\n");
    s.push_str(".mu-log-lines{margin:0;padding:8px 10px;list-style:none;background:var(--bg-screen);border:1px solid var(--border-wireframe);border-radius:4px;font-size:12.5px;overflow-x:auto}\n");
    s.push_str(".mu-log-line{display:flex;flex-wrap:wrap;gap:8px;padding:3px 0}\n");
    s.push_str(".mu-log-viewer[data-wrap=\"true\"] .mu-log-text{white-space:pre-wrap;word-break:break-word}\n");
    s.push_str(".mu-log-viewer[data-wrap=\"false\"] .mu-log-text{white-space:pre}\n");
    s.push_str(".mu-gate-list,.mu-trainway-tracks,.mu-session-roles,.mu-role-badges{margin:0;padding:0;list-style:none;display:flex;flex-direction:column;gap:8px}\n");
    s.push_str(".mu-role-badges{flex-direction:row;flex-wrap:wrap}\n");
    s.push_str(".mu-gate-item,.mu-role-row,.mu-pagination,.mu-banner{display:flex;flex-wrap:wrap;align-items:center;justify-content:space-between;gap:8px;padding:8px 10px;background:var(--bg-panel);border:1px solid var(--border-wireframe);border-radius:4px}\n");
    s.push_str(".mu-kv{margin:0;display:grid;gap:6px}\n");
    s.push_str(".mu-kv-row{display:flex;flex-wrap:wrap;justify-content:space-between;gap:8px;padding:4px 0;border-bottom:1px solid var(--border-wireframe)}\n");
    s.push_str(".mu-mono{font-family:var(--font-crt)}\n");
    s.push_str("[data-tone=\"pass\"]{border-color:var(--status-pass)}[data-tone=\"warn\"]{border-color:var(--status-warn)}[data-tone=\"fail\"]{border-color:var(--status-fail)}[data-tone=\"info\"]{border-color:var(--status-info)}\n");
    s.push_str("dialog.mu-dialog{background:var(--bg-panel);color:var(--text-primary);border:1px solid var(--border-wireframe);border-radius:8px;width:min(560px,calc(100vw - 24px));max-height:calc(100dvh - 32px);overflow-y:auto;padding:0;margin:auto;font-family:var(--font-crt);box-sizing:border-box}\n");
    s.push_str("dialog.mu-dialog[open]{position:fixed;inset:0;z-index:200;height:fit-content;box-shadow:0 24px 64px rgba(0,0,0,0.85),0 0 0 100vmax rgba(5,7,10,0.82)}\n");
    s.push_str("dialog.mu-dialog::backdrop{background:rgba(5,7,10,0.82)}\n");
    s.push_str(".mu-dialog-header{display:flex;align-items:center;justify-content:space-between;gap:12px;padding:14px 16px;border-bottom:1px solid var(--border-wireframe)}\n");
    s.push_str(".mu-dialog-body{display:flex;flex-direction:column;gap:12px;padding:16px}\n");
    s.push_str(".mu-dialog-footer{display:flex;flex-wrap:wrap;justify-content:flex-end;gap:8px;padding-top:8px}\n");
    s.push_str(".mu-profile-extra{display:flex;flex-direction:column;gap:12px;padding:12px 16px 16px;border-top:1px solid var(--border-wireframe)}\n");
    s.push_str(
        ".mu-pref-group,.mu-field,.mu-passkey-panel{display:flex;flex-direction:column;gap:8px}\n",
    );
    s.push_str(".mu-pref-header{display:flex;flex-wrap:wrap;align-items:center;justify-content:space-between;gap:8px;font-size:13px}\n");
    s.push_str(".mu-pref-current{font-size:12px;color:var(--accent-neon-cyan);font-weight:700}\n");
    s.push_str(".mu-btn-row{display:flex;flex-wrap:wrap;gap:8px}\n");
    s.push_str(".mu-invite-secret{padding:10px;background:var(--bg-screen);border:1px solid var(--status-pass);border-radius:4px;overflow-x:auto;word-break:break-all}\n");
    s.push_str("@media (max-width:520px){.mu-console-scene{padding:8px 10px 20px}.mu-scene-grid{grid-template-columns:1fr}.mu-btn,.mu-tab{min-width:44px;min-height:44px}input,select,textarea,.mu-input{font-size:16px}}\n");
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::{HURMIT, OPEN_DYSLEXIC};

    #[test]
    fn css_contains_tokens_and_invariant() {
        let css = tokens_css();
        assert!(css.contains("--mat-dark-slate:#0e1116;"));
        assert!(css.contains("--accent-neon-cyan:#00f0ff;"));
        assert!(css.contains("--bg-disabled:#0c1016;"));
        assert!(css.contains("background:var(--mat-dark-slate)"));
        assert!(css.contains("html[data-skin=\"wireframe\"]"));
        assert!(css.contains("overflow:hidden;height:100dvh"));
        assert!(css.contains("html[data-skin=\"html\"]"));
        assert!(css.contains("overflow-y:auto"));
        assert!(css.contains("dialog.mu-dialog[open]{position:fixed;inset:0;z-index:200"));
        assert!(css.contains("dialog.mu-dialog::backdrop"));
        assert!(css.contains("font-size:16px"));
        assert!(css.contains("[aria-disabled=\"true\"]"));
        assert!(css.contains(".mu-status-lamps{display:flex;flex-wrap:wrap;align-items:center;gap:8px;margin-left:auto}"));
        assert!(css
            .contains(".mu-lamp{display:inline-flex;align-items:center;gap:6px;padding:3px 8px;"));
        assert!(css
            .contains("#muconsole-session-error,#muconsole-auth-denied{color:var(--status-fail)}"));
        assert!(css.contains(".mu-btn[aria-pressed=\"true\"]"));
        assert!(css.contains(".mu-profile-extra{"));
        assert!(css.contains(".mu-pref-group,"));
        assert!(css.contains(".mu-btn-row{"));
        assert!(css.contains("html[data-crt-font=\"opendyslexic\"],#muconsole-shell[data-crt-font=\"opendyslexic\"]{--font-crt:\"OpenDyslexicM Nerd Font Mono\",\"MuCRT\",monospace}"));
        assert!(css.contains("html[data-crt-font=\"opendyslexic\"] svg text,#muconsole-shell[data-crt-font=\"opendyslexic\"] svg text{font-family:var(--font-crt)}"));
        assert_eq!(css, tokens_css(), "deterministic");
    }

    #[test]
    fn committed_tokens_css_has_not_drifted() {
        assert_eq!(
            include_str!("../assets/tokens.css"),
            tokens_css(),
            "regenerate: cargo run -q --bin mu-console-tokens-css > assets/tokens.css"
        );
    }

    #[test]
    fn font_faces_bind_aliases() {
        let h = font_faces(&HURMIT, "/fonts", "?v=abc");
        assert!(h.contains("MuCRT;font-weight:400;font-display:block"));
        assert!(h.contains("\"Hurmit Nerd Font Mono\";font-weight:400;font-display:block"));
        assert!(h.contains("\"Hurmit Nerd Font Mono\";font-weight:700;font-display:block"));
        assert!(h.contains("/fonts/HurmitNerdFontMono-Bold.woff2?v=abc"));
        assert_eq!(h.matches("@font-face").count(), 4);
        let d = font_faces(&OPEN_DYSLEXIC, "/fonts", "");
        assert!(d.contains("\"OpenDyslexicM Nerd Font Mono\";font-weight:400;font-display:block"));
        // No bold weight: both MuCRT and concrete family faces use the regular file (4 total).
        assert_eq!(
            d.matches("OpenDyslexicMNerdFontMono-Regular.woff2").count(),
            4
        );
    }
}
