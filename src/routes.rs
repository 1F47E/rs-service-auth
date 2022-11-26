use rocket::response::status::Unauthorized;
use rocket::response::status::Conflict;

use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

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

    let user = DB::init().get_user_by_username(auth_data.username.clone());
    match user {
        Some(user) => {
            if user.check_pwd(&auth_data.password) {
                let token = Token::create_for_user(&user);
                println!("Successful login for user: {}", user.id);
                Result::Ok(Some(Json(token)))
            } else {
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
pub fn sign_up(auth_data: Json<AuthData>) -> Result<Option<Json<Token>>, Conflict<Value>> {
    // same as sign in but creating user
    println!("creating user: {:?}", auth_data);
    let user = DB::init().get_user_by_username(auth_data.username.clone());
    match user {
        Some(_) => {
            println!("User already exist: {}", &auth_data.username);
            Result::Err(Conflict(Some(json!({
                "status": "error",
                "reason": "user already exist"
            }))))
        }
        None => {
            let user = DB::init().create_user(&auth_data.username, &auth_data.password).unwrap();
            let token = Token::create_for_user(&user);
            println!("Successful created and logged in user: {}", user.id);
            Result::Ok(Some(Json(token)))
        }
    }
}

#[get("/refresh")]
pub fn refresh() -> &'static str {
    "Refresh"
}
// CORS
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("CORS Fairing triggered");
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
pub fn all_options() {
    println!("Options triggered");
    /* Intentionally left empty */
}


// ERROR CATCHERS

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
