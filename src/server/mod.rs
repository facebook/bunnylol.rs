/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod routes;
pub mod web;

use rocket::State;
use rocket::response::Redirect;
use rocket::request::{self, Request, FromRequest};

use crate::{BunnylolCommandRegistry, BunnylolConfig, History, utils};

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
#[rocket::get("/?<cmd>")]
fn search(cmd: &str, config: &State<BunnylolConfig>, client_ip: ClientIP) -> Redirect {
    println!("bunnylol command: {}", cmd);

    let command = utils::get_command_from_query_string(cmd);
    let redirect_url =
        BunnylolCommandRegistry::process_command_with_config(command, cmd, Some(config.inner()));
    println!("redirecting to: {}", redirect_url);

    // Track command in history if enabled
    if config.history.enabled
        && let Some(history) = History::new(config.inner())
        && let Err(e) = history.add(cmd, &client_ip.0)
    {
        eprintln!("Warning: Failed to save command to history: {}", e);
    }

    Redirect::to(redirect_url)
}

// Root path without query parameters -> redirect to bindings
#[rocket::get("/", rank = 2)]
fn root() -> Redirect {
    Redirect::to("/bindings")
}

// Health check endpoint for Docker healthcheck (no verbose logging)
#[rocket::get("/health")]
fn health() -> &'static str {
    "ok"
}

// Catch 404 errors and redirect to bindings page
#[rocket::catch(404)]
fn not_found() -> Redirect {
    Redirect::to("/bindings")
}

/// Launch the Bunnylol web server with the given configuration
pub async fn launch(config: BunnylolConfig) -> Result<(), Box<rocket::Error>> {
    println!(
        "Bunnylol server starting with default search: {}",
        config.default_search
    );
    println!("Server configured for {}:{}", config.server.address, config.server.port);

    // Configure Rocket with address and port from config
    // Environment variables can override config file values
    let address = std::env::var("ROCKET_ADDRESS")
        .unwrap_or_else(|_| config.server.address.clone());
    let port: u16 = std::env::var("ROCKET_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(config.server.port);
    let log_level = std::env::var("ROCKET_LOG_LEVEL")
        .unwrap_or_else(|_| config.server.log_level.clone());

    let figment = rocket::Config::figment()
        .merge(("address", address))
        .merge(("port", port))
        .merge(("log_level", log_level));

    let _rocket = rocket::custom(figment)
        .manage(config)
        .mount("/", rocket::routes![search, root, health, routes::bindings_web])
        .register("/", rocket::catchers![not_found])
        .launch()
        .await?;
    Ok(())
}
