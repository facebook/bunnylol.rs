/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::OnceLock;

use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{BunnylolCommandInfo, BunnylolCommandRegistry, BunnylolConfig};

static LANDING_PAGE_HTML_CACHE: OnceLock<String> = OnceLock::new();

/// Render the landing page HTML with the given config
pub fn render_landing_page_html(config: &BunnylolConfig) -> String {
    LANDING_PAGE_HTML_CACHE
        .get_or_init(|| {
            let display_url = config.server.get_display_url();
            let body_content = leptos::ssr::render_to_string(move || {
                view! {
                    <LandingPage server_display_url=display_url.clone() />
                }
            })
            .to_string();

            // Wrap in proper HTML document with favicon
            format!(
                r#"<!DOCTYPE html>
                    <html lang="en">
                    <head>
                        <meta charset="UTF-8">
                        <meta name="viewport" content="width=device-width, initial-scale=1.0">
                        <title>bunnylol</title>
                        <link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🐰</text></svg>">
                        <link rel="preconnect" href="https://fonts.googleapis.com">
                        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                        <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;700&display=swap" rel="stylesheet">
                        <style>
                            :root {{
                                --gradient-start: #87CEEB;
                                --gradient-end: #6D28D9;
                                --accent-blue: #008ECD;
                                --accent-purple: #532ED1;
                                --text-gray: #9CA3AF;
                                --text-dark: #333;
                                --text-medium: #666;
                                --text-light: #888;
                                --bg-white: white;
                                --bg-light-gray: #f5f7fa;
                                --bg-gradient-gray: #c3cfe2;
                                --border-light: #e0e0e0;
                            }}
                            * {{ margin: 0; padding: 0; box-sizing: border-box; }}
                            body {{
                                font-family: 'JetBrains Mono', monospace;
                                background: linear-gradient(135deg, var(--gradient-start) 0%, var(--gradient-end) 100%);
                                background-attachment: fixed;
                                min-height: 100vh;
                                padding: 20px;
                            }}
                            .binding-card {{
                                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                                cursor: pointer;
                            }}
                            .binding-card:hover {{
                                transform: translateY(-5px);
                                box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
                            }}
                        </style>
                    </head>
                    <body>
                        {}
                    </body>
                </html>"#,
                body_content
            )
        })
        .clone()
}

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
            style:background="linear-gradient(135deg, var(--bg-light-gray) 0%, var(--bg-gradient-gray) 100%)"
            style:border-radius="8px"
            style:padding="20px"
            style:transition="transform 0.2s, box-shadow 0.2s"
            style:border="2px solid var(--border-light)"
        >
            <div
                style:font-family="'JetBrains Mono', monospace"
                style:font-size="1.4em"
                style:font-weight="700"
                style:color="var(--accent-blue)"
                style:margin-bottom="10px"
                style:background="var(--bg-white)"
                style:padding="8px 12px"
                style:border-radius="4px"
                style:display="inline-block"
            >
                {binding.command}
            </div>
            <div
                style:color="var(--text-dark)"
                style:margin-bottom="15px"
                style:line-height="1.5"
            >
                {binding.description}
            </div>
            <div
                style:background="var(--bg-white)"
                style:padding="10px"
                style:border-radius="4px"
                style:border-left="3px solid var(--accent-blue)"
            >
                <div
                    style:font-size="0.85em"
                    style:color="var(--text-medium)"
                    style:margin-bottom="5px"
                    style:font-weight="600"
                >
                    "Example:"
                </div>
                <div
                    style:font-family="'JetBrains Mono', monospace"
                    style:color="var(--accent-purple)"
                    style:font-weight="500"
                >
                    {binding.example}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn LandingPage(server_display_url: String) -> impl IntoView {
    let mut bindings: Vec<BindingData> = BunnylolCommandRegistry::get_all_commands()
        .iter()
        .map(|cmd| (*cmd).clone().into())
        .collect();

    // Sort bindings alphabetically by command name
    bindings.sort_by(|a, b| a.command.to_lowercase().cmp(&b.command.to_lowercase()));

    // Clone server_display_url for use in the view
    let example_url = format!("{}/?cmd=gh facebook/bunnylol.rs", server_display_url);

    view! {
        <div
            style:max-width="1200px"
            style:margin="0 auto 10px auto"
            style:background="var(--bg-white)"
            style:border-radius="12px"
            style:padding="20px 30px 30px 30px"
            style:box-shadow="0 20px 60px rgba(0, 0, 0, 0.3)"
            style:font-family="'JetBrains Mono', monospace"
        >
            <h1
                style:color="var(--text-dark)"
                style:text-align="center"
                style:margin-bottom="2px"
                style:margin-top="5px"
                style:font-size="3em"
                style:font-weight="700"
            >
                "bunnylol"
            </h1>
            <div
                style:text-align="center"
                style:margin-bottom="20px"
            >
                <a
                    href="https://github.com/facebook/bunnylol.rs"
                    target="_blank"
                    rel="noopener noreferrer"
                    style:color="var(--accent-blue)"
                    style:text-decoration="none"
                    style:font-size="0.95em"
                    style:font-weight="500"
                    style:display="inline-flex"
                    style:align-items="center"
                    style:gap="6px"
                    style:transition="all 0.2s"
                >
                    // GitHub icon SVG
                    <svg
                        width="20"
                        height="20"
                        viewBox="0 0 16 16"
                        fill="currentColor"
                        style:display="inline-block"
                    >
                        <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
                    </svg>
                    <span>
                        <span style:color="var(--accent-purple)" style:font-weight="600">"facebook"</span>
                        <span style:color="var(--text-dark)" style:padding-left="2px" style:padding-right="2px">"/"</span>
                        <span style:color="var(--accent-blue)" style:font-weight="600">"bunnylol.rs"</span>
                    </span>
                </a>
            </div>

            // Web Usage section
            <div
                style:background="var(--bg-light-gray)"
                style:padding="20px"
                style:border-radius="6px"
                style:margin-bottom="20px"
                style:border="1px solid var(--border-light)"
            >
                <div style:max-width="700px" style:margin="0 auto" style:color="var(--text-medium)" style:line-height="1.6" style:text-align="center">
                    <p style:margin-bottom="10px">
                        "This server is available at "
                        <code
                            style:font-family="'JetBrains Mono', monospace"
                            style:background="var(--bg-white)"
                            style:padding="4px 8px"
                            style:border-radius="4px"
                            style:color="var(--text-dark)"
                            style:border="1px solid var(--border-light)"
                            style:font-size="0.9em"
                        >
                            {server_display_url.clone()}
                        </code>
                        ", so try:"
                    </p>
                    <a
                        href={example_url.clone()}
                        target="_blank"
                        rel="noopener noreferrer"
                        style:font-family="'JetBrains Mono', monospace"
                        style:background="var(--bg-white)"
                        style:padding="12px 16px"
                        style:border-radius="4px"
                        style:display="inline-block"
                        style:color="var(--accent-blue)"
                        style:border="1px solid var(--accent-blue)"
                        style:text-decoration="none"
                        style:transition="all 0.2s"
                        style:font-size="0.9em"
                    >{example_url.clone()}</a>

                    // Setup guides within web usage section
                    <div style:margin-top="15px">
                        <div style:font-weight="600" style:margin-bottom="15px" style:color="var(--text-dark)" style:font-size="1em" style:text-align="center">
                            "Set bunnylol as your default search engine!"
                        </div>
                        <p style:margin-bottom="15px" style:text-align="center" style:color="var(--text-medium)" style:line-height="1.8" style:max-width="800" style:margin-left="auto" style:margin-right="auto">
                            "Once configured, just enter "
                            <code
                                style:font-family="'JetBrains Mono', monospace"
                                style:background="var(--bg-white)"
                                style:padding="4px 8px"
                                style:border-radius="4px"
                                style:color="var(--text-dark)"
                                style:border="1px solid var(--border-light)"
                                style:font-size="0.9em"
                                style:white-space="nowrap"
                            >
                                "gh facebook/bunnylol.rs"
                            </code>
                            " in your address bar to get the same result."
                        </p>
                        <p style:margin-bottom="15px" style:text-align="center" style:color="var(--text-medium)" style:line-height="1.8" style:max-width="800" style:margin-left="auto" style:margin-right="auto">
                            "Use this URL as your search engine: "
                            <code
                                style:font-family="'JetBrains Mono', monospace"
                                style:background="var(--bg-white)"
                                style:padding="4px 8px"
                                style:border-radius="4px"
                                style:color="var(--text-dark)"
                                style:border="1px solid var(--border-light)"
                                style:font-size="0.9em"
                                style:white-space="nowrap"
                            >
                                {format!("{}/?cmd=%s", server_display_url)}
                            </code>
                        </p>
                        <div style:color="var(--text-medium)" style:line-height="1.8" style:max-width="600px" style:margin="0 auto">
                            <div style:display="grid" style:grid-template-columns="repeat(auto-fit, minmax(200px, 1fr))" style:gap="10px" style:margin-bottom="15px">
                                <div style:text-align="center">
                                    "🖥️ "
                                    <a
                                        href="https://support.google.com/chrome/answer/95426?hl=en&co=GENIE.Platform%3DDesktop"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        style:color="var(--accent-blue)"
                                        style:text-decoration="none"
                                        style:font-weight="500"
                                    >
                                        "Desktop Chrome"
                                    </a>
                                </div>
                                <div style:text-align="center">
                                    "🦊 "
                                    <a
                                        href="https://support.mozilla.org/en-US/kb/add-custom-search-engine-firefox"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        style:color="var(--accent-blue)"
                                        style:text-decoration="none"
                                        style:font-weight="500"
                                    >
                                        "Desktop Firefox"
                                    </a>
                                </div>
                                <div style:text-align="center">
                                    "📱 "
                                    <a
                                        href="https://support.mozilla.org/en-US/kb/change-your-default-search-engine-firefox-ios"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        style:color="var(--accent-blue)"
                                        style:text-decoration="none"
                                        style:font-weight="500"
                                    >
                                        "iOS Firefox"
                                    </a>
                                </div>
                                <div style:text-align="center">
                                    "📱 "
                                    <a
                                        href="https://support.mozilla.org/en-US/kb/manage-my-default-search-engines-firefox-android"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        style:color="var(--accent-blue)"
                                        style:text-decoration="none"
                                        style:font-weight="500"
                                    >
                                        "Android Firefox"
                                    </a>
                                </div>
                            </div>
                            <p style:font-size="0.85em" style:margin-top="10px" style:color="var(--text-light)" style:font-style="italic" style:text-align="center">
                                "Note: iOS Safari does not support custom search engines."
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            <div
                style:text-align="center"
                style:color="var(--text-medium)"
                style:margin-bottom="20px"
                style:font-size="1.1em"
                style:font-weight="600"
            >
                "Available Commands"
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
        </div>
    }
}
