/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
mod commands;
mod routes;
mod utils;
mod web;

use utils::bunnylol_command::BunnylolCommandRegistry;

// http://localhost:8000/?cmd=gh
#[get("/?<cmd>")]
fn search(cmd: &str) -> Redirect {
    println!("bunnylol command: {}", cmd);

    let command = utils::get_command_from_query_string(cmd);
    let redirect_url = BunnylolCommandRegistry::process_command(command, cmd);
    println!("redirecting to: {}", redirect_url);

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
    let _rocket = rocket::build()
        .mount("/", routes![search, root, health, routes::bindings_web])
        .register("/", catchers![not_found])
        .launch()
        .await?;
    Ok(())
}
