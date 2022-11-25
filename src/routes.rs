use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};

use crate::user::AuthData;
use crate::token::Token;
use crate::db::DB;

#[get("/")]
pub fn home(token: Token) -> &'static str {
    println!("token: {:?}", token);
    "Hello, user!"
}

#[get("/verify")]
pub fn verify(token: Token) -> &'static str {
    println!("token: {:?}", token);
    "Hello, user!"
}

// Using format = json forces “application/json” to be set
#[post("/signin", format = "json", data = "<auth_data>")]
pub fn sign_in(auth_data: Json<AuthData>) -> Result<Option<Json<Token>>, Unauthorized<Value>> {
    // println!("{:#?}", &auth_data);

    let user = DB::get_user_by_username(&auth_data.username);
    match user {
        Some(user) => {
            if user.check_pwd(&auth_data.password) {
                let token = Token::create_for_user(&user);
                println!("Successful login for user: {}", user.id);
                Result::Ok(Some(Json(token)))
            } else {
                // print error wrong password for user id
                println!("Wrong password for user: {}", user.id);
                Result::Err(Unauthorized(Some(json!({
                    "status": "error",
                    "reason": "wrong password"
                }))))
            }
        }
        None => {
            println!("User not found: {}", &auth_data.username);
            Result::Err(Unauthorized(Some(json!({
            "status": "error",
            "reason": "user not found"
        }))))}
    }
    
}

#[post("/signup", format = "json", data = "<auth_data>")]
pub fn sign_up(auth_data: Json<AuthData>) -> Option<Json<Token>> {
    // same as sign in but creating user
    println!("creating user: {:?}", auth_data);
    // check if already exists
    // create user
    // sign in user
    let token = Token::new();
    Some(Json(token))
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

#[catch(500)]
pub fn internal_error() -> Value {
    json!({
        "success": false,
        "error": "Internal Server Error"
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
