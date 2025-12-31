/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// Server runtime (routes, web UI) - only needed for server feature
#[cfg(feature = "server")]
pub mod metrics;
#[cfg(feature = "server")]
pub mod routes;
#[cfg(feature = "server")]
pub mod stats;
#[cfg(feature = "server")]
pub mod web;

// Service management - only needed for CLI feature
#[cfg(feature = "cli")]
pub mod service;

// Server runtime code below only compiled with server feature
#[cfg(feature = "server")]
use rocket::State;
#[cfg(feature = "server")]
use rocket::request::{self, FromRequest, Request};
#[cfg(feature = "server")]
use rocket::response::Redirect;

#[cfg(feature = "server")]
use crate::{BunnylolCommandRegistry, BunnylolConfig, History, utils};

#[cfg(feature = "server")]
mod server_impl {
    use super::*;

    // Request guard to extract client IP address
    pub(super) struct ClientIP(pub String);

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
    pub(super) fn search(
        cmd: &str,
        config: &State<BunnylolConfig>,
        client_ip: ClientIP,
    ) -> Redirect {
        // Track active requests
        super::metrics::increment_active_requests();
        let start_time = std::time::Instant::now();

        println!("bunnylol command: {}", cmd);

        let command = utils::get_command_from_query_string(cmd);
        let redirect_url = BunnylolCommandRegistry::process_command_with_config(
            command,
            cmd,
            Some(config.inner()),
        );
        println!("redirecting to: {}", redirect_url);

        // Track command in history if enabled
        if config.history.enabled
            && let Some(history) = History::new(config.inner())
            && let Err(e) = history.add(cmd, &client_ip.0)
        {
            eprintln!("Warning: Failed to save command to history: {}", e);
        }

        // Track metrics
        let duration = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
        super::metrics::track_request(command, true);
        super::metrics::track_request_duration(command, duration);
        super::metrics::track_command_usage(command);
        super::metrics::decrement_active_requests();

        Redirect::to(redirect_url)
    }

    // Root path without query parameters -> redirect to bindings
    #[rocket::get("/", rank = 2)]
    pub(super) fn root() -> Redirect {
        Redirect::to("/bindings")
    }

    // Health check endpoint for Docker healthcheck (no verbose logging)
    #[rocket::get("/health")]
    pub(super) fn health() -> &'static str {
        "ok"
    }

    // Prometheus metrics endpoint
    #[rocket::get("/metrics")]
    pub(super) fn metrics_endpoint() -> (rocket::http::Status, String) {
        (rocket::http::Status::Ok, super::metrics::get_metrics())
    }

    // Usage statistics dashboard
    #[rocket::get("/stats")]
    pub(super) fn stats_web(
        config: &State<BunnylolConfig>,
    ) -> rocket::response::content::RawHtml<String> {
        web::render_stats_page(config.inner())
    }

    // Catch 404 errors and redirect to bindings page
    #[rocket::catch(404)]
    pub(super) fn not_found() -> Redirect {
        Redirect::to("/bindings")
    }
}

#[cfg(feature = "server")]
use server_impl::*;

/// Launch the Bunnylol web server with the given configuration
#[cfg(feature = "server")]
pub async fn launch(config: BunnylolConfig) -> Result<(), Box<rocket::Error>> {
    // Initialize metrics exporter
    metrics::init_metrics();

    println!(
        "Bunnylol server starting with default search: {}",
        config.default_search
    );
    println!(
        "Server listening on {}:{}",
        config.server.address, config.server.port
    );

    let figment = rocket::Config::figment()
        .merge(("address", config.server.address.clone()))
        .merge(("port", config.server.port))
        .merge(("log_level", config.server.log_level.clone()))
        .merge(("ident", format!("Bunnylol/{}", env!("CARGO_PKG_VERSION"))));

    let _rocket = rocket::custom(figment)
        .manage(config)
        .mount(
            "/",
            rocket::routes![
                search,
                root,
                health,
                metrics_endpoint,
                stats_web,
                routes::bindings_web
            ],
        )
        .register("/", rocket::catchers![not_found])
        .launch()
        .await?;
    Ok(())
}
