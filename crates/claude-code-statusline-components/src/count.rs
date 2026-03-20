//! Count component for integer values.
//!
//! Used for: token counts, `cost.total_lines_added`, `cost.total_lines_removed`

/// Formatting style for count values.
#[derive(Debug, Clone, Default)]
pub enum CountStyle {
    /// Plain number: `12345`
    #[default]
    Plain,
    /// Comma-separated: `12,345`
    Comma,
    /// Compact with SI suffix: `12.3k`, `1.2M`
    Compact,
}

/// Configuration for count formatting.
#[derive(Debug, Clone)]
pub struct Count {
    /// Formatting style.
    pub style: CountStyle,
    /// Optional prefix (e.g., `"+"`, `"-"`).
    pub prefix: String,
    /// Optional suffix (e.g., `" tokens"`, `" lines"`).
    pub suffix: String,
}

impl Default for Count {
    fn default() -> Self {
        Self {
            style: CountStyle::default(),
            prefix: String::new(),
            suffix: String::new(),
        }
    }
}

impl Count {
    /// Render a count value.
    pub fn render(&self, value: u64) -> String {
        let formatted = match &self.style {
            CountStyle::Plain => value.to_string(),
            CountStyle::Comma => format_with_commas(value),
            CountStyle::Compact => format_compact(value),
        };
        format!("{}{formatted}{}", self.prefix, self.suffix)
    }
}

fn format_with_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result
}

fn format_compact(n: u64) -> String {
    if n >= 1_000_000 {
        let v = n as f64 / 1_000_000.0;
        if v >= 100.0 {
            format!("{:.0}M", v)
        } else if v >= 10.0 {
            format!("{:.1}M", v)
        } else {
            format!("{:.2}M", v)
        }
    } else if n >= 1_000 {
        let v = n as f64 / 1_000.0;
        if v >= 100.0 {
            format!("{:.0}k", v)
        } else if v >= 10.0 {
            format!("{:.1}k", v)
        } else {
            format!("{:.2}k", v)
        }
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain() {
        assert_eq!(Count::default().render(12345), "12345");
    }

    #[test]
    fn comma() {
        let c = Count { style: CountStyle::Comma, ..Default::default() };
        assert_eq!(c.render(0), "0");
        assert_eq!(c.render(999), "999");
        assert_eq!(c.render(1_000), "1,000");
        assert_eq!(c.render(1_234_567), "1,234,567");
    }

    #[test]
    fn compact() {
        let c = Count { style: CountStyle::Compact, ..Default::default() };
        assert_eq!(c.render(500), "500");
        assert_eq!(c.render(1_500), "1.50k");
        assert_eq!(c.render(15_000), "15.0k");
        assert_eq!(c.render(150_000), "150k");
        assert_eq!(c.render(1_500_000), "1.50M");
        assert_eq!(c.render(15_000_000), "15.0M");
        assert_eq!(c.render(150_000_000), "150M");
    }

    #[test]
    fn with_prefix_and_suffix() {
        let c = Count {
            style: CountStyle::Compact,
            prefix: "+".to_string(),
            suffix: " lines".to_string(),
        };
        assert_eq!(c.render(156), "+156 lines");
    }
}
