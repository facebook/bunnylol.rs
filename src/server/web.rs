/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use leptos::*;
use leptos_meta::*;
use rocket::response::content::RawHtml;
use serde::{Deserialize, Serialize};

use super::stats::UsageStats;
use crate::{BunnylolCommandInfo, BunnylolCommandRegistry, BunnylolConfig};

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
pub fn BindingsPage() -> impl IntoView {
    let mut bindings: Vec<BindingData> = BunnylolCommandRegistry::get_all_commands()
        .iter()
        .map(|cmd| (*cmd).clone().into())
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

#[component]
pub fn StatsPage(stats: UsageStats) -> impl IntoView {
    view! {
        <Html lang="en"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Title text="Bunnylol Usage Statistics"/>
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
                .bar {
                    background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
                    height: 30px;
                    margin-bottom: 8px;
                    border-radius: 4px;
                    display: flex;
                    align-items: center;
                    padding: 0 12px;
                    color: white;
                    font-weight: 600;
                    transition: all 0.3s ease;
                }
                .bar:hover {
                    transform: scaleX(1.02);
                    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
                }
                .stat-card {
                    background: white;
                    border-radius: 8px;
                    padding: 20px;
                    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
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
        >
            <h1
                style:color="#333"
                style:text-align="center"
                style:margin-bottom="10px"
                style:font-size="2.5em"
            >
                "📊 Usage Statistics"
            </h1>
            <div
                style:text-align="center"
                style:color="#666"
                style:margin-bottom="30px"
                style:font-size="1.1em"
            >
                "Your bunnylol command history at a glance"
            </div>

            // Summary stats
            <div
                style:display="grid"
                style:grid-template-columns="repeat(auto-fit, minmax(200px, 1fr))"
                style:gap="20px"
                style:margin-bottom="40px"
            >
                <div class="stat-card">
                    <div style:font-size="2.5em" style:font-weight="700" style:color="#667eea" style:text-align="center">
                        {stats.total_commands.to_string()}
                    </div>
                    <div style:text-align="center" style:color="#666" style:margin-top="8px">
                        "Total Commands"
                    </div>
                </div>
                <div class="stat-card">
                    <div style:font-size="2.5em" style:font-weight="700" style:color="#764ba2" style:text-align="center">
                        {stats.unique_commands.to_string()}
                    </div>
                    <div style:text-align="center" style:color="#666" style:margin-top="8px">
                        "Unique Commands"
                    </div>
                </div>
            </div>

            // Top 10 commands
            <div style:margin-bottom="40px">
                <h2 style:color="#333" style:margin-bottom="20px" style:font-size="1.8em">
                    "🏆 Top 10 Commands"
                </h2>
                <div style:background="#f5f7fa" style:padding="20px" style:border-radius="8px">
                    {if stats.top_commands.is_empty() {
                        view! {
                            <p style:text-align="center" style:color="#666" style:padding="20px">
                                "No command history yet. Start using bunnylol to see stats!"
                            </p>
                        }.into_view()
                    } else {
                        let max_count = stats.top_commands.first().map(|c| c.count).unwrap_or(1);
                        view! {
                            <For
                                each=move || stats.top_commands.clone()
                                key=|cmd| cmd.command.clone()
                                children=move |cmd| {
                                    let width_percent = (cmd.count as f64 / max_count as f64 * 100.0).min(100.0);
                                    view! {
                                        <div style:margin-bottom="12px">
                                            <div style:display="flex" style:justify-content="space-between" style:margin-bottom="4px">
                                                <span style:font-weight="600" style:color="#333">{cmd.command.clone()}</span>
                                                <span style:color="#666">{format!("{} uses ({}%)", cmd.count, cmd.percentage.round() as i32)}</span>
                                            </div>
                                            <div
                                                class="bar"
                                                style=format!("width: {}%", width_percent)
                                            >
                                            </div>
                                        </div>
                                    }
                                }
                            />
                        }.into_view()
                    }}
                </div>
            </div>

            // Least used commands
            {if !stats.least_used_commands.is_empty() {
                view! {
                    <div>
                        <h2 style:color="#333" style:margin-bottom="20px" style:font-size="1.8em">
                            "📉 Least Used Commands"
                        </h2>
                        <div style:background="#f5f7fa" style:padding="20px" style:border-radius="8px">
                            <p style:color="#666" style:margin-bottom="15px">
                                "These commands haven't seen much action. Consider removing them or using them more!"
                            </p>
                            <For
                                each=move || stats.least_used_commands.clone()
                                key=|cmd| cmd.command.clone()
                                children=move |cmd| {
                                    view! {
                                        <div style:display="flex" style:justify-content="space-between" style:padding="8px 12px" style:background="white" style:margin-bottom="6px" style:border-radius="4px">
                                            <span style:font-weight="600" style:color="#333">{cmd.command.clone()}</span>
                                            <span style:color="#666">{format!("{} uses", cmd.count)}</span>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </div>
                }.into_view()
            } else {
                view! { <div></div> }.into_view()
            }}

            <div style:margin-top="30px" style:text-align="center">
                <a
                    href="/bindings"
                    style:display="inline-block"
                    style:padding="12px 24px"
                    style:background="linear-gradient(135deg, #667eea 0%, #764ba2 100%)"
                    style:color="white"
                    style:text-decoration="none"
                    style:border-radius="6px"
                    style:font-weight="600"
                    style:transition="transform 0.2s"
                >
                    "← Back to Commands"
                </a>
            </div>
        </div>
    }
}

/// Render the stats page to HTML string
pub fn render_stats_page(config: &BunnylolConfig) -> RawHtml<String> {
    let stats = UsageStats::from_history(config).unwrap_or_else(|| UsageStats {
        total_commands: 0,
        unique_commands: 0,
        top_commands: vec![],
        least_used_commands: vec![],
        all_commands: vec![],
    });

    let html = leptos::ssr::render_to_string(move || {
        view! {
            <StatsPage stats=stats />
        }
    })
    .to_string();

    RawHtml(html)
}
