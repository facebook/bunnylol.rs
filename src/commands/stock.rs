use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::config::BunnylolConfig;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

pub struct StockCommand;

impl StockCommand {
    /// Get homepage URL for a provider
    fn get_provider_homepage(provider: &str) -> String {
        match provider {
            "finviz" => "https://finviz.com/",
            "tradingview" | "tv" => "https://www.tradingview.com/",
            "google" | "gf" => "https://www.google.com/finance/",
            "investing" | "inv" => "https://www.investing.com/",
            _ => "https://finance.yahoo.com/",
        }
        .to_string()
    }

    /// Process a ticker with $ prefix (e.g., "$META")
    /// Uses config preference, defaults to yahoo if no config
    pub fn process_ticker(
        ticker_with_dollar: &str,
        config: Option<&BunnylolConfig>,
    ) -> String {
        // Get provider from config or default to yahoo
        let provider = config
            .map(|cfg| cfg.stock_provider.as_str())
            .unwrap_or("yahoo");

        if ticker_with_dollar.len() <= 1 {
            // No ticker - return provider homepage
            return Self::get_provider_homepage(provider);
        }

        let ticker = &ticker_with_dollar[1..];
        Self::build_url_for_provider(ticker, provider)
    }

    /// Build stock URL for a specific provider
    fn build_url_for_provider(ticker: &str, provider: &str) -> String {
        let encoded = utf8_percent_encode(ticker, NON_ALPHANUMERIC).to_string();

        match provider {
            "yahoo" => format!("https://finance.yahoo.com/quote/{}/", encoded),
            "finviz" => format!("https://finviz.com/quote.ashx?t={}", ticker),
            "tradingview" | "tv" => format!("https://www.tradingview.com/symbols/{}/", ticker),
            "google" | "gf" => format!("https://www.google.com/finance/quote/{}", ticker),
            "investing" | "inv" => format!("https://www.investing.com/search/?q={}", encoded),
            _ => format!("https://finance.yahoo.com/quote/{}/", encoded),
        }
    }

    /// Parse provider from query (e.g., "finviz AAPL" or "AAPL")
    /// Returns (Option<provider>, ticker)
    fn parse_provider_and_ticker(query: &str) -> (Option<&str>, &str) {
        let parts: Vec<&str> = query.split_whitespace().collect();

        if parts.len() >= 2 {
            let potential_provider = parts[0].to_lowercase();
            let known_providers = ["yahoo", "finviz", "tradingview", "tv", "google", "gf", "investing", "inv"];

            if known_providers.contains(&potential_provider.as_str()) {
                // Return provider and rest of query, ticker starts after first whitespace + provider length
                let ticker_start = query.find(char::is_whitespace)
                    .map(|pos| query[pos..].trim_start())
                    .unwrap_or("");
                return (Some(parts[0]), ticker_start);
            }
        }

        // no provider is specified
        (None, query)
    }
}

impl BunnylolCommand for StockCommand {
    const BINDINGS: &'static [&'static str] = &["stock", "stocks", "finance"];

    fn process_args(args: &str) -> String {
        Self::process_args_with_config(args, None)
    }

    fn process_args_with_config(
        args: &str,
        config: Option<&BunnylolConfig>,
    ) -> String {
        let query = Self::get_command_args(args);

        // Get provider from config or default to yahoo
        let provider = config
            .map(|cfg| cfg.stock_provider.as_str())
            .unwrap_or("yahoo");

        if query.is_empty() {
            // When no tickers, home page
            return Self::get_provider_homepage(provider);
        }

        let (provider_override, ticker) = Self::parse_provider_and_ticker(query);

        // Override if specified, otherwise use config default
        let final_provider = provider_override.unwrap_or(provider);
        Self::build_url_for_provider(ticker, final_provider)
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: vec![
                "stock".to_string(),
                "stocks".to_string(),
                "finance".to_string(),
                "$<ticker>".to_string(),
            ],
            description: "Look up stock prices on Yahoo Finance, Finviz, TradingView, Google Finance, or Investing.com".to_string(),
            example: "stock META  or  stock finviz META  or  $META".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic behavior
    #[test]
    fn test_stock_command_default_yahoo() {
        assert_eq!(
            StockCommand::process_args_with_config("stock META", None),
            "https://finance.yahoo.com/quote/META/"
        );
    }

    #[test]
    fn test_stock_command_no_ticker() {
        assert_eq!(
            StockCommand::process_args_with_config("stock", None),
            "https://finance.yahoo.com/"
        );
    }

    // Provider overrides (one per provider)
    #[test]
    fn test_stock_command_finviz_override() {
        assert_eq!(
            StockCommand::process_args_with_config("stock finviz META", None),
            "https://finviz.com/quote.ashx?t=META"
        );
    }

    #[test]
    fn test_stock_command_tradingview_alias() {
        assert_eq!(
            StockCommand::process_args_with_config("stock tv AAPL", None),
            "https://www.tradingview.com/symbols/AAPL/"
        );
    }

    #[test]
    fn test_stock_command_google_alias() {
        assert_eq!(
            StockCommand::process_args_with_config("stock gf META:NASDAQ", None),
            "https://www.google.com/finance/quote/META:NASDAQ"
        );
    }

    // Config-based defaults (one example)
    #[test]
    fn test_stock_command_with_config() {
        let mut config = BunnylolConfig::default();
        config.stock_provider = "finviz".to_string();

        assert_eq!(
            StockCommand::process_args_with_config("stock META", Some(&config)),
            "https://finviz.com/quote.ashx?t=META"
        );
    }

    #[test]
    fn test_stock_command_no_ticker_with_config() {
        let mut config = BunnylolConfig::default();
        config.stock_provider = "finviz".to_string();

        assert_eq!(
            StockCommand::process_args_with_config("stock", Some(&config)),
            "https://finviz.com/"
        );
    }

    #[test]
    fn test_stock_command_with_equals() {
        assert_eq!(
            StockCommand::process_args_with_config("stock RTY=F", None),
            "https://finance.yahoo.com/quote/RTY%3DF/"
        );
    }

    // testing with override priority
    #[test]
    fn test_stock_command_override_beats_config() {
        let mut config = BunnylolConfig::default();
        config.stock_provider = "finviz".to_string();

        assert_eq!(
            StockCommand::process_args_with_config("stock yahoo META", Some(&config)),
            "https://finance.yahoo.com/quote/META/"
        );
    }



    // $TICKER syntax
    #[test]
    fn test_dollar_ticker_default() {
        assert_eq!(
            StockCommand::process_ticker("$META", None),
            "https://finance.yahoo.com/quote/META/"
        );
    }

    #[test]
    fn test_dollar_ticker_with_config() {
        let mut config = BunnylolConfig::default();
        config.stock_provider = "finviz".to_string();

        assert_eq!(
            StockCommand::process_ticker("$AAPL", Some(&config)),
            "https://finviz.com/quote.ashx?t=AAPL"
        );
    }

    // Special characters
    #[test]
    fn test_stock_command_special_chars() {
        assert_eq!(
            StockCommand::process_args_with_config("stock BRK.B", None),
            "https://finance.yahoo.com/quote/BRK%2EB/"
        );
    }

    #[test]
    fn test_stock_ticker_prefix_edge_case_empty_ticker() {
        // Test that "$" alone doesn't panic
        assert_eq!(
            StockCommand::process_ticker("$", None),
            "https://finance.yahoo.com/"
        );
    }

    #[test]
    fn test_stock_ticker_prefix_edge_case_empty_string() {
        // Test that empty string doesn't panic
        assert_eq!(
            StockCommand::process_ticker("", None),
            "https://finance.yahoo.com/"
        );
    }
}
