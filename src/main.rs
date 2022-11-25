#[macro_use]
extern crate rocket;

mod db;
mod key;
mod routes;
mod token;
mod user;
use crate::db::DB;
use crate::key::Key;



#[launch]
fn rocket() -> _ {
    // read the key from the file
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
