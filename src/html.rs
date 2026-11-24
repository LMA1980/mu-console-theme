// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! HTML5 void-aware writer ([`HtmlWriter`]) and [`HtmlCoreSkin`] semantic
//! HTML+CSS renderer for [`ConsoleScene`] and [`DialogModel`].
//!
//! [`HtmlWriter`] distinguishes HTML5 void elements (`<input>`, `<br>`,
//! `<hr>`, `<meta>`, `<link>`, `<img ...>`, etc.) from normal container
//! elements (`<div>`, `<button>`, `<section>`, `<table>`, `<dialog>`) so that
//! generated markup is valid HTML5 without self-closing trailing slashes or
//! stray closing tags on void elements.

use std::fmt;

use crate::scene::{
    ActionControl, ConsoleScene, DialogModel, GateChecklist, InviteCreatedOnceDialog, KeyValueList,
    LogViewer, Pagination, PasskeyEnrollmentDialog, PlatformPanel, RiskConfirmDialog, SceneBanner,
    SceneComponent, SceneRenderer, SceneSection, SceneViewport, SessionRolesDialog, SparklineCard,
    StatusMatrix, TableGrid, TrainwayGraph, UserProfileDialog,
};
use crate::skin::{SemanticTone, SignalGlyph};
use crate::svg::{escape_into, num, sparkline_path, AttrValue, PathData, SvgWriter};
use crate::tokens::wireframe as pal;

/// Canonical HTML5 void element tag names (matched ASCII case-insensitively).
pub const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

/// Return `true` when `tag_name` is an HTML5 void element that cannot have children or `</tag>`.
#[must_use]
pub fn is_void_element(tag_name: &str) -> bool {
    VOID_ELEMENTS
        .iter()
        .any(|void| void.eq_ignore_ascii_case(tag_name))
}

/// Errors returned by [`HtmlWriter::finish`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlError {
    /// Document finished with one or more unclosed non-void tags.
    UnclosedTag { name: &'static str },
    /// [`HtmlWriter::close`] was called when no tag was open.
    UnmatchedClose,
    /// A void element (`input`, `br`, `hr`, ...) was opened for children or given text/close tag.
    VoidElementWithChildren { name: &'static str },
    /// [`HtmlTag::void`] was called on a non-void element such as `div` or `dialog`.
    NonVoidUsedAsVoid { name: &'static str },
}

impl fmt::Display for HtmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnclosedTag { name } => write!(f, "unclosed HTML element <{name}>"),
            Self::UnmatchedClose => write!(f, "close() called with no open HTML element"),
            Self::VoidElementWithChildren { name } => {
                write!(
                    f,
                    "HTML5 void element <{name}> cannot wrap children or text"
                )
            }
            Self::NonVoidUsedAsVoid { name } => {
                write!(f, "non-void HTML5 element <{name}> cannot end via void()")
            }
        }
    }
}

impl std::error::Error for HtmlError {}

/// Deterministic streaming HTML5 writer.
#[derive(Debug, Default)]
pub struct HtmlWriter {
    buf: String,
    stack: Vec<&'static str>,
    err: Option<HtmlError>,
}

impl HtmlWriter {
    /// Construct an empty [`HtmlWriter`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Begin emitting a tag `<name`. Finish the start tag via [`HtmlTag::void`],
    /// [`HtmlTag::open`], [`HtmlTag::text`], or [`HtmlTag::empty`].
    pub fn tag(&mut self, name: &'static str) -> HtmlTag<'_> {
        self.buf.push('<');
        self.buf.push_str(name);
        HtmlTag { w: self, name }
    }

    /// Close the innermost open non-void element (`</name>`).
    pub fn close(&mut self) {
        if let Some(name) = self.stack.pop() {
            self.buf.push_str("</");
            self.buf.push_str(name);
            self.buf.push('>');
        } else if self.err.is_none() {
            self.err = Some(HtmlError::UnmatchedClose);
        }
    }

    /// Append escaped text content.
    pub fn text(&mut self, s: &str) {
        escape_into(&mut self.buf, s);
    }

    /// Append an already-validated inline `<svg>` fragment emitted by [`SvgWriter`].
    pub fn raw_svg(&mut self, svg_fragment: &str) {
        self.buf.push_str(svg_fragment);
    }

    /// Current open tag nesting depth.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Validate balanced elements and HTML5 void rules, returning the serialized HTML.
    pub fn finish(self) -> Result<String, HtmlError> {
        if let Some(err) = self.err {
            return Err(err);
        }
        if let Some(&name) = self.stack.last() {
            return Err(HtmlError::UnclosedTag { name });
        }
        Ok(self.buf)
    }

    /// Close all still-open elements and return the HTML string when error-free.
    pub fn finish_closing(mut self) -> Result<String, HtmlError> {
        while !self.stack.is_empty() {
            self.close();
        }
        self.finish()
    }

    fn record_error(&mut self, err: HtmlError) {
        if self.err.is_none() {
            self.err = Some(err);
        }
    }
}

/// Start-tag builder returned by [`HtmlWriter::tag`].
#[must_use = "must close the start tag via void(), open(), empty(), or text()"]
pub struct HtmlTag<'a> {
    w: &'a mut HtmlWriter,
    name: &'static str,
}

impl<'a> HtmlTag<'a> {
    /// Emit ` key="escaped_value"`.
    pub fn attr(self, key: &str, value: impl AttrValue) -> Self {
        self.w.buf.push(' ');
        self.w.buf.push_str(key);
        self.w.buf.push_str("=\"");
        value.write_attr(&mut self.w.buf);
        self.w.buf.push('"');
        self
    }

    /// Emit ` key="value"` only when `value` is `Some(_)`.
    pub fn attr_opt<V: AttrValue>(self, key: &str, value: Option<V>) -> Self {
        if let Some(v) = value {
            self.attr(key, v)
        } else {
            self
        }
    }

    /// Emit an HTML5 boolean attribute (` disabled`, ` open`, ` required`) when `present` is true.
    pub fn bool_attr(self, key: &str, present: bool) -> Self {
        if present {
            self.w.buf.push(' ');
            self.w.buf.push_str(key);
        }
        self
    }

    /// Finish an HTML5 void element (`>` with NO `/>` and NO closing tag).
    pub fn void(self) {
        if !is_void_element(self.name) {
            self.w
                .record_error(HtmlError::NonVoidUsedAsVoid { name: self.name });
        }
        self.w.buf.push('>');
    }

    /// Finish an empty non-void element (`></tag>`).
    pub fn empty(self) {
        if is_void_element(self.name) {
            self.w
                .record_error(HtmlError::VoidElementWithChildren { name: self.name });
            self.w.buf.push('>');
            return;
        }
        self.w.buf.push_str("></");
        self.w.buf.push_str(self.name);
        self.w.buf.push('>');
    }

    /// Open a non-void element (`>`) and push it onto the close stack.
    pub fn open(self) {
        if is_void_element(self.name) {
            self.w
                .record_error(HtmlError::VoidElementWithChildren { name: self.name });
            self.w.buf.push('>');
            return;
        }
        self.w.buf.push('>');
        self.w.stack.push(self.name);
    }

    /// Finish a non-void element containing only escaped text (`>text</tag>`).
    pub fn text(self, body: &str) {
        if is_void_element(self.name) {
            self.w
                .record_error(HtmlError::VoidElementWithChildren { name: self.name });
            self.w.buf.push('>');
            return;
        }
        self.w.buf.push('>');
        escape_into(&mut self.w.buf, body);
        self.w.buf.push_str("</");
        self.w.buf.push_str(self.name);
        self.w.buf.push('>');
    }
}

/// Primary semantic HTML5 + CSS skin (`data-skin="html"`).
#[derive(Debug, Clone, Copy, Default)]
pub struct HtmlCoreSkin;

impl HtmlCoreSkin {
    /// Emit `<div class="mu-console-scene" data-skin="html"...>...</div>`.
    #[must_use]
    pub fn render_scene_html(&self, scene: &ConsoleScene, viewport: SceneViewport) -> String {
        let mut w = HtmlWriter::new();
        self.write_scene(&mut w, scene, viewport);
        w.finish().unwrap_or_default()
    }

    /// Emit a `<dialog open ...>...</dialog>` modal for `dialog`.
    #[must_use]
    pub fn render_dialog_html(&self, dialog: &DialogModel) -> String {
        let mut w = HtmlWriter::new();
        self.write_dialog(&mut w, dialog);
        w.finish().unwrap_or_default()
    }

    fn write_scene(&self, w: &mut HtmlWriter, scene: &ConsoleScene, viewport: SceneViewport) {
        let vp_class = if viewport.is_phone() {
            "phone"
        } else if viewport.is_compact_landscape() {
            "compact"
        } else {
            "wide"
        };
        w.tag("div")
            .attr("class", "mu-console-scene")
            .attr("data-skin", "html")
            .attr("data-area", &scene.active_area)
            .attr_opt("data-subview", scene.active_subview.as_deref())
            .attr("data-viewport", vp_class)
            .open();

        self.write_header(w, scene);
        if !scene.subviews.is_empty() {
            self.write_subviews(w, scene);
        }

        w.tag("main")
            .attr("class", "mu-scene-main")
            .attr("id", "muconsole-main")
            .attr("tabindex", "-1")
            .open();

        for banner in &scene.banners {
            self.write_banner(w, banner);
        }

        w.tag("div").attr("class", "mu-scene-grid").open();
        for section in &scene.sections {
            self.write_section(w, section);
        }
        w.close(); // div.mu-scene-grid
        w.close(); // main

        if let Some(footer) = &scene.footer_status {
            w.tag("footer")
                .attr("class", "mu-scene-footer")
                .attr("role", "contentinfo")
                .text(footer);
        }

        w.close(); // div.mu-console-scene
    }

    fn write_header(&self, w: &mut HtmlWriter, scene: &ConsoleScene) {
        w.tag("header")
            .attr("class", "mu-scene-header")
            .attr("role", "banner")
            .open();

        w.tag("div").attr("class", "mu-scene-brand").open();
        w.tag("span")
            .attr("class", "mu-scene-title")
            .text("MuConsole");
        w.tag("span")
            .attr("class", "mu-area-pill")
            .attr("data-active-area", &scene.active_area)
            .text(&scene.area_label);
        w.close(); // div.mu-scene-brand

        if !scene.area_tabs.is_empty() {
            w.tag("nav")
                .attr("class", "mu-area-nav")
                .attr("aria-label", "Primary console areas")
                .attr("role", "navigation")
                .open();
            w.tag("div")
                .attr("class", "mu-segmented")
                .attr("role", "tablist")
                .attr("aria-label", "Console areas")
                .open();
            for tab in &scene.area_tabs {
                let ctrl = tab.as_action_control();
                let t = w
                    .tag("button")
                    .attr("type", "button")
                    .attr("role", "tab")
                    .attr("class", "mu-tab")
                    .bool_attr("disabled", ctrl.disabled);
                let mut t = ctrl.fold_attrs(t, |b, k, v| b.attr(k, v));
                if tab.active {
                    t = t.attr("data-active", "true");
                }
                t.open();
                w.tag("span").attr("class", "mu-tab-label").text(&tab.label);
                if let Some(badge) = tab.badge {
                    w.tag("span")
                        .attr("class", "mu-badge")
                        .text(&badge.to_string());
                }
                w.close();
            }
            w.close(); // div.mu-segmented
            w.close(); // nav
        }

        if let Some(repo) = &scene.repo_selector {
            w.tag("div")
                .attr("class", "mu-repo-selector")
                .attr("role", "group")
                .attr("aria-label", "Repository and submodule selector")
                .attr("data-control-id", &repo.control_id)
                .open();
            self.write_action_button(w, &repo.prev_outer, "mu-btn mu-btn-compact");
            w.tag("span").attr("class", "mu-repo-label").text(&format!(
                "{} ({}/{})",
                repo.outer_label,
                repo.outer_index + 1,
                repo.outer_total.max(1)
            ));
            self.write_action_button(w, &repo.next_outer, "mu-btn mu-btn-compact");
            self.write_action_button(w, &repo.prev_inner, "mu-btn mu-btn-compact");
            w.tag("span")
                .attr("class", "mu-submodule-label")
                .text(&format!(
                    "{} ({}/{})",
                    repo.inner_label,
                    repo.inner_index + 1,
                    repo.inner_total.max(1)
                ));
            self.write_action_button(w, &repo.next_inner, "mu-btn mu-btn-compact");
            w.close();
        }

        if !scene.status_lamps.is_empty() {
            w.tag("div")
                .attr("class", "mu-status-lamps")
                .attr("role", "status")
                .attr("aria-label", "System status indicators")
                .open();
            for lamp in &scene.status_lamps {
                if let Some(act) = &lamp.action {
                    self.write_action_button(w, act, "mu-lamp-btn");
                } else {
                    w.tag("span")
                        .attr("class", "mu-lamp")
                        .attr("data-control-id", &lamp.control_id)
                        .attr("data-tone", lamp.tone.as_str())
                        .attr("data-lit", if lamp.lit { "true" } else { "false" })
                        .open();
                    w.tag("span").attr("class", "mu-lamp-key").text(&lamp.label);
                    w.text(" ");
                    w.tag("span").attr("class", "mu-lamp-val").text(&lamp.value);
                    w.close();
                }
            }
            w.close();
        }

        if scene.font_toggle.is_some() || !scene.header_actions.is_empty() {
            w.tag("div").attr("class", "mu-header-actions").open();
            if let Some(font) = &scene.font_toggle {
                self.write_action_button(w, font, "mu-btn");
            }
            for act in &scene.header_actions {
                self.write_action_button(w, act, "mu-btn");
            }
            w.close();
        }

        w.close(); // header
    }

    fn write_subviews(&self, w: &mut HtmlWriter, scene: &ConsoleScene) {
        w.tag("nav")
            .attr("class", "mu-subview-nav")
            .attr("aria-label", "Sub-views")
            .attr("role", "navigation")
            .open();
        w.tag("div")
            .attr("class", "mu-segmented")
            .attr("role", "tablist")
            .attr("aria-label", "Area sub-views")
            .open();
        for sv in &scene.subviews {
            let ctrl = sv.as_action_control();
            let t = w
                .tag("button")
                .attr("type", "button")
                .attr("role", "tab")
                .attr("class", "mu-tab mu-subview-tab")
                .bool_attr("disabled", ctrl.disabled);
            let mut t = ctrl.fold_attrs(t, |b, k, v| b.attr(k, v));
            if sv.active {
                t = t.attr("data-active", "true");
            }
            t.open();
            w.tag("span").attr("class", "mu-tab-label").text(&sv.label);
            if let Some(badge) = sv.badge {
                w.tag("span")
                    .attr("class", "mu-badge")
                    .text(&badge.to_string());
            }
            w.close();
        }
        w.close();
        w.close();
    }

    fn write_banner(&self, w: &mut HtmlWriter, banner: &SceneBanner) {
        w.tag("div")
            .attr("class", "mu-banner")
            .attr("role", "alert")
            .attr("data-banner-id", &banner.id)
            .attr("data-tone", banner.tone.as_str())
            .open();
        w.tag("div").attr("class", "mu-banner-copy").open();
        w.tag("strong").text(&banner.title);
        w.tag("p").text(&banner.message);
        w.close();
        if let Some(act) = &banner.action {
            self.write_action_button(w, act, "mu-btn");
        }
        w.close();
    }

    fn write_section(&self, w: &mut HtmlWriter, section: &SceneSection) {
        let label = section.aria_label.as_deref().unwrap_or(&section.title);
        w.tag("section")
            .attr("class", "mu-section")
            .attr("data-section-id", &section.id)
            .attr_opt(
                "data-full-width",
                section.id.starts_with("sec-dash-").then_some("true"),
            )
            .attr("data-tone", section.tone.as_str())
            .attr("aria-label", label)
            .open();

        w.tag("header").attr("class", "mu-section-header").open();
        w.tag("h2").text(&section.title);
        if let Some(sub) = &section.subtitle {
            w.tag("p").attr("class", "mu-section-subtitle").text(sub);
        }
        w.close();

        for comp in &section.components {
            self.write_component(w, comp);
        }

        w.close();
    }

    fn write_component(&self, w: &mut HtmlWriter, comp: &SceneComponent) {
        match comp {
            SceneComponent::StatusMatrix(m) => self.write_status_matrix(w, m),
            SceneComponent::TableGrid(t) => self.write_table_grid(w, t),
            SceneComponent::LogViewer(l) => self.write_log_viewer(w, l),
            SceneComponent::TrainwayGraph(g) => self.write_trainway(w, g),
            SceneComponent::GateChecklist(c) => self.write_gate_checklist(w, c),
            SceneComponent::SparklineCard(s) => self.write_sparkline(w, s),
            SceneComponent::KeyValueList(kv) => self.write_kv_list(w, kv),
            SceneComponent::PlatformPanel(p) => self.write_platform_panel(w, p),
        }
    }

    fn write_status_matrix(&self, w: &mut HtmlWriter, m: &StatusMatrix) {
        w.tag("div")
            .attr("class", "mu-card mu-status-matrix")
            .attr("data-component-id", &m.id)
            .open();
        w.tag("h3").attr("class", "mu-card-title").text(&m.title);
        w.tag("div").attr("class", "mu-matrix-grid").open();
        for cell in &m.cells {
            if let Some(act) = &cell.action {
                let effective_tone = matrix_cell_effective_tone(cell.tone, act.tone);
                let tag = w
                    .tag("button")
                    .attr("type", "button")
                    .attr("class", "mu-matrix-cell mu-matrix-btn")
                    .attr("data-cell-id", &cell.id)
                    .attr("data-tone", effective_tone.as_str())
                    .bool_attr("disabled", act.disabled);
                let tag = act.fold_attrs(
                    tag,
                    |b, k, v| {
                        if k == "data-tone" {
                            b
                        } else {
                            b.attr(k, v)
                        }
                    },
                );
                tag.open();
                write_matrix_cell_inner(
                    w,
                    effective_tone,
                    cell.signal,
                    &cell.label,
                    &cell.value,
                    cell.detail.as_deref(),
                );
                w.close();
            } else {
                w.tag("div")
                    .attr("class", "mu-matrix-cell")
                    .attr("data-cell-id", &cell.id)
                    .attr("data-tone", cell.tone.as_str())
                    .open();
                write_matrix_cell_inner(
                    w,
                    cell.tone,
                    cell.signal,
                    &cell.label,
                    &cell.value,
                    cell.detail.as_deref(),
                );
                w.close();
            }
        }
        w.close();
        w.close();
    }

    fn write_table_grid(&self, w: &mut HtmlWriter, table: &TableGrid) {
        w.tag("div")
            .attr("class", "mu-card mu-table-wrap")
            .attr("data-component-id", &table.id)
            .open();
        w.tag("table").attr("class", "mu-table").open();
        w.tag("caption").text(&table.title);
        let extra_action_col = table
            .rows
            .iter()
            .any(|r| r.row_action.is_some() && r.cells.len() >= table.columns.len());
        let total_cols = (table.columns.len() + usize::from(extra_action_col)).max(1);
        w.tag("thead").open();
        w.tag("tr").open();
        for col in &table.columns {
            w.tag("th")
                .attr("scope", "col")
                .attr("data-col-key", &col.key)
                .attr_opt("data-numeric", col.numeric.then_some("true"))
                .text(&col.label);
        }
        if extra_action_col {
            w.tag("th")
                .attr("scope", "col")
                .attr("class", "mu-col-action")
                .text("Action");
        }
        w.close();
        w.close();

        w.tag("tbody").open();
        if table.rows.is_empty() {
            let span = total_cols.to_string();
            w.tag("tr").open();
            w.tag("td")
                .attr("colspan", span)
                .attr("class", "mu-table-empty")
                .text(table.empty_message.as_deref().unwrap_or("No entries"));
            w.close();
        } else {
            for row in &table.rows {
                w.tag("tr")
                    .attr("data-row-id", &row.id)
                    .attr("data-tone", row.tone.as_str())
                    .open();
                for cell in &row.cells {
                    w.tag("td")
                        .attr("data-tone", cell.tone.as_str())
                        .attr_opt("class", cell.mono.then_some("mu-mono"))
                        .open();
                    if let Some(sig) = cell.signal {
                        write_signal_span(w, sig, cell.tone);
                    }
                    if let Some(act) = &cell.action {
                        self.write_action_button(w, act, "mu-btn mu-btn-inline");
                    } else {
                        w.text(&cell.text);
                    }
                    w.close();
                }
                if let Some(row_act) = &row.row_action {
                    w.tag("td").attr("class", "mu-cell-action").open();
                    self.write_action_button(w, row_act, "mu-btn mu-btn-inline");
                    w.close();
                } else if extra_action_col && row.cells.len() <= table.columns.len() {
                    w.tag("td").attr("class", "mu-cell-action").open();
                    w.close();
                }
                w.close();
            }
        }
        w.close(); // tbody
        w.close(); // table

        if let Some(pg) = &table.pagination {
            self.write_pagination(w, pg);
        }
        w.close();
    }

    fn write_log_viewer(&self, w: &mut HtmlWriter, log: &LogViewer) {
        w.tag("div")
            .attr("class", "mu-card mu-log-viewer")
            .attr("data-component-id", &log.id)
            .attr("data-wrap", if log.wrap { "true" } else { "false" })
            .attr_opt("data-stream-id", log.stream_id.as_deref())
            .open();

        w.tag("div").attr("class", "mu-card-bar").open();
        w.tag("h3").attr("class", "mu-card-title").text(&log.title);
        if !log.controls.is_empty() {
            w.tag("div").attr("class", "mu-btn-row").open();
            for act in &log.controls {
                self.write_action_button(w, act, "mu-btn mu-btn-compact");
            }
            w.close();
        }
        w.close();

        w.tag("ol")
            .attr("class", "mu-log-lines")
            .attr("role", "log")
            .attr("aria-live", "off")
            .attr_opt("data-font-alias", log.font_alias.as_deref())
            .open();
        for entry in &log.entries {
            w.tag("li")
                .attr("class", "mu-log-line")
                .attr("data-tone", entry.level.as_str())
                .attr_opt("data-trace-id", entry.trace_id.as_deref())
                .attr_opt("data-log-key", entry.anchor_key.as_deref())
                .open();
            if let Some(ln) = entry.line_no {
                w.tag("span")
                    .attr("class", "mu-log-ln")
                    .text(&ln.to_string());
            }
            if let Some(ts) = &entry.timestamp {
                w.tag("span").attr("class", "mu-log-ts").text(ts);
            }
            if let Some(sig) = entry.signal {
                write_signal_span(w, sig, entry.level);
            }
            w.tag("code").attr("class", "mu-log-text").text(&entry.text);
            if let Some(act) = &entry.action {
                self.write_action_button(w, act, "mu-btn mu-btn-inline");
            }
            w.close();
        }
        w.close();

        if let Some(pg) = &log.pagination {
            self.write_pagination(w, pg);
        }
        w.close();
    }

    fn write_trainway(&self, w: &mut HtmlWriter, graph: &TrainwayGraph) {
        w.tag("div")
            .attr("class", "mu-card mu-trainway")
            .attr("data-component-id", &graph.id)
            .open();
        w.tag("h3")
            .attr("class", "mu-card-title")
            .text(&graph.title);

        let station_count = graph.stations.len().max(1);
        let lane_count = graph.tracks.len().max(1);
        let svg_w = 640.0;
        let svg_h = 28.0 + (lane_count as f64) * 28.0;
        let mut svg = SvgWriter::new();
        svg.tag("svg")
            .attr("class", "mu-trainway-svg")
            .attr("viewBox", format!("0 0 {} {}", num(svg_w), num(svg_h)))
            .attr("role", "img")
            .attr("aria-label", format!("{} pipeline diagram", graph.title))
            .open();

        let left_pad = 136.0;
        let usable_w = (svg_w - left_pad - 24.0).max(40.0);
        for (i, _) in graph.stations.iter().enumerate() {
            let x = if station_count <= 1 {
                left_pad + usable_w * 0.5
            } else {
                left_pad + (i as f64 / (station_count - 1) as f64) * usable_w
            };
            svg.tag("line")
                .attr("x1", x)
                .attr("y1", 12.0)
                .attr("x2", x)
                .attr("y2", svg_h - 8.0)
                .attr("stroke", pal::BORDER)
                .attr("stroke-width", 1.0)
                .end();
        }
        for (idx, track) in graph.tracks.iter().enumerate() {
            let y = 26.0 + (idx as f64) * 28.0;
            let fork_i = (track.fork_stage as usize).min(station_count.saturating_sub(1));
            let start_x = if station_count <= 1 {
                left_pad
            } else {
                left_pad + (fork_i as f64 / (station_count - 1) as f64) * usable_w
            };
            let d = PathData::new().m(start_x, y).l(svg_w - 20.0, y).build();
            let stroke = if track.node_lost {
                pal::FAIL
            } else if track.fading {
                pal::TEXT_MUTED
            } else {
                pal::BORDER
            };
            let mut path = svg
                .tag("path")
                .attr("d", d)
                .attr("fill", "none")
                .attr("stroke", stroke)
                .attr("stroke-width", 1.6);
            if track.dashed {
                path = path.attr("stroke-dasharray", "4 3");
            }
            path.end();
            for cp in &track.checkpoints {
                let ci = (cp.stage_index as usize).min(station_count.saturating_sub(1));
                let cx = if station_count <= 1 {
                    left_pad + usable_w * 0.5
                } else {
                    left_pad + (ci as f64 / (station_count - 1) as f64) * usable_w
                };
                svg.tag("circle")
                    .attr("cx", cx)
                    .attr("cy", y)
                    .attr("r", 4.5)
                    .attr("fill", tone_hex(cp.tone))
                    .end();
            }
        }
        svg.close();
        if let Ok(fragment) = svg.finish() {
            w.raw_svg(&fragment);
        }

        if !graph.stations.is_empty() {
            w.tag("ol").attr("class", "mu-trainway-stations").open();
            for st in &graph.stations {
                w.tag("li").text(st);
            }
            w.close();
        }

        w.tag("ul").attr("class", "mu-trainway-tracks").open();
        for track in &graph.tracks {
            w.tag("li")
                .attr("class", "mu-trainway-track")
                .attr("data-track-id", &track.id)
                .attr_opt("data-node-lost", track.node_lost.then_some("true"))
                .attr_opt("data-fading", track.fading.then_some("true"))
                .open();
            w.tag("code").attr("class", "mu-mono").text(&track.ref_name);
            for cp in &track.checkpoints {
                w.tag("span")
                    .attr("class", "mu-checkpoint")
                    .attr("data-tone", cp.tone.as_str())
                    .text(&format!("{} {}", cp.signal.symbol(), cp.stage_name));
            }
            if let Some(node) = &track.node_tag {
                w.tag("span").attr("class", "mu-node-tag").text(node);
            }
            if let Some(act) = &track.action {
                self.write_action_button(w, act, "mu-btn mu-btn-inline");
            }
            w.close();
        }
        w.close();
        w.close();
    }

    fn write_gate_checklist(&self, w: &mut HtmlWriter, list: &GateChecklist) {
        w.tag("div")
            .attr("class", "mu-card mu-gate-checklist")
            .attr("data-component-id", &list.id)
            .attr("data-tone", list.overall_tone.as_str())
            .open();

        w.tag("div").attr("class", "mu-card-bar").open();
        w.tag("h3").attr("class", "mu-card-title").text(&list.title);
        if let Some(env) = &list.environment {
            w.tag("span").attr("class", "mu-env-pill").text(env);
        }
        w.close();

        w.tag("ul").attr("class", "mu-gate-list").open();
        for item in &list.items {
            w.tag("li")
                .attr("class", "mu-gate-item")
                .attr("data-gate-id", &item.id)
                .attr("data-tone", item.tone.as_str())
                .attr(
                    "data-required",
                    if item.required { "true" } else { "false" },
                )
                .open();
            write_signal_span(w, item.signal, item.tone);
            w.tag("span")
                .attr("class", "mu-gate-label")
                .text(&item.label);
            w.tag("span")
                .attr("class", "mu-gate-status")
                .text(&item.status_label);
            if let Some(detail) = &item.detail {
                w.tag("span").attr("class", "mu-gate-detail").text(detail);
            }
            if let Some(act) = &item.action {
                self.write_action_button(w, act, "mu-btn mu-btn-compact");
            }
            w.close();
        }
        w.close();

        if !list.actions.is_empty() {
            w.tag("div").attr("class", "mu-gate-actions").open();
            for act in &list.actions {
                self.write_action_button(w, act, "mu-btn");
            }
            w.close();
        }
        w.close();
    }

    fn write_sparkline(&self, w: &mut HtmlWriter, card: &SparklineCard) {
        w.tag("article")
            .attr("class", "mu-card mu-sparkline-card")
            .attr("data-component-id", &card.id)
            .attr("data-tone", card.tone.as_str())
            .open();
        w.tag("h3").attr("class", "mu-card-title").text(&card.title);
        w.tag("div").attr("class", "mu-sparkline-kpi").open();
        w.tag("strong")
            .attr("class", "mu-kpi-value")
            .text(&card.metric_value);
        if let Some(unit) = &card.unit {
            w.tag("span").attr("class", "mu-kpi-unit").text(unit);
        }
        if let Some(delta) = &card.delta_label {
            w.tag("span").attr("class", "mu-kpi-delta").text(delta);
        }
        w.close();

        if !card.samples.is_empty() {
            let mut svg = SvgWriter::new();
            svg.tag("svg")
                .attr("class", "mu-sparkline-svg")
                .attr("viewBox", "0 0 160 40")
                .attr("role", "img")
                .attr("aria-label", format!("{} trend sparkline", card.title))
                .open();
            let d = sparkline_path(&card.samples, 0.0, 0.0, 160.0, 40.0, 4.0);
            svg.tag("path")
                .attr("d", d)
                .attr("fill", "none")
                .attr("stroke", tone_hex(card.tone))
                .attr("stroke-width", 1.8)
                .end();
            svg.close();
            if let Ok(fragment) = svg.finish() {
                w.raw_svg(&fragment);
            }
        }

        if let Some(range) = &card.range_label {
            w.tag("p").attr("class", "mu-sparkline-range").text(range);
        }
        if let Some(act) = &card.action {
            self.write_action_button(w, act, "mu-btn mu-btn-compact");
        }
        w.close();
    }

    fn write_kv_list(&self, w: &mut HtmlWriter, kv: &KeyValueList) {
        w.tag("div")
            .attr("class", "mu-card mu-kv-list")
            .attr("data-component-id", &kv.id)
            .open();
        w.tag("h3").attr("class", "mu-card-title").text(&kv.title);
        write_kv_entries(self, w, &kv.entries);
        w.close();
    }

    fn write_platform_panel(&self, w: &mut HtmlWriter, panel: &PlatformPanel) {
        w.tag("article")
            .attr("class", "mu-card mu-platform-panel")
            .attr("data-component-id", &panel.id)
            .attr("data-tone", panel.tone.as_str())
            .open();
        w.tag("h3")
            .attr("class", "mu-card-title")
            .text(&panel.title);
        if let Some(summary) = &panel.summary {
            w.tag("p").attr("class", "mu-panel-summary").text(summary);
        }
        if !panel.attributes.is_empty() {
            write_kv_entries(self, w, &panel.attributes);
        }
        if !panel.controls.is_empty() {
            w.tag("div").attr("class", "mu-panel-controls").open();
            for ctrl in &panel.controls {
                self.write_action_button(w, ctrl, "mu-btn");
            }
            w.close();
        }
        w.close();
    }

    fn write_pagination(&self, w: &mut HtmlWriter, pg: &Pagination) {
        w.tag("nav")
            .attr("class", "mu-pagination")
            .attr("aria-label", "Pagination")
            .open();
        self.write_action_button(w, &pg.prev, "mu-btn mu-btn-compact");
        let label = match pg.total_items {
            Some(total) => format!(
                "Page {}/{} ({total} total)",
                pg.current_page, pg.total_pages
            ),
            None => format!("Page {}/{}", pg.current_page, pg.total_pages),
        };
        w.tag("span").attr("class", "mu-page-info").text(&label);
        self.write_action_button(w, &pg.next, "mu-btn mu-btn-compact");
        w.close();
    }

    fn write_action_button(&self, w: &mut HtmlWriter, ctrl: &ActionControl, class_name: &str) {
        let tag = w
            .tag("button")
            .attr("type", "button")
            .attr("class", class_name)
            .bool_attr("disabled", ctrl.disabled);
        let tag = ctrl.fold_attrs(tag, |b, k, v| b.attr(k, v));
        tag.open();
        w.text(&ctrl.label);
        if let Some(badge) = &ctrl.badge {
            w.tag("span").attr("class", "mu-badge").text(badge);
        }
        w.close();
    }

    fn write_dialog(&self, w: &mut HtmlWriter, dialog: &DialogModel) {
        let title_id = format!("{}-title", dialog.dialog_id());
        w.tag("dialog")
            .bool_attr("open", true)
            .attr("class", "mu-dialog")
            .attr("id", dialog.dialog_id())
            .attr("data-dialog-kind", dialog.dialog_kind())
            .attr(
                "data-dismissible",
                if dialog.is_dismissible() {
                    "true"
                } else {
                    "false"
                },
            )
            .attr("data-tone", dialog.tone().as_str())
            .attr("role", "dialog")
            .attr("aria-modal", "true")
            .attr("aria-labelledby", &title_id)
            .open();

        w.tag("div").attr("class", "mu-dialog-header").open();
        w.tag("h2")
            .attr("id", &title_id)
            .attr("class", "mu-dialog-title")
            .text(dialog.title());
        if let Some(dismiss_ctrl) = dialog_dismiss_action(dialog) {
            self.write_action_button(w, dismiss_ctrl, "mu-btn mu-btn-close");
        }
        w.close();

        w.tag("div").attr("class", "mu-dialog-body").open();
        match dialog {
            DialogModel::PasskeyEnrollmentGate(d) => self.write_dialog_passkey(w, d),
            DialogModel::UserProfile(d) => self.write_dialog_user_profile(w, d),
            DialogModel::SessionRoles(d) => self.write_dialog_session_roles(w, d),
            DialogModel::RiskConfirm(d) => self.write_dialog_risk_confirm(w, d),
            DialogModel::InviteCreatedOnce(d) => self.write_dialog_invite(w, d),
        }
        w.close(); // div.mu-dialog-body
        w.close(); // dialog
    }

    fn write_dialog_passkey(&self, w: &mut HtmlWriter, d: &PasskeyEnrollmentDialog) {
        w.tag("p").attr("class", "mu-dialog-lead").text(&d.reason);
        w.tag("dl").attr("class", "mu-kv").open();
        write_kv_pair(w, "Account subject", &d.account_subject, true);
        write_kv_pair(w, "Enrolled passkeys", &d.passkey_count.to_string(), true);
        w.close();
        w.tag("div").attr("class", "mu-dialog-footer").open();
        self.write_action_button(w, &d.enroll_action, "mu-btn mu-btn-primary");
        let dismiss = ActionControl::new("dlg-passkey-dismiss", "Not Now", "dlg-passkey-dismiss");
        self.write_action_button(w, &dismiss, "mu-btn");
        if let Some(sign_out) = &d.sign_out_action {
            self.write_action_button(w, sign_out, "mu-btn");
        }
        w.close();
    }

    fn write_role_badge_group(
        w: &mut HtmlWriter,
        heading: &str,
        state_attr: &str,
        aria_label: &str,
        chip_class: &str,
        badges: &[crate::scene::RoleBadge],
    ) {
        if badges.is_empty() {
            return;
        }
        w.tag("div")
            .attr("class", "mu-role-group")
            .attr("data-role-state", state_attr)
            .open();
        w.tag("span")
            .attr("class", "mu-role-group-label")
            .text(heading);
        w.tag("ul")
            .attr("class", "mu-role-badges")
            .attr("aria-label", aria_label)
            .open();
        for badge in badges {
            let label = match &badge.detail {
                Some(det) if !det.is_empty() => format!("{} ({det})", badge.role),
                _ => badge.role.clone(),
            };
            w.tag("li")
                .attr("class", chip_class)
                .attr("data-tone", badge.tone.as_str())
                .text(&label);
        }
        w.close();
        w.close();
    }

    fn write_dialog_user_profile(&self, w: &mut HtmlWriter, d: &UserProfileDialog) {
        w.tag("dl").attr("class", "mu-kv").open();
        write_kv_pair(w, "Handle", &d.handle, true);
        write_kv_pair(w, "Subject", &d.subject_id, true);
        if let Some(email) = &d.email {
            write_kv_pair(w, "Email", email, false);
        }
        write_kv_pair(w, "Passkeys", &d.passkey_count.to_string(), true);
        if let Some(exp) = &d.session_expires_at {
            write_kv_pair(w, "Session expires", exp, true);
        }
        w.close();

        Self::write_role_badge_group(
            w,
            "Active Roles",
            "active",
            "Active roles",
            "mu-role-chip",
            &d.active_roles,
        );
        Self::write_role_badge_group(
            w,
            "Dormant Roles",
            "dormant",
            "Dormant roles",
            "mu-role-chip mu-role-chip-dormant",
            &d.dormant_roles,
        );
        if let Some(manage_roles) = &d.manage_roles_action {
            w.tag("div").attr("class", "mu-role-actions").open();
            self.write_action_button(w, manage_roles, "mu-btn mu-btn-primary");
            w.close();
        }

        w.tag("div").attr("class", "mu-dialog-footer").open();
        if let Some(lock) = &d.lock_action {
            self.write_action_button(w, lock, "mu-btn");
        }
        if let Some(pk) = &d.manage_passkeys_action {
            self.write_action_button(w, pk, "mu-btn");
        }
        if let Some(sign_out) = &d.sign_out_action {
            self.write_action_button(w, sign_out, "mu-btn");
        }
        w.close();
    }

    fn write_dialog_session_roles(&self, w: &mut HtmlWriter, d: &SessionRolesDialog) {
        w.tag("p").attr("class", "mu-dialog-lead").text(&format!(
            "Session roles for {} (step-up verified: {})",
            d.subject_handle,
            if d.step_up_verified { "yes" } else { "no" }
        ));

        if let Some(err) = d.error_message.as_deref().filter(|s| !s.is_empty()) {
            w.tag("div")
                .attr("class", "mu-banner mu-dialog-error")
                .attr("role", "alert")
                .attr("data-tone", "fail")
                .open();
            write_signal_span(w, SignalGlyph::BlockFailed, SemanticTone::Fail);
            w.tag("span").attr("class", "mu-banner-text").text(err);
            w.close();
        }

        w.tag("ul").attr("class", "mu-session-roles").open();
        for role in &d.roles {
            w.tag("li")
                .attr("class", "mu-role-row")
                .attr("data-tone", role.tone.as_str())
                .attr("data-active", if role.active { "true" } else { "false" })
                .attr_opt("data-disabled", role.action.disabled.then_some("true"))
                .open();
            w.tag("div").attr("class", "mu-role-meta").open();
            w.tag("strong").text(&role.role);
            w.tag("span")
                .attr("class", "mu-role-desc")
                .text(&role.description);
            if let Some(exp) = &role.expires_at {
                w.tag("span")
                    .attr("class", "mu-role-expiry")
                    .text(&format!("Expires {exp}"));
            }
            if role.requires_step_up {
                w.tag("span")
                    .attr("class", "mu-role-stepup")
                    .text("Requires WebAuthn step-up");
            }
            w.close();
            self.write_action_button(w, &role.action, "mu-btn mu-btn-compact");
            w.close();
        }
        w.close();
    }

    fn write_dialog_risk_confirm(&self, w: &mut HtmlWriter, d: &RiskConfirmDialog) {
        w.tag("p").attr("class", "mu-dialog-lead").text(&d.summary);
        w.tag("dl").attr("class", "mu-kv").open();
        write_kv_pair(w, "Target", &d.target_resource, true);
        write_kv_pair(w, "Consequence", &d.consequence, false);
        w.close();

        if let Some(phrase) = &d.confirmation_phrase {
            w.tag("div").attr("class", "mu-confirm-phrase-group").open();
            w.tag("label").attr("for", &d.phrase_input_id).open();
            w.text("Type ");
            w.tag("code").attr("class", "mu-mono").text(phrase);
            w.text(" to arm confirmation");
            w.close();
            w.tag("input")
                .attr("type", "text")
                .attr("id", &d.phrase_input_id)
                .attr("class", "mu-input")
                .attr("data-control-id", &d.phrase_input_id)
                .attr("data-action", &d.phrase_input_action)
                .attr(
                    "data-phrase-matched",
                    if d.phrase_matched { "true" } else { "false" },
                )
                .attr("value", &d.typed_confirmation)
                .attr("autocomplete", "off")
                .void();
            w.close();
        }

        w.tag("div").attr("class", "mu-dialog-footer").open();
        self.write_action_button(w, &d.cancel_action, "mu-btn");
        self.write_action_button(w, &d.confirm_action, "mu-btn mu-btn-danger");
        w.close();
    }

    fn write_dialog_invite(&self, w: &mut HtmlWriter, d: &InviteCreatedOnceDialog) {
        w.tag("div")
            .attr("class", "mu-banner")
            .attr("role", "note")
            .attr("data-tone", "warn")
            .text(&d.warning);
        w.tag("dl").attr("class", "mu-kv").open();
        write_kv_pair(w, "Invitee", &d.invitee_hint, false);
        write_kv_pair(w, "Roles", &d.assigned_roles.join(", "), true);
        write_kv_pair(w, "Expires", &d.expires_at, true);
        w.close();

        w.tag("div").attr("class", "mu-invite-secret").open();
        w.tag("code")
            .attr("class", "mu-mono mu-invite-url")
            .text(&d.invite_url);
        w.close();

        w.tag("div").attr("class", "mu-dialog-footer").open();
        self.write_action_button(w, &d.copy_action, "mu-btn mu-btn-primary");
        self.write_action_button(w, &d.close_action, "mu-btn");
        w.close();
    }
}

impl SceneRenderer for HtmlCoreSkin {
    fn skin_id(&self) -> &'static str {
        "html"
    }

    fn render_scene(&self, scene: &ConsoleScene, viewport: SceneViewport) -> String {
        self.render_scene_html(scene, viewport)
    }

    fn render_dialog(&self, dialog: &DialogModel, _viewport: SceneViewport) -> String {
        self.render_dialog_html(dialog)
    }

    fn render_scene_with_dialog(
        &self,
        scene: &ConsoleScene,
        dialog: Option<&DialogModel>,
        viewport: SceneViewport,
    ) -> String {
        let mut w = HtmlWriter::new();
        self.write_scene(&mut w, scene, viewport);
        if let Some(dlg) = dialog {
            self.write_dialog(&mut w, dlg);
        }
        w.finish().unwrap_or_default()
    }
}

fn dialog_dismiss_action(dialog: &DialogModel) -> Option<&ActionControl> {
    match dialog {
        // RiskConfirm and InviteCreatedOnce render cancel/close in their footers,
        // so omitting a duplicate header close keeps control IDs unique per dialog.
        DialogModel::PasskeyEnrollmentGate(_)
        | DialogModel::RiskConfirm(_)
        | DialogModel::InviteCreatedOnce(_) => None,
        DialogModel::UserProfile(d) => Some(&d.close_action),
        DialogModel::SessionRoles(d) => Some(&d.close_action),
    }
}

pub(crate) const fn matrix_cell_effective_tone(
    cell_tone: SemanticTone,
    action_tone: SemanticTone,
) -> SemanticTone {
    if !matches!(cell_tone, SemanticTone::Neutral) {
        cell_tone
    } else {
        action_tone
    }
}

fn write_matrix_cell_inner(
    w: &mut HtmlWriter,
    tone: SemanticTone,
    signal: Option<SignalGlyph>,
    label: &str,
    value: &str,
    detail: Option<&str>,
) {
    if let Some(sig) = signal {
        write_signal_span(w, sig, tone);
    }
    w.tag("span").attr("class", "mu-matrix-label").text(label);
    w.tag("strong").attr("class", "mu-matrix-value").text(value);
    if let Some(det) = detail {
        w.tag("span").attr("class", "mu-matrix-detail").text(det);
    }
}

fn write_signal_span(w: &mut HtmlWriter, signal: SignalGlyph, tone: SemanticTone) {
    w.tag("span")
        .attr("class", "mu-signal")
        .attr("data-tone", tone.as_str())
        .attr("aria-hidden", "true")
        .text(signal.symbol());
}

fn write_kv_entries(
    skin: &HtmlCoreSkin,
    w: &mut HtmlWriter,
    entries: &[crate::scene::KeyValueEntry],
) {
    w.tag("dl").attr("class", "mu-kv").open();
    for entry in entries {
        w.tag("div")
            .attr("class", "mu-kv-row")
            .attr("data-tone", entry.tone.as_str())
            .open();
        w.tag("dt").text(&entry.key);
        w.tag("dd")
            .attr_opt("class", entry.mono.then_some("mu-mono"))
            .open();
        w.text(&entry.value);
        if let Some(det) = &entry.detail {
            w.tag("span").attr("class", "mu-kv-detail").text(det);
        }
        if let Some(act) = &entry.action {
            skin.write_action_button(w, act, "mu-btn mu-btn-inline");
        }
        w.close();
        w.close();
    }
    w.close();
}

fn write_kv_pair(w: &mut HtmlWriter, key: &str, value: &str, mono: bool) {
    w.tag("div").attr("class", "mu-kv-row").open();
    w.tag("dt").text(key);
    w.tag("dd")
        .attr_opt("class", mono.then_some("mu-mono"))
        .text(value);
    w.close();
}

fn tone_hex(tone: SemanticTone) -> &'static str {
    match tone {
        SemanticTone::Neutral => pal::TEXT_PRIMARY,
        SemanticTone::Pass => pal::PASS,
        SemanticTone::Warn => pal::WARN,
        SemanticTone::Fail => pal::FAIL,
        SemanticTone::Info => pal::INFO,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::scene::{
        GateChecklist, GateItem, InviteCreatedOnceDialog, KeyValueEntry, KeyValueList, LogEntry,
        LogViewer, MatrixCell, NavTab, PasskeyEnrollmentDialog, PlatformPanel, RepoSelector,
        RiskConfirmDialog, RoleBadge, SceneBanner, SceneComponent, SceneSection, SessionRoleOption,
        SessionRolesDialog, SparklineCard, StatusLamp, StatusMatrix, SubViewTab, TableCell,
        TableColumn, TableGrid, TableRow, TrainwayCheckpoint, TrainwayGraph, TrainwayTrack,
        UserProfileDialog,
    };

    #[test]
    fn html_writer_distinguishes_void_and_non_void_elements() {
        assert!(is_void_element("input"));
        assert!(is_void_element("INPUT"));
        assert!(is_void_element("br"));
        assert!(is_void_element("hr"));
        assert!(!is_void_element("div"));
        assert!(!is_void_element("button"));
        assert!(!is_void_element("dialog"));

        let mut w = HtmlWriter::new();
        w.tag("div").attr("class", "wrap").open();
        w.tag("input")
            .attr("type", "text")
            .attr("value", "a&b\"<c>")
            .bool_attr("disabled", true)
            .void();
        w.tag("br").void();
        w.tag("hr").void();
        w.tag("button")
            .attr("type", "button")
            .bool_attr("disabled", true)
            .text("Go");
        w.tag("span").empty();
        w.close();
        let html = w.finish().expect("valid HTML");
        assert_eq!(
            html,
            "<div class=\"wrap\"><input type=\"text\" value=\"a&amp;b&quot;&lt;c&gt;\" disabled><br><hr><button type=\"button\" disabled>Go</button><span></span></div>"
        );
    }

    #[test]
    fn html_writer_rejects_void_children_and_non_void_void_call() {
        let mut w1 = HtmlWriter::new();
        w1.tag("input").open();
        assert_eq!(
            w1.finish(),
            Err(HtmlError::VoidElementWithChildren { name: "input" })
        );

        let mut w2 = HtmlWriter::new();
        w2.tag("dialog").void();
        assert_eq!(
            w2.finish(),
            Err(HtmlError::NonVoidUsedAsVoid { name: "dialog" })
        );

        let mut w3 = HtmlWriter::new();
        w3.tag("section").open();
        assert_eq!(w3.finish(), Err(HtmlError::UnclosedTag { name: "section" }));
    }

    pub(crate) fn sample_scene() -> ConsoleScene {
        let mut scene = ConsoleScene::new("delivery", "Delivery");
        scene.active_subview = Some("gates".into());
        scene.area_tabs = vec![
            NavTab {
                id: "overview".into(),
                control_id: "nav-ovr".into(),
                label: "Overview".into(),
                badge: None,
                tone: SemanticTone::Info,
                active: false,
                disabled: false,
            },
            NavTab {
                id: "delivery".into(),
                control_id: "nav-cd".into(),
                label: "Delivery".into(),
                badge: Some(3),
                tone: SemanticTone::Pass,
                active: true,
                disabled: false,
            },
        ];
        scene.subviews = vec![SubViewTab {
            id: "gates".into(),
            control_id: "sub-gates".into(),
            area: "delivery".into(),
            label: "Release Gates".into(),
            badge: Some(1),
            tone: SemanticTone::Warn,
            active: true,
            disabled: false,
        }];
        scene.repo_selector = Some(RepoSelector {
            control_id: "repo-rotary".into(),
            outer_label: "LMA1980/superproject".into(),
            outer_index: 0,
            outer_total: 4,
            inner_label: "mu-console-theme".into(),
            inner_index: 1,
            inner_total: 6,
            filter_query: String::new(),
            prev_outer: ActionControl::new("repo-prev", "Prev repo", "prev-repo"),
            next_outer: ActionControl::new("repo-next", "Next repo", "next-repo"),
            prev_inner: ActionControl::new("sub-prev", "Prev mod", "prev-mod"),
            next_inner: ActionControl::new("sub-next", "Next mod", "next-mod"),
        });
        scene.font_toggle = Some(
            ActionControl::new("font-toggle", "OpenDyslexic", "toggle-font").with_pressed(false),
        );
        scene.status_lamps = vec![StatusLamp {
            control_id: "lamp-cluster".into(),
            label: "Cluster".into(),
            value: "NOMINAL".into(),
            lit: true,
            tone: SemanticTone::Pass,
            action: None,
        }];
        scene.header_actions = vec![
            ActionControl::new("hdr-user", "@operator", "open-profile"),
            ActionControl::new("hdr-role", "+role", "open-session-roles"),
        ];
        scene.banners = vec![SceneBanner {
            id: "banner-readonly".into(),
            tone: SemanticTone::Warn,
            title: "Read-only observer".into(),
            message: "Step up WebAuthn session to promote releases.".into(),
            action: Some(ActionControl::new(
                "banner-roles",
                "Elevate",
                "open-session-roles",
            )),
        }];
        scene.sections = vec![
            SceneSection::new("sec-overview", "Pipeline & Telemetry")
                .with_component(SceneComponent::StatusMatrix(StatusMatrix {
                    id: "mat-nodes".into(),
                    title: "Fleet Nodes".into(),
                    cells: vec![
                        MatrixCell {
                            id: "node-a".into(),
                            label: "node-a".into(),
                            value: "99.9%".into(),
                            detail: Some("healthy".into()),
                            tone: SemanticTone::Pass,
                            signal: Some(SignalGlyph::TestPassed),
                            action: None,
                        },
                        MatrixCell {
                            id: "worker-riscv64".into(),
                            label: "worker-riscv64".into(),
                            value: "BUSY".into(),
                            detail: Some("slot 1/1".into()),
                            tone: SemanticTone::Warn,
                            signal: Some(SignalGlyph::BlockRunning),
                            action: Some(ActionControl::new(
                                "cell-worker",
                                "Inspect",
                                "inspect-node",
                            )),
                        },
                    ],
                }))
                .with_component(SceneComponent::SparklineCard(SparklineCard {
                    id: "spk-pass".into(),
                    title: "Pass Rate".into(),
                    metric_value: "98.4".into(),
                    unit: Some("%".into()),
                    delta_label: Some("+0.6%".into()),
                    tone: SemanticTone::Pass,
                    samples: vec![95.0, 96.2, 97.1, 98.4],
                    range_label: Some("last 30 runs".into()),
                    action: None,
                }))
                .with_component(SceneComponent::TrainwayGraph(TrainwayGraph {
                    id: "tw-main".into(),
                    title: "CI Trainway".into(),
                    stations: vec!["fmt".into(), "clippy".into(), "test".into(), "cd".into()],
                    tracks: vec![TrainwayTrack {
                        id: "trk-1".into(),
                        ref_name: "feat/scene".into(),
                        lane: 0,
                        fork_stage: 0,
                        dashed: false,
                        fading: false,
                        checkpoints: vec![TrainwayCheckpoint {
                            stage_index: 2,
                            stage_name: "test".into(),
                            signal: SignalGlyph::BlockPassed,
                            tone: SemanticTone::Pass,
                            label: Some("ok".into()),
                        }],
                        node_tag: Some("node-a".into()),
                        node_lost: false,
                        action: Some(ActionControl::new("trk-open", "Inspect", "open-run")),
                    }],
                })),
            SceneSection::new("sec-ops", "Gates, Logs & Platform")
                .with_component(SceneComponent::GateChecklist(GateChecklist {
                    id: "gate-cd".into(),
                    title: "Promotion Checklist".into(),
                    environment: Some("staging".into()),
                    overall_tone: SemanticTone::Warn,
                    items: vec![GateItem {
                        id: "g-smoke".into(),
                        label: "Canary smoke".into(),
                        status_label: "PASSED".into(),
                        detail: Some("42/42 assertions".into()),
                        signal: SignalGlyph::TestPassed,
                        tone: SemanticTone::Pass,
                        required: true,
                        action: None,
                    }],
                    actions: vec![ActionControl::new("cd-promote", "Promote", "promote")
                        .with_area("delivery")
                        .with_subview("gates")
                        .disabled(true)
                        .with_aria_label("Promote blocked pending approval")
                        .with_attr("data-cd-gate", "staging")],
                }))
                .with_component(SceneComponent::TableGrid(TableGrid {
                    id: "tbl-runs".into(),
                    title: "Recent Runs".into(),
                    columns: vec![
                        TableColumn {
                            key: "id".into(),
                            label: "Run".into(),
                            numeric: false,
                        },
                        TableColumn {
                            key: "dur".into(),
                            label: "Duration".into(),
                            numeric: true,
                        },
                    ],
                    rows: vec![TableRow {
                        id: "run-42".into(),
                        tone: SemanticTone::Pass,
                        cells: vec![
                            TableCell {
                                text: "#42".into(),
                                tone: SemanticTone::Pass,
                                signal: Some(SignalGlyph::BlockPassed),
                                mono: true,
                                action: None,
                            },
                            TableCell {
                                text: "14s".into(),
                                tone: SemanticTone::Neutral,
                                signal: None,
                                mono: true,
                                action: Some(ActionControl::new("run-42-open", "Open", "open-run")),
                            },
                        ],
                        row_action: Some(ActionControl::new(
                            "run-42-logs",
                            "Logs",
                            "open-run-logs",
                        )),
                    }],
                    empty_message: None,
                    pagination: Some(Pagination {
                        current_page: 1,
                        total_pages: 3,
                        total_items: Some(25),
                        prev: ActionControl::new("tbl-prev", "Prev", "page-prev").disabled(true),
                        next: ActionControl::new("tbl-next", "Next", "page-next"),
                    }),
                }))
                .with_component(SceneComponent::LogViewer(LogViewer {
                    id: "log-cd".into(),
                    title: "Deployment Log".into(),
                    font_alias: Some("MuCRT".into()),
                    wrap: false,
                    stream_id: Some("cd-42".into()),
                    entries: vec![LogEntry {
                        line_no: Some(1),
                        timestamp: Some("12:00:01Z".into()),
                        level: SemanticTone::Info,
                        signal: Some(SignalGlyph::TestRunning),
                        text: "Applying deployment manifest".into(),
                        trace_id: Some("tr-99".into()),
                        anchor_key: Some("1700000000000:abcd".into()),
                        action: Some(ActionControl::new("tr-99-btn", "Trace", "open-trace")),
                    }],
                    controls: vec![ActionControl::new("log-wrap", "Wrap", "toggle-wrap")],
                    pagination: None,
                }))
                .with_component(SceneComponent::KeyValueList(KeyValueList {
                    id: "kv-build".into(),
                    title: "Build Provenance".into(),
                    entries: vec![KeyValueEntry {
                        key: "Commit".into(),
                        value: "deadbeef".into(),
                        detail: Some("signed".into()),
                        tone: SemanticTone::Pass,
                        mono: true,
                        action: None,
                    }],
                }))
                .with_component(SceneComponent::PlatformPanel(PlatformPanel {
                    id: "plat-ctrl".into(),
                    title: "Platform Operations".into(),
                    summary: Some("Service daemon controls".into()),
                    tone: SemanticTone::Info,
                    attributes: vec![KeyValueEntry {
                        key: "Mode".into(),
                        value: "active".into(),
                        detail: None,
                        tone: SemanticTone::Pass,
                        mono: true,
                        action: None,
                    }],
                    controls: vec![ActionControl::new(
                        "plat-restart",
                        "Restart Worker",
                        "request-confirm",
                    )
                    .with_tone(SemanticTone::Warn)],
                })),
        ];
        scene.footer_status = Some("Connected".into());
        scene
    }

    #[test]
    fn html_core_skin_renders_all_eight_components_deterministically() {
        let skin = HtmlCoreSkin;
        let scene = sample_scene();
        let out_a = skin.render_scene(&scene, SceneViewport::DESKTOP);
        let out_b = skin.render_scene(&scene, SceneViewport::DESKTOP);
        assert_eq!(out_a, out_b);
        assert!(out_a.contains("data-skin=\"html\""));
        assert!(out_a.contains("<nav class=\"mu-area-nav\""));
        assert!(out_a.contains("<section class=\"mu-section\""));
        assert!(out_a.contains("<table class=\"mu-table\">"));
        assert!(out_a.contains("data-component-id=\"mat-nodes\""));
        assert!(out_a.contains("data-component-id=\"log-cd\""));
        assert!(out_a.contains("data-log-key=\"1700000000000:abcd\""));
        assert!(out_a.contains("data-component-id=\"tw-main\""));
        assert!(out_a.contains("data-component-id=\"gate-cd\""));
        assert!(out_a.contains("data-component-id=\"spk-pass\""));
        assert!(out_a.contains("data-component-id=\"kv-build\""));
        assert!(out_a.contains("data-component-id=\"plat-ctrl\""));
        assert!(out_a.contains("data-control-id=\"cd-promote\""));
        assert!(
            out_a.contains(
                r#"<span class="mu-lamp-key">Cluster</span> <span class="mu-lamp-val">NOMINAL</span>"#
            ),
            "status lamp label and value must stay separated by explicit whitespace"
        );
        assert!(out_a.contains("disabled"));
        assert!(out_a.contains("aria-disabled=\"true\""));
    }

    pub(crate) fn sample_dialogs() -> Vec<DialogModel> {
        vec![
            DialogModel::PasskeyEnrollmentGate(PasskeyEnrollmentDialog {
                dialog_id: "dlg-passkey".into(),
                title: "Passkey required".into(),
                reason: "Register a WebAuthn passkey before operating cluster nodes.".into(),
                account_subject: "usr-01".into(),
                passkey_count: 0,
                enroll_action: ActionControl::new(
                    "btn-enroll",
                    "Register Passkey",
                    "enroll-passkey",
                ),
                sign_out_action: None,
                tone: SemanticTone::Warn,
            }),
            DialogModel::UserProfile(UserProfileDialog {
                dialog_id: "dlg-profile".into(),
                handle: "@operator".into(),
                display_name: "Operator".into(),
                subject_id: "usr-01".into(),
                email: Some("ops@example.invalid".into()),
                active_roles: vec![RoleBadge {
                    role: "release-operator".into(),
                    tone: SemanticTone::Pass,
                    detail: None,
                }],
                dormant_roles: vec![RoleBadge {
                    role: "platform-admin".into(),
                    tone: SemanticTone::Info,
                    detail: Some("Timed".into()),
                }],
                session_expires_at: Some("2026-11-15T00:00:00Z".into()),
                passkey_count: 2,
                manage_roles_action: Some(ActionControl::new(
                    "btn-manage-roles",
                    "Manage Session Roles",
                    "open-session-roles",
                )),
                lock_action: Some(ActionControl::new(
                    "btn-lock",
                    "Lock Console",
                    "toggle-lock",
                )),
                manage_passkeys_action: None,
                sign_out_action: None,
                close_action: ActionControl::new("btn-close", "Close", "dismiss-dialog"),
            }),
            DialogModel::SessionRoles(SessionRolesDialog {
                dialog_id: "dlg-roles".into(),
                title: "Session Roles".into(),
                subject_handle: "@operator".into(),
                step_up_verified: true,
                roles: vec![
                    SessionRoleOption {
                        role: "platform-admin".into(),
                        description: "Manage worker units".into(),
                        active: false,
                        expires_at: Some("15m".into()),
                        requires_step_up: true,
                        tone: SemanticTone::Warn,
                        action: ActionControl::new("grant-admin", "Grant", "grant-role"),
                    },
                    SessionRoleOption {
                        role: "super-operator".into(),
                        description: "Unassigned catalog role".into(),
                        active: false,
                        expires_at: None,
                        requires_step_up: false,
                        tone: SemanticTone::Neutral,
                        action: ActionControl::new(
                            "unassigned-super",
                            "Unassigned",
                            "unassigned-role",
                        )
                        .disabled(true),
                    },
                ],
                error_message: Some("Passkey verification was cancelled.".into()),
                close_action: ActionControl::new("roles-close", "Close", "dismiss-dialog"),
            }),
            DialogModel::RiskConfirm(RiskConfirmDialog {
                dialog_id: "dlg-confirm".into(),
                title: "Restart worker".into(),
                summary: "Confirm draining active CI slot.".into(),
                target_resource: "worker-riscv64".into(),
                consequence: "In-flight builds wait for slot drain.".into(),
                confirmation_phrase: Some("RESTART worker-riscv64".into()),
                typed_confirmation: String::new(),
                phrase_matched: false,
                phrase_input_id: "confirm-phrase-input".into(),
                phrase_input_action: "update-confirm-phrase".into(),
                tone: SemanticTone::Warn,
                confirm_action: ActionControl::new("btn-confirm", "Restart", "confirm-risk")
                    .disabled(true),
                cancel_action: ActionControl::new("btn-cancel", "Cancel", "dismiss-dialog"),
            }),
            DialogModel::InviteCreatedOnce(InviteCreatedOnceDialog {
                dialog_id: "dlg-invite".into(),
                title: "Invite Created".into(),
                invitee_hint: "New Engineer".into(),
                assigned_roles: vec!["viewer".into()],
                invite_url: "https://console.example.invalid/invite/once-token".into(),
                expires_at: "2026-11-15T12:00:00Z".into(),
                warning: "Displayed once. Store securely before closing.".into(),
                copy_action: ActionControl::new("btn-copy-inv", "Copy URL", "copy-invite"),
                close_action: ActionControl::new("btn-close-inv", "Done", "dismiss-dialog"),
            }),
        ]
    }

    #[test]
    fn html_core_skin_renders_all_five_dialog_variants() {
        let skin = HtmlCoreSkin;
        let dialogs = sample_dialogs();

        let gate_html = skin.render_dialog(&dialogs[0], SceneViewport::DESKTOP);
        assert!(gate_html.contains("<dialog open"));
        assert!(gate_html.contains("data-dismissible=\"true\""));
        assert!(gate_html.contains("data-control-id=\"dlg-passkey-dismiss\""));
        assert!(!gate_html.contains("mu-btn-close"));

        let profile_html = skin.render_dialog(&dialogs[1], SceneViewport::DESKTOP);
        assert!(profile_html.contains("data-dialog-kind=\"user-profile\""));
        assert!(profile_html.contains("@operator"));
        assert!(profile_html.contains("data-control-id=\"btn-close\""));

        let roles_html = skin.render_dialog(&dialogs[2], SceneViewport::DESKTOP);
        assert!(roles_html.contains("data-dialog-kind=\"session-roles\""));
        assert!(!roles_html.contains("<input"));

        let confirm_html = skin.render_dialog(&dialogs[3], SceneViewport::DESKTOP);
        assert!(confirm_html.contains("<input type=\"text\""));
        assert!(!confirm_html.contains("</input>"));
        assert!(!confirm_html.contains("/>"));

        let invite_html = skin.render_dialog(&dialogs[4], SceneViewport::DESKTOP);
        assert!(invite_html.contains("data-dialog-kind=\"invite-created-once\""));
        assert!(invite_html.contains("once-token"));
    }

    #[test]
    fn status_matrix_buttons_emit_cell_data_tone_even_when_action_is_neutral_and_disabled() {
        let mut scene = ConsoleScene::new("dashboard", "Dashboard");
        scene.sections = vec![SceneSection::new("sec-dash", "Fleet").with_component(
            SceneComponent::StatusMatrix(StatusMatrix {
                id: "dash-cards".into(),
                title: "Subsystems".into(),
                cells: vec![
                    MatrixCell {
                        id: "card-maint".into(),
                        label: "Registry".into(),
                        value: "In Maintenance".into(),
                        detail: None,
                        tone: SemanticTone::Info,
                        signal: Some(SignalGlyph::BlockRunning),
                        action: Some(
                            ActionControl::new(
                                "dash-card:registry",
                                "In Maintenance",
                                "dash-card:registry",
                            )
                            .disabled(true),
                        ),
                    },
                    MatrixCell {
                        id: "card-offline".into(),
                        label: "Delivery".into(),
                        value: "Offline".into(),
                        detail: None,
                        tone: SemanticTone::Fail,
                        signal: Some(SignalGlyph::BlockFailed),
                        action: Some(
                            ActionControl::new(
                                "dash-card:delivery",
                                "Offline",
                                "dash-card:delivery",
                            )
                            .disabled(true),
                        ),
                    },
                    MatrixCell {
                        id: "card-live".into(),
                        label: "CI".into(),
                        value: "Live".into(),
                        detail: None,
                        tone: SemanticTone::Pass,
                        signal: Some(SignalGlyph::TestPassed),
                        action: Some(ActionControl::new("dash-card:ci", "Live", "dash-card:ci")),
                    },
                ],
            }),
        )];

        let html = HtmlCoreSkin.render_scene(&scene, SceneViewport::DESKTOP);
        assert!(html.contains(
            r#"<button type="button" class="mu-matrix-cell mu-matrix-btn" data-cell-id="card-maint" data-tone="info" disabled data-control-id="dash-card:registry""#
        ));
        assert!(html.contains(
            r#"<button type="button" class="mu-matrix-cell mu-matrix-btn" data-cell-id="card-offline" data-tone="fail" disabled data-control-id="dash-card:delivery""#
        ));
        assert!(html.contains(
            r#"<button type="button" class="mu-matrix-cell mu-matrix-btn" data-cell-id="card-live" data-tone="pass" data-control-id="dash-card:ci""#
        ));
        // Ensure data-tone is not duplicated or left neutral from ActionControl defaults.
        for btn_tag in html.split('<').filter(|t| t.starts_with("button ")) {
            let open_end = btn_tag.find('>').expect("button closing bracket");
            let open_tag = &btn_tag[..open_end];
            assert_eq!(open_tag.matches("data-tone=").count(), 1, "{open_tag}");
            assert!(!open_tag.contains(r#"data-tone="neutral""#), "{open_tag}");
        }
    }

    #[test]
    fn user_profile_and_session_roles_dialogs_render_dormant_roles_manage_btn_and_error_banner() {
        let skin = HtmlCoreSkin;
        let dialogs = sample_dialogs();

        let profile_html = skin.render_dialog(&dialogs[1], SceneViewport::DESKTOP);
        let active_pos = profile_html
            .find(r#"data-role-state="active""#)
            .expect("active roles section");
        let dormant_pos = profile_html
            .find(r#"data-role-state="dormant""#)
            .expect("dormant roles section");
        let manage_pos = profile_html
            .find(r#"data-control-id="btn-manage-roles""#)
            .expect("manage session roles button");
        assert!(
            active_pos < dormant_pos && dormant_pos < manage_pos,
            "dormant roles must render right below active roles along with manage button"
        );
        assert!(profile_html.contains("platform-admin (Timed)"));
        assert!(profile_html.contains("Manage Session Roles"));

        let roles_html = skin.render_dialog(&dialogs[2], SceneViewport::DESKTOP);
        assert!(roles_html
            .contains(r#"<div class="mu-banner mu-dialog-error" role="alert" data-tone="fail">"#));
        assert!(roles_html.contains("Passkey verification was cancelled."));
        assert!(roles_html.contains(
            r#"<li class="mu-role-row" data-tone="neutral" data-active="false" data-disabled="true">"#
        ));
    }

    #[test]
    fn dashboard_sections_emit_data_full_width_and_non_dashboard_sections_do_not() {
        let mut scene = ConsoleScene::new("dashboard", "Dashboard");
        scene.sections = vec![
            SceneSection::new("sec-dash-applications", "Applications"),
            SceneSection::new("sec-dash-fleet", "Fleet"),
            SceneSection::new("sec-dash-attention", "Needs Attention"),
            SceneSection::new("sec-overview", "Non-Dashboard"),
        ];
        let html = HtmlCoreSkin.render_scene(&scene, SceneViewport::DESKTOP);
        assert!(html.contains(r#"data-section-id="sec-dash-applications" data-full-width="true""#));
        assert!(html.contains(r#"data-section-id="sec-dash-fleet" data-full-width="true""#));
        assert!(html.contains(r#"data-section-id="sec-dash-attention" data-full-width="true""#));
        assert!(!html.contains(r#"data-section-id="sec-overview" data-full-width="true""#));
    }
}
