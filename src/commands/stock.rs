use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::config::get_global_config;
use crate::utils::url_encoding::encode_url_special_char;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Stock provider configuration
struct StockInfoProvider {
    aliases: &'static [&'static str],
    homepage: &'static str,
    ticker_url_template: &'static str,
    needs_encoding: bool, // Whether to percent-encode the ticker
}

/// supported stock providers
static PROVIDERS: &[StockInfoProvider] = &[
    StockInfoProvider {
        aliases: &["yahoo"],
        homepage: "https://finance.yahoo.com/",
        ticker_url_template: "https://finance.yahoo.com/quote/{}/",
        needs_encoding: true,
    },
    StockInfoProvider {
        aliases: &["finviz"],
        homepage: "https://finviz.com/",
        ticker_url_template: "https://finviz.com/quote.ashx?t={}",
        needs_encoding: false,
    },
    StockInfoProvider {
        aliases: &["tradingview", "tv"],
        homepage: "https://www.tradingview.com/",
        ticker_url_template: "https://www.tradingview.com/symbols/{}/",
        needs_encoding: false,
    },
    StockInfoProvider {
        aliases: &["google", "gf"],
        homepage: "https://www.google.com/finance/",
        ticker_url_template: "https://www.google.com/finance/quote/{}",
        needs_encoding: false,
    },
    StockInfoProvider {
        aliases: &["investing", "inv"],
        homepage: "https://www.investing.com/",
        ticker_url_template: "https://www.investing.com/search/?q={}",
        needs_encoding: true,
    },
];

/// lookup table (alias -> provider) for stocks
static PROVIDER_LOOKUP: LazyLock<HashMap<&'static str, &'static StockInfoProvider>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();
        for p in PROVIDERS {
            for &alias in p.aliases {
                map.insert(alias, p);
            }
        }
        map
    });

pub struct StockCommand;

impl StockCommand {
    fn get_provider(name: &str) -> &'static StockInfoProvider {
        match PROVIDER_LOOKUP.get(name).copied() {
            Some(provider) => provider,
            None => {
                eprintln!(
                    "Warning: Unknown stock provider '{}', using yahoo as fallback",
                    name
                );
                &PROVIDERS[0] // Default to yahoo (first provider)
            }
        }
    }

    fn configured_provider() -> &'static str {
        get_global_config()
            .map(|cfg| cfg.stock_provider.as_str())
            .unwrap_or("yahoo")
    }

    /// Process a ticker with $ prefix (e.g., "$META")
    /// Uses config preference, defaults to yahoo if no config
    pub fn process_ticker(ticker_with_dollar: &str) -> String {
        Self::process_ticker_with_provider(ticker_with_dollar, Self::configured_provider())
    }

    /// testable version of process_ticker that takes an explicit provider name
    fn process_ticker_with_provider(ticker_with_dollar: &str, provider_name: &str) -> String {
        if ticker_with_dollar.len() <= 1 {
            // No ticker - return provider homepage
            let provider = Self::get_provider(provider_name);
            return provider.homepage.to_string();
        }

        let ticker = &ticker_with_dollar[1..];
        Self::build_url_for_provider(ticker, provider_name)
    }

    /// Build stock URL for a specific provider
    fn build_url_for_provider(ticker: &str, provider_name: &str) -> String {
        let provider = Self::get_provider(provider_name);

        let ticker_str = if provider.needs_encoding {
            encode_url_special_char(ticker)
        } else {
            ticker.to_string()
        };

        provider.ticker_url_template.replace("{}", &ticker_str)
    }

    /// Testable version of process_args that takes an explicit provider name
    fn process_args_with_provider(args: &str, provider_name: &str) -> String {
        let query = Self::get_command_args(args);

        if query.is_empty() {
            let provider = Self::get_provider(provider_name);
            return provider.homepage.to_string();
        }

        let (provider_override, ticker) = Self::parse_provider_and_ticker(query);
        let final_provider = provider_override.unwrap_or(provider_name);
        Self::build_url_for_provider(ticker, final_provider)
    }

    /// Parse provider from query (e.g., "finviz AAPL" or "AAPL")
    /// Returns (Option<provider>, ticker)
    fn parse_provider_and_ticker(query: &str) -> (Option<&str>, &str) {
        let parts: Vec<&str> = query.split_whitespace().collect();

        if parts.len() >= 2 {
            let potential_provider = parts[0].to_lowercase();

            if PROVIDER_LOOKUP.contains_key(potential_provider.as_str()) {
                // Return provider and rest of query, ticker starts after first whitespace + provider length
                let ticker_start = query
                    .find(char::is_whitespace)
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
        Self::process_args_with_provider(args, Self::configured_provider())
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            &["stock", "stocks", "finance", "$<ticker>"],
            "Look up stock prices on Yahoo Finance, Finviz, TradingView, Google Finance, or Investing.com",
            "stock META  or  stock finviz META  or  $META",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic behavior (no global config initialized = yahoo default)
    #[test]
    fn test_stock_command_default_yahoo() {
        assert_eq!(
            StockCommand::process_args_with_provider("stock META", "yahoo"),
            "https://finance.yahoo.com/quote/META/"
        );
    }

    #[test]
    fn test_stock_command_no_ticker() {
        assert_eq!(
            StockCommand::process_args_with_provider("stock", "yahoo"),
            "https://finance.yahoo.com/"
        );
    }

    // Provider overrides (one per provider)
    #[test]
    fn test_stock_command_finviz_override() {
        assert_eq!(
            StockCommand::process_args_with_provider("stock finviz META", "yahoo"),
            "https://finviz.com/quote.ashx?t=META"
        );
    }

    #[test]
    fn test_stock_command_tradingview_alias() {
        assert_eq!(
            StockCommand::process_args_with_provider("stock tv AAPL", "yahoo"),
            "https://www.tradingview.com/symbols/AAPL/"
        );
    }

    #[test]
    fn test_stock_command_google_alias() {
        assert_eq!(
            StockCommand::process_args_with_provider("stock gf META:NASDAQ", "yahoo"),
            "https://www.google.com/finance/quote/META:NASDAQ"
        );
    }

    // Config-based defaults via explicit provider
    #[test]
    fn test_stock_command_with_finviz_default() {
        assert_eq!(
            StockCommand::process_args_with_provider("stock META", "finviz"),
            "https://finviz.com/quote.ashx?t=META"
        );
    }

    #[test]
    fn test_stock_command_no_ticker_with_finviz_default() {
        assert_eq!(
            StockCommand::process_args_with_provider("stock", "finviz"),
            "https://finviz.com/"
        );
    }

    #[test]
    fn test_stock_command_with_equals() {
        assert_eq!(
            StockCommand::process_args_with_provider("stock RTY=F", "yahoo"),
            "https://finance.yahoo.com/quote/RTY%3DF/"
        );
    }

    // Override in query beats config default
    #[test]
    fn test_stock_command_override_beats_config() {
        assert_eq!(
            StockCommand::process_args_with_provider("stock yahoo META", "finviz"),
            "https://finance.yahoo.com/quote/META/"
        );
    }

    // $TICKER syntax
    #[test]
    fn test_dollar_ticker_default() {
        assert_eq!(
            StockCommand::process_ticker_with_provider("$META", "yahoo"),
            "https://finance.yahoo.com/quote/META/"
        );
    }

    #[test]
    fn test_dollar_ticker_with_finviz() {
        assert_eq!(
            StockCommand::process_ticker_with_provider("$AAPL", "finviz"),
            "https://finviz.com/quote.ashx?t=AAPL"
        );
    }

    // Special characters
    #[test]
    fn test_stock_command_special_chars() {
        assert_eq!(
            StockCommand::process_args_with_provider("stock BRK.B", "yahoo"),
            "https://finance.yahoo.com/quote/BRK%2EB/"
        );
    }

    #[test]
    fn test_stock_ticker_prefix_edge_case_empty_ticker() {
        assert_eq!(
            StockCommand::process_ticker_with_provider("$", "yahoo"),
            "https://finance.yahoo.com/"
        );
    }

    #[test]
    fn test_stock_ticker_prefix_edge_case_empty_string() {
        assert_eq!(
            StockCommand::process_ticker_with_provider("", "yahoo"),
            "https://finance.yahoo.com/"
        );
    }
}
