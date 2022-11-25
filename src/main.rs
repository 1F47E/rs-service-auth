#[macro_use]
extern crate rocket;

mod user;
mod routes;
mod token;
mod key;
mod db;
use crate::key::Key;
use crate::db::DB;

#[launch]
fn rocket() -> _ {

    let key = Key::read();
    match key {
        Some(_) => println!("Key exist"),
        None => {
            println!("Key not exist, creating");
            let key = Key::gen();
            Key::write(key.to_bytes());
            println!("Key created");
        }
    }

    // init and fill demo db
    _ = DB::init();


    rocket::build()
        .mount("/", routes![routes::home])
        .mount("/auth", routes![routes::sign_in, routes::sign_up])
        .mount("/token", routes![routes::verify, routes::refresh])
        .mount("/debug", routes![routes::debug_ping, routes::debug_json])
        // catch error
        .register("/", catchers![routes::not_found, routes::bad_request, routes::internal_error])
}
