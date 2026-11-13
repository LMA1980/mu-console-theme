// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! Shared geometry and control-action primitives for [`crate::wireframe::WireframeSkin`].

pub mod switch;

/// Axis-aligned rectangle in CSS px.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    #[must_use]
    pub const fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }

    #[must_use]
    pub fn right(&self) -> f64 {
        self.x + self.w
    }

    #[must_use]
    pub fn bottom(&self) -> f64 {
        self.y + self.h
    }

    /// Shrink by the given insets.
    #[must_use]
    pub fn inset(&self, left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self::new(
            self.x + left,
            self.y + top,
            (self.w - left - right).max(0.0),
            (self.h - top - bottom).max(0.0),
        )
    }
}
