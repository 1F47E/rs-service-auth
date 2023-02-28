use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::{
    http::Status,
    http::Header,
    http::ContentType,
    response::{self, Responder},
};
use rocket_db_pools::Connection;

use crate::user::AuthData;
use crate::token::Token;
use crate::user::User;
use crate::db::PostgresPool;

#[derive(Debug)]
pub struct ApiResponse {
    json: Value,
    status: Status,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

// DB custom error
struct DatabaseError(rocket_db_pools::sqlx::Error);

impl<'r> Responder<'r, 'r> for DatabaseError {
    fn respond_to(self, _request: &Request) -> response::Result<'r> {
        Err(Status::InternalServerError)
    }
}

impl From<rocket_db_pools::sqlx::Error> for DatabaseError {
    fn from(error: rocket_db_pools::sqlx::Error) -> Self {
        DatabaseError(error)
    }
}

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
pub async fn sign_in(pool: Connection<PostgresPool>, auth_data: Json<AuthData>) -> Result<Option<Json<Token>>, ApiResponse> {
    let user = User::get_by_username(pool, &auth_data.username).await;
    match user {
        Some(user) => {
            if user.check_pwd(&auth_data.password) {
                let token = Token::create_for_user(user.id);
                println!("Successful login for user: {}", user.id);
                Result::Ok(Some(Json(token)))
            } else {
                println!("Wrong password for user: {}", user.id);
                Result::Err(ApiResponse {
                    json: json!({"error": "wrong username or password"}),
                    status: Status::Unauthorized,
                })
            }
        },
        None => { 
            println!("User not found: {}", &auth_data.username);
            Result::Err(ApiResponse {
                json: json!({"error": "wrong username or password"}),
                status: Status::Unauthorized,
            })
        },
    }
}

#[post("/signup", format = "json", data = "<auth_data>")]
pub async fn sign_up(pool: Connection<PostgresPool>, auth_data: Json<AuthData>) -> Result<Option<Json<Token>>, ApiResponse> {
    let user = User::create_user(pool, &auth_data.username, &auth_data.password).await;
    match user {
        Some(user) => {
            let token = Token::create_for_user(user.id);
            println!("Successful created and logged in user: {}", user.id);
            Result::Ok(Some(Json(token)))
        }
        None => {
            println!("User already exist: {}", &auth_data.username);
            Result::Err(ApiResponse {
                json: json!({"error": "user already exist"}),
                status: Status::Conflict,
            })
        }
    }
}

#[get("/refresh")]
pub fn refresh() -> &'static str {
    // TODO: implement refresh token
    "Refresh"
}

#[get("/me")]
pub async fn me() -> &'static str {
    // TODO: check user auth and return user data or 401
    "Me"
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
