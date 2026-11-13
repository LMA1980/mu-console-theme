// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! Colour and chrome tokens for `WireframeSkin`.
//!
//! Colours are `#rrggbb` strings so they can be written straight into SVG
//! attributes and CSS custom properties.

/// Chrome and shell layout colours (header, status indicators, border accents).
pub mod chrome {
    pub const DARK_SLATE: &str = "#0e1116";
    pub const NEON_CYAN: &str = "#00f0ff";
    pub const GREEN_GLOW: &str = "#4dff7a";
}

/// Wireframe surface, border, typography, and semantic status colours.
pub mod wireframe {
    pub const BG_CANVAS: &str = "#0e1116";
    pub const BG_PANEL: &str = "#141922";
    pub const BG_SCREEN: &str = "#090c10";
    pub const BG_CONTROL: &str = "#1b2230";
    pub const BORDER: &str = "#2b3440";
    pub const TEXT_PRIMARY: &str = "#e6edf3";
    pub const TEXT_MUTED: &str = "#8b949e";
    pub const PASS: &str = "#33f08a";
    pub const WARN: &str = "#ffb43a";
    pub const FAIL: &str = "#ff4d4d";
    pub const INFO: &str = "#5fb3ff";
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_hex(c: &str) -> bool {
        c.len() == 7 && c.starts_with('#') && c[1..].chars().all(|ch| ch.is_ascii_hexdigit())
    }

    #[test]
    fn colours_are_hex() {
        for c in [
            chrome::DARK_SLATE,
            chrome::NEON_CYAN,
            chrome::GREEN_GLOW,
            wireframe::BG_CANVAS,
            wireframe::BG_PANEL,
            wireframe::BG_SCREEN,
            wireframe::BG_CONTROL,
            wireframe::BORDER,
            wireframe::TEXT_PRIMARY,
            wireframe::TEXT_MUTED,
            wireframe::PASS,
            wireframe::WARN,
            wireframe::FAIL,
            wireframe::INFO,
        ] {
            assert!(is_hex(c), "{c}");
        }
    }
}
