// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! Skin-agnostic scene and modal dialog descriptors ([`ConsoleScene`] and
//! [`DialogModel`]).
//!
//! Both [`crate::html::HtmlCoreSkin`] (semantic HTML5 + responsive CSS grid)
//! and [`crate::wireframe::WireframeSkin`] (deterministic SVG wireframe) render
//! from these same structures so navigation, ARIA semantics, disabled control
//! states, and action routing remain invariant across skins.

use crate::skin::{SemanticTone, SignalGlyph};

/// Logical viewport bounds passed to [`SceneRenderer`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SceneViewport {
    pub width: f64,
    pub height: f64,
}

impl SceneViewport {
    /// Standard desktop reference viewport (`1512 x 860`).
    pub const DESKTOP: Self = Self {
        width: 1512.0,
        height: 860.0,
    };

    /// Tablet / medium viewport (`1024 x 768`).
    pub const TABLET: Self = Self {
        width: 1024.0,
        height: 768.0,
    };

    /// Compact landscape viewport (`780 x 400`).
    pub const COMPACT_LANDSCAPE: Self = Self {
        width: 780.0,
        height: 400.0,
    };

    /// Mobile portrait phone viewport (`402 x 720`).
    pub const PHONE: Self = Self {
        width: 402.0,
        height: 720.0,
    };

    /// Construct a clamped positive viewport (`width >= 1.0`, `height >= 1.0`).
    #[must_use]
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width: if width.is_finite() {
                width.max(1.0)
            } else {
                Self::DESKTOP.width
            },
            height: if height.is_finite() {
                height.max(1.0)
            } else {
                Self::DESKTOP.height
            },
        }
    }

    /// True when width falls below the mobile phone breakpoint (`< 520 px`).
    #[must_use]
    pub fn is_phone(self) -> bool {
        self.width < 520.0
    }

    /// True when vertical height is tight (`< 480 px`).
    #[must_use]
    pub fn is_compact_landscape(self) -> bool {
        self.height < 480.0
    }
}

impl Default for SceneViewport {
    fn default() -> Self {
        Self::DESKTOP
    }
}

/// Skin-agnostic renderer for [`ConsoleScene`] and [`DialogModel`].
pub trait SceneRenderer {
    /// Machine identifier of the skin (`"html"` or `"wireframe"`).
    fn skin_id(&self) -> &'static str;

    /// Render a [`ConsoleScene`] at `viewport`.
    fn render_scene(&self, scene: &ConsoleScene, viewport: SceneViewport) -> String;

    /// Render a [`DialogModel`] modal overlay or `<dialog>` element at `viewport`.
    fn render_dialog(&self, dialog: &DialogModel, viewport: SceneViewport) -> String;

    /// Render a [`ConsoleScene`] together with an optional active [`DialogModel`].
    fn render_scene_with_dialog(
        &self,
        scene: &ConsoleScene,
        dialog: Option<&DialogModel>,
        viewport: SceneViewport,
    ) -> String;
}

/// Interactive button, toggle, or link descriptor shared across skins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionControl {
    pub control_id: String,
    pub label: String,
    pub action: String,
    pub area: Option<String>,
    pub subview: Option<String>,
    pub target_id: Option<String>,
    pub tone: SemanticTone,
    pub disabled: bool,
    pub selected: Option<bool>,
    pub pressed: Option<bool>,
    pub aria_label: Option<String>,
    pub badge: Option<String>,
    pub extra_attrs: Vec<(String, String)>,
}

impl ActionControl {
    /// Construct an enabled [`ActionControl`] with neutral tone.
    #[must_use]
    pub fn new(
        control_id: impl Into<String>,
        label: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            control_id: control_id.into(),
            label: label.into(),
            action: action.into(),
            area: None,
            subview: None,
            target_id: None,
            tone: SemanticTone::Neutral,
            disabled: false,
            selected: None,
            pressed: None,
            aria_label: None,
            badge: None,
            extra_attrs: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_area(mut self, area: impl Into<String>) -> Self {
        self.area = Some(area.into());
        self
    }

    #[must_use]
    pub fn with_subview(mut self, subview: impl Into<String>) -> Self {
        self.subview = Some(subview.into());
        self
    }

    #[must_use]
    pub fn with_target(mut self, target_id: impl Into<String>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }

    #[must_use]
    pub const fn with_tone(mut self, tone: SemanticTone) -> Self {
        self.tone = tone;
        self
    }

    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub const fn with_selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    #[must_use]
    pub const fn with_pressed(mut self, pressed: bool) -> Self {
        self.pressed = Some(pressed);
        self
    }

    #[must_use]
    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    #[must_use]
    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    #[must_use]
    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra_attrs.push((key.into(), value.into()));
        self
    }

    /// Effective accessible label (`aria_label` if present, otherwise `label`).
    #[must_use]
    pub fn accessible_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or(&self.label)
    }

    /// Invoke `f(attr_name, attr_value)` for every skin-invariant identity,
    /// routing, state, and ARIA attribute carried by this control.
    pub fn visit_attrs<'a>(&'a self, mut f: impl FnMut(&'a str, &'a str)) {
        f("data-control-id", &self.control_id);
        f("data-action", &self.action);
        f("data-tone", self.tone.as_str());
        if let Some(area) = &self.area {
            f("data-area", area);
        }
        if let Some(subview) = &self.subview {
            f("data-subview", subview);
        }
        if let Some(target) = &self.target_id {
            f("data-target", target);
        }
        if let Some(selected) = self.selected {
            f("aria-selected", if selected { "true" } else { "false" });
        }
        if let Some(pressed) = self.pressed {
            f("aria-pressed", if pressed { "true" } else { "false" });
        }
        if let Some(aria) = &self.aria_label {
            f("aria-label", aria);
        }
        if self.disabled {
            f("aria-disabled", "true");
            f("data-disabled", "true");
        }
        for (k, v) in &self.extra_attrs {
            f(k, v);
        }
    }

    /// Fold all canonical attributes through an accumulator-passing builder (such as
    /// [`crate::html::HtmlTag`] or [`crate::svg::Tag`]) without heap allocation.
    #[must_use]
    pub fn fold_attrs<'a, T>(&'a self, acc: T, mut f: impl FnMut(T, &'a str, &'a str) -> T) -> T {
        let mut opt = Some(acc);
        self.visit_attrs(|k, v| {
            if let Some(cur) = opt.take() {
                opt = Some(f(cur, k, v));
            }
        });
        opt.expect("fold_attrs preserves accumulator")
    }
}

/// Top-level area tab inside primary navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavTab {
    pub id: String,
    pub control_id: String,
    pub label: String,
    pub badge: Option<u32>,
    pub tone: SemanticTone,
    pub active: bool,
    pub disabled: bool,
}

impl NavTab {
    /// Convert this area tab into a uniform [`ActionControl`] descriptor.
    #[must_use]
    pub fn as_action_control(&self) -> ActionControl {
        let mut ctrl = ActionControl::new(&self.control_id, &self.label, "select-area")
            .with_area(&self.id)
            .with_tone(self.tone)
            .with_selected(self.active)
            .disabled(self.disabled);
        if let Some(badge) = self.badge {
            ctrl = ctrl.with_badge(badge.to_string());
        }
        ctrl
    }
}

/// Secondary sub-view tab within the active area.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubViewTab {
    pub id: String,
    pub control_id: String,
    pub area: String,
    pub label: String,
    pub badge: Option<u32>,
    pub tone: SemanticTone,
    pub active: bool,
    pub disabled: bool,
}

impl SubViewTab {
    /// Convert this sub-view tab into a uniform [`ActionControl`] descriptor.
    #[must_use]
    pub fn as_action_control(&self) -> ActionControl {
        let mut ctrl = ActionControl::new(&self.control_id, &self.label, "select-subview")
            .with_area(&self.area)
            .with_subview(&self.id)
            .with_tone(self.tone)
            .with_selected(self.active)
            .disabled(self.disabled);
        if let Some(badge) = self.badge {
            ctrl = ctrl.with_badge(badge.to_string());
        }
        ctrl
    }
}

/// Dual-level repository and submodule selector descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoSelector {
    pub control_id: String,
    pub outer_label: String,
    pub outer_index: usize,
    pub outer_total: usize,
    pub inner_label: String,
    pub inner_index: usize,
    pub inner_total: usize,
    pub filter_query: String,
    pub prev_outer: ActionControl,
    pub next_outer: ActionControl,
    pub prev_inner: ActionControl,
    pub next_inner: ActionControl,
}

/// Compact status lamp indicator in the shell bar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusLamp {
    pub control_id: String,
    pub label: String,
    pub value: String,
    pub lit: bool,
    pub tone: SemanticTone,
    pub action: Option<ActionControl>,
}

/// Alert or operational state banner rendered above workspace sections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SceneBanner {
    pub id: String,
    pub tone: SemanticTone,
    pub title: String,
    pub message: String,
    pub action: Option<ActionControl>,
}

/// Pagination bar shared by [`TableGrid`] and [`LogViewer`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pagination {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: Option<u32>,
    pub prev: ActionControl,
    pub next: ActionControl,
}

/// Complete declarative scene rendered by either [`crate::html::HtmlCoreSkin`]
/// or [`crate::wireframe::WireframeSkin`].
#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleScene {
    pub active_area: String,
    pub area_label: String,
    pub active_subview: Option<String>,
    pub area_tabs: Vec<NavTab>,
    pub subviews: Vec<SubViewTab>,
    pub repo_selector: Option<RepoSelector>,
    pub font_toggle: Option<ActionControl>,
    pub status_lamps: Vec<StatusLamp>,
    pub header_actions: Vec<ActionControl>,
    pub banners: Vec<SceneBanner>,
    pub sections: Vec<SceneSection>,
    pub footer_status: Option<String>,
}

impl ConsoleScene {
    /// Construct a minimal scene shell for `active_area`.
    #[must_use]
    pub fn new(active_area: impl Into<String>, area_label: impl Into<String>) -> Self {
        Self {
            active_area: active_area.into(),
            area_label: area_label.into(),
            active_subview: None,
            area_tabs: Vec::new(),
            subviews: Vec::new(),
            repo_selector: None,
            font_toggle: None,
            status_lamps: Vec::new(),
            header_actions: Vec::new(),
            banners: Vec::new(),
            sections: Vec::new(),
            footer_status: None,
        }
    }
}

/// Named content region rendered as a semantic `<section>` (HTML) or panel (Wireframe).
#[derive(Debug, Clone, PartialEq)]
pub struct SceneSection {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub tone: SemanticTone,
    pub aria_label: Option<String>,
    pub components: Vec<SceneComponent>,
}

impl SceneSection {
    #[must_use]
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            subtitle: None,
            tone: SemanticTone::Neutral,
            aria_label: None,
            components: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_component(mut self, component: SceneComponent) -> Self {
        self.components.push(component);
        self
    }

    #[must_use]
    pub fn with_components(mut self, components: Vec<SceneComponent>) -> Self {
        self.components = components;
        self
    }
}

/// Eight canonical semantic view components supported across both skins.
#[derive(Debug, Clone, PartialEq)]
pub enum SceneComponent {
    StatusMatrix(StatusMatrix),
    TableGrid(TableGrid),
    LogViewer(LogViewer),
    TrainwayGraph(TrainwayGraph),
    GateChecklist(GateChecklist),
    SparklineCard(SparklineCard),
    KeyValueList(KeyValueList),
    PlatformPanel(PlatformPanel),
}

/// Grid of compact status tiles (never colour-alone; supports [`SignalGlyph`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusMatrix {
    pub id: String,
    pub title: String,
    pub cells: Vec<MatrixCell>,
}

/// One cell in a [`StatusMatrix`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixCell {
    pub id: String,
    pub label: String,
    pub value: String,
    pub detail: Option<String>,
    pub tone: SemanticTone,
    pub signal: Option<SignalGlyph>,
    pub action: Option<ActionControl>,
}

/// Tabular dataset with semantic columns, rows, and optional pagination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableGrid {
    pub id: String,
    pub title: String,
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub empty_message: Option<String>,
    pub pagination: Option<Pagination>,
}

/// Column header definition for [`TableGrid`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableColumn {
    pub key: String,
    pub label: String,
    pub numeric: bool,
}

/// One row inside [`TableGrid`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow {
    pub id: String,
    pub tone: SemanticTone,
    pub cells: Vec<TableCell>,
    pub row_action: Option<ActionControl>,
}

/// One cell inside [`TableRow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCell {
    pub text: String,
    pub tone: SemanticTone,
    pub signal: Option<SignalGlyph>,
    pub mono: bool,
    pub action: Option<ActionControl>,
}

/// Monospaced streaming or paged log viewport.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogViewer {
    pub id: String,
    pub title: String,
    pub font_alias: Option<String>,
    pub wrap: bool,
    pub stream_id: Option<String>,
    pub entries: Vec<LogEntry>,
    pub controls: Vec<ActionControl>,
    pub pagination: Option<Pagination>,
}

/// One formatted log line inside [`LogViewer`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub line_no: Option<u32>,
    pub timestamp: Option<String>,
    pub level: SemanticTone,
    pub signal: Option<SignalGlyph>,
    pub text: String,
    pub trace_id: Option<String>,
    pub action: Option<ActionControl>,
}

/// Pipeline trainway diagram across CI/CD station columns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrainwayGraph {
    pub id: String,
    pub title: String,
    pub stations: Vec<String>,
    pub tracks: Vec<TrainwayTrack>,
}

/// One branch/PR railway line in [`TrainwayGraph`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrainwayTrack {
    pub id: String,
    pub ref_name: String,
    pub lane: u32,
    pub fork_stage: u32,
    pub dashed: bool,
    pub fading: bool,
    pub checkpoints: Vec<TrainwayCheckpoint>,
    pub node_tag: Option<String>,
    pub node_lost: bool,
    pub action: Option<ActionControl>,
}

/// Stage checkpoint along a [`TrainwayTrack`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrainwayCheckpoint {
    pub stage_index: u32,
    pub stage_name: String,
    pub signal: SignalGlyph,
    pub tone: SemanticTone,
    pub label: Option<String>,
}

/// Release/delivery gate verification checklist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateChecklist {
    pub id: String,
    pub title: String,
    pub environment: Option<String>,
    pub overall_tone: SemanticTone,
    pub items: Vec<GateItem>,
    pub actions: Vec<ActionControl>,
}

/// One verification gate inside [`GateChecklist`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateItem {
    pub id: String,
    pub label: String,
    pub status_label: String,
    pub detail: Option<String>,
    pub signal: SignalGlyph,
    pub tone: SemanticTone,
    pub required: bool,
    pub action: Option<ActionControl>,
}

/// Compact time-series KPI card rendered with an inline SVG sparkline.
#[derive(Debug, Clone, PartialEq)]
pub struct SparklineCard {
    pub id: String,
    pub title: String,
    pub metric_value: String,
    pub unit: Option<String>,
    pub delta_label: Option<String>,
    pub tone: SemanticTone,
    pub samples: Vec<f64>,
    pub range_label: Option<String>,
    pub action: Option<ActionControl>,
}

/// Structured key/value metadata list (`<dl>` in HTML).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValueList {
    pub id: String,
    pub title: String,
    pub entries: Vec<KeyValueEntry>,
}

/// One entry in [`KeyValueList`] or [`PlatformPanel`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValueEntry {
    pub key: String,
    pub value: String,
    pub detail: Option<String>,
    pub tone: SemanticTone,
    pub mono: bool,
    pub action: Option<ActionControl>,
}

/// Operational platform panel combining summary telemetry, metadata, and operator actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPanel {
    pub id: String,
    pub title: String,
    pub summary: Option<String>,
    pub tone: SemanticTone,
    pub attributes: Vec<KeyValueEntry>,
    pub controls: Vec<ActionControl>,
}

/// Modal dialog descriptor rendered as `<dialog>` (`HtmlCoreSkin`) or modal frame (`WireframeSkin`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum DialogModel {
    /// Guided WebAuthn/passkey onboarding modal (`is_dismissible() == true`).
    PasskeyEnrollmentGate(PasskeyEnrollmentDialog),
    /// `@user` identity and session overview modal.
    UserProfile(UserProfileDialog),
    /// `+role` session role elevation/revocation modal (explicit controls, zero TTL inputs).
    SessionRoles(SessionRolesDialog),
    /// Explicit risk-confirmation modal replacing hardware-style hold-to-arm guards.
    RiskConfirm(RiskConfirmDialog),
    /// One-time invite URL handoff modal (displayed once upon creation).
    InviteCreatedOnce(InviteCreatedOnceDialog),
}

impl DialogModel {
    /// Stable DOM / SVG identifier for the dialog.
    #[must_use]
    pub fn dialog_id(&self) -> &str {
        match self {
            Self::PasskeyEnrollmentGate(d) => &d.dialog_id,
            Self::UserProfile(d) => &d.dialog_id,
            Self::SessionRoles(d) => &d.dialog_id,
            Self::RiskConfirm(d) => &d.dialog_id,
            Self::InviteCreatedOnce(d) => &d.dialog_id,
        }
    }

    /// Heading text of the dialog.
    #[must_use]
    pub fn title(&self) -> &str {
        match self {
            Self::PasskeyEnrollmentGate(d) => &d.title,
            Self::UserProfile(d) => &d.display_name,
            Self::SessionRoles(d) => &d.title,
            Self::RiskConfirm(d) => &d.title,
            Self::InviteCreatedOnce(d) => &d.title,
        }
    }

    /// Machine variant identifier emitted on `data-dialog-kind`.
    #[must_use]
    pub const fn dialog_kind(&self) -> &'static str {
        match self {
            Self::PasskeyEnrollmentGate(_) => "passkey-enrollment-gate",
            Self::UserProfile(_) => "user-profile",
            Self::SessionRoles(_) => "session-roles",
            Self::RiskConfirm(_) => "risk-confirm",
            Self::InviteCreatedOnce(_) => "invite-created-once",
        }
    }

    /// Return `true` for dismissible modals (`PasskeyEnrollmentGate` can be dismissed to browse public views).
    #[must_use]
    pub const fn is_dismissible(&self) -> bool {
        true
    }

    /// Primary semantic tone of the dialog border and status accent.
    #[must_use]
    pub const fn tone(&self) -> SemanticTone {
        match self {
            Self::PasskeyEnrollmentGate(d) => d.tone,
            Self::UserProfile(_) => SemanticTone::Info,
            Self::SessionRoles(_) => SemanticTone::Info,
            Self::RiskConfirm(d) => d.tone,
            Self::InviteCreatedOnce(_) => SemanticTone::Pass,
        }
    }
}

/// Blocking passkey enrollment modal (`PasskeyEnrollmentGate`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasskeyEnrollmentDialog {
    pub dialog_id: String,
    pub title: String,
    pub reason: String,
    pub account_subject: String,
    pub passkey_count: u32,
    pub enroll_action: ActionControl,
    pub sign_out_action: Option<ActionControl>,
    pub tone: SemanticTone,
}

/// Role chip displayed inside [`UserProfileDialog`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleBadge {
    pub role: String,
    pub tone: SemanticTone,
    pub detail: Option<String>,
}

/// `@user` profile modal descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfileDialog {
    pub dialog_id: String,
    pub handle: String,
    pub display_name: String,
    pub subject_id: String,
    pub email: Option<String>,
    pub active_roles: Vec<RoleBadge>,
    pub dormant_roles: Vec<RoleBadge>,
    pub session_expires_at: Option<String>,
    pub passkey_count: u32,
    pub manage_roles_action: Option<ActionControl>,
    pub lock_action: Option<ActionControl>,
    pub manage_passkeys_action: Option<ActionControl>,
    pub sign_out_action: Option<ActionControl>,
    pub close_action: ActionControl,
}

/// `+role` session role modal descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRolesDialog {
    pub dialog_id: String,
    pub title: String,
    pub subject_handle: String,
    pub step_up_verified: bool,
    pub roles: Vec<SessionRoleOption>,
    pub error_message: Option<String>,
    pub close_action: ActionControl,
}

/// One grantable or revocable session role row inside [`SessionRolesDialog`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRoleOption {
    pub role: String,
    pub description: String,
    pub active: bool,
    pub expires_at: Option<String>,
    pub requires_step_up: bool,
    pub tone: SemanticTone,
    pub action: ActionControl,
}

/// Risk-confirmation dialog replacing tactile hold-to-arm switch guards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskConfirmDialog {
    pub dialog_id: String,
    pub title: String,
    pub summary: String,
    pub target_resource: String,
    pub consequence: String,
    pub confirmation_phrase: Option<String>,
    pub typed_confirmation: String,
    pub phrase_matched: bool,
    pub phrase_input_id: String,
    pub phrase_input_action: String,
    pub tone: SemanticTone,
    pub confirm_action: ActionControl,
    pub cancel_action: ActionControl,
}

/// One-time invite URL handoff dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InviteCreatedOnceDialog {
    pub dialog_id: String,
    pub title: String,
    pub invitee_hint: String,
    pub assigned_roles: Vec<String>,
    pub invite_url: String,
    pub expires_at: String,
    pub warning: String,
    pub copy_action: ActionControl,
    pub close_action: ActionControl,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_breakpoints_and_clamping() {
        assert!(!SceneViewport::DESKTOP.is_phone());
        assert!(!SceneViewport::TABLET.is_phone());
        assert!(SceneViewport::COMPACT_LANDSCAPE.is_compact_landscape());
        assert!(SceneViewport::PHONE.is_phone());

        let clamped = SceneViewport::new(0.0, f64::NAN);
        assert_eq!(clamped.width, 1.0);
        assert_eq!(clamped.height, SceneViewport::DESKTOP.height);
    }

    #[test]
    fn passkey_gate_and_other_dialogs_are_dismissible() {
        let close = ActionControl::new("dlg-close", "Close", "dismiss-dialog");
        let gate = DialogModel::PasskeyEnrollmentGate(PasskeyEnrollmentDialog {
            dialog_id: "passkey-gate".into(),
            title: "Passkey enrollment required".into(),
            reason: "Register a hardware security key or passkey to continue.".into(),
            account_subject: "usr_ops_01".into(),
            passkey_count: 0,
            enroll_action: ActionControl::new("enroll-passkey", "Register passkey", "enroll")
                .with_tone(SemanticTone::Warn),
            sign_out_action: None,
            tone: SemanticTone::Warn,
        });
        assert!(gate.is_dismissible());
        assert_eq!(gate.dialog_kind(), "passkey-enrollment-gate");

        let profile = DialogModel::UserProfile(UserProfileDialog {
            dialog_id: "user-profile".into(),
            handle: "@operator".into(),
            display_name: "Operator Profile".into(),
            subject_id: "usr_ops_01".into(),
            email: None,
            active_roles: vec![RoleBadge {
                role: "viewer".into(),
                tone: SemanticTone::Pass,
                detail: None,
            }],
            dormant_roles: vec![RoleBadge {
                role: "platform-admin".into(),
                tone: SemanticTone::Info,
                detail: Some("Timed".into()),
            }],
            session_expires_at: None,
            passkey_count: 2,
            manage_roles_action: Some(ActionControl::new(
                "profile-manage-roles",
                "Manage Session Roles",
                "open-session-roles",
            )),
            lock_action: None,
            manage_passkeys_action: None,
            sign_out_action: None,
            close_action: close,
        });
        assert!(profile.is_dismissible());
        assert_eq!(profile.dialog_kind(), "user-profile");
    }

    #[test]
    fn action_control_builder_preserves_metadata_and_disabled_state() {
        let btn = ActionControl::new("promote-btn", "Promote release", "cd-promote")
            .with_area("delivery")
            .with_subview("gates")
            .with_target("rel-42")
            .with_tone(SemanticTone::Pass)
            .disabled(true)
            .with_aria_label("Promote release rel-42 (blocked by gate)")
            .with_attr("data-cd-gate", "smoke");

        assert!(btn.disabled);
        assert_eq!(btn.area.as_deref(), Some("delivery"));
        assert_eq!(btn.subview.as_deref(), Some("gates"));
        assert_eq!(btn.target_id.as_deref(), Some("rel-42"));
        assert_eq!(
            btn.accessible_label(),
            "Promote release rel-42 (blocked by gate)"
        );
        assert_eq!(
            btn.extra_attrs,
            vec![("data-cd-gate".to_owned(), "smoke".to_owned())]
        );
    }
}
