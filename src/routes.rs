use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};

use crate::user::User;
use crate::user::AuthData;
use crate::token::Token;


#[get("/")]
pub fn home(token: Token<'_>) -> &'static str {
    println!("token: {:?}", token);
    "Hello, user!"
}

// Using format = json forces “application/json” to be set
#[post("/signin", format = "json", data = "<auth_data>")]
pub fn sign_in(auth_data: Json<AuthData>) -> Json<User> {
    println!("{:#?}", &auth_data);

    Json(User::from_auth(auth_data.into_inner()))
}

#[get("/signout")]
pub fn sign_out() -> &'static str {
    "Sign out"
}

#[get("/refresh")]
pub fn refresh() -> &'static str {
    "Refresh"
}

// DEBUG HANDLERS

#[get("/ping")]
pub fn debug_ping() -> &'static str {
    "pong"
}

#[get("/json")]
pub fn debug_json() -> Value {
    json!({
        "success": true,
        "payload": {
            "ping": "pong",
        },
    })
}
