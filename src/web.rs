/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};

use crate::utils::bunnylol_command::{BunnylolCommandRegistry, CommandInfo};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct BindingData {
    pub command: String,
    pub description: String,
    pub example: String,
}

impl From<CommandInfo> for BindingData {
    fn from(info: CommandInfo) -> Self {
        Self {
            command: info.bindings.first().unwrap_or(&"(default)".to_string()).clone(),
            description: info.description,
            example: info.example,
        }
    }
}

#[component]
fn BindingCard(binding: BindingData) -> impl IntoView {
    view! {
        <div class="binding-card">
            <div class="command-name">
                {binding.command}
            </div>
            <div class="command-description">
                {binding.description}
            </div>
            <div class="example-box">
                <div class="example-label">
                    "Example:"
                </div>
                <div class="example-text">
                    {binding.example}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn BindingsPage() -> impl IntoView {
    let mut bindings: Vec<BindingData> = BunnylolCommandRegistry::get_all_commands()
        .into_iter()
        .map(|cmd| cmd.into())
        .collect();

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
                    background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
                    border-radius: 8px;
                    padding: 20px;
                    transition: transform 0.2s, box-shadow 0.2s;
                    border: 2px solid #e0e0e0;
                    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                    cursor: pointer;
                }
                .binding-card:hover {
                    transform: translateY(-5px);
                    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
                }
                .command-name {
                    font-family: 'JetBrains Mono', monospace;
                    font-size: 1.4em;
                    font-weight: 700;
                    color: #667eea;
                    margin-bottom: 10px;
                    background: white;
                    padding: 8px 12px;
                    border-radius: 4px;
                    display: inline-block;
                }
                .command-description {
                    color: #333;
                    margin-bottom: 15px;
                    line-height: 1.5;
                }
                .example-box {
                    background: white;
                    padding: 10px;
                    border-radius: 4px;
                    border-left: 3px solid #667eea;
                }
                .example-label {
                    font-size: 0.85em;
                    color: #666;
                    margin-bottom: 5px;
                    font-weight: 600;
                }
                .example-text {
                    font-family: 'JetBrains Mono', monospace;
                    color: #764ba2;
                    font-weight: 500;
                }
                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                    background: white;
                    border-radius: 12px;
                    padding: 30px;
                    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
                    font-family: 'JetBrains Mono', monospace;
                }
                .page-title {
                    color: #333;
                    text-align: center;
                    margin-bottom: 10px;
                    font-size: 2.5em;
                }
                .page-subtitle {
                    text-align: center;
                    color: #666;
                    margin-bottom: 30px;
                    font-size: 1.1em;
                }
                .bindings-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
                    gap: 20px;
                    margin-top: 30px;
                }
                .footer {
                    text-align: center;
                    margin-top: 40px;
                    color: #666;
                    font-size: 0.9em;
                }
                .info-box {
                    background: #f5f7fa;
                    padding: 15px;
                    border-radius: 6px;
                    margin-top: 15px;
                    border: 1px solid #e0e0e0;
                }
                .info-box-title {
                    font-weight: 600;
                    margin-bottom: 8px;
                    color: #333;
                }
                .example-code {
                    font-family: 'JetBrains Mono', monospace;
                    background: white;
                    padding: 8px 12px;
                    border-radius: 4px;
                    display: inline-block;
                    color: #667eea;
                    border: 1px solid #667eea;
                }
                .setup-guides {
                    background: #f5f7fa;
                    padding: 20px;
                    border-radius: 6px;
                    margin-top: 20px;
                    border: 1px solid #e0e0e0;
                }
                .setup-guides-title {
                    font-weight: 600;
                    margin-bottom: 12px;
                    color: #333;
                    font-size: 1.1em;
                }
                .setup-guides-content {
                    color: #666;
                    line-height: 1.8;
                }
                .setup-guides-content p {
                    margin-bottom: 10px;
                }
                .setup-guides-list {
                    margin-left: 15px;
                }
                .setup-guides-list p {
                    margin-bottom: 5px;
                }
                .setup-guides-list p:last-child {
                    margin-bottom: 0;
                }
                .setup-link {
                    color: #667eea;
                    text-decoration: none;
                    font-weight: 500;
                }
                .setup-link:hover {
                    text-decoration: underline;
                }
            "#
        </Style>

        <div class="container">
            <h1 class="page-title">
                "🐰 Bunnylol Command Bindings"
            </h1>
            <div class="page-subtitle">
                "All currently registered URL shortcuts"
            </div>

            <div class="bindings-grid">
                <For
                    each=move || bindings.clone()
                    key=|binding| binding.command.clone()
                    children=|binding| view! { <BindingCard binding=binding /> }
                />
            </div>

            <footer class="footer">
                <p style:margin-bottom="10px">"💡 Tip: Use these commands in your browser search bar to quickly navigate to your favorite sites!"</p>
                <div class="info-box">
                    <div class="info-box-title">"Example URL:"</div>
                    <code class="example-code">
                        "http://localhost:8000/?cmd=gh facebook/bunnylol.rs"
                    </code>
                </div>

                <div class="setup-guides">
                    <div class="setup-guides-title">
                        "📚 Setup Guides"
                    </div>
                    <div class="setup-guides-content">
                        <p>"Set bunnylol as your default search engine to use these commands directly from your address bar:"</p>
                        <div class="setup-guides-list">
                            <p>
                                "🖥️ Desktop Chrome: "
                                <a
                                    class="setup-link"
                                    href="https://support.google.com/chrome/answer/95426?hl=en&co=GENIE.Platform%3DDesktop"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    "Setup Guide"
                                </a>
                            </p>
                            <p>
                                "🦊 Desktop Firefox: "
                                <a
                                    class="setup-link"
                                    href="https://support.mozilla.org/en-US/kb/add-custom-search-engine-firefox"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    "Setup Guide"
                                </a>
                            </p>
                            <p>
                                "📱 Mobile (Firefox on iOS): "
                                <a
                                    class="setup-link"
                                    href="https://support.mozilla.org/en-US/kb/change-your-default-search-engine-firefox-ios"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    "Setup Guide"
                                </a>
                            </p>
                        </div>
                    </div>
                </div>
            </footer>
        </div>
    }
}
