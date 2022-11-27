#[macro_use]
extern crate rocket;

mod config;
mod db;
mod key;
mod routes;
mod token;
mod user;

use crate::config::Config;
use crate::db::DB;


#[launch]
fn rocket() -> _ {

    let config = Config::get();
    // assert if envs are not set
    assert!(!config.key_base64.is_empty());

    // init and fill demo db
    _ = DB::init();


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
        .mount("/", routes![routes::all_options])
        .attach(routes::Cors)
}
