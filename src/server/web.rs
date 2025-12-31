/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};

use crate::{BunnylolCommandInfo, BunnylolCommandRegistry};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct BindingData {
    pub command: String,
    pub description: String,
    pub example: String,
}

impl From<BunnylolCommandInfo> for BindingData {
    fn from(info: BunnylolCommandInfo) -> Self {
        Self {
            command: info
                .bindings
                .first()
                .unwrap_or(&"(default)".to_string())
                .clone(),
            description: info.description,
            example: info.example,
        }
    }
}

#[component]
fn BindingCard(binding: BindingData) -> impl IntoView {
    view! {
        <div
            class="binding-card"
            style:background="linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)"
            style:border-radius="8px"
            style:padding="20px"
            style:transition="transform 0.2s, box-shadow 0.2s"
            style:border="2px solid #e0e0e0"
        >
            <div
                style:font-family="'JetBrains Mono', monospace"
                style:font-size="1.4em"
                style:font-weight="700"
                style:color="#667eea"
                style:margin-bottom="10px"
                style:background="white"
                style:padding="8px 12px"
                style:border-radius="4px"
                style:display="inline-block"
            >
                {binding.command}
            </div>
            <div
                style:color="#333"
                style:margin-bottom="15px"
                style:line-height="1.5"
            >
                {binding.description}
            </div>
            <div
                style:background="white"
                style:padding="10px"
                style:border-radius="4px"
                style:border-left="3px solid #667eea"
            >
                <div
                    style:font-size="0.85em"
                    style:color="#666"
                    style:margin-bottom="5px"
                    style:font-weight="600"
                >
                    "Example:"
                </div>
                <div
                    style:font-family="'JetBrains Mono', monospace"
                    style:color="#764ba2"
                    style:font-weight="500"
                >
                    {binding.example}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn BindingsPage(
    #[prop(default = None)] config: Option<crate::config::BunnylolConfig>,
) -> impl IntoView {
    let commands = if let Some(ref cfg) = config {
        BunnylolCommandRegistry::get_all_commands_filtered(Some(cfg))
    } else {
        BunnylolCommandRegistry::get_all_commands().clone()
    };

    let mut bindings: Vec<BindingData> = commands.iter().map(|cmd| (*cmd).clone().into()).collect();

    // Sort bindings alphabetically by command name
    bindings.sort_by(|a, b| a.command.to_lowercase().cmp(&b.command.to_lowercase()));

    view! {
        <Html lang="en"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Title text="Bunnylol Command Bindings"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;700&display=swap" rel="stylesheet"/>
        <Style>
            r#"
                * { margin: 0; padding: 0; box-sizing: border-box; }
                body, html { height: 100%; width: 100%; }
                body {
                    font-family: 'JetBrains Mono', monospace;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    min-height: 100vh;
                    padding: 20px;
                }
                .binding-card {
                    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                    cursor: pointer;
                }
                .binding-card:hover {
                    transform: translateY(-5px);
                    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
                }
            "#
        </Style>

        <div
            style:max-width="1200px"
            style:margin="0 auto"
            style:background="white"
            style:border-radius="12px"
            style:padding="30px"
            style:box-shadow="0 20px 60px rgba(0, 0, 0, 0.3)"
            style:font-family="'JetBrains Mono', monospace"
        >
            <h1
                style:color="#333"
                style:text-align="center"
                style:margin-bottom="10px"
                style:font-size="2.5em"
            >
                "🐰 Bunnylol Command Bindings"
            </h1>
            <div
                style:text-align="center"
                style:color="#666"
                style:margin-bottom="30px"
                style:font-size="1.1em"
            >
                "All currently registered URL shortcuts"
            </div>

            <div
                style:display="grid"
                style:grid-template-columns="repeat(auto-fill, minmax(350px, 1fr))"
                style:gap="20px"
                style:margin-top="30px"
            >
                <For
                    each=move || bindings.clone()
                    key=|binding| binding.command.clone()
                    children=|binding| view! { <BindingCard binding=binding /> }
                />
            </div>

            <footer
                style:text-align="center"
                style:margin-top="40px"
                style:color="#666"
                style:font-size="0.9em"
            >
                <p style:margin-bottom="10px">"💡 Tip: Use these commands in your browser search bar to quickly navigate to your favorite sites!"</p>
                <div
                    style:background="#f5f7fa"
                    style:padding="15px"
                    style:border-radius="6px"
                    style:margin-top="15px"
                    style:border="1px solid #e0e0e0"
                >
                    <div style:font-weight="600" style:margin-bottom="8px" style:color="#333">"Example URL:"</div>
                    <code
                        style:font-family="'JetBrains Mono', monospace"
                        style:background="white"
                        style:padding="8px 12px"
                        style:border-radius="4px"
                        style:display="inline-block"
                        style:color="#667eea"
                        style:border="1px solid #667eea"
                    >
                        "http://localhost:8000/?cmd=gh facebook/bunnylol.rs"
                    </code>
                    <p style:font-size="0.85em" style:margin-top="10px" style:color="#888" style:line-height="1.5">
                        "Note: This example assumes you've deployed bunnylol locally. For best results, deploy bunnylol on a networked server accessible from all your devices."
                    </p>
                </div>

                <div
                    style:background="#f5f7fa"
                    style:padding="20px"
                    style:border-radius="6px"
                    style:margin-top="20px"
                    style:border="1px solid #e0e0e0"
                >
                    <div style:font-weight="600" style:margin-bottom="12px" style:color="#333" style:font-size="1.1em">
                        "📚 Setup Guides"
                    </div>
                    <div style:color="#666" style:line-height="1.8">
                        <p style:margin-bottom="10px">"Set bunnylol as your default search engine to use these commands directly from your address bar:"</p>
                        <div style:margin-left="15px">
                            <p style:margin-bottom="5px">
                                "🖥️ Desktop Chrome: "
                                <a
                                    href="https://support.google.com/chrome/answer/95426?hl=en&co=GENIE.Platform%3DDesktop"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    style:color="#667eea"
                                    style:text-decoration="none"
                                    style:font-weight="500"
                                >
                                    "Setup Guide"
                                </a>
                            </p>
                            <p style:margin-bottom="5px">
                                "🦊 Desktop Firefox: "
                                <a
                                    href="https://support.mozilla.org/en-US/kb/add-custom-search-engine-firefox"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    style:color="#667eea"
                                    style:text-decoration="none"
                                    style:font-weight="500"
                                >
                                    "Setup Guide"
                                </a>
                            </p>
                            <p style:margin-bottom="5px">
                                "📱 iOS Firefox: "
                                <a
                                    href="https://support.mozilla.org/en-US/kb/change-your-default-search-engine-firefox-ios"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    style:color="#667eea"
                                    style:text-decoration="none"
                                    style:font-weight="500"
                                >
                                    "Setup Guide"
                                </a>
                            </p>
                            <p>
                                "📱 Android Firefox: "
                                <a
                                    href="https://support.mozilla.org/en-US/kb/manage-my-default-search-engines-firefox-android"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    style:color="#667eea"
                                    style:text-decoration="none"
                                    style:font-weight="500"
                                >
                                    "Setup Guide"
                                </a>
                            </p>
                        </div>
                        <p style:font-size="0.85em" style:margin-top="12px" style:color="#888" style:font-style="italic">
                            "Note: iOS Safari does not support custom search engines. Please use Firefox on iOS instead."
                        </p>
                    </div>
                </div>
            </footer>
        </div>
    }
}
