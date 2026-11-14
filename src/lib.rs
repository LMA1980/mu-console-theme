// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! `HtmlCoreSkin` and `WireframeSkin` theme suite for MuConsole (dark only).
//!
//! - [`tokens`]: colour and chrome tokens for the flat high-contrast UI.
//! - [`glyphs`]: Nerd Font icon table; also drives the font subset list.
//! - [`metrics`]: measured font metrics used by the layout solver.
//! - [`svg`]: dependency-free, deterministic SVG writer.
//! - [`instruments`]: shared geometry and switch-action primitives.
//! - [`skin`]: low-level geometric layout instruments and [`skin::Skin`] trait.
//! - [`scene`]: skin-agnostic [`scene::ConsoleScene`] and [`scene::DialogModel`] descriptors.
//! - [`html`]: HTML5 void-aware [`html::HtmlWriter`] and semantic [`html::HtmlCoreSkin`].
//! - [`wireframe`]: flat high-contrast SVG [`wireframe::WireframeSkin`].
//! - [`css`]: generated `tokens.css`.
#![forbid(unsafe_code)]

pub mod css;
pub mod glyphs;
pub mod html;
pub mod instruments;
pub mod metrics;
pub mod scene;
pub mod skin;
pub mod svg;
pub mod tokens;
pub mod wireframe;
