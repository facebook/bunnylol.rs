/// Amazon command handler
/// Supports:
/// - az/amzn/azn/amazon -> https://amazon.com/
/// - az orders -> https://www.amazon.com/gp/css/order-history?ref_=nav_orders_first
/// - az account -> https://www.amazon.com/gp/css/homepage.html?ref_=nav_youraccount_btn
/// - az messages -> https://www.amazon.com/gp/message
/// - az cart -> https://www.amazon.com/gp/cart/view.html/
/// - az pay/wallet -> https://www.amazon.com/cpe/yourpayments/wallet
/// - az [search terms] -> https://www.amazon.com/s?k=[search terms]
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct AmazonCommand;

impl BunnylolCommand for AmazonCommand {
    const BINDINGS: &'static [&'static str] = &["az", "amzn", "azn", "amazon"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://amazon.com/".to_string()
        } else {
            match query {
                "orders" => {
                    "https://www.amazon.com/gp/css/order-history?ref_=nav_orders_first".to_string()
                }
                "account" => "https://www.amazon.com/gp/css/homepage.html?ref_=nav_youraccount_btn"
                    .to_string(),
                "messages" => "https://www.amazon.com/gp/message".to_string(),
                "cart" => "https://www.amazon.com/gp/cart/view.html/".to_string(),
                "pay" | "wallet" => "https://www.amazon.com/cpe/yourpayments/wallet".to_string(),
                _ => build_search_url("https://www.amazon.com/s", "k", query),
            }
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Amazon or search for products".to_string(),
            example: "az headphones".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amazon_command_base() {
        assert_eq!(AmazonCommand::process_args("az"), "https://amazon.com/");
        assert_eq!(AmazonCommand::process_args("amzn"), "https://amazon.com/");
        assert_eq!(AmazonCommand::process_args("azn"), "https://amazon.com/");
        assert_eq!(AmazonCommand::process_args("amazon"), "https://amazon.com/");
    }

    #[test]
    fn test_amazon_command_search() {
        assert_eq!(
            AmazonCommand::process_args("az headphones"),
            "https://www.amazon.com/s?k=headphones"
        );
        assert_eq!(
            AmazonCommand::process_args("amazon wireless mouse"),
            "https://www.amazon.com/s?k=wireless%20mouse"
        );
    }

    #[test]
    fn test_amazon_command_subcommands() {
        assert_eq!(
            AmazonCommand::process_args("az orders"),
            "https://www.amazon.com/gp/css/order-history?ref_=nav_orders_first"
        );
        assert_eq!(
            AmazonCommand::process_args("az account"),
            "https://www.amazon.com/gp/css/homepage.html?ref_=nav_youraccount_btn"
        );
        assert_eq!(
            AmazonCommand::process_args("az messages"),
            "https://www.amazon.com/gp/message"
        );
        assert_eq!(
            AmazonCommand::process_args("az cart"),
            "https://www.amazon.com/gp/cart/view.html/"
        );
        assert_eq!(
            AmazonCommand::process_args("az pay"),
            "https://www.amazon.com/cpe/yourpayments/wallet"
        );
        assert_eq!(
            AmazonCommand::process_args("az wallet"),
            "https://www.amazon.com/cpe/yourpayments/wallet"
        );
    }
}
