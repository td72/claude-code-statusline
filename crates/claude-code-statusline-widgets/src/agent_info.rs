//! Agent information widget.
//!
//! Displays the active agent name. Returns `None` when no agent is
//! configured (i.e., not running with `--agent` or agent settings).

use claude_code_statusline_components::label::Label;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the active agent name.
///
/// Returns `None` when no agent is active.
pub struct AgentInfo {
    /// Label formatter.
    pub label: Label,
}

impl Default for AgentInfo {
    fn default() -> Self {
        Self {
            label: Label { prefix: "🤖 ".into(), ..Default::default() },
        }
    }
}

impl Widget for AgentInfo {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let agent = input.agent.as_ref()?;
        Some(self.label.render(&agent.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::*;

    fn make_input(agent: Option<Agent>) -> StatusLineInput {
        StatusLineInput {
            agent,
            ..Default::default()
        }
    }

    #[test]
    fn renders_agent() {
        let w = AgentInfo::default();
        let input = make_input(Some(Agent { name: "security-reviewer".into() }));
        let result = w.render(&input).unwrap();
        assert!(result.contains("security-reviewer"));
    }

    #[test]
    fn returns_none_without_agent() {
        let w = AgentInfo::default();
        let input = make_input(None);
        assert!(w.render(&input).is_none());
    }
}
