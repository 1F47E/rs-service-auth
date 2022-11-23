#[macro_use]
extern crate rocket;

mod user;
mod routes;
mod token;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::home])
        .mount("/auth", routes![routes::sign_in, routes::sign_out])
        .mount("/token", routes![routes::refresh])
        .mount("/debug", routes![routes::debug_ping, routes::debug_json])
}
