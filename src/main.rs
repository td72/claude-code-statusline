mod builder;
mod config;

use std::io::Read;
use std::process;

use claude_code_statusline_model::StatusLineInput;
use claude_code_statusline_widgets::Widget;

use crate::builder::build_widget;
use crate::config::{Config, WidgetConfig};

fn main() {
    // Parse optional --config flag
    let args: Vec<String> = std::env::args().collect();
    let config_path = args
        .windows(2)
        .find(|w| w[0] == "--config")
        .map(|w| w[1].as_str());

    let config = Config::load_or_default(config_path);

    // Read JSON from stdin
    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read stdin: {e}");
        process::exit(1);
    }

    let data: StatusLineInput = match serde_json::from_str(&input) {
        Ok(d) => d,
        Err(e) => {
            // Print error to stderr (visible with `claude --debug`)
            // and show a fallback line so the statusline doesn't go blank.
            eprintln!("claude-code-statusline: parse error: {e}");
            println!("⚠ statusline parse error");
            process::exit(0);
        }
    };

    // Render each line
    let empty_cfg = WidgetConfig::default();
    for line_cfg in &config.line {
        let rendered: Vec<String> = line_cfg
            .widgets
            .iter()
            .filter_map(|name| {
                let widget_cfg = config.widget.get(name).unwrap_or(&empty_cfg);
                let widget: Box<dyn Widget> = build_widget(name, widget_cfg)?;
                widget.render(&data)
            })
            .collect();

        if !rendered.is_empty() {
            println!("{}", rendered.join(&line_cfg.separator));
        }
    }
}
