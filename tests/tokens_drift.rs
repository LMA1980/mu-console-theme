// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! Drift verification suite for `mu-console-theme` CSS token generation.
//! Ensures `tokens_css()` output matches both `assets/tokens.css` and `dist/tokens.css`.

use mu_console_theme::css::tokens_css;

#[test]
fn test_tokens_css_matches_committed_assets() {
    let generated = tokens_css();
    let assets = include_str!("../assets/tokens.css");
    assert_eq!(
        generated, assets,
        "assets/tokens.css has drifted from tokens_css()!\n\
         Regenerate with:\n\
         cargo run -q --bin mu-console-tokens-css > assets/tokens.css"
    );
}

#[test]
fn test_tokens_css_matches_checked_in_dist() {
    let generated = tokens_css();
    let dist = include_str!("../dist/tokens.css");
    assert_eq!(
        generated, dist,
        "dist/tokens.css has drifted from tokens_css()!\n\
         Regenerate and copy with:\n\
         cargo run -q --bin mu-console-tokens-css > assets/tokens.css && \
         cp assets/tokens.css dist/tokens.css"
    );
}

#[test]
fn test_tokens_css_contains_wireframe_skin_m67_rules() {
    let css = tokens_css();
    for selector in [
        "#muconsole-shell",
        "#muconsole-header",
        ".skip-link",
        ".skip-link:focus",
        "#muconsole-filter",
        "#muconsole-dashboard",
    ] {
        assert!(
            css.contains(selector),
            "tokens_css() must define selector '{selector}' for Milestone 6.7"
        );
    }
}
