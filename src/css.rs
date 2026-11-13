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
        ("border-wireframe", wireframe::BORDER),
        ("text-primary", wireframe::TEXT_PRIMARY),
        ("text-muted", wireframe::TEXT_MUTED),
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

/// Full `tokens.css` (custom properties plus the zero-scroll invariant).
#[must_use]
pub fn tokens_css() -> String {
    let mut s = String::from(":root{color-scheme:dark;--font-crt:\"MuCRT\",monospace;");
    for (name, value) in color_tokens() {
        let _ = write!(s, "--{name}:{value};");
    }
    s.push_str("}\n");
    s.push_str("html,body{margin:0;padding:0;height:100vh;overflow:hidden;height:100dvh;background:var(--mat-dark-slate)}\n");
    s.push_str("#muconsole-shell{display:flex;flex-direction:column;height:100vh;height:100dvh;overflow:hidden;margin:0;padding:0}\n");
    s.push_str("#muconsole-header{display:flex;flex-direction:row;align-items:center;justify-content:flex-start;height:48px;background:var(--mat-dark-slate);border-bottom:1px solid var(--accent-neon-cyan);gap:12px;padding:0 16px;box-sizing:border-box;flex-shrink:0}\n");
    s.push_str(".skip-link{position:absolute;top:-9999px;left:-9999px}\n");
    s.push_str(".skip-link:focus{top:8px;left:8px;z-index:1000;padding:8px 12px;background:var(--accent-neon-cyan);color:#000;font-family:var(--font-crt);text-decoration:none;border-radius:2px;font-weight:700}\n");
    s.push_str("#muconsole-filter{font-family:var(--font-crt);font-size:16px;color:var(--accent-neon-cyan);background:rgba(0,0,0,0.45);border:1px solid var(--accent-neon-cyan);border-radius:3px;padding:4px 10px;outline:none;box-sizing:border-box}\n");
    s.push_str("#muconsole-filter::placeholder{color:var(--accent-neon-cyan);opacity:0.5}\n");
    s.push_str("#muconsole-filter:focus{border-color:var(--accent-neon-cyan);box-shadow:0 0 4px var(--accent-neon-cyan)}\n");
    s.push_str("#muconsole-aria-polite,#muconsole-aria-assertive{font-family:var(--font-crt);font-size:12px;color:var(--color-green-glow);padding:2px 8px;border-radius:2px;background:rgba(0,0,0,0.35);white-space:nowrap;overflow:hidden;text-overflow:ellipsis;box-sizing:border-box}\n");
    s.push_str(
        "#muconsole-dashboard{flex:1 1 auto;overflow:hidden;position:relative;display:flex}\n",
    );
    s.push_str("#muconsole-dashboard>svg{width:100%;height:100%;display:block}\n");
    s.push_str("#muconsole-auth-link{margin-left:auto;font-family:var(--font-crt);font-size:13px;color:var(--accent-neon-cyan);text-decoration:none;border:1px solid var(--accent-neon-cyan);padding:4px 10px;border-radius:3px;background:rgba(0,0,0,0.45);white-space:nowrap;box-sizing:border-box}\n");
    s.push_str("#muconsole-auth-link:hover,#muconsole-auth-link:focus{background:var(--accent-neon-cyan);color:#000}\n");
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
        assert!(css.contains("background:var(--mat-dark-slate)"));
        assert!(css.contains("overflow:hidden;height:100dvh"));
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
