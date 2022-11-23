#[macro_use] extern crate rocket;


// ROUTES
#[get("/")]
fn home() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn ping() -> &'static str {
    "pong"
}

struct User {
    user_email: String,
    user_password: String,
}



#[get("/signin")]
fn sign_in() -> &'static str {
    "Sign in"
}

#[get("/signout")]
fn sign_out() -> &'static str {
    "Sign out"
}

#[get("/refresh")]
fn refresh() -> &'static str {
    "Refresh"
}


#[launch]
fn rocket() -> _ {

    rocket::build()
        .mount("/", routes![home])
        .mount("/ping", routes![ping])
        .mount("/signin", routes![sign_in])
        .mount("/signout", routes![sign_out])
        .mount("/refresh", routes![refresh])
}