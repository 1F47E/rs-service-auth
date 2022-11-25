#[macro_use]
extern crate rocket;

mod user;
mod routes;
mod token;
mod key;
use crate::key::Key;

#[launch]
fn rocket() -> _ {

    // check is key exist
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

    rocket::build()
        .mount("/", routes![routes::home])
        .mount("/verify", routes![routes::verify])
        .mount("/auth", routes![routes::sign_in, routes::sign_out])
        .mount("/token", routes![routes::refresh])
        .mount("/debug", routes![routes::debug_ping, routes::debug_json])
        // catch error
        .register("/", catchers![routes::not_found, routes::bad_request, routes::internal_error])
}
