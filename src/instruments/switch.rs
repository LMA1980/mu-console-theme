// SPDX-License-Identifier: LGPL-3.0-only WITH LGPL-3.0-linking-exception
//! Semantic switch action primitives.

/// Switch action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SwitchAction {
    /// Maintained N-position selector (area, CRT font, live feed).
    #[default]
    Maintained,
    /// Spring-return-to-centre.
    SpringReturn,
    /// Hold-to-arm spring-return guard for state-changing actions.
    PullToTurn,
}

impl SwitchAction {
    #[must_use]
    pub const fn springs_back(self) -> bool {
        matches!(self, Self::SpringReturn | Self::PullToTurn)
    }

    #[must_use]
    pub const fn needs_pull(self) -> bool {
        matches!(self, Self::PullToTurn)
    }
}

#[cfg(test)]
mod tests {
    use super::SwitchAction;

    #[test]
    fn switch_action_predicates() {
        assert!(!SwitchAction::Maintained.springs_back());
        assert!(!SwitchAction::Maintained.needs_pull());

        assert!(SwitchAction::SpringReturn.springs_back());
        assert!(!SwitchAction::SpringReturn.needs_pull());

        assert!(SwitchAction::PullToTurn.springs_back());
        assert!(SwitchAction::PullToTurn.needs_pull());
    }
}
