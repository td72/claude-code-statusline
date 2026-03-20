//! Currency component for monetary values.
//!
//! Formats a floating-point amount with a configurable currency symbol and
//! decimal precision.
//!
//! Typical data source: `cost.total_cost_usd`.

/// Configuration for currency formatting.
///
/// # Examples
///
/// ```
/// use claude_code_statusline_components::currency::Currency;
///
/// assert_eq!(Currency::default().render(0.05), "$0.05");
/// ```
#[derive(Debug, Clone)]
pub struct Currency {
    /// Currency symbol prefix.
    pub symbol: String,
    /// Number of decimal places.
    pub decimals: usize,
}

impl Default for Currency {
    fn default() -> Self {
        Self {
            symbol: "$".to_string(),
            decimals: 2,
        }
    }
}

impl Currency {
    /// Render a monetary value as a formatted currency string.
    ///
    /// The output is `"{symbol}{value}"` with the configured number of
    /// decimal places.
    pub fn render(&self, value: f64) -> String {
        format!("{}{:.*}", self.symbol, self.decimals, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_format() {
        assert_eq!(Currency::default().render(0.01234), "$0.01");
    }

    #[test]
    fn zero() {
        assert_eq!(Currency::default().render(0.0), "$0.00");
    }

    #[test]
    fn large_value() {
        assert_eq!(Currency::default().render(12.5), "$12.50");
    }

    #[test]
    fn custom_decimals() {
        let c = Currency { decimals: 4, ..Default::default() };
        assert_eq!(c.render(0.01234), "$0.0123");
    }

    #[test]
    fn custom_symbol() {
        let c = Currency { symbol: "¥".to_string(), ..Default::default() };
        assert_eq!(c.render(100.0), "¥100.00");
    }
}
