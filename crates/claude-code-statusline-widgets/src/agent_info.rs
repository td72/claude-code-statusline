//! Agent information widget.

use claude_code_statusline_components::label::Label;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the active agent name.
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
            cwd: "/test".into(),
            session_id: "s".into(),
            transcript_path: "/t".into(),
            model: Model { id: "m".into(), display_name: "M".into() },
            workspace: Workspace {
                current_dir: "/test".into(),
                project_dir: "/test".into(),
                added_dirs: None,
            },
            version: "1.0".into(),
            output_style: OutputStyle { name: "default".into() },
            cost: Cost {
                total_cost_usd: 0.0,
                total_duration_ms: 0,
                total_api_duration_ms: 0,
                total_lines_added: 0,
                total_lines_removed: 0,
            },
            context_window: ContextWindow {
                total_input_tokens: 0,
                total_output_tokens: 0,
                context_window_size: 200_000,
                used_percentage: None,
                remaining_percentage: None,
                current_usage: None,
            },
            exceeds_200k_tokens: false,
            vim: None,
            agent,
            worktree: None,
            rate_limits: None,
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
