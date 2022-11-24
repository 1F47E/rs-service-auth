use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};

use rocket::http::Status;
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
pub fn sign_in(auth_data: Json<AuthData>) -> Result<Value, Unauthorized<Value>> {
    // println!("{:#?}", &auth_data);

    // let user;
    if auth_data.username == "test" && auth_data.password == "test" {
        // TODO create JWT token
        // user = User::new(1, auth_data.username.clone(), auth_data.password.clone())
        println!("user: {:?}", auth_data);
        let demo_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.MhQjMm2TlY0uesawWXiQ9dIisMx5yAB7voGCWTeDkO8";
        // TODO: make cucstom responder with success/error/data
        Result::Ok(json!({"access_token": demo_token }))
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
