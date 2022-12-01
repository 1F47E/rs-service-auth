#[macro_use]
extern crate rocket;

mod config;
mod db;
mod key;
mod routes;
mod token;
mod user;

use crate::db::PostgresPool;
use crate::key::Key;


use rocket_db_pools::Database;

#[launch]
fn rocket() -> _ {
    // will panic if no key is set
    _ = Key::read();

    rocket::build()
        .mount("/", routes![routes::home])
        .mount("/auth", routes![routes::sign_in, routes::sign_up])
        .mount("/token", routes![routes::verify, routes::refresh])
        .mount("/debug", routes![routes::debug_ping, routes::debug_json])
        // error catchers
        .register(
            "/",
            catchers![
                routes::not_found,
                routes::bad_request,
                routes::internal_error
            ],
        )
        // CORS stuff
        // for debug only
        .attach(routes::Cors)
        .mount("/", routes![routes::all_options])
        .attach(PostgresPool::init())
}
