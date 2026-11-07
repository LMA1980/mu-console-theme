// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! [`WireframeSkin`]: high-contrast, flat-fill UX baseline skin for Phase 3.
//!
//! Renders every semantic instrument (`Panel`, `Selector`, `Rotary`, `Lamp`,
//! `Readout`, `Screen`) inside the exact [`crate::instruments::Rect`] bounds
//! produced by `muconsole-layout` using:
//! - flat `#0e1116` canvas / `#141922` panel fill;
//! - crisp `1 px` `#2b3440` borders;
//! - solid semantic status colours (`#33f08a` pass, `#ffb43a` warn/active,
//!   `#ff4d4d` fail, `#5fb3ff` info);
//! - geometric signal shapes (`■`, `▢`, `✕`, `▫`, `●`, `◯`, `○`);
//! - zero decorative filters (`<feGaussianBlur>`, wood veneer, tungsten
//!   noise, or 5×7 dot-matrix stipple).

use crate::defs::{crt_glass_id, ids, url, write_defs};
use crate::instruments::surface::{chassis_plate, riveted_sub_plate, wood};
use crate::instruments::switch::{indicator_lamp, screw, LensColor, SwitchAction};
use crate::instruments::Rect;
use crate::skin::{
    Lamp, Panel, Readout, Rotary, Screen, ScreenTrack, Selector, SemanticTone, SignalGlyph, Skin,
};
use crate::svg::{PathData, SvgWriter};
use crate::tokens::{Effects, CRT_CD, CRT_CI};

pub const BG_CANVAS: &str = "#0e1116";
pub const BG_PANEL: &str = "#141922";
pub const BG_SCREEN: &str = "#090c10";
pub const BG_CONTROL: &str = "#1b2230";
pub const BORDER: &str = "#2b3440";
pub const TEXT_PRIMARY: &str = "#e6edf3";
pub const TEXT_MUTED: &str = "#8b949e";
pub const COLOR_PASS: &str = "#33f08a";
pub const COLOR_WARN: &str = "#ffb43a";
pub const COLOR_FAIL: &str = "#ff4d4d";
pub const COLOR_INFO: &str = "#5fb3ff";

/// Flat high-contrast wireframe skin (Phase 3 UX baseline).
#[derive(Debug, Clone, Copy, Default)]
pub struct WireframeSkin;

/// Tactile avionics skin sharing identical semantic hit-targets, ARIA tree,
/// and bounding boxes with [`WireframeSkin`].
#[derive(Debug, Clone, Copy)]
pub struct AvionicsSkin {
    pub effects: Effects,
}

impl Default for AvionicsSkin {
    fn default() -> Self {
        Self {
            effects: Effects::LITE,
        }
    }
}

impl SemanticTone {
    /// Foreground / indicator hex colour in `WireframeSkin`.
    #[must_use]
    pub const fn wireframe_fg(self) -> &'static str {
        match self {
            Self::Neutral => TEXT_PRIMARY,
            Self::Pass => COLOR_PASS,
            Self::Warn => COLOR_WARN,
            Self::Fail => COLOR_FAIL,
            Self::Info => COLOR_INFO,
        }
    }

    /// Stroke / accent hex colour in `WireframeSkin`.
    #[must_use]
    pub const fn wireframe_stroke(self) -> &'static str {
        match self {
            Self::Neutral => BORDER,
            Self::Pass => COLOR_PASS,
            Self::Warn => COLOR_WARN,
            Self::Fail => COLOR_FAIL,
            Self::Info => COLOR_INFO,
        }
    }
}

fn truncate_chars(s: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let count = s.chars().count();
    if count <= max_chars {
        return s.to_owned();
    }
    if max_chars == 1 {
        return "…".to_owned();
    }
    let mut out: String = s.chars().take(max_chars - 1).collect();
    out.push('…');
    out
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn fit_box(s: &str, width_px: f64, font_px: f64) -> String {
    let char_w = (font_px * 0.62).max(1.0);
    let cols = ((width_px - 8.0).max(0.0) / char_w).floor() as usize;
    truncate_chars(s, cols.max(1))
}

impl Skin for WireframeSkin {
    fn name(&self) -> &'static str {
        "wireframe"
    }

    fn begin_frame(&self, w: &mut SvgWriter, width: f64, height: f64) {
        w.tag("rect")
            .attr("x", 0.0)
            .attr("y", 0.0)
            .attr("width", width)
            .attr("height", height)
            .attr("fill", BG_CANVAS)
            .attr("data-skin", "wireframe")
            .end();
    }

    fn panel(&self, w: &mut SvgWriter, p: &Panel<'_>) {
        let r = p.rect;
        w.tag("g")
            .attr("data-panel-id", p.id)
            .attr_opt("role", p.role)
            .attr_opt("aria-label", p.aria_label)
            .open();
        w.tag("rect")
            .attr("x", r.x)
            .attr("y", r.y)
            .attr("width", r.w)
            .attr("height", r.h)
            .attr("rx", 4.0)
            .attr("fill", BG_PANEL)
            .attr("stroke", BORDER)
            .attr("stroke-width", 1.0)
            .end();
        if !p.title.is_empty() && r.h >= 24.0 {
            let label = fit_box(p.title, r.w - 16.0, 11.0);
            w.tag("text")
                .attr("x", r.x + 10.0)
                .attr("y", r.y + 15.0)
                .attr("fill", TEXT_MUTED)
                .attr("font-family", "MuCRT, monospace")
                .attr("font-size", 11.0)
                .attr("font-weight", "bold")
                .text(&label);
        }
        w.close();
    }

    #[allow(clippy::cast_precision_loss, clippy::too_many_lines)]
    fn selector(&self, w: &mut SvgWriter, s: &Selector<'_>) {
        let r = s.rect;
        if s.control_id == "c2-font-switch" {
            let dyslexic_on = s.selected == 1;
            let border = if dyslexic_on { COLOR_PASS } else { "#3d5270" };
            let bg = if dyslexic_on { "#122b22" } else { "#182232" };
            w.tag("g")
                .attr("data-control-id", s.control_id)
                .attr("data-dyslexia-toggle", "true")
                .attr("data-bbox-x", r.x)
                .attr("data-bbox-y", r.y)
                .attr("data-bbox-w", r.w)
                .attr("data-bbox-h", r.h)
                .attr("role", "switch")
                .attr("aria-checked", if dyslexic_on { "true" } else { "false" })
                .attr("aria-label", "Use dyslexia-friendly font")
                .attr("tabindex", 0_u32)
                .open();
            w.tag("rect")
                .attr("x", r.x)
                .attr("y", r.y)
                .attr("width", r.w)
                .attr("height", r.h)
                .attr("rx", (r.h * 0.45).min(12.0))
                .attr("fill", bg)
                .attr("stroke", border)
                .attr("stroke-width", 1.5)
                .end();

            // Checkbox / switch indicator box on the left.
            let box_sz = (r.h - 14.0).clamp(14.0, 18.0);
            let box_x = r.x + 8.0;
            let box_y = r.y + (r.h - box_sz) * 0.5;
            w.tag("rect")
                .attr("x", box_x)
                .attr("y", box_y)
                .attr("width", box_sz)
                .attr("height", box_sz)
                .attr("rx", 4.0)
                .attr("fill", if dyslexic_on { COLOR_PASS } else { BG_SCREEN })
                .attr("stroke", if dyslexic_on { COLOR_PASS } else { "#6e8299" })
                .attr("stroke-width", 1.5)
                .end();
            if dyslexic_on {
                let ck = PathData::new()
                    .m(box_x + 3.5, box_y + box_sz * 0.52)
                    .l(box_x + box_sz * 0.42, box_y + box_sz - 3.5)
                    .l(box_x + box_sz - 3.0, box_y + 3.5)
                    .build();
                w.tag("path")
                    .attr("d", ck)
                    .attr("fill", "none")
                    .attr("stroke", "#06140d")
                    .attr("stroke-width", 2.2)
                    .end();
            }

            let label_x = box_x + box_sz + 6.0;
            let avail_w = (r.right() - label_x - 5.0).max(20.0);
            let (raw_lbl, f_sz) = if avail_w >= 148.0 {
                ("Dyslexia-friendly font", 11.5)
            } else {
                ("Dyslexia font", 11.0)
            };
            let lbl = fit_box(raw_lbl, avail_w, f_sz);
            w.tag("text")
                .attr("x", label_x)
                .attr("y", r.y + r.h * 0.5 + 4.0)
                .attr(
                    "fill",
                    if dyslexic_on {
                        COLOR_PASS
                    } else {
                        TEXT_PRIMARY
                    },
                )
                .attr("font-family", "MuCRT, monospace")
                .attr("font-size", f_sz)
                .attr("font-weight", "bold")
                .text(&lbl);
            w.close();
            return;
        }

        let role = match s.action {
            SwitchAction::Maintained => "radiogroup",
            SwitchAction::SpringReturn | SwitchAction::PullToTurn => "group",
        };
        w.tag("g")
            .attr("data-control-id", s.control_id)
            .attr("data-bbox-x", r.x)
            .attr("data-bbox-y", r.y)
            .attr("data-bbox-w", r.w)
            .attr("data-bbox-h", r.h)
            .attr("role", role)
            .attr("aria-label", s.label)
            .attr("tabindex", 0_u32)
            .open();
        w.tag("rect")
            .attr("x", r.x)
            .attr("y", r.y)
            .attr("width", r.w)
            .attr("height", r.h)
            .attr("rx", 5.0)
            .attr(
                "fill",
                if s.action == SwitchAction::PullToTurn {
                    "#261c14"
                } else {
                    BG_CONTROL
                },
            )
            .attr(
                "stroke",
                if s.pulled || s.action == SwitchAction::PullToTurn {
                    COLOR_WARN
                } else {
                    BORDER
                },
            )
            .attr("stroke-width", 1.2)
            .end();

        let n = s.options.len().max(1);
        let pad = 4.0;
        let inner = r.inset(pad, pad, pad, pad);
        for (i, opt) in s.options.iter().enumerate() {
            let active = i == s.selected;
            let cell = if s.horizontal {
                let cw = inner.w / n as f64;
                Rect::new(inner.x + i as f64 * cw, inner.y, cw, inner.h)
            } else {
                let ch = inner.h / n as f64;
                Rect::new(inner.x, inner.y + i as f64 * ch, inner.w, ch)
            };
            let is_button_group = s.action != SwitchAction::Maintained;
            let stroke = if active {
                opt.tone.wireframe_stroke()
            } else if is_button_group {
                "#3d5270"
            } else {
                BORDER
            };
            let fill = if active {
                if s.action == SwitchAction::PullToTurn && i == 1 {
                    "#3b2314"
                } else {
                    "#1f3452"
                }
            } else if is_button_group {
                "#182232"
            } else {
                BG_PANEL
            };
            let opt_id = format!("{}:{}", s.control_id, opt.id);
            w.tag("g")
                .attr("data-control-id", opt_id)
                .attr_opt(
                    "data-area",
                    (s.control_id == "c1-area-selector").then_some(opt.id),
                )
                .attr_opt(
                    "data-subview",
                    (s.control_id == "c2-subview-bar").then_some(opt.id),
                )
                .attr_opt(
                    "data-trainway-restart",
                    (s.control_id == "c5-restart-switch" && opt.id == "rst").then_some("true"),
                )
                .attr_opt(
                    "data-ecosystem",
                    (s.control_id == "reg-pkg-filter").then_some(opt.id),
                )
                .attr_opt(
                    "data-purepath-filter",
                    (s.control_id == "obs-pp-filter").then_some(opt.id),
                )
                .attr_opt(
                    "data-log-severity",
                    (s.control_id == "obs-log-filter").then_some(opt.id),
                )
                .attr("data-option-index", i)
                .attr("role", if is_button_group { "button" } else { "radio" })
                .attr("aria-checked", if active { "true" } else { "false" })
                .attr("aria-label", opt.label)
                .open();
            w.tag("rect")
                .attr("x", cell.x + 1.5)
                .attr("y", cell.y + 1.5)
                .attr("width", (cell.w - 3.0).max(1.0))
                .attr("height", (cell.h - 3.0).max(1.0))
                .attr("rx", if is_button_group { 5.0 } else { 3.5 })
                .attr("fill", fill)
                .attr("stroke", stroke)
                .attr("stroke-width", if active { 1.8 } else { 1.0 })
                .end();
            // Active accent bar on maintained navigation tabs.
            if active && !is_button_group {
                if s.horizontal {
                    w.tag("rect")
                        .attr("x", cell.x + 4.0)
                        .attr("y", cell.y + 2.0)
                        .attr("width", (cell.w - 8.0).max(2.0))
                        .attr("height", 2.5)
                        .attr("fill", opt.tone.wireframe_fg())
                        .end();
                } else {
                    w.tag("rect")
                        .attr("x", cell.x + 2.0)
                        .attr("y", cell.y + 5.0)
                        .attr("width", 3.0)
                        .attr("height", (cell.h - 10.0).max(2.0))
                        .attr("fill", opt.tone.wireframe_fg())
                        .end();
                }
            }
            let text_raw = match opt.badge.filter(|b| *b > 0) {
                Some(b) => format!("{} ({b})", opt.label),
                None => opt.label.to_owned(),
            };
            let font_sz = if cell.w < 76.0 && text_raw.len() >= 7 {
                10.5
            } else {
                12.0
            };
            let text = fit_box(&text_raw, cell.w, font_sz);
            w.tag("text")
                .attr("x", cell.x + cell.w * 0.5)
                .attr("y", cell.y + cell.h * 0.5 + 4.0)
                .attr("text-anchor", "middle")
                .attr(
                    "fill",
                    if active {
                        TEXT_PRIMARY
                    } else if is_button_group {
                        COLOR_INFO
                    } else {
                        TEXT_MUTED
                    },
                )
                .attr("font-family", "MuCRT, monospace")
                .attr("font-size", font_sz)
                .attr_opt("font-weight", active.then_some("bold"))
                .text(&text);
            w.close();
        }
        w.close();
    }

    #[allow(clippy::too_many_lines)]
    fn rotary(&self, w: &mut SvgWriter, r: &Rotary<'_>) {
        let rect = r.rect;
        let repo_step = match r.control_id {
            "c2-repo-prev" => Some("prev"),
            "c2-repo-next" | "c2-repo-select" => Some("next"),
            "c2-sub-prev" => Some("sub-prev"),
            "c2-sub-next" => Some("sub-next"),
            _ => None,
        };
        w.tag("g")
            .attr("data-control-id", r.control_id)
            .attr_opt("data-repo-step", repo_step)
            .attr("data-bbox-x", rect.x)
            .attr("data-bbox-y", rect.y)
            .attr("data-bbox-w", rect.w)
            .attr("data-bbox-h", rect.h)
            .attr("role", "group")
            .attr("aria-label", "Repository and Submodule Selector")
            .attr("tabindex", 0_u32)
            .open();
        w.tag("rect")
            .attr("x", rect.x)
            .attr("y", rect.y)
            .attr("width", rect.w)
            .attr("height", rect.h)
            .attr("rx", 4.0)
            .attr("fill", BG_PANEL)
            .attr("stroke", if r.settling { COLOR_WARN } else { BORDER })
            .attr("stroke-width", 1.0)
            .end();

        // Panel header title on desktop/tablet (phone sheet already has its own dialog title).
        let hdr_h = if rect.y < 120.0 {
            let hdr_lbl = fit_box("REPOSITORY & SUBMODULE SELECTOR", rect.w - 16.0, 11.0);
            w.tag("text")
                .attr("x", rect.x + 10.0)
                .attr("y", rect.y + 14.0)
                .attr("fill", TEXT_MUTED)
                .attr("font-family", "MuCRT, monospace")
                .attr("font-size", 11.0)
                .attr("font-weight", "bold")
                .text(&hdr_lbl);
            18.0
        } else {
            4.0
        };

        let inner = Rect::new(
            rect.x + 8.0,
            rect.y + hdr_h + 4.0,
            (rect.w - 16.0).max(40.0),
            (rect.h - hdr_h - 10.0).max(48.0),
        );
        let row_h = (inner.h / 2.65).max(20.0);
        let step_btn_w = 28.0_f64.min(inner.w * 0.14);
        let disp_w = (inner.w - 2.0 * step_btn_w - 8.0).max(32.0);

        // Outer ring row (Repository) + clickable ◀ / ▶ stepper buttons.
        let outer_rect = Rect::new(inner.x, inner.y, inner.w, row_h - 3.0);
        w.tag("g")
            .attr("data-control-id", format!("{}:outer", r.control_id))
            .attr("role", "spinbutton")
            .attr("aria-label", "Outer Ring Repository")
            .attr("aria-valuenow", r.outer_index)
            .attr("aria-valuemin", 0_usize)
            .attr("aria-valuemax", r.outer_total.saturating_sub(1))
            .open();
        w.tag("rect")
            .attr("x", outer_rect.x)
            .attr("y", outer_rect.y)
            .attr("width", disp_w)
            .attr("height", outer_rect.h)
            .attr("rx", 3.0)
            .attr("fill", BG_SCREEN)
            .attr("stroke", COLOR_INFO)
            .attr("stroke-width", 1.2)
            .end();
        let outer_txt = fit_box(
            &format!(
                "REPO [{}/{}]: {}",
                r.outer_index + usize::from(r.outer_total > 0),
                r.outer_total,
                r.outer_label
            ),
            disp_w,
            12.0,
        );
        w.tag("text")
            .attr("x", outer_rect.x + 8.0)
            .attr("y", outer_rect.y + outer_rect.h * 0.5 + 4.0)
            .attr("fill", TEXT_PRIMARY)
            .attr("font-family", "MuCRT, monospace")
            .attr("font-size", 12.0)
            .text(&outer_txt);
        draw_stepper_button(
            w,
            &format!("{}:outer-prev", r.control_id),
            "Previous Repository",
            "◀",
            Rect::new(
                outer_rect.x + disp_w + 4.0,
                outer_rect.y,
                step_btn_w,
                outer_rect.h,
            ),
        );
        draw_stepper_button(
            w,
            &format!("{}:outer-next", r.control_id),
            "Next Repository",
            "▶",
            Rect::new(
                outer_rect.x + disp_w + step_btn_w + 8.0,
                outer_rect.y,
                step_btn_w,
                outer_rect.h,
            ),
        );
        w.close();

        // Inner ring row (Submodule) + clickable ◀ / ▶ stepper buttons.
        let sub_rect = Rect::new(inner.x, inner.y + row_h, inner.w, row_h - 3.0);
        w.tag("g")
            .attr("data-control-id", format!("{}:inner", r.control_id))
            .attr("role", "spinbutton")
            .attr("aria-label", "Inner Ring Submodule")
            .attr("aria-valuenow", r.inner_index)
            .attr("aria-valuemin", 0_usize)
            .attr("aria-valuemax", r.inner_total.saturating_sub(1))
            .open();
        w.tag("rect")
            .attr("x", sub_rect.x)
            .attr("y", sub_rect.y)
            .attr("width", disp_w)
            .attr("height", sub_rect.h)
            .attr("rx", 3.0)
            .attr("fill", BG_SCREEN)
            .attr("stroke", BORDER)
            .attr("stroke-width", 1.0)
            .end();
        let sub_txt = fit_box(
            &format!(
                "SUBMODULE [{}/{}]: {}",
                r.inner_index,
                r.inner_total.saturating_sub(1),
                r.inner_label
            ),
            disp_w,
            12.0,
        );
        w.tag("text")
            .attr("x", sub_rect.x + 8.0)
            .attr("y", sub_rect.y + sub_rect.h * 0.5 + 4.0)
            .attr("fill", TEXT_PRIMARY)
            .attr("font-family", "MuCRT, monospace")
            .attr("font-size", 12.0)
            .text(&sub_txt);
        draw_stepper_button(
            w,
            &format!("{}:inner-prev", r.control_id),
            "Previous Submodule",
            "◀",
            Rect::new(
                sub_rect.x + disp_w + 4.0,
                sub_rect.y,
                step_btn_w,
                sub_rect.h,
            ),
        );
        draw_stepper_button(
            w,
            &format!("{}:inner-next", r.control_id),
            "Next Submodule",
            "▶",
            Rect::new(
                sub_rect.x + disp_w + step_btn_w + 8.0,
                sub_rect.y,
                step_btn_w,
                sub_rect.h,
            ),
        );
        w.close();

        // Filter / status row.
        let flt_y = inner.y + 2.0 * row_h;
        let flt_h = (inner.bottom() - flt_y).max(16.0);
        let flt_rect = Rect::new(inner.x, flt_y, inner.w, flt_h);
        let status = if !r.filter_query.is_empty() {
            format!("FILTER: /{}", r.filter_query)
        } else if r.settling {
            "APPLYING SELECTION (350ms)…".to_owned()
        } else {
            "Press [/] to filter repositories by name".to_owned()
        };
        let flt_txt = fit_box(&status, flt_rect.w, 11.0);
        w.tag("text")
            .attr("x", flt_rect.x + 4.0)
            .attr("y", flt_rect.y + flt_rect.h * 0.5 + 4.0)
            .attr("fill", if r.settling { COLOR_WARN } else { TEXT_MUTED })
            .attr("font-family", "MuCRT, monospace")
            .attr("font-size", 11.0)
            .text(&flt_txt);

        w.close();
    }

    fn lamp(&self, w: &mut SvgWriter, l: &Lamp<'_>) {
        let r = l.rect;
        let stroke = if l.lit {
            l.tone.wireframe_stroke()
        } else {
            "#252d38"
        };
        let fg = if l.lit {
            l.tone.wireframe_fg()
        } else {
            "#636e7b"
        };
        w.tag("g")
            .attr("data-control-id", l.control_id)
            .attr("data-bbox-x", r.x)
            .attr("data-bbox-y", r.y)
            .attr("data-bbox-w", r.w)
            .attr("data-bbox-h", r.h)
            .attr("role", "status")
            .attr("aria-label", l.label)
            .attr("data-lit", if l.lit { "true" } else { "false" })
            .open();
        // Passive pill badge with internal LED dot (visually distinct from buttons).
        let pill_h = r.h.min(24.0);
        let pill_y = r.y + (r.h - pill_h) * 0.5;
        w.tag("rect")
            .attr("x", r.x)
            .attr("y", pill_y)
            .attr("width", r.w)
            .attr("height", pill_h)
            .attr("rx", pill_h * 0.5)
            .attr("fill", if l.lit { "#1c222d" } else { "#0c1016" })
            .attr("stroke", stroke)
            .attr("stroke-width", if l.lit { 1.4 } else { 1.0 })
            .attr_opt("stroke-dasharray", (!l.lit).then_some("3 2"))
            .end();
        let dot_x = r.x + 8.0;
        let dot_y = r.y + r.h * 0.5;
        w.tag("circle")
            .attr("cx", dot_x)
            .attr("cy", dot_y)
            .attr("r", if l.lit { 3.5 } else { 2.5 })
            .attr("fill", if l.lit { fg } else { "none" })
            .attr("stroke", fg)
            .attr("stroke-width", 1.2)
            .end();
        let txt = fit_box(l.label, (r.w - 8.0).max(16.0), 10.0);
        w.tag("text")
            .attr("x", r.x + 14.0 + (r.w - 16.0).max(0.0) * 0.5)
            .attr("y", r.y + r.h * 0.5 + 3.5)
            .attr("text-anchor", "middle")
            .attr("fill", fg)
            .attr("font-family", "MuCRT, monospace")
            .attr("font-size", 10.0)
            .text(&txt);
        w.close();
    }

    fn readout(&self, w: &mut SvgWriter, r: &Readout<'_>) {
        let rect = r.rect;
        let repo_step = match r.control_id {
            "c2-repo-prev" => Some("prev"),
            "c2-repo-next" | "c2-repo-select" => Some("next"),
            "c2-sub-prev" => Some("sub-prev"),
            "c2-sub-next" => Some("sub-next"),
            _ => None,
        };
        w.tag("g")
            .attr("data-control-id", r.control_id)
            .attr_opt("data-repo-step", repo_step)
            .attr("data-bbox-x", rect.x)
            .attr("data-bbox-y", rect.y)
            .attr("data-bbox-w", rect.w)
            .attr("data-bbox-h", rect.h)
            .attr("role", if r.interactive { "button" } else { "status" })
            .attr(
                "aria-label",
                if r.label.is_empty() {
                    r.value.to_owned()
                } else {
                    format!("{}: {}", r.label, r.value)
                },
            )
            .attr_opt("tabindex", r.interactive.then_some(0_u32))
            .open();

        // Distinguish interactive buttons/toggles vs carousel page bar vs passive state displays.
        if r.control_id == "c4-page-dots" {
            w.tag("rect")
                .attr("x", rect.x)
                .attr("y", rect.y)
                .attr("width", rect.w)
                .attr("height", rect.h)
                .attr("rx", 3.0)
                .attr("fill", "#0c1017")
                .attr("stroke", "#1f2733")
                .attr("stroke-width", 1.0)
                .end();
            let txt = fit_box(&format!("{}: {}", r.label, r.value), rect.w, 11.5);
            w.tag("text")
                .attr("x", rect.x + rect.w * 0.5)
                .attr("y", rect.y + rect.h * 0.5 + 4.0)
                .attr("text-anchor", "middle")
                .attr("fill", COLOR_INFO)
                .attr("font-family", "MuCRT, monospace")
                .attr("font-size", 11.5)
                .text(&txt);
            w.close();
            return;
        }

        let is_button = r.interactive && !r.control_id.starts_with("c2-");
        let fill = if is_button { "#182232" } else { BG_SCREEN };
        let stroke = if is_button {
            r.tone.wireframe_stroke()
        } else if r.control_id.starts_with("c2-") {
            "#263242"
        } else {
            r.tone.wireframe_stroke()
        };
        w.tag("rect")
            .attr("x", rect.x)
            .attr("y", rect.y)
            .attr("width", rect.w)
            .attr("height", rect.h)
            .attr("rx", if is_button { 6.0 } else { 3.0 })
            .attr("fill", fill)
            .attr("stroke", stroke)
            .attr("stroke-width", if is_button { 1.5 } else { 1.0 })
            .end();
        let combined = if r.label.is_empty() {
            r.value.to_owned()
        } else {
            format!("{}: {}", r.label, r.value)
        };
        let txt = fit_box(&combined, rect.w, 12.0);
        w.tag("text")
            .attr(
                "x",
                if is_button {
                    rect.x + rect.w * 0.5
                } else {
                    rect.x + 8.0
                },
            )
            .attr("y", rect.y + rect.h * 0.5 + 4.0)
            .attr_opt("text-anchor", is_button.then_some("middle"))
            .attr("fill", r.tone.wireframe_fg())
            .attr("font-family", "MuCRT, monospace")
            .attr("font-size", 12.0)
            .attr_opt("font-weight", is_button.then_some("bold"))
            .text(&txt);
        w.close();
    }

    #[allow(clippy::cast_precision_loss, clippy::too_many_lines)]
    fn screen(&self, w: &mut SvgWriter, s: &Screen<'_>) {
        let r = s.rect;
        w.tag("g")
            .attr("data-control-id", s.control_id)
            .attr("data-bbox-x", r.x)
            .attr("data-bbox-y", r.y)
            .attr("data-bbox-w", r.w)
            .attr("data-bbox-h", r.h)
            .attr("role", "region")
            .attr("aria-label", s.title)
            .attr("data-cols", s.cols)
            .attr("data-rows", s.rows)
            .open();
        w.tag("rect")
            .attr("x", r.x)
            .attr("y", r.y)
            .attr("width", r.w)
            .attr("height", r.h)
            .attr("rx", 3.0)
            .attr("fill", BG_SCREEN)
            .attr("stroke", BORDER)
            .attr("stroke-width", 1.0)
            .end();

        let font_px = s.font_px.max(10.0);
        let row_h = (font_px * 1.3).max(14.0);
        let max_rows = s.rows.max(1);
        let pad_x = 8.0;
        let pad_y = 8.0;

        if !s.tracks.is_empty() && !s.stations.is_empty() && r.h >= 180.0 {
            draw_integrated_trainway(w, s, r, font_px, row_h, max_rows, pad_x, pad_y);
        } else {
            if !s.tracks.is_empty() && !s.stations.is_empty() {
                draw_tracks_compact(w, r, s.stations, s.tracks);
            }
            let visible_count = s.lines.len().min(max_rows).max(1);
            let step_h = if r.h >= 220.0 && visible_count >= 6 {
                ((r.h - 2.0 * pad_y - 8.0) / visible_count as f64).clamp(row_h, row_h * 1.65)
            } else {
                row_h
            };
            for (idx, row) in s.lines.iter().take(max_rows).enumerate() {
                let y = r.y + pad_y + (idx as f64 + 0.8) * step_h;
                if y > r.bottom() - 4.0 {
                    break;
                }
                emit_screen_row(w, s, r, row, r.x + pad_x, y, font_px, row_h);
            }
        }

        // Discrete pager pill + clickable ◀ / ▶ buttons when total_pages > 1.
        if s.total_pages > 1 && r.h >= 24.0 {
            let pager_txt = format!("PAGE {}/{}", s.page + 1, s.total_pages);
            if r.w >= 140.0 {
                let btn_w = 22.0;
                let btn_h = 16.0_f64.min((r.h - 4.0).max(12.0));
                let btn_y = (r.bottom() - btn_h - 3.0).max(r.y + 2.0);
                let next_x = (r.right() - btn_w - 6.0).max(r.x + 2.0);
                let prev_x = (next_x - btn_w - 4.0).max(r.x + 2.0);
                let txt_x = (prev_x - 6.0).max(r.x + 8.0);
                w.tag("text")
                    .attr("x", txt_x)
                    .attr("y", btn_y + btn_h * 0.5 + 3.5)
                    .attr("text-anchor", "end")
                    .attr("fill", COLOR_INFO)
                    .attr("font-family", s.font_family)
                    .attr("font-size", 11.0)
                    .attr("data-pager", s.control_id)
                    .text(&pager_txt);
                draw_pager_button(
                    w,
                    &format!("{}:prev", s.control_id),
                    s.control_id,
                    "prev",
                    "Previous Page",
                    "◀",
                    Rect::new(prev_x, btn_y, btn_w, btn_h),
                );
                draw_pager_button(
                    w,
                    &format!("{}:next", s.control_id),
                    s.control_id,
                    "next",
                    "Next Page",
                    "▶",
                    Rect::new(next_x, btn_y, btn_w, btn_h),
                );
            } else {
                w.tag("text")
                    .attr("x", r.right() - 8.0)
                    .attr("y", r.bottom() - 6.0)
                    .attr("text-anchor", "end")
                    .attr("fill", COLOR_INFO)
                    .attr("font-family", s.font_family)
                    .attr("font-size", 11.0)
                    .attr("data-pager", s.control_id)
                    .text(&pager_txt);
            }
        }
        w.close();
    }
}

fn draw_pager_button(
    w: &mut SvgWriter,
    id: &str,
    screen_id: &str,
    step_dir: &str,
    aria_label: &str,
    glyph: &str,
    rect: Rect,
) {
    w.tag("g")
        .attr("data-control-id", id)
        .attr("data-page-step", step_dir)
        .attr("data-bbox-x", rect.x)
        .attr("data-bbox-y", rect.y)
        .attr("data-bbox-w", rect.w)
        .attr("data-bbox-h", rect.h)
        .attr("role", "button")
        .attr("aria-label", aria_label)
        .attr("tabindex", 0_u32)
        .open();
    w.tag("rect")
        .attr("x", rect.x)
        .attr("y", rect.y)
        .attr("width", rect.w)
        .attr("height", rect.h)
        .attr("rx", 3.0)
        .attr("fill", "#1a2536")
        .attr("stroke", "#3d5270")
        .attr("stroke-width", 1.1)
        .end();
    w.tag("text")
        .attr("x", rect.x + rect.w * 0.5)
        .attr("y", rect.y + rect.h * 0.5 + 3.5)
        .attr("text-anchor", "middle")
        .attr("fill", COLOR_INFO)
        .attr("font-family", "MuCRT, monospace")
        .attr("font-size", 10.5)
        .attr("font-weight", "bold")
        .attr("data-pager", screen_id)
        .text(glyph);
    w.close();
}

fn draw_stepper_button(w: &mut SvgWriter, id: &str, aria_label: &str, glyph: &str, rect: Rect) {
    w.tag("g")
        .attr("data-control-id", id)
        .attr("data-bbox-x", rect.x)
        .attr("data-bbox-y", rect.y)
        .attr("data-bbox-w", rect.w)
        .attr("data-bbox-h", rect.h)
        .attr("role", "button")
        .attr("aria-label", aria_label)
        .attr("tabindex", 0_u32)
        .open();
    w.tag("rect")
        .attr("x", rect.x)
        .attr("y", rect.y)
        .attr("width", rect.w)
        .attr("height", rect.h)
        .attr("rx", 4.0)
        .attr("fill", "#1a2536")
        .attr("stroke", "#3d5270")
        .attr("stroke-width", 1.2)
        .end();
    w.tag("text")
        .attr("x", rect.x + rect.w * 0.5)
        .attr("y", rect.y + rect.h * 0.5 + 4.0)
        .attr("text-anchor", "middle")
        .attr("fill", COLOR_INFO)
        .attr("font-family", "MuCRT, monospace")
        .attr("font-size", 11.5)
        .attr("font-weight", "bold")
        .text(glyph);
    w.close();
}

#[allow(clippy::too_many_arguments)]
fn emit_screen_row(
    w: &mut SvgWriter,
    s: &Screen<'_>,
    r: Rect,
    row: &crate::skin::ScreenRow<'_>,
    x: f64,
    y: f64,
    font_px: f64,
    row_h: f64,
) {
    let prefix = match (row.signal, row.critical) {
        (Some(sig), true) => format!("{} [C] ", sig.symbol()),
        (Some(sig), false) => format!("{} ", sig.symbol()),
        (None, true) => "[C] ".to_owned(),
        (None, false) => String::new(),
    };
    let full = format!("{prefix}{}", row.text);
    let clipped = truncate_chars(&full, s.cols.max(1));
    if let Some(cid) = row.control_id {
        w.tag("g")
            .attr("data-control-id", cid)
            .attr_opt("data-trace-id", cid.strip_prefix("trace:"))
            .attr_opt("data-select-trace", cid.strip_prefix("select-trace:"))
            .attr_opt("data-page-step", cid.strip_prefix("page-step:"))
            .attr_opt("data-cd-gate", cid.strip_prefix("gate:"))
            .attr_opt("data-package", cid.strip_prefix("pkg:"))
            .attr_opt("data-entity-id", cid.strip_prefix("entity:"))
            .attr_opt("data-span-id", cid.strip_prefix("span:"))
            .attr_opt("data-ecosystem", cid.strip_prefix("eco:"))
            .attr_opt("data-purepath-filter", cid.strip_prefix("ppfilter:"))
            .attr_opt("data-log-severity", cid.strip_prefix("logsev:"))
            .attr("role", "button")
            .attr("tabindex", 0_u32)
            .attr("aria-selected", if row.selected { "true" } else { "false" })
            .open();
        if row.selected {
            w.tag("rect")
                .attr("x", r.x + 2.0)
                .attr("y", (y - font_px).max(r.y + 1.0))
                .attr("width", (r.w - 4.0).max(1.0))
                .attr("height", row_h.min((r.bottom() - y + font_px).max(1.0)))
                .attr("fill", "#1b2433")
                .end();
        }
    }
    w.tag("text")
        .attr("x", x)
        .attr("y", y)
        .attr("fill", row.tone.wireframe_fg())
        .attr("font-family", s.font_family)
        .attr("font-size", font_px)
        .text(&clipped);
    if row.control_id.is_some() {
        w.close();
    }
}

#[allow(clippy::cast_precision_loss, clippy::too_many_arguments)]
fn draw_integrated_trainway(
    w: &mut SvgWriter,
    s: &Screen<'_>,
    r: Rect,
    font_px: f64,
    row_h: f64,
    max_rows: usize,
    pad_x: f64,
    pad_y: f64,
) {
    let mut emitted_rows = 0_usize;

    // 1. Top telemetry header row (s.lines[0]).
    let hdr_y = r.y + pad_y + 0.8 * row_h;
    if let Some(first) = s.lines.first() {
        emit_screen_row(w, s, r, first, r.x + pad_x, hdr_y, font_px, row_h);
        emitted_rows += 1;
    }
    let sep_top_y = (r.y + pad_y + row_h + 4.0).min(r.bottom() - 4.0);
    w.tag("path")
        .attr(
            "d",
            PathData::new()
                .m(r.x + 4.0, sep_top_y)
                .h(r.right() - 4.0)
                .build(),
        )
        .attr("stroke", "#1f2633")
        .attr("stroke-width", 1.0)
        .end();

    // 2. Station column headers + vertical dashed guide lines.
    let n_st = s.stations.len().max(1);
    let left = r.x + 28.0;
    let right = (r.right() - 28.0).max(left + 10.0);
    let span = (right - left) / n_st.saturating_sub(1).max(1) as f64;
    let st_hdr_y = (sep_top_y + 15.0).min(r.bottom() - 20.0);

    let footer_start = (1 + s.tracks.len()).min(s.lines.len());
    let footer_count = s
        .lines
        .len()
        .saturating_sub(footer_start)
        .min(max_rows.saturating_sub(1 + s.tracks.len()));
    let footer_h = if footer_count > 0 {
        (footer_count as f64) * (row_h + 2.0) + 10.0
    } else {
        8.0
    };
    let track_top = (st_hdr_y + 10.0).min(r.bottom() - footer_h - 16.0);
    let track_bottom = (r.bottom() - footer_h - 6.0).max(track_top + 20.0);

    for (i, &st_lbl) in s.stations.iter().enumerate() {
        let sx = left + i as f64 * span;
        let guide = PathData::new()
            .m(sx, st_hdr_y + 4.0)
            .v(track_bottom)
            .build();
        w.tag("path")
            .attr("d", guide)
            .attr("stroke", "#18202c")
            .attr("stroke-width", 1.0)
            .attr("stroke-dasharray", "2 4")
            .end();
        let display_lbl = if span < 62.0 {
            match st_lbl {
                "CLIPPY" => "CLIP",
                "AUDIT" => "AUDT",
                "BENCH" => "BNCH",
                "RELEASE" => "REL",
                "MERGE" => "MERG",
                other => other,
            }
        } else {
            st_lbl
        };
        w.tag("text")
            .attr("x", sx)
            .attr("y", st_hdr_y)
            .attr("text-anchor", "middle")
            .attr("fill", TEXT_MUTED)
            .attr("font-family", s.font_family)
            .attr("font-size", (font_px - 2.0).max(10.0))
            .attr("data-pager", "station-hdr")
            .text(display_lbl);
    }

    // 3. Integrated track bands: draw schematic track paths and signals first, then render
    //    each track's label over a clean background backing strip so curves never cross text.
    let n_tracks = s.tracks.len().max(1);
    let band_h = ((track_bottom - track_top) / n_tracks as f64).max(22.0);
    let trunk_y = (track_top + 0.68 * band_h).min(track_bottom - 4.0);

    for (idx, tr) in s.tracks.iter().enumerate() {
        let ty = (track_top + (idx as f64 + 0.68) * band_h).min(track_bottom - 4.0);
        let fork_x =
            left + f64::from(tr.fork_stage.min(u32::try_from(n_st - 1).unwrap_or(0))) * span;
        let path = if idx == 0 || tr.lane == 0 {
            PathData::new().m(left, ty).h(right).build()
        } else {
            PathData::new()
                .m(fork_x, trunk_y)
                .q(fork_x + 10.0, ty, (fork_x + 22.0).min(right), ty)
                .h(right)
                .build()
        };
        let stroke = if tr.node_lost {
            COLOR_FAIL
        } else if tr.fading {
            TEXT_MUTED
        } else if idx == 0 || tr.lane == 0 {
            COLOR_PASS
        } else {
            COLOR_INFO
        };
        w.tag("path")
            .attr("d", path)
            .attr("fill", "none")
            .attr("stroke", stroke)
            .attr(
                "stroke-width",
                if idx == 0 || tr.lane == 0 { 2.2 } else { 1.6 },
            )
            .attr_opt("stroke-dasharray", tr.dashed.then_some("5 3"))
            .attr("data-track-ref", tr.ref_name)
            .end();

        for &(st_idx, glyph, tone) in tr.signals {
            let sx = left + f64::from(st_idx) * span;
            render_signal_shape(w, sx, ty, glyph, tone);
        }
    }

    for (idx, _tr) in s.tracks.iter().enumerate() {
        let ty = (track_top + (idx as f64 + 0.68) * band_h).min(track_bottom - 4.0);
        let label_y = (ty - 12.0).max(st_hdr_y + 12.0);
        if emitted_rows < max_rows {
            if let Some(row) = s.lines.get(1 + idx) {
                let est_w = ((row.text.chars().count() + 4) as f64 * font_px * 0.64)
                    .min((r.w - 2.0 * pad_x).max(40.0));
                w.tag("rect")
                    .attr("x", r.x + pad_x - 2.0)
                    .attr("y", (label_y - font_px).max(r.y + 2.0))
                    .attr("width", est_w)
                    .attr("height", font_px + 6.0)
                    .attr("rx", 2.0)
                    .attr("fill", BG_SCREEN)
                    .end();
                emit_screen_row(w, s, r, row, r.x + pad_x, label_y, font_px, row_h);
                emitted_rows += 1;
            }
        }
    }

    // 4. Footer rows (YARD +N, INTERCHANGE ⇄ ...).
    if footer_count > 0 {
        let sep_bot_y = (r.bottom() - footer_h).max(track_top);
        w.tag("path")
            .attr(
                "d",
                PathData::new()
                    .m(r.x + 4.0, sep_bot_y)
                    .h(r.right() - 4.0)
                    .build(),
            )
            .attr("stroke", "#1f2633")
            .attr("stroke-width", 1.0)
            .end();
        for (fi, row) in s
            .lines
            .iter()
            .skip(footer_start)
            .take(max_rows.saturating_sub(emitted_rows))
            .enumerate()
        {
            let fy = sep_bot_y + 4.0 + (fi as f64 + 0.8) * row_h;
            if fy <= r.bottom() - 4.0 {
                emit_screen_row(w, s, r, row, r.x + pad_x, fy, font_px, row_h);
            }
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn draw_tracks_compact(w: &mut SvgWriter, r: Rect, stations: &[&str], tracks: &[ScreenTrack<'_>]) {
    let n_st = stations.len().max(1);
    let left = r.x + 24.0;
    let right = (r.right() - 24.0).max(left + 10.0);
    let span = (right - left) / n_st.saturating_sub(1).max(1) as f64;
    let base_y = (r.bottom() - 22.0).max(r.y + 18.0);
    let lane_pitch = 16.0;

    for tr in tracks {
        let y = (base_y - f64::from(tr.lane) * lane_pitch).max(r.y + 12.0);
        let fork_x =
            left + f64::from(tr.fork_stage.min(u32::try_from(n_st - 1).unwrap_or(0))) * span;
        let path = if tr.lane == 0 {
            PathData::new().m(left, base_y).h(right).build()
        } else {
            PathData::new()
                .m(fork_x, base_y)
                .q(fork_x + 12.0, y, (fork_x + 24.0).min(right), y)
                .h(right)
                .build()
        };
        let stroke = if tr.node_lost {
            COLOR_FAIL
        } else if tr.fading {
            TEXT_MUTED
        } else {
            COLOR_INFO
        };
        w.tag("path")
            .attr("d", path)
            .attr("fill", "none")
            .attr("stroke", stroke)
            .attr("stroke-width", if tr.lane == 0 { 2.0 } else { 1.5 })
            .attr_opt("stroke-dasharray", tr.dashed.then_some("4 3"))
            .attr("data-track-ref", tr.ref_name)
            .end();

        for &(st_idx, glyph, tone) in tr.signals {
            let sx = left + f64::from(st_idx) * span;
            render_signal_shape(w, sx, y, glyph, tone);
        }
    }
}

fn render_signal_shape(
    w: &mut SvgWriter,
    cx: f64,
    cy: f64,
    glyph: SignalGlyph,
    tone: SemanticTone,
) {
    let color = tone.wireframe_fg();
    match glyph {
        SignalGlyph::BlockPassed => {
            w.tag("rect")
                .attr("x", cx - 3.5)
                .attr("y", cy - 3.5)
                .attr("width", 7.0)
                .attr("height", 7.0)
                .attr("fill", color)
                .end();
        }
        SignalGlyph::BlockRunning => {
            w.tag("rect")
                .attr("x", cx - 3.5)
                .attr("y", cy - 3.5)
                .attr("width", 7.0)
                .attr("height", 7.0)
                .attr("fill", BG_SCREEN)
                .attr("stroke", color)
                .attr("stroke-width", 1.5)
                .end();
        }
        SignalGlyph::BlockFailed => {
            let d = PathData::new()
                .m(cx - 3.5, cy - 3.5)
                .l(cx + 3.5, cy + 3.5)
                .m(cx + 3.5, cy - 3.5)
                .l(cx - 3.5, cy + 3.5)
                .build();
            w.tag("path")
                .attr("d", d)
                .attr("stroke", color)
                .attr("stroke-width", 1.8)
                .end();
        }
        SignalGlyph::BlockPending => {
            w.tag("rect")
                .attr("x", cx - 2.0)
                .attr("y", cy - 2.0)
                .attr("width", 4.0)
                .attr("height", 4.0)
                .attr("fill", TEXT_MUTED)
                .end();
        }
        SignalGlyph::TestPassed => {
            w.tag("circle")
                .attr("cx", cx)
                .attr("cy", cy)
                .attr("r", 3.5)
                .attr("fill", color)
                .end();
        }
        SignalGlyph::TestRunning => {
            w.tag("circle")
                .attr("cx", cx)
                .attr("cy", cy)
                .attr("r", 3.5)
                .attr("fill", BG_SCREEN)
                .attr("stroke", color)
                .attr("stroke-width", 1.5)
                .end();
        }
        SignalGlyph::TestPending => {
            w.tag("circle")
                .attr("cx", cx)
                .attr("cy", cy)
                .attr("r", 2.5)
                .attr("fill", "none")
                .attr("stroke", TEXT_MUTED)
                .attr("stroke-width", 1.0)
                .end();
        }
    }
}

impl Skin for AvionicsSkin {
    fn name(&self) -> &'static str {
        "avionics"
    }

    fn begin_frame(&self, w: &mut SvgWriter, width: f64, height: f64) {
        write_defs(w, self.effects);
        wood(w, width, height);
    }

    fn panel(&self, w: &mut SvgWriter, p: &Panel<'_>) {
        let r = p.rect;
        let is_sub_plate =
            matches!(p.id, "c4-card-plate" | "c3-phone-sheet") || p.id.ends_with("-card-plate");
        w.tag("g")
            .attr("data-panel-id", p.id)
            .attr_opt("role", p.role)
            .attr_opt("aria-label", p.aria_label)
            .open();
        if is_sub_plate {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let col = (r.x.max(0.0) / (r.w.max(60.0) * 0.85)).round() as usize;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let row = (r.y.max(0.0) / (r.h.max(60.0) * 0.85)).round() as usize;
            let variant = (col + row * 2) % ids::BRUSHED_SUB_VARIANTS.len();
            riveted_sub_plate(w, r, variant, true);
        } else {
            let brush_id = match p.id {
                "c1-nav-panel" => ids::BRUSHED_RAIL,
                "c2-top-bar" => ids::BRUSHED_BAR,
                _ => ids::BRUSHED,
            };
            chassis_plate(w, r, brush_id, true);
        }
        if !p.title.is_empty() && r.h >= 24.0 {
            let title_x = if r.w >= 96.0 { r.x + 17.0 } else { r.x + 14.0 };
            let avail_w = (r.right() - title_x - 10.0).max(16.0);
            let label = fit_box(p.title, avail_w, 11.0);
            w.tag("text")
                .attr("x", title_x)
                .attr("y", r.y + 15.0)
                .attr("fill", if is_sub_plate { "#e4ecf5" } else { "#d8c6a8" })
                .attr("font-family", "MuCRT, monospace")
                .attr("font-size", 11.0)
                .attr("font-weight", "bold")
                .text(&label);
        }
        w.close();
    }

    #[allow(clippy::cast_precision_loss, clippy::too_many_lines)]
    fn selector(&self, w: &mut SvgWriter, s: &Selector<'_>) {
        let r = s.rect;
        if s.control_id == "c2-font-switch" {
            let dyslexic_on = s.selected == 1;
            let border = if dyslexic_on { COLOR_PASS } else { "#555a63" };
            w.tag("g")
                .attr("data-control-id", s.control_id)
                .attr("data-dyslexia-toggle", "true")
                .attr("data-bbox-x", r.x)
                .attr("data-bbox-y", r.y)
                .attr("data-bbox-w", r.w)
                .attr("data-bbox-h", r.h)
                .attr("role", "switch")
                .attr("aria-checked", if dyslexic_on { "true" } else { "false" })
                .attr("aria-label", "Use dyslexia-friendly font")
                .attr("tabindex", 0_u32)
                .open();
            w.tag("rect")
                .attr("x", r.x)
                .attr("y", r.y)
                .attr("width", r.w)
                .attr("height", r.h)
                .attr("rx", 4.0)
                .attr("fill", url(ids::PARKERIZED))
                .attr("stroke", border)
                .attr("stroke-width", 1.4)
                .end();
            if r.w >= 64.0 && r.h >= 20.0 {
                screw(w, r.x + 4.0, r.y + 4.0, 1.5, 30.0);
                screw(w, r.right() - 4.0, r.bottom() - 4.0, 1.5, 120.0);
            }

            let box_sz = (r.h - 14.0).clamp(14.0, 18.0);
            let box_x = r.x + 8.0;
            let lamp_r = (box_sz * 0.34).clamp(3.2, 5.0);
            indicator_lamp(
                w,
                box_x + box_sz * 0.5,
                r.y + r.h * 0.5,
                LensColor::Green,
                dyslexic_on,
                lamp_r,
            );

            let label_x = box_x + box_sz + 6.0;
            let avail_w = (r.right() - label_x - 5.0).max(20.0);
            let (raw_lbl, f_sz) = if avail_w >= 148.0 {
                ("Dyslexia-friendly font", 11.5)
            } else {
                ("Dyslexia font", 11.0)
            };
            let lbl = fit_box(raw_lbl, avail_w, f_sz);
            w.tag("text")
                .attr("x", label_x)
                .attr("y", r.y + r.h * 0.5 + 4.0)
                .attr("fill", if dyslexic_on { "#b8ffb0" } else { "#e6dcc8" })
                .attr("font-family", "MuCRT, monospace")
                .attr("font-size", f_sz)
                .attr("font-weight", "bold")
                .text(&lbl);
            w.close();
            return;
        }

        let role = match s.action {
            SwitchAction::Maintained => "radiogroup",
            SwitchAction::SpringReturn | SwitchAction::PullToTurn => "group",
        };
        w.tag("g")
            .attr("data-control-id", s.control_id)
            .attr("data-bbox-x", r.x)
            .attr("data-bbox-y", r.y)
            .attr("data-bbox-w", r.w)
            .attr("data-bbox-h", r.h)
            .attr("role", role)
            .attr("aria-label", s.label)
            .attr("tabindex", 0_u32)
            .open();
        w.tag("rect")
            .attr("x", r.x)
            .attr("y", r.y)
            .attr("width", r.w)
            .attr("height", r.h)
            .attr("rx", 4.0)
            .attr(
                "fill",
                if s.action == SwitchAction::PullToTurn {
                    "#28120e".to_owned()
                } else {
                    url(ids::PARKERIZED)
                },
            )
            .attr(
                "stroke",
                if s.pulled || s.action == SwitchAction::PullToTurn {
                    "#ff5533"
                } else {
                    "#4a4e57"
                },
            )
            .attr("stroke-width", 1.3)
            .end();
        if r.w >= 48.0 && r.h >= 24.0 {
            screw(w, r.x + 4.0, r.y + 4.0, 1.5, 20.0);
            screw(w, r.right() - 4.0, r.bottom() - 4.0, 1.5, 110.0);
        }

        let n = s.options.len().max(1);
        let pad = 4.0;
        let inner = r.inset(pad, pad, pad, pad);
        for (i, opt) in s.options.iter().enumerate() {
            let active = i == s.selected;
            let cell = if s.horizontal {
                let cw = inner.w / n as f64;
                Rect::new(inner.x + i as f64 * cw, inner.y, cw, inner.h)
            } else {
                let ch = inner.h / n as f64;
                Rect::new(inner.x, inner.y + i as f64 * ch, inner.w, ch)
            };
            let is_button_group = s.action != SwitchAction::Maintained;
            let stroke = if active {
                opt.tone.wireframe_stroke()
            } else if is_button_group {
                "#5a606b"
            } else {
                "#353942"
            };
            let fill = if active {
                if s.action == SwitchAction::PullToTurn && i == 1 {
                    "#4a1c12".to_owned()
                } else {
                    "#1d2c1e".to_owned()
                }
            } else {
                url(ids::RUBBER)
            };
            let opt_id = format!("{}:{}", s.control_id, opt.id);
            w.tag("g")
                .attr("data-control-id", opt_id)
                .attr_opt(
                    "data-area",
                    (s.control_id == "c1-area-selector").then_some(opt.id),
                )
                .attr_opt(
                    "data-subview",
                    (s.control_id == "c2-subview-bar").then_some(opt.id),
                )
                .attr_opt(
                    "data-trainway-restart",
                    (s.control_id == "c5-restart-switch" && opt.id == "rst").then_some("true"),
                )
                .attr_opt(
                    "data-ecosystem",
                    (s.control_id == "reg-pkg-filter").then_some(opt.id),
                )
                .attr_opt(
                    "data-purepath-filter",
                    (s.control_id == "obs-pp-filter").then_some(opt.id),
                )
                .attr_opt(
                    "data-log-severity",
                    (s.control_id == "obs-log-filter").then_some(opt.id),
                )
                .attr("data-option-index", i)
                .attr("role", if is_button_group { "button" } else { "radio" })
                .attr("aria-checked", if active { "true" } else { "false" })
                .attr("aria-label", opt.label)
                .open();
            w.tag("rect")
                .attr("x", cell.x + 1.5)
                .attr("y", cell.y + 1.5)
                .attr("width", (cell.w - 3.0).max(1.0))
                .attr("height", (cell.h - 3.0).max(1.0))
                .attr("rx", 3.5)
                .attr("fill", fill)
                .attr("stroke", stroke)
                .attr("stroke-width", if active { 1.8 } else { 1.0 })
                .end();
            if active {
                w.tag("circle")
                    .attr("cx", (cell.x + 6.5).min(cell.right() - 2.0))
                    .attr("cy", cell.y + cell.h * 0.5)
                    .attr("r", 2.4)
                    .attr("fill", opt.tone.wireframe_fg())
                    .attr("filter", url(ids::LAMP_GLOW))
                    .end();
            }
            let text_raw = match opt.badge.filter(|b| *b > 0) {
                Some(b) => format!("{} ({b})", opt.label),
                None => opt.label.to_owned(),
            };
            let font_sz = if cell.w < 76.0 && text_raw.len() >= 7 {
                10.5
            } else {
                12.0
            };
            let text = fit_box(&text_raw, cell.w, font_sz);
            w.tag("text")
                .attr("x", cell.x + cell.w * 0.5)
                .attr("y", cell.y + cell.h * 0.5 + 4.0)
                .attr("text-anchor", "middle")
                .attr(
                    "fill",
                    if active {
                        "#fff3d6"
                    } else if is_button_group {
                        "#d8c6a8"
                    } else {
                        "#a89f91"
                    },
                )
                .attr("font-family", "MuCRT, monospace")
                .attr("font-size", font_sz)
                .attr_opt("font-weight", active.then_some("bold"))
                .text(&text);
            w.close();
        }
        w.close();
    }

    #[allow(clippy::too_many_lines)]
    fn rotary(&self, w: &mut SvgWriter, r: &Rotary<'_>) {
        let rect = r.rect;
        let repo_step = match r.control_id {
            "c2-repo-prev" => Some("prev"),
            "c2-repo-next" | "c2-repo-select" => Some("next"),
            "c2-sub-prev" => Some("sub-prev"),
            "c2-sub-next" => Some("sub-next"),
            _ => None,
        };
        w.tag("g")
            .attr("data-control-id", r.control_id)
            .attr_opt("data-repo-step", repo_step)
            .attr("data-bbox-x", rect.x)
            .attr("data-bbox-y", rect.y)
            .attr("data-bbox-w", rect.w)
            .attr("data-bbox-h", rect.h)
            .attr("role", "group")
            .attr("aria-label", "Repository and Submodule Selector")
            .attr("tabindex", 0_u32)
            .open();
        w.tag("rect")
            .attr("x", rect.x)
            .attr("y", rect.y)
            .attr("width", rect.w)
            .attr("height", rect.h)
            .attr("rx", 4.0)
            .attr("fill", url(ids::PARKERIZED))
            .attr("stroke", if r.settling { COLOR_WARN } else { "#55585e" })
            .attr("stroke-width", 1.2)
            .end();
        if rect.w >= 80.0 && rect.h >= 60.0 {
            screw(w, rect.x + 5.0, rect.y + 5.0, 1.8, 15.0);
            screw(w, rect.right() - 5.0, rect.y + 5.0, 1.8, 105.0);
            screw(w, rect.x + 5.0, rect.bottom() - 5.0, 1.8, 195.0);
            screw(w, rect.right() - 5.0, rect.bottom() - 5.0, 1.8, 285.0);
        }

        let hdr_h = if rect.y < 120.0 {
            let hdr_lbl = fit_box("REPOSITORY & SUBMODULE SELECTOR", rect.w - 16.0, 11.0);
            w.tag("text")
                .attr("x", rect.x + 10.0)
                .attr("y", rect.y + 14.0)
                .attr("fill", "#d8c6a8")
                .attr("font-family", "MuCRT, monospace")
                .attr("font-size", 11.0)
                .attr("font-weight", "bold")
                .text(&hdr_lbl);
            18.0
        } else {
            4.0
        };

        let inner = Rect::new(
            rect.x + 8.0,
            rect.y + hdr_h + 4.0,
            (rect.w - 16.0).max(40.0),
            (rect.h - hdr_h - 10.0).max(48.0),
        );
        let row_h = (inner.h / 2.65).max(20.0);
        let step_btn_w = 28.0_f64.min(inner.w * 0.14);
        let disp_w = (inner.w - 2.0 * step_btn_w - 8.0).max(32.0);

        // Outer ring row (Repository) with STN LCD backlit readout.
        let outer_rect = Rect::new(inner.x, inner.y, inner.w, row_h - 3.0);
        w.tag("g")
            .attr("data-control-id", format!("{}:outer", r.control_id))
            .attr("role", "spinbutton")
            .attr("aria-label", "Outer Ring Repository")
            .attr("aria-valuenow", r.outer_index)
            .attr("aria-valuemin", 0_usize)
            .attr("aria-valuemax", r.outer_total.saturating_sub(1))
            .open();
        w.tag("rect")
            .attr("x", outer_rect.x)
            .attr("y", outer_rect.y)
            .attr("width", disp_w)
            .attr("height", outer_rect.h)
            .attr("rx", 3.0)
            .attr("fill", "#0b0e08")
            .attr("stroke", "#4f6236")
            .attr("stroke-width", 1.2)
            .end();
        w.tag("rect")
            .attr("x", outer_rect.x + 1.5)
            .attr("y", outer_rect.y + 1.5)
            .attr("width", (disp_w - 3.0).max(1.0))
            .attr("height", (outer_rect.h - 3.0).max(1.0))
            .attr("rx", 2.0)
            .attr("fill", url(ids::LCD_BACKLIGHT))
            .attr("opacity", 0.35)
            .end();
        let outer_txt = fit_box(
            &format!(
                "REPO [{}/{}]: {}",
                r.outer_index + usize::from(r.outer_total > 0),
                r.outer_total,
                r.outer_label
            ),
            disp_w,
            12.0,
        );
        w.tag("text")
            .attr("x", outer_rect.x + 8.0)
            .attr("y", outer_rect.y + outer_rect.h * 0.5 + 4.0)
            .attr("fill", "#d8ff8a")
            .attr("font-family", "MuCRT, monospace")
            .attr("font-size", 12.0)
            .text(&outer_txt);
        draw_stepper_button(
            w,
            &format!("{}:outer-prev", r.control_id),
            "Previous Repository",
            "◀",
            Rect::new(
                outer_rect.x + disp_w + 4.0,
                outer_rect.y,
                step_btn_w,
                outer_rect.h,
            ),
        );
        draw_stepper_button(
            w,
            &format!("{}:outer-next", r.control_id),
            "Next Repository",
            "▶",
            Rect::new(
                outer_rect.x + disp_w + step_btn_w + 8.0,
                outer_rect.y,
                step_btn_w,
                outer_rect.h,
            ),
        );
        w.close();

        // Inner ring row (Submodule) with STN LCD backlit readout.
        let sub_rect = Rect::new(inner.x, inner.y + row_h, inner.w, row_h - 3.0);
        w.tag("g")
            .attr("data-control-id", format!("{}:inner", r.control_id))
            .attr("role", "spinbutton")
            .attr("aria-label", "Inner Ring Submodule")
            .attr("aria-valuenow", r.inner_index)
            .attr("aria-valuemin", 0_usize)
            .attr("aria-valuemax", r.inner_total.saturating_sub(1))
            .open();
        w.tag("rect")
            .attr("x", sub_rect.x)
            .attr("y", sub_rect.y)
            .attr("width", disp_w)
            .attr("height", sub_rect.h)
            .attr("rx", 3.0)
            .attr("fill", "#0b0e08")
            .attr("stroke", "#4f6236")
            .attr("stroke-width", 1.0)
            .end();
        w.tag("rect")
            .attr("x", sub_rect.x + 1.5)
            .attr("y", sub_rect.y + 1.5)
            .attr("width", (disp_w - 3.0).max(1.0))
            .attr("height", (sub_rect.h - 3.0).max(1.0))
            .attr("rx", 2.0)
            .attr("fill", url(ids::LCD_BACKLIGHT))
            .attr("opacity", 0.30)
            .end();
        let sub_txt = fit_box(
            &format!(
                "SUBMODULE [{}/{}]: {}",
                r.inner_index,
                r.inner_total.saturating_sub(1),
                r.inner_label
            ),
            disp_w,
            12.0,
        );
        w.tag("text")
            .attr("x", sub_rect.x + 8.0)
            .attr("y", sub_rect.y + sub_rect.h * 0.5 + 4.0)
            .attr("fill", "#d8ff8a")
            .attr("font-family", "MuCRT, monospace")
            .attr("font-size", 12.0)
            .text(&sub_txt);
        draw_stepper_button(
            w,
            &format!("{}:inner-prev", r.control_id),
            "Previous Submodule",
            "◀",
            Rect::new(
                sub_rect.x + disp_w + 4.0,
                sub_rect.y,
                step_btn_w,
                sub_rect.h,
            ),
        );
        draw_stepper_button(
            w,
            &format!("{}:inner-next", r.control_id),
            "Next Submodule",
            "▶",
            Rect::new(
                sub_rect.x + disp_w + step_btn_w + 8.0,
                sub_rect.y,
                step_btn_w,
                sub_rect.h,
            ),
        );
        w.close();

        // Filter / status row.
        let flt_y = inner.y + 2.0 * row_h;
        let flt_h = (inner.bottom() - flt_y).max(16.0);
        let flt_rect = Rect::new(inner.x, flt_y, inner.w, flt_h);
        let status = if !r.filter_query.is_empty() {
            format!("FILTER: /{}", r.filter_query)
        } else if r.settling {
            "APPLYING SELECTION (350ms)…".to_owned()
        } else {
            "Press [/] to filter repositories by name".to_owned()
        };
        let flt_txt = fit_box(&status, flt_rect.w, 11.0);
        w.tag("text")
            .attr("x", flt_rect.x + 4.0)
            .attr("y", flt_rect.y + flt_rect.h * 0.5 + 4.0)
            .attr("fill", if r.settling { COLOR_WARN } else { "#d8c6a8" })
            .attr("font-family", "MuCRT, monospace")
            .attr("font-size", 11.0)
            .text(&flt_txt);

        w.close();
    }

    fn lamp(&self, w: &mut SvgWriter, l: &Lamp<'_>) {
        let r = l.rect;
        let stroke = if l.lit {
            l.tone.wireframe_stroke()
        } else {
            "#3b3e45"
        };
        let fg = if l.lit { "#ffe8b8" } else { "#8c8273" };
        let lens = match l.tone {
            SemanticTone::Pass | SemanticTone::Info => LensColor::Green,
            SemanticTone::Warn => LensColor::Amber,
            SemanticTone::Fail | SemanticTone::Neutral => LensColor::Red,
        };
        w.tag("g")
            .attr("data-control-id", l.control_id)
            .attr("data-bbox-x", r.x)
            .attr("data-bbox-y", r.y)
            .attr("data-bbox-w", r.w)
            .attr("data-bbox-h", r.h)
            .attr("role", "status")
            .attr("aria-label", l.label)
            .attr("data-lit", if l.lit { "true" } else { "false" })
            .open();
        let pill_h = r.h.min(24.0);
        let pill_y = r.y + (r.h - pill_h) * 0.5;
        w.tag("rect")
            .attr("x", r.x)
            .attr("y", pill_y)
            .attr("width", r.w)
            .attr("height", pill_h)
            .attr("rx", 3.5)
            .attr("fill", "#101215")
            .attr("stroke", stroke)
            .attr("stroke-width", if l.lit { 1.4 } else { 1.0 })
            .end();
        let dot_x = r.x + 8.5;
        let dot_y = r.y + r.h * 0.5;
        indicator_lamp(w, dot_x, dot_y, lens, l.lit, 3.2);
        let txt = fit_box(l.label, (r.w - 8.0).max(16.0), 10.0);
        w.tag("text")
            .attr("x", r.x + 14.0 + (r.w - 16.0).max(0.0) * 0.5)
            .attr("y", r.y + r.h * 0.5 + 3.5)
            .attr("text-anchor", "middle")
            .attr("fill", fg)
            .attr("font-family", "MuCRT, monospace")
            .attr("font-size", 10.0)
            .text(&txt);
        w.close();
    }

    fn readout(&self, w: &mut SvgWriter, r: &Readout<'_>) {
        let rect = r.rect;
        let repo_step = match r.control_id {
            "c2-repo-prev" => Some("prev"),
            "c2-repo-next" | "c2-repo-select" => Some("next"),
            "c2-sub-prev" => Some("sub-prev"),
            "c2-sub-next" => Some("sub-next"),
            _ => None,
        };
        w.tag("g")
            .attr("data-control-id", r.control_id)
            .attr_opt("data-repo-step", repo_step)
            .attr("data-bbox-x", rect.x)
            .attr("data-bbox-y", rect.y)
            .attr("data-bbox-w", rect.w)
            .attr("data-bbox-h", rect.h)
            .attr("role", if r.interactive { "button" } else { "status" })
            .attr(
                "aria-label",
                if r.label.is_empty() {
                    r.value.to_owned()
                } else {
                    format!("{}: {}", r.label, r.value)
                },
            )
            .attr_opt("tabindex", r.interactive.then_some(0_u32))
            .open();

        if r.control_id == "c4-page-dots" {
            w.tag("rect")
                .attr("x", rect.x)
                .attr("y", rect.y)
                .attr("width", rect.w)
                .attr("height", rect.h)
                .attr("rx", 3.0)
                .attr("fill", "#0c0e09")
                .attr("stroke", "#3d4430")
                .attr("stroke-width", 1.0)
                .end();
            let txt = fit_box(&format!("{}: {}", r.label, r.value), rect.w, 11.5);
            w.tag("text")
                .attr("x", rect.x + rect.w * 0.5)
                .attr("y", rect.y + rect.h * 0.5 + 4.0)
                .attr("text-anchor", "middle")
                .attr("fill", "#d8ff8a")
                .attr("font-family", "MuCRT, monospace")
                .attr("font-size", 11.5)
                .text(&txt);
            w.close();
            return;
        }

        let is_button = r.interactive && !r.control_id.starts_with("c2-");
        let stroke = if is_button {
            r.tone.wireframe_stroke()
        } else {
            "#475432"
        };
        w.tag("rect")
            .attr("x", rect.x)
            .attr("y", rect.y)
            .attr("width", rect.w)
            .attr("height", rect.h)
            .attr("rx", if is_button { 4.5 } else { 2.5 })
            .attr(
                "fill",
                if is_button {
                    url(ids::PARKERIZED)
                } else {
                    "#0a0d07".to_owned()
                },
            )
            .attr("stroke", stroke)
            .attr("stroke-width", if is_button { 1.5 } else { 1.0 })
            .end();
        if !is_button && rect.w >= 6.0 && rect.h >= 6.0 {
            w.tag("rect")
                .attr("x", rect.x + 1.5)
                .attr("y", rect.y + 1.5)
                .attr("width", (rect.w - 3.0).max(1.0))
                .attr("height", (rect.h - 3.0).max(1.0))
                .attr("rx", 1.5)
                .attr("fill", url(ids::LCD_BACKLIGHT))
                .attr("opacity", 0.32)
                .end();
        }
        let combined = if r.label.is_empty() {
            r.value.to_owned()
        } else {
            format!("{}: {}", r.label, r.value)
        };
        let txt = fit_box(&combined, rect.w, 12.0);
        let fg = match r.tone {
            SemanticTone::Neutral => "#d8ff8a",
            SemanticTone::Pass => "#7aff9e",
            SemanticTone::Warn => "#ffc44d",
            SemanticTone::Fail => "#ff6b6b",
            SemanticTone::Info => "#9ce8ff",
        };
        w.tag("text")
            .attr(
                "x",
                if is_button {
                    rect.x + rect.w * 0.5
                } else {
                    rect.x + 8.0
                },
            )
            .attr("y", rect.y + rect.h * 0.5 + 4.0)
            .attr_opt("text-anchor", is_button.then_some("middle"))
            .attr("fill", fg)
            .attr("font-family", "MuCRT, monospace")
            .attr("font-size", 12.0)
            .attr_opt("font-weight", is_button.then_some("bold"))
            .text(&txt);
        w.close();
    }

    #[allow(clippy::cast_precision_loss, clippy::too_many_lines)]
    fn screen(&self, w: &mut SvgWriter, s: &Screen<'_>) {
        let r = s.rect;
        let phosphor = if s.control_id.starts_with("c6-") || s.control_id.starts_with("cd-") {
            &CRT_CD
        } else {
            &CRT_CI
        };
        w.tag("g")
            .attr("data-control-id", s.control_id)
            .attr("data-bbox-x", r.x)
            .attr("data-bbox-y", r.y)
            .attr("data-bbox-w", r.w)
            .attr("data-bbox-h", r.h)
            .attr("role", "region")
            .attr("aria-label", s.title)
            .attr("data-cols", s.cols)
            .attr("data-rows", s.rows)
            .open();
        w.tag("rect")
            .attr("x", r.x)
            .attr("y", r.y)
            .attr("width", r.w)
            .attr("height", r.h)
            .attr("rx", 5.0)
            .attr("fill", "#080a0c")
            .attr("stroke", "#3a3f47")
            .attr("stroke-width", 1.5)
            .end();
        w.tag("rect")
            .attr("x", r.x + 1.0)
            .attr("y", r.y + 1.0)
            .attr("width", (r.w - 2.0).max(1.0))
            .attr("height", (r.h - 2.0).max(1.0))
            .attr("rx", 4.0)
            .attr("fill", url(crt_glass_id(phosphor)))
            .end();

        w.tag("g")
            .attr("class", format!("mu-crt mu-crt-{}", phosphor.name))
            .attr_opt("filter", self.effects.bloom.then(|| url(ids::PHOSPHOR)))
            .open();

        let font_px = s.font_px.max(10.0);
        let row_h = (font_px * 1.3).max(14.0);
        let max_rows = s.rows.max(1);
        let pad_x = 8.0;
        let pad_y = 8.0;

        if !s.tracks.is_empty() && !s.stations.is_empty() && r.h >= 180.0 {
            draw_integrated_trainway(w, s, r, font_px, row_h, max_rows, pad_x, pad_y);
        } else {
            if !s.tracks.is_empty() && !s.stations.is_empty() {
                draw_tracks_compact(w, r, s.stations, s.tracks);
            }
            let visible_count = s.lines.len().min(max_rows).max(1);
            let step_h = if r.h >= 220.0 && visible_count >= 6 {
                ((r.h - 2.0 * pad_y - 8.0) / visible_count as f64).clamp(row_h, row_h * 1.65)
            } else {
                row_h
            };
            for (idx, row) in s.lines.iter().take(max_rows).enumerate() {
                let y = r.y + pad_y + (idx as f64 + 0.8) * step_h;
                if y > r.bottom() - 4.0 {
                    break;
                }
                emit_screen_row(w, s, r, row, r.x + pad_x, y, font_px, row_h);
            }
        }
        w.close();

        if self.effects.scanlines {
            w.tag("rect")
                .attr("x", r.x + 1.0)
                .attr("y", r.y + 1.0)
                .attr("width", (r.w - 2.0).max(1.0))
                .attr("height", (r.h - 2.0).max(1.0))
                .attr("rx", 4.0)
                .attr("fill", url(ids::SCANLINES))
                .end();
        }
        w.tag("rect")
            .attr("x", r.x + 1.0)
            .attr("y", r.y + 1.0)
            .attr("width", (r.w - 2.0).max(1.0))
            .attr("height", (r.h - 2.0).max(1.0))
            .attr("rx", 4.0)
            .attr("fill", url(ids::VIGNETTE))
            .end();
        w.tag("rect")
            .attr("x", r.x + 1.0)
            .attr("y", r.y + 1.0)
            .attr("width", (r.w - 2.0).max(1.0))
            .attr("height", (r.h - 2.0).max(1.0))
            .attr("rx", 4.0)
            .attr("fill", url(ids::GLASS))
            .end();

        if s.total_pages > 1 && r.h >= 24.0 {
            let pager_txt = format!("PAGE {}/{}", s.page + 1, s.total_pages);
            if r.w >= 140.0 {
                let btn_w = 22.0;
                let btn_h = 16.0_f64.min((r.h - 4.0).max(12.0));
                let btn_y = (r.bottom() - btn_h - 3.0).max(r.y + 2.0);
                let next_x = (r.right() - btn_w - 6.0).max(r.x + 2.0);
                let prev_x = (next_x - btn_w - 4.0).max(r.x + 2.0);
                let txt_x = (prev_x - 6.0).max(r.x + 8.0);
                w.tag("text")
                    .attr("x", txt_x)
                    .attr("y", btn_y + btn_h * 0.5 + 3.5)
                    .attr("text-anchor", "end")
                    .attr("fill", COLOR_INFO)
                    .attr("font-family", s.font_family)
                    .attr("font-size", 11.0)
                    .attr("data-pager", s.control_id)
                    .text(&pager_txt);
                draw_pager_button(
                    w,
                    &format!("{}:prev", s.control_id),
                    s.control_id,
                    "prev",
                    "Previous Page",
                    "◀",
                    Rect::new(prev_x, btn_y, btn_w, btn_h),
                );
                draw_pager_button(
                    w,
                    &format!("{}:next", s.control_id),
                    s.control_id,
                    "next",
                    "Next Page",
                    "▶",
                    Rect::new(next_x, btn_y, btn_w, btn_h),
                );
            } else {
                w.tag("text")
                    .attr("x", r.right() - 8.0)
                    .attr("y", r.bottom() - 6.0)
                    .attr("text-anchor", "end")
                    .attr("fill", COLOR_INFO)
                    .attr("font-family", s.font_family)
                    .attr("font-size", 11.0)
                    .attr("data-pager", s.control_id)
                    .text(&pager_txt);
            }
        }
        w.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skin::{ScreenRow, SelectorOption};

    fn render_sample(skin: &dyn Skin) -> String {
        let mut w = SvgWriter::document(800.0, 600.0);
        skin.begin_frame(&mut w, 800.0, 600.0);
        skin.panel(
            &mut w,
            &Panel {
                id: "nav-rail",
                title: "NAVIGATION",
                rect: Rect::new(6.0, 6.0, 80.0, 588.0),
                role: Some("navigation"),
                aria_label: Some("Area Navigation"),
            },
        );
        let opts = [
            SelectorOption {
                id: "ovr",
                label: "OVR",
                badge: Some(2),
                tone: SemanticTone::Info,
            },
            SelectorOption {
                id: "ci",
                label: "CI",
                badge: None,
                tone: SemanticTone::Pass,
            },
        ];
        skin.selector(
            &mut w,
            &Selector {
                control_id: "c1-area-selector",
                label: "Area Selector",
                rect: Rect::new(10.0, 30.0, 72.0, 120.0),
                action: SwitchAction::Maintained,
                options: &opts,
                selected: 0,
                pulled: false,
                horizontal: false,
            },
        );
        skin.rotary(
            &mut w,
            &Rotary {
                control_id: "c3-rotary",
                rect: Rect::new(100.0, 30.0, 280.0, 160.0),
                outer_label: "fixture/superproject",
                outer_index: 0,
                outer_total: 10,
                inner_label: "code/engine",
                inner_index: 2,
                inner_total: 8,
                filter_query: "",
                settling: false,
            },
        );
        skin.lamp(
            &mut w,
            &Lamp {
                control_id: "lamp-stop",
                label: "STOP",
                rect: Rect::new(400.0, 30.0, 64.0, 24.0),
                lit: true,
                tone: SemanticTone::Fail,
            },
        );
        skin.readout(
            &mut w,
            &Readout {
                control_id: "c2-breadcrumb-repo",
                label: "REPO",
                value: "fixture/superproject",
                rect: Rect::new(100.0, 6.0, 220.0, 22.0),
                tone: SemanticTone::Info,
                interactive: true,
            },
        );
        let rows = [ScreenRow {
            text: "it_boot  1200ms",
            tone: SemanticTone::Pass,
            signal: Some(SignalGlyph::TestPassed),
            critical: true,
            control_id: Some("gate-test-0"),
            selected: true,
        }];
        skin.screen(
            &mut w,
            &Screen {
                control_id: "c6-delivery-screen",
                title: "Delivery Integration Gate",
                rect: Rect::new(100.0, 200.0, 680.0, 380.0),
                font_family: "MuCRT, monospace",
                font_px: 14.0,
                cols: 60,
                rows: 16,
                lines: &rows,
                stations: &["FMT", "CLIPPY"],
                tracks: &[ScreenTrack {
                    ref_name: "main",
                    lane: 0,
                    fork_stage: 0,
                    dashed: false,
                    fading: false,
                    signals: &[(0, SignalGlyph::BlockPassed, SemanticTone::Pass)],
                    node_tag: Some("NODE-A"),
                    node_lost: false,
                }],
                page: 0,
                total_pages: 2,
            },
        );
        w.finish_closing()
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn wireframe_skin_emits_zero_decorative_filters_and_matches_avionics_control_parity() {
        let wf_svg = render_sample(&WireframeSkin);
        let av_svg = render_sample(&AvionicsSkin::default());
        let wf_doc = roxmltree::Document::parse(&wf_svg).expect("valid wireframe XML");
        let av_doc = roxmltree::Document::parse(&av_svg).expect("valid avionics XML");

        assert!(
            !wf_svg.contains("feGaussianBlur"),
            "no blur filters in wireframe"
        );
        assert!(!wf_svg.contains("mu-wood"), "no wood veneer in wireframe");
        assert!(
            !wf_svg.contains("mu-brushed"),
            "no brushed tungsten filter in wireframe"
        );

        type ControlSig = (
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        );
        let extract_controls = |doc: &roxmltree::Document<'_>| -> Vec<ControlSig> {
            doc.descendants()
                .filter_map(|n| {
                    let cid = n.attribute("data-control-id")?.to_owned();
                    let role = n.attribute("role").map(str::to_owned);
                    let label = n.attribute("aria-label").map(str::to_owned);
                    let bbox_w = n.attribute("data-bbox-w").map(str::to_owned);
                    let bbox_h = n.attribute("data-bbox-h").map(str::to_owned);
                    Some((cid, role, label, bbox_w, bbox_h))
                })
                .collect()
        };

        let wf_controls = extract_controls(&wf_doc);
        let av_controls = extract_controls(&av_doc);
        assert_eq!(
            wf_controls, av_controls,
            "WireframeSkin and AvionicsSkin must emit identical control IDs, ARIA attributes, and bounding boxes"
        );
        for token in [
            ids::WOODGRAIN,
            ids::BRUSHED,
            ids::PARKERIZED,
            ids::LCD_BACKLIGHT,
            ids::LENS_RED,
            ids::CRT_CI_GLASS,
            ids::VIGNETTE,
            ids::GLASS,
        ] {
            assert!(
                av_svg.contains(token),
                "AvionicsSkin must emit tactile material/instrument token '{token}'"
            );
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn avionics_skin_separates_carrier_plate_from_riveted_sub_plates_with_finite_bristle_strikes() {
        let skin = AvionicsSkin::default();
        let mut w = SvgWriter::document(1280.0, 720.0);
        skin.begin_frame(&mut w, 1280.0, 720.0);
        skin.panel(
            &mut w,
            &Panel {
                id: "c1-nav-panel",
                title: "",
                rect: Rect::new(4.0, 4.0, 88.0, 712.0),
                role: Some("navigation"),
                aria_label: Some("Area Navigation"),
            },
        );
        skin.panel(
            &mut w,
            &Panel {
                id: "c2-top-bar",
                title: "",
                rect: Rect::new(98.0, 4.0, 1178.0, 40.0),
                role: Some("region"),
                aria_label: Some("Context Bar"),
            },
        );
        skin.panel(
            &mut w,
            &Panel {
                id: "ci-scorecards-panel",
                title: "CI REPOSITORY SCORECARDS",
                rect: Rect::new(98.0, 48.0, 1178.0, 668.0),
                role: Some("region"),
                aria_label: Some("CI Repository Scorecards"),
            },
        );
        skin.panel(
            &mut w,
            &Panel {
                id: "c4-card-plate",
                title: "LMA1980/ActionRunner",
                rect: Rect::new(112.0, 82.0, 276.0, 148.0),
                role: Some("group"),
                aria_label: Some("Scorecard LMA1980/ActionRunner"),
            },
        );
        skin.panel(
            &mut w,
            &Panel {
                id: "c4-card-plate",
                title: "LMA1980/MuIsolation",
                rect: Rect::new(398.0, 82.0, 276.0, 148.0),
                role: Some("group"),
                aria_label: Some("Scorecard LMA1980/MuIsolation"),
            },
        );
        let svg = w.finish_closing();
        let doc = roxmltree::Document::parse(&svg).expect("valid avionics XML");

        // 1. Every brushed filter uses finite-strike bristle gating + micro-chatter displacement
        //    and a distinct primary noise seed so strikes never align across seams.
        let mut seeds = std::collections::BTreeSet::new();
        for fid in [
            ids::BRUSHED,
            ids::BRUSHED_RAIL,
            ids::BRUSHED_BAR,
            ids::BRUSHED_SUB_A,
            ids::BRUSHED_SUB_B,
            ids::BRUSHED_SUB_C,
            ids::BRUSHED_SUB_D,
        ] {
            let filter = doc
                .descendants()
                .find(|n| n.has_tag_name("filter") && n.attribute("id") == Some(fid))
                .unwrap_or_else(|| panic!("missing filter #{fid}"));
            let turb_seed = filter
                .children()
                .find(|n| n.has_tag_name("feTurbulence") && n.attribute("result") == Some("t"))
                .and_then(|n| n.attribute("seed"))
                .expect("primary t feTurbulence seed");
            assert!(
                seeds.insert(turb_seed.to_owned()),
                "duplicate primary brush seed {turb_seed} on #{fid}"
            );
            assert!(
                filter.children().any(|n| {
                    n.has_tag_name("feComposite")
                        && n.attribute("in") == Some("hair_raw")
                        && n.attribute("in2") == Some("gate")
                        && n.attribute("operator") == Some("in")
                }),
                "filter #{fid} must gate bristle strikes to finite contact lengths"
            );
            assert!(
                filter
                    .children()
                    .any(|n| n.has_tag_name("feDisplacementMap")),
                "filter #{fid} must include micro-chatter displacement"
            );
        }

        // 2. Carrier plate vs riveted sub-plates use distinct metal stock gradients,
        //    distinct brush filters, and distinct fasteners (cabinet holes vs domed rivets).
        let carrier = doc
            .descendants()
            .find(|n| n.attribute("data-panel-id") == Some("ci-scorecards-panel"))
            .expect("ci-scorecards-panel group");
        assert!(carrier.descendants().any(|n| {
            n.has_tag_name("rect")
                && n.attribute("fill") == Some("url(#mu-tungsten)")
                && n.attribute("filter") == Some("url(#mu-brushed)")
        }));
        let carrier_title = carrier
            .descendants()
            .find(|n| n.has_tag_name("text"))
            .expect("carrier title");
        let carrier_title_x: f64 = carrier_title
            .attribute("x")
            .and_then(|v| v.parse().ok())
            .expect("carrier title x");
        assert!(
            carrier_title_x >= 98.0 + 16.0,
            "carrier title x={carrier_title_x} must clear corner fastener at x=107"
        );

        let cards: Vec<_> = doc
            .descendants()
            .filter(|n| n.attribute("data-panel-id") == Some("c4-card-plate"))
            .collect();
        assert_eq!(cards.len(), 2);
        let mut card_filters = Vec::new();
        for card in &cards {
            let brushed_rect = card
                .descendants()
                .find(|n| {
                    n.has_tag_name("rect") && n.attribute("fill") == Some("url(#mu-tungsten-sub)")
                })
                .expect("riveted sub-plate must use secondary alloy gradient #mu-tungsten-sub");
            let flt = brushed_rect
                .attribute("filter")
                .expect("sub-plate brush filter")
                .to_owned();
            assert!(
                flt.starts_with("url(#mu-brushed-sub-"),
                "sub-plate must use dedicated sub-plate brush filter, got {flt}"
            );
            card_filters.push(flt);
            assert!(
                !card
                    .descendants()
                    .any(|n| n.has_tag_name("circle") && n.attribute("fill") == Some("#050505")),
                "riveted sub-plate must not punch open #050505 cabinet holes through the backing plate"
            );
            assert!(
                card.descendants()
                    .any(|n| n.has_tag_name("circle")
                        && n.attribute("fill") == Some("url(#mu-steel)")),
                "riveted sub-plate must render domed steel rivets"
            );
        }
        assert_ne!(
            card_filters[0], card_filters[1],
            "adjacent riveted sub-plates must use distinct brush filter variants"
        );
    }
}
