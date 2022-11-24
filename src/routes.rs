use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};

use rocket::http::Status;
use crate::user::User;
use crate::user::AuthData;
use crate::token::Token;


#[get("/")]
pub fn home(token: Token) -> &'static str {
    println!("token: {:?}", token);
    "Hello, user!"
}

// Using format = json forces “application/json” to be set
#[post("/signin", format = "json", data = "<auth_data>")]
pub fn sign_in(auth_data: Json<AuthData>) -> Result<Option<Json<Token>>, Unauthorized<Value>> {
    // println!("{:#?}", &auth_data);

    if auth_data.username == "test" && auth_data.password == "test" {
        // TODO create JWT token
        // user = User::new(1, auth_data.username.clone(), auth_data.password.clone())
        println!("user: {:?}", auth_data);
        let token = Token::new();
        // Result::Ok(json!({"access_token": demo_token }))
        Result::Ok(Some(Json(token)))
    } else {
        Result::Err(Unauthorized(Some(json!({"status": "error" }))))
    }
}

#[get("/signout")]
pub fn sign_out() -> &'static str {
    "Sign out"
}

#[get("/refresh")]
pub fn refresh() -> &'static str {
    "Refresh"
}

// CATCHERS

#[catch(400)]
pub fn bad_request() -> Value {
    json!({
        "success": false,
        "error": "bad request"
    })
}

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "success": false,
        "error": "not found."
    })
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
