/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#[macro_use]
extern crate rocket;

use rocket::State;
use rocket::response::Redirect;
use rocket::request::{self, Request, FromRequest};
mod routes;
mod web;

use bunnylol::{BunnylolCommandRegistry, BunnylolConfig, History, utils};

// Request guard to extract client IP address
struct ClientIP(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientIP {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let ip = req
            .client_ip()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        request::Outcome::Success(ClientIP(ip))
    }
}

// http://localhost:8000/?cmd=gh
#[get("/?<cmd>")]
fn search(cmd: &str, config: &State<BunnylolConfig>, client_ip: ClientIP) -> Redirect {
    println!("bunnylol command: {}", cmd);

    let command = utils::get_command_from_query_string(cmd);
    let redirect_url =
        BunnylolCommandRegistry::process_command_with_config(command, cmd, Some(config.inner()));
    println!("redirecting to: {}", redirect_url);

    // Track command in history if enabled
    if config.history.enabled {
        if let Some(history) = History::new(config.inner()) {
            if let Err(e) = history.add(cmd, &client_ip.0) {
                eprintln!("Warning: Failed to save command to history: {}", e);
            }
        }
    }

    Redirect::to(redirect_url)
}

// Root path without query parameters -> redirect to bindings
#[get("/", rank = 2)]
fn root() -> Redirect {
    Redirect::to("/bindings")
}

// Health check endpoint for Docker healthcheck (no verbose logging)
#[get("/health")]
fn health() -> &'static str {
    "ok"
}

// Catch 404 errors and redirect to bindings page
#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to("/bindings")
}

#[rocket::main]
async fn main() -> Result<(), Box<rocket::Error>> {
    // Load configuration once at startup
    let config = BunnylolConfig::load().unwrap_or_else(|e| {
        eprintln!("(ignorable) warning: Failed to load config: {}", e);
        eprintln!("Using default configuration...");
        BunnylolConfig::default()
    });

    println!(
        "Bunnylol server starting with default search: {}",
        config.default_search
    );

    let _rocket = rocket::build()
        .manage(config)
        .mount("/", routes![search, root, health, routes::bindings_web])
        .register("/", catchers![not_found])
        .launch()
        .await?;
    Ok(())
}
