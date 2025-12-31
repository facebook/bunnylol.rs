/// Charles Schwab command
///
/// Shortcuts to various Charles Schwab pages:
/// - Base: Account summary
/// - billpay: Bill pay page
/// - orders: Order status page
/// - trade: Trading page
/// - transfer/transfers/payments: Transfers and payments page
/// - security: Security settings page
/// - contact/contactus/call: Contact us page
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct SchwabCommand;

impl BunnylolCommand for SchwabCommand {
    const BINDINGS: &'static [&'static str] = &["schwab"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);

        match query {
            "billpay" => "https://client.schwab.com/app/accounts/billpay/#/billpay".to_string(),
            "orders" => "https://client.schwab.com/app/trade/orderstatus/#/orderstatus".to_string(),
            "trade" => "https://client.schwab.com/app/trade/tom/trade".to_string(),
            "transfer" | "transfers" | "payments" => {
                "https://client.schwab.com/app/accounts/transfers_and_payments_overview/#/"
                    .to_string()
            }
            "security" => "https://client.schwab.com/app/access/securitysettings".to_string(),
            "contact" | "contactus" | "call" => {
                "https://client.schwab.com/app/service/contactus/contactus".to_string()
            }
            _ => "https://client.schwab.com/app/accounts/summary/".to_string(),
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description:
                "Charles Schwab shortcuts (billpay, orders, trade, transfer, security, contact)"
                    .to_string(),
            example: "schwab trade".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schwab_base() {
        assert_eq!(
            SchwabCommand::process_args("schwab"),
            "https://client.schwab.com/app/accounts/summary/"
        );
    }

    #[test]
    fn test_schwab_billpay() {
        assert_eq!(
            SchwabCommand::process_args("schwab billpay"),
            "https://client.schwab.com/app/accounts/billpay/#/billpay"
        );
    }

    #[test]
    fn test_schwab_orders() {
        assert_eq!(
            SchwabCommand::process_args("schwab orders"),
            "https://client.schwab.com/app/trade/orderstatus/#/orderstatus"
        );
    }

    #[test]
    fn test_schwab_trade() {
        assert_eq!(
            SchwabCommand::process_args("schwab trade"),
            "https://client.schwab.com/app/trade/tom/trade"
        );
    }

    #[test]
    fn test_schwab_transfer() {
        assert_eq!(
            SchwabCommand::process_args("schwab transfer"),
            "https://client.schwab.com/app/accounts/transfers_and_payments_overview/#/"
        );
    }

    #[test]
    fn test_schwab_transfers() {
        assert_eq!(
            SchwabCommand::process_args("schwab transfers"),
            "https://client.schwab.com/app/accounts/transfers_and_payments_overview/#/"
        );
    }

    #[test]
    fn test_schwab_payments() {
        assert_eq!(
            SchwabCommand::process_args("schwab payments"),
            "https://client.schwab.com/app/accounts/transfers_and_payments_overview/#/"
        );
    }

    #[test]
    fn test_schwab_security() {
        assert_eq!(
            SchwabCommand::process_args("schwab security"),
            "https://client.schwab.com/app/access/securitysettings"
        );
    }

    #[test]
    fn test_schwab_contact() {
        assert_eq!(
            SchwabCommand::process_args("schwab contact"),
            "https://client.schwab.com/app/service/contactus/contactus"
        );
    }

    #[test]
    fn test_schwab_contactus() {
        assert_eq!(
            SchwabCommand::process_args("schwab contactus"),
            "https://client.schwab.com/app/service/contactus/contactus"
        );
    }

    #[test]
    fn test_schwab_call() {
        assert_eq!(
            SchwabCommand::process_args("schwab call"),
            "https://client.schwab.com/app/service/contactus/contactus"
        );
    }
}
