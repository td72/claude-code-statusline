use claude_code_statusline_components::color::{Color, Threshold};
use claude_code_statusline_components::count::{Count, CountStyle};
use claude_code_statusline_components::indicator::Indicator;
use claude_code_statusline_components::label::Label;
use claude_code_statusline_components::path::{Path, PathStyle};
use claude_code_statusline_components::progress_bar::ProgressBar;
use claude_code_statusline_widgets::rate_limit::RateLimitWindowKind;
use claude_code_statusline_widgets::*;

use crate::config::{parse_bracket, parse_color, WidgetConfig};

/// Build a boxed Widget from a widget name and its config.
pub fn build_widget(name: &str, cfg: &WidgetConfig) -> Option<Box<dyn Widget>> {
    match name {
        "model" => Some(Box::new(build_model_info(cfg))),
        "workspace" => Some(Box::new(build_workspace_info(cfg))),
        "agent" => Some(Box::new(build_agent_info(cfg))),
        "worktree" => Some(Box::new(build_worktree_info(cfg))),
        "git_branch" => Some(Box::new(build_git_branch(cfg))),
        "vim" => Some(Box::new(build_vim_status(cfg))),
        "context_usage" => Some(Box::new(build_context_usage(cfg))),
        "cost_summary" => Some(Box::new(build_cost_summary(cfg))),
        "token_alert" => Some(Box::new(build_token_alert(cfg))),
        "rate_limit_5h" => Some(Box::new(build_rate_limit(cfg, RateLimitWindowKind::FiveHour))),
        "rate_limit_7d" => Some(Box::new(build_rate_limit(cfg, RateLimitWindowKind::SevenDay))),
        _ => {
            eprintln!("Unknown widget: {name}");
            None
        }
    }
}

fn build_label(cfg: &WidgetConfig) -> Label {
    let has_bg = cfg.bg.is_some();
    Label {
        color: cfg.color.as_deref().map(parse_color),
        bg: cfg.bg.as_deref().map(parse_color),
        bracket: cfg.bracket.as_deref().and_then(parse_bracket),
        prefix: cfg.prefix.clone().unwrap_or_default(),
        pad: has_bg,
    }
}

fn build_thresholds(cfg: &WidgetConfig) -> Vec<Threshold> {
    let mut thresholds = Vec::new();
    if let Some(t) = &cfg.thresholds {
        if let Some(v) = t.red {
            thresholds.push(Threshold { min: v, color: Color::Red });
        }
        if let Some(v) = t.yellow {
            thresholds.push(Threshold { min: v, color: Color::Yellow });
        }
    }
    if thresholds.is_empty() {
        // Default thresholds
        thresholds.push(Threshold { min: 90.0, color: Color::Red });
        thresholds.push(Threshold { min: 70.0, color: Color::Yellow });
    }
    // Sort descending by min
    thresholds.sort_by(|a, b| b.min.partial_cmp(&a.min).unwrap());
    thresholds
}

fn build_progress_bar(cfg: &WidgetConfig) -> ProgressBar {
    ProgressBar {
        width: cfg.bar_width.unwrap_or(10),
        filled_char: cfg.filled_char.as_deref().and_then(|s| s.chars().next()).unwrap_or('█'),
        empty_char: cfg.empty_char.as_deref().and_then(|s| s.chars().next()).unwrap_or('░'),
        thresholds: build_thresholds(cfg),
        default_color: Color::Green,
        show_label: cfg.show_label.unwrap_or(true),
        prefix: cfg.prefix.clone().unwrap_or_default(),
        suffix: String::new(),
    }
}

fn build_model_info(cfg: &WidgetConfig) -> ModelInfo {
    ModelInfo {
        label: build_label(cfg),
        short: cfg.short.unwrap_or(false),
    }
}

fn build_workspace_info(cfg: &WidgetConfig) -> WorkspaceInfo {
    let style = match cfg.style.as_deref() {
        Some("full") => PathStyle::Full,
        Some("home_shortened") => PathStyle::HomeShortened,
        _ => PathStyle::BaseName,
    };
    let has_bg = cfg.bg.is_some();
    WorkspaceInfo {
        path: Path {
            style,
            // When bg is set, prefix goes to label (badge style), not path
            prefix: if has_bg { String::new() } else { cfg.prefix.clone().unwrap_or_default() },
            home_dir: cfg.home_dir.clone().unwrap_or_default(),
        },
        label: build_label(cfg),
    }
}

fn build_agent_info(cfg: &WidgetConfig) -> AgentInfo {
    AgentInfo { label: build_label(cfg) }
}

fn build_worktree_info(cfg: &WidgetConfig) -> WorktreeInfo {
    WorktreeInfo {
        label: build_label(cfg),
    }
}

fn build_git_branch(cfg: &WidgetConfig) -> GitBranch {
    GitBranch { label: build_label(cfg) }
}

fn build_vim_status(cfg: &WidgetConfig) -> VimStatus {
    VimStatus {
        normal_bg: cfg.normal_bg.as_deref().map(parse_color),
        insert_bg: cfg.insert_bg.as_deref().map(parse_color),
        normal_fg: cfg.normal_fg.as_deref().map(parse_color),
        insert_fg: cfg.insert_fg.as_deref().map(parse_color),
    }
}

fn build_context_usage(cfg: &WidgetConfig) -> ContextUsage {
    let token_count = if cfg.show_tokens.unwrap_or(false) {
        let style = match cfg.token_style.as_deref() {
            Some("comma") => CountStyle::Comma,
            Some("compact") => CountStyle::Compact,
            _ => CountStyle::Plain,
        };
        Some(Count { style, ..Default::default() })
    } else {
        None
    };

    ContextUsage {
        bar: build_progress_bar(cfg),
        token_count,
    }
}

fn build_cost_summary(cfg: &WidgetConfig) -> CostSummary {
    let line_count = if cfg.show_lines.unwrap_or(false) {
        Some(Count::default())
    } else {
        None
    };
    CostSummary {
        line_count,
        cost_prefix: cfg.cost_prefix.clone().unwrap_or_default(),
        duration_prefix: cfg.duration_prefix.clone().unwrap_or_default(),
        lines_prefix: cfg.lines_prefix.clone().unwrap_or_default(),
        ..Default::default()
    }
}

fn build_token_alert(cfg: &WidgetConfig) -> TokenAlert {
    TokenAlert {
        indicator: Indicator {
            on_text: cfg.on_text.clone().unwrap_or("⚠".into()),
            off_text: cfg.off_text.clone().unwrap_or_default(),
            on_color: Some(cfg.on_color.as_deref().map(parse_color).unwrap_or(Color::Red)),
            off_color: None,
        },
    }
}

fn build_rate_limit(cfg: &WidgetConfig, default_window: RateLimitWindowKind) -> RateLimit {
    let window = match cfg.window.as_deref() {
        Some("7d") => RateLimitWindowKind::SevenDay,
        Some("5h") => RateLimitWindowKind::FiveHour,
        _ => default_window,
    };
    RateLimit {
        bar: build_progress_bar(cfg),
        window,
        separator: cfg.reset_separator.clone().unwrap_or(" resets in ".to_string()),
        ..Default::default()
    }
}
