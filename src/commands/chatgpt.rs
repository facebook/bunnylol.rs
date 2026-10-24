/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/// ChatGPT command handler
/// Supports:
/// - chatgpt -> https://chatgpt.com
/// - chatgpt [prompt] -> https://chatgpt.com/?q=[prompt] (new chat with the prompt)
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct ChatGPTCommand;

impl BunnylolCommand for ChatGPTCommand {
    const BINDINGS: &'static [&'static str] = &["chatgpt"];

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        if query.is_empty() {
            "https://chatgpt.com".to_string()
        } else {
            build_search_url("https://chatgpt.com/", "q", query)
        }
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to ChatGPT or start a chat with a prompt",
            "chatgpt explain rust lifetimes",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chatgpt_command() {
        assert_eq!(
            ChatGPTCommand::process_args("chatgpt"),
            "https://chatgpt.com"
        );
    }

    #[test]
    fn test_chatgpt_command_with_args() {
        assert_eq!(
            ChatGPTCommand::process_args("chatgpt some args"),
            "https://chatgpt.com/?q=some%20args"
        );
    }

    #[test]
    fn test_chatgpt_prompt_special_chars() {
        assert_eq!(
            ChatGPTCommand::process_args("chatgpt is 2+2=4 & why?"),
            "https://chatgpt.com/?q=is%202%2B2%3D4%20%26%20why?"
        );
    }
}
