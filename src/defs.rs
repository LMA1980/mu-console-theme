// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! Shared `<defs>`: filters, gradients, patterns and LCD glyph symbols.
//!
//! Instruments reference these by id (see [`ids`]); write them once per root
//! `<svg>` with [`write_defs`]. Filters for the CRT pipeline are only emitted
//! when the resolved [`Effects`] need them.

use crate::lcd::{self, GLYPHS, GLYPH_COLS, GLYPH_ROWS};
use crate::svg::{num, SvgWriter};
use crate::tokens::{self, Effects, Phosphor, CRT_CD, CRT_CI};

/// Element ids used by the instrument library.
pub mod ids {
    pub const WOODGRAIN: &str = "mu-woodgrain";
    pub const BRUSHED: &str = "mu-brushed";
    pub const BRUSHED_RAIL: &str = "mu-brushed-rail";
    pub const BRUSHED_BAR: &str = "mu-brushed-bar";
    pub const BRUSHED_SUB_A: &str = "mu-brushed-sub-a";
    pub const BRUSHED_SUB_B: &str = "mu-brushed-sub-b";
    pub const BRUSHED_SUB_C: &str = "mu-brushed-sub-c";
    pub const BRUSHED_SUB_D: &str = "mu-brushed-sub-d";
    pub const BRUSHED_SUB_VARIANTS: [&str; 4] =
        [BRUSHED_SUB_A, BRUSHED_SUB_B, BRUSHED_SUB_C, BRUSHED_SUB_D];
    pub const SHEEN: &str = "mu-sheen";
    pub const SHEEN_SUB: &str = "mu-sheen-sub";
    pub const TUNGSTEN: &str = "mu-tungsten";
    pub const TUNGSTEN_SUB: &str = "mu-tungsten-sub";
    pub const STEEL: &str = "mu-steel";
    pub const PARKERIZED: &str = "mu-park";
    pub const RUBBER: &str = "mu-rubber";
    pub const BAKELITE: &str = "mu-bakelite";
    pub const ESCUTCHEON: &str = "mu-esc";
    pub const LENS_RED: &str = "mu-lens-red";
    pub const LENS_GREEN: &str = "mu-lens-green";
    pub const LENS_AMBER: &str = "mu-lens-amber";
    pub const LENS_OFF: &str = "mu-lens-off";
    pub const LCD_BACKLIGHT: &str = "mu-lcdbg";
    pub const CRT_CI_GLASS: &str = "mu-crt-ci";
    pub const CRT_CD_GLASS: &str = "mu-crt-cd";
    pub const VIGNETTE: &str = "mu-vignette";
    pub const GLASS: &str = "mu-glass";
    pub const SCANLINES: &str = "mu-scan";
    pub const PHOSPHOR: &str = "mu-phos";
    pub const LAMP_GLOW: &str = "mu-lamp";
    pub const LCD_GHOST: &str = "mu-lcd-ghost";
}

/// `url(#id)` reference.
#[must_use]
pub fn url(id: &str) -> String {
    format!("url(#{id})")
}

/// Symbol id of an LCD glyph (`mu-lcd-41` for `A`).
#[must_use]
pub fn lcd_symbol_id(c: char) -> String {
    format!("mu-lcd-{:02x}", u32::from(c))
}

/// Glass gradient id for a phosphor.
#[must_use]
pub fn crt_glass_id(p: &Phosphor) -> &'static str {
    if p.name == CRT_CD.name {
        ids::CRT_CD_GLASS
    } else {
        ids::CRT_CI_GLASS
    }
}

type Stop<'a> = (f64, &'a str, Option<f64>);

fn stops(w: &mut SvgWriter, list: &[Stop<'_>]) {
    for &(offset, color, opacity) in list {
        w.tag("stop")
            .attr("offset", offset)
            .attr("stop-color", color)
            .attr_opt("stop-opacity", opacity)
            .end();
    }
}

fn linear(w: &mut SvgWriter, id: &str, dir: (f64, f64, f64, f64), list: &[Stop<'_>]) {
    w.tag("linearGradient")
        .attr("id", id)
        .attr("x1", dir.0)
        .attr("y1", dir.1)
        .attr("x2", dir.2)
        .attr("y2", dir.3)
        .open();
    stops(w, list);
    w.close();
}

fn radial(w: &mut SvgWriter, id: &str, c: (f64, f64, f64), list: &[Stop<'_>]) {
    w.tag("radialGradient")
        .attr("id", id)
        .attr("cx", c.0)
        .attr("cy", c.1)
        .attr("r", c.2)
        .open();
    stops(w, list);
    w.close();
}

fn turbulence(w: &mut SvgWriter, freq: (f64, f64), octaves: u32, seed: u32, result: &str) {
    w.tag("feTurbulence")
        .attr("type", "fractalNoise")
        .attr("baseFrequency", format!("{} {}", num(freq.0), num(freq.1)))
        .attr("numOctaves", octaves)
        .attr("seed", seed)
        .attr("result", result)
        .end();
}

fn color_matrix(w: &mut SvgWriter, input: &str, values: &str, result: &str) {
    w.tag("feColorMatrix")
        .attr("in", input)
        .attr("type", "matrix")
        .attr("values", values)
        .attr("result", result)
        .end();
}

fn merge(w: &mut SvgWriter, inputs: &[&str], result: Option<&str>) {
    w.tag("feMerge").attr_opt("result", result).open();
    for i in inputs {
        w.tag("feMergeNode").attr("in", *i).end();
    }
    w.close();
}

fn full_filter(w: &mut SvgWriter, id: &str) {
    w.tag("filter")
        .attr("id", id)
        .attr("x", "0")
        .attr("y", "0")
        .attr("width", "100%")
        .attr("height", "100%")
        .open();
}

fn glow_filter(w: &mut SvgWriter, id: &str, std_dev: f64, pad_pct: u32) {
    let pad = format!("-{pad_pct}%");
    let size = format!("{}%", 100 + 2 * pad_pct);
    w.tag("filter")
        .attr("id", id)
        .attr("x", pad.as_str())
        .attr("y", pad.as_str())
        .attr("width", size.as_str())
        .attr("height", size.as_str())
        .open();
    w.tag("feGaussianBlur")
        .attr("stdDeviation", std_dev)
        .attr("result", "b")
        .end();
    merge(w, &["b", "SourceGraphic"], None);
    w.close();
}

/// Cabinet-scale wood veneer filter (fine long grain, broad figure, pores).
fn wood_filter(w: &mut SvgWriter) {
    use tokens::wood::{FIGURE_FREQ, GRAIN_FREQ};
    full_filter(w, ids::WOODGRAIN);
    turbulence(w, GRAIN_FREQ, 4, 11, "n");
    color_matrix(
        w,
        "n",
        "0 0 0 0 0.25  0 0 0 0 0.15  0 0 0 0 0.085  2.0 0 0 0 -0.88",
        "grain",
    );
    turbulence(w, FIGURE_FREQ, 2, 4, "f");
    color_matrix(
        w,
        "f",
        "0 0 0 0 0.20  0 0 0 0 0.12  0 0 0 0 0.07  1.2 0 0 0 -0.45",
        "figure",
    );
    turbulence(w, (0.035, 1.1), 1, 9, "p");
    color_matrix(
        w,
        "p",
        "0 0 0 0 0.02  0 0 0 0 0.01  0 0 0 0 0.005  3.0 0 0 0 -1.75",
        "pores",
    );
    merge(w, &["figure", "grain", "pores"], None);
    w.close();
}

struct BrushPieceSpec {
    id: &'static str,
    hair_freq: (f64, f64),
    hair_seed: u32,
    hair_matrix: &'static str,
    gate_freq: (f64, f64),
    gate_seed: u32,
    counter_freq: (f64, f64),
    counter_seed: u32,
    counter_matrix: &'static str,
    chatter_freq: (f64, f64),
    chatter_seed: u32,
    chatter_scale: f64,
}

fn write_brushed_piece_filter(w: &mut SvgWriter, s: &BrushPieceSpec) {
    full_filter(w, s.id);
    // 1. Primary satin bristle grooves (crisp hairline pitch, finite horizontal strike correlation).
    turbulence(w, s.hair_freq, 3, s.hair_seed, "t");
    color_matrix(w, "t", s.hair_matrix, "hair_raw");
    // 2. Staggered bristle-strike contact envelope so individual strikes start/stop independently
    //    on every row rather than forming continuous ruler-straight horizontal stripes.
    turbulence(w, s.gate_freq, 2, s.gate_seed, "g");
    color_matrix(
        w,
        "g",
        "0 0 0 0 1  0 0 0 0 1  0 0 0 0 1  2.2 0 0 0 -0.45",
        "gate",
    );
    w.tag("feComposite")
        .attr("in", "hair_raw")
        .attr("in2", "gate")
        .attr("operator", "in")
        .attr("result", "hair")
        .end();
    // 3. Shorter overlapping counter-strikes at an independent pitch and seed.
    turbulence(w, s.counter_freq, 2, s.counter_seed, "t2");
    color_matrix(w, "t2", s.counter_matrix, "dark");
    merge(w, &["hair", "dark"], Some("m"));
    // 4. Subtle rotary/belt bristle micro-wander to break up rigid horizontal scanline alignment.
    turbulence(w, s.chatter_freq, 2, s.chatter_seed, "chatter");
    w.tag("feDisplacementMap")
        .attr("in", "m")
        .attr("in2", "chatter")
        .attr("scale", s.chatter_scale)
        .attr("xChannelSelector", "R")
        .attr("yChannelSelector", "G")
        .attr("result", "warped")
        .end();
    w.tag("feComposite")
        .attr("in", "warped")
        .attr("in2", "SourceGraphic")
        .attr("operator", "in")
        .attr("result", "sc")
        .end();
    merge(w, &["SourceGraphic", "sc"], None);
    w.close();
}

/// Multi-workpiece satin-brushed metal filters: separate chassis and riveted
/// sub-plate profiles with independent seeds, bristle strike lengths, and
/// micro-chatter so brush strikes never align across riveted metal seams.
fn brushed_filter(w: &mut SvgWriter) {
    use tokens::tungsten::{
        CHATTER_FREQ, CHATTER_SCALE, COUNTER_FREQ, HAIR_FREQ, SUB_CHATTER_FREQ, SUB_CHATTER_SCALE,
        SUB_COUNTER_FREQ, SUB_HAIR_FREQ,
    };
    let chassis_hair = "0 0 0 0 0.78  0 0 0 0 0.80  0 0 0 0 0.83  0.30 0 0 0 -0.11";
    let chassis_dark = "0 0 0 0 0  0 0 0 0 0  0 0 0 0 0  0.24 0 0 0 -0.09";
    let sub_hair = "0 0 0 0 0.86  0 0 0 0 0.90  0 0 0 0 0.96  0.32 0 0 0 -0.12";
    let sub_dark = "0 0 0 0 0.04  0 0 0 0 0.05  0 0 0 0 0.07  0.26 0 0 0 -0.10";

    let specs = [
        BrushPieceSpec {
            id: ids::BRUSHED,
            hair_freq: HAIR_FREQ,
            hair_seed: 13,
            hair_matrix: chassis_hair,
            gate_freq: (0.065, 0.65),
            gate_seed: 29,
            counter_freq: COUNTER_FREQ,
            counter_seed: 43,
            counter_matrix: chassis_dark,
            chatter_freq: CHATTER_FREQ,
            chatter_seed: 59,
            chatter_scale: CHATTER_SCALE,
        },
        BrushPieceSpec {
            id: ids::BRUSHED_RAIL,
            hair_freq: (0.022, 1.24),
            hair_seed: 197,
            hair_matrix: chassis_hair,
            gate_freq: (0.074, 0.68),
            gate_seed: 211,
            counter_freq: (0.039, 0.98),
            counter_seed: 223,
            counter_matrix: chassis_dark,
            chatter_freq: (0.038, 0.24),
            chatter_seed: 229,
            chatter_scale: 0.68,
        },
        BrushPieceSpec {
            id: ids::BRUSHED_BAR,
            hair_freq: (0.020, 1.19),
            hair_seed: 239,
            hair_matrix: chassis_hair,
            gate_freq: (0.069, 0.62),
            gate_seed: 251,
            counter_freq: (0.036, 0.95),
            counter_seed: 263,
            counter_matrix: chassis_dark,
            chatter_freq: (0.036, 0.21),
            chatter_seed: 271,
            chatter_scale: 0.62,
        },
        BrushPieceSpec {
            id: ids::BRUSHED_SUB_A,
            hair_freq: SUB_HAIR_FREQ,
            hair_seed: 71,
            hair_matrix: sub_hair,
            gate_freq: (0.095, 0.78),
            gate_seed: 83,
            counter_freq: SUB_COUNTER_FREQ,
            counter_seed: 97,
            counter_matrix: sub_dark,
            chatter_freq: SUB_CHATTER_FREQ,
            chatter_seed: 109,
            chatter_scale: SUB_CHATTER_SCALE,
        },
        BrushPieceSpec {
            id: ids::BRUSHED_SUB_B,
            hair_freq: (0.036, 1.48),
            hair_seed: 113,
            hair_matrix: sub_hair,
            gate_freq: (0.102, 0.82),
            gate_seed: 127,
            counter_freq: (0.062, 1.14),
            counter_seed: 137,
            counter_matrix: sub_dark,
            chatter_freq: (0.052, 0.30),
            chatter_seed: 149,
            chatter_scale: 0.78,
        },
        BrushPieceSpec {
            id: ids::BRUSHED_SUB_C,
            hair_freq: (0.029, 1.36),
            hair_seed: 157,
            hair_matrix: sub_hair,
            gate_freq: (0.089, 0.74),
            gate_seed: 167,
            counter_freq: (0.052, 1.04),
            counter_seed: 173,
            counter_matrix: sub_dark,
            chatter_freq: (0.044, 0.26),
            chatter_seed: 181,
            chatter_scale: 0.72,
        },
        BrushPieceSpec {
            id: ids::BRUSHED_SUB_D,
            hair_freq: (0.038, 1.52),
            hair_seed: 191,
            hair_matrix: sub_hair,
            gate_freq: (0.108, 0.85),
            gate_seed: 277,
            counter_freq: (0.066, 1.18),
            counter_seed: 283,
            counter_matrix: sub_dark,
            chatter_freq: (0.055, 0.32),
            chatter_seed: 293,
            chatter_scale: 0.80,
        },
    ];
    for spec in &specs {
        write_brushed_piece_filter(w, spec);
    }
}

fn materials(w: &mut SvgWriter) {
    use tokens::hardware as hw;
    let white = "#fff";
    linear(
        w,
        ids::SHEEN,
        (0.0, 0.0, 1.0, 0.0),
        &[
            (0.0, white, Some(0.0)),
            (0.42, white, Some(0.045)),
            (0.5, white, Some(tokens::tungsten::SHEEN_PEAK)),
            (0.58, white, Some(0.045)),
            (1.0, white, Some(0.0)),
        ],
    );
    linear(
        w,
        ids::SHEEN_SUB,
        (0.0, 0.0, 1.0, 0.22),
        &[
            (0.0, white, Some(0.0)),
            (0.24, white, Some(0.05)),
            (0.34, white, Some(tokens::tungsten::SHEEN_PEAK)),
            (0.46, white, Some(0.035)),
            (1.0, white, Some(0.0)),
        ],
    );
    linear(
        w,
        ids::TUNGSTEN,
        (0.0, 0.0, 0.0, 1.0),
        &[
            (0.0, tokens::tungsten::TOP, None),
            (1.0, tokens::tungsten::BOTTOM, None),
        ],
    );
    linear(
        w,
        ids::TUNGSTEN_SUB,
        (0.0, 0.0, 0.25, 1.0),
        &[
            (0.0, tokens::tungsten::SUB_TOP, None),
            (0.55, "#2b3038", None),
            (1.0, tokens::tungsten::SUB_BOTTOM, None),
        ],
    );
    linear(
        w,
        ids::STEEL,
        (0.0, 0.0, 1.0, 0.0),
        &[
            (0.0, hw::STEEL_DARK, None),
            (0.4, hw::STEEL_LIGHT, None),
            (0.62, "#8a8e94", None),
            (1.0, "#3a3d42", None),
        ],
    );
    linear(
        w,
        ids::PARKERIZED,
        (0.0, 0.0, 0.0, 1.0),
        &[
            (0.0, hw::PARKERIZED_TOP, None),
            (0.5, "#26282b", None),
            (1.0, hw::PARKERIZED_BOTTOM, None),
        ],
    );
    radial(
        w,
        ids::RUBBER,
        (0.35, 0.3, 0.8),
        &[
            (0.0, "#3a3a3a", None),
            (0.6, hw::RUBBER, None),
            (1.0, "#0a0a0a", None),
        ],
    );
    radial(
        w,
        ids::BAKELITE,
        (0.35, 0.3, 0.85),
        &[
            (0.0, tokens::switch::BAKELITE_HIGHLIGHT, None),
            (0.35, "#1c1c1e", None),
            (1.0, tokens::switch::BAKELITE_SHADOW, None),
        ],
    );
    linear(
        w,
        ids::ESCUTCHEON,
        (0.0, 0.0, 0.0, 1.0),
        &[
            (0.0, tokens::switch::ESCUTCHEON_TOP, None),
            (1.0, tokens::switch::ESCUTCHEON_BOTTOM, None),
        ],
    );
    for (id, hi, mid, lo) in [
        (ids::LENS_RED, "#ffd0c8", tokens::indicator::RED, "#7a0d08"),
        (
            ids::LENS_GREEN,
            "#d8ffe0",
            tokens::indicator::GREEN,
            "#0b5a22",
        ),
        (
            ids::LENS_AMBER,
            "#fff0c8",
            tokens::indicator::AMBER,
            "#7a4a08",
        ),
    ] {
        radial(
            w,
            id,
            (0.4, 0.35, 0.7),
            &[(0.0, hi, None), (0.35, mid, None), (1.0, lo, None)],
        );
    }
    radial(
        w,
        ids::LENS_OFF,
        (0.4, 0.35, 0.7),
        &[
            (0.0, "#5a4a48", None),
            (0.5, "#2a1e1d", None),
            (1.0, "#100b0b", None),
        ],
    );
    linear(
        w,
        ids::LCD_BACKLIGHT,
        (0.0, 0.0, 0.0, 1.0),
        &[
            (0.0, tokens::lcd::BACKLIGHT_TOP, None),
            (1.0, tokens::lcd::BACKLIGHT_BOTTOM, None),
        ],
    );
    for p in [CRT_CI, CRT_CD] {
        radial(
            w,
            crt_glass_id(&p),
            (0.5, 0.5, 0.75),
            &[(0.0, p.glass_center, None), (1.0, p.glass_edge, None)],
        );
    }
    radial(
        w,
        ids::VIGNETTE,
        (0.5, 0.5, 0.72),
        &[(0.6, "#000", Some(0.0)), (1.0, "#000", Some(0.75))],
    );
    linear(
        w,
        ids::GLASS,
        (0.0, 0.0, 1.0, 1.0),
        &[(0.0, white, Some(0.09)), (0.35, white, Some(0.0))],
    );
}

fn scanlines(w: &mut SvgWriter) {
    use tokens::crt::{SCANLINE_HEIGHT, SCANLINE_OPACITY, SCANLINE_PITCH};
    w.tag("pattern")
        .attr("id", ids::SCANLINES)
        .attr("width", SCANLINE_PITCH)
        .attr("height", SCANLINE_PITCH)
        .attr("patternUnits", "userSpaceOnUse")
        .open();
    w.tag("rect")
        .attr("width", SCANLINE_PITCH)
        .attr("height", SCANLINE_HEIGHT)
        .attr("fill", "#000")
        .attr("opacity", SCANLINE_OPACITY)
        .end();
    w.close();
}

fn lcd_symbols(w: &mut SvgWriter) {
    let viewbox = format!("0 0 {GLYPH_COLS} {GLYPH_ROWS}");
    for (c, g) in GLYPHS {
        if *c == ' ' {
            continue;
        }
        w.tag("symbol")
            .attr("id", lcd_symbol_id(*c))
            .attr("viewBox", viewbox.as_str())
            .attr("overflow", "visible")
            .open();
        for (col, row) in lcd::dots(g) {
            dot(w, col, row);
        }
        w.close();
    }
    w.tag("symbol")
        .attr("id", ids::LCD_GHOST)
        .attr("viewBox", viewbox.as_str())
        .open();
    for row in 0..GLYPH_ROWS {
        for col in 0..GLYPH_COLS {
            dot(w, col, row);
        }
    }
    w.close();
}

fn dot(w: &mut SvgWriter, col: usize, row: usize) {
    w.tag("rect")
        .attr("x", col)
        .attr("y", row)
        .attr("width", tokens::lcd::DOT_FILL)
        .attr("height", tokens::lcd::DOT_FILL)
        .end();
}

/// Write the shared `<defs>` block.
pub fn write_defs(w: &mut SvgWriter, effects: Effects) {
    w.tag("defs").open();
    wood_filter(w);
    brushed_filter(w);
    materials(w);
    if effects.scanlines {
        scanlines(w);
    }
    if effects.bloom {
        glow_filter(w, ids::PHOSPHOR, tokens::crt::BLOOM_STD_DEV, 20);
    }
    // The lamp glow is the only filter kept in Lite mode (L1 layer).
    glow_filter(w, ids::LAMP_GLOW, 3.0, 60);
    lcd_symbols(w);
    w.close();
}
