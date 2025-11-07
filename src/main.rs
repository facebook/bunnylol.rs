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
    println!("You typed in {}", cmd);

    let command = utils::get_command_from_query_string(cmd);
    let redirect_url = BunnylolCommandRegistry::process_command(command, cmd);

    Redirect::to(redirect_url)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![search, routes::bindings_web],
        )
        .launch()
        .await?;
    Ok(())
}
