/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use leptos::*;
use rocket::response::content::RawHtml;

use crate::web::BindingsPage;

// http://localhost:8000/bindings (Leptos SSR)
#[get("/bindings")]
pub fn bindings_web() -> RawHtml<String> {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <BindingsPage />
        }
    });

    RawHtml(html.to_string())
}
