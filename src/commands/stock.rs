use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

pub struct StockCommand;

impl StockCommand {
    /// Process a ticker with $ prefix (e.g., "$META")
    pub fn process_ticker(ticker_with_dollar: &str) -> String {
        // Handle edge case: if string is empty or just "$", go to Yahoo Finance homepage
        if ticker_with_dollar.len() <= 1 {
            return "https://finance.yahoo.com/".to_string();
        }

        // Remove the $ prefix
        let ticker = &ticker_with_dollar[1..];
        Self::build_yahoo_finance_url(ticker)
    }

    /// Build Yahoo Finance URL for a ticker symbol
    fn build_yahoo_finance_url(ticker: &str) -> String {
        // URL encode the ticker using NON_ALPHANUMERIC
        // This encodes all special characters (=, ^, -, etc.)
        let encoded = utf8_percent_encode(ticker, NON_ALPHANUMERIC).to_string();

        // Build Yahoo Finance URL
        format!("https://finance.yahoo.com/quote/{}/", encoded)
    }
}

impl BunnylolCommand for StockCommand {
    const BINDINGS: &'static [&'static str] = &["stock", "stocks", "finance"];

    fn process_args(args: &str) -> String {
        let ticker = Self::get_command_args(args);

        if ticker.is_empty() {
            // No ticker provided, go to Yahoo Finance homepage
            return "https://finance.yahoo.com/".to_string();
        }

        Self::build_yahoo_finance_url(ticker)
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: vec![
                "stock".to_string(),
                "stocks".to_string(),
                "finance".to_string(),
                "$<ticker>".to_string(),
            ],
            description: "Look up stock prices on Yahoo Finance".to_string(),
            example: "stock META  or  $META".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Prefix notation tests ($TICKER)
    #[test]
    fn test_stock_ticker_prefix_simple() {
        assert_eq!(
            StockCommand::process_ticker("$META"),
            "https://finance.yahoo.com/quote/META/"
        );
    }

    #[test]
    fn test_stock_ticker_prefix_with_equals() {
        assert_eq!(
            StockCommand::process_ticker("$RTY=F"),
            "https://finance.yahoo.com/quote/RTY%3DF/"
        );
    }

    #[test]
    fn test_stock_ticker_prefix_with_dot() {
        assert_eq!(
            StockCommand::process_ticker("$BRK.B"),
            "https://finance.yahoo.com/quote/BRK%2EB/"
        );
    }

    #[test]
    fn test_stock_ticker_prefix_with_caret() {
        assert_eq!(
            StockCommand::process_ticker("$^GSPC"),
            "https://finance.yahoo.com/quote/%5EGSPC/"
        );
    }

    // Standard binding tests (stock TICKER)
    #[test]
    fn test_stock_command_simple() {
        assert_eq!(
            StockCommand::process_args("stock META"),
            "https://finance.yahoo.com/quote/META/"
        );
    }

    #[test]
    fn test_stocks_command_simple() {
        assert_eq!(
            StockCommand::process_args("stocks AAPL"),
            "https://finance.yahoo.com/quote/AAPL/"
        );
    }

    #[test]
    fn test_finance_command_simple() {
        assert_eq!(
            StockCommand::process_args("finance GOOGL"),
            "https://finance.yahoo.com/quote/GOOGL/"
        );
    }

    #[test]
    fn test_stock_command_with_equals() {
        assert_eq!(
            StockCommand::process_args("stock RTY=F"),
            "https://finance.yahoo.com/quote/RTY%3DF/"
        );
    }

    #[test]
    fn test_stock_command_no_ticker() {
        assert_eq!(
            StockCommand::process_args("stock"),
            "https://finance.yahoo.com/"
        );
    }

    #[test]
    fn test_stock_ticker_prefix_edge_case_empty_ticker() {
        // Test that "$" alone doesn't panic
        assert_eq!(
            StockCommand::process_ticker("$"),
            "https://finance.yahoo.com/"
        );
    }

    #[test]
    fn test_stock_ticker_prefix_edge_case_empty_string() {
        // Test that empty string doesn't panic
        assert_eq!(
            StockCommand::process_ticker(""),
            "https://finance.yahoo.com/"
        );
    }
}
