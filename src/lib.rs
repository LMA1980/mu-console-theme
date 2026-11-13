// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! `WireframeSkin` theme for MuConsole (dark only).
//!
//! - [`tokens`]: colour and chrome tokens for the flat high-contrast UI.
//! - [`glyphs`]: Nerd Font icon table; also drives the font subset list.
//! - [`metrics`]: measured font metrics used by the layout solver.
//! - [`svg`]: dependency-free, deterministic SVG writer.
//! - [`instruments`]: shared geometry and switch-action primitives.
//! - [`interaction`]: switch state machine (maintained, spring-return,
//!   PULL-TO-TURN).
//! - [`css`]: generated `tokens.css`.
#![forbid(unsafe_code)]

pub mod css;
pub mod glyphs;
pub mod instruments;
pub mod interaction;
pub mod metrics;
pub mod skin;
pub mod svg;
pub mod tokens;
pub mod wireframe;
