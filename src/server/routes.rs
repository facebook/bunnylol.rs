/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::OnceLock;

use leptos::*;
use rocket::response::content::RawHtml;

use super::web::BindingsPage;

static BINDINGS_HTML_CACHE: OnceLock<String> = OnceLock::new();

fn render_bindings_page() -> String {
    leptos::ssr::render_to_string(|| {
        view! {
            <BindingsPage />
        }
    }).to_string()
}

// http://localhost:8000/bindings (Leptos SSR)
#[rocket::get("/bindings")]
pub fn bindings_web() -> RawHtml<String> {
    let html = BINDINGS_HTML_CACHE.get_or_init(render_bindings_page);
    RawHtml(html.clone())
}
