#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::request::{FromRequest, Outcome, Request };
use rocket::http::Status;


// USER

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
struct User {
    // #[field(validate = len(..30))]
    // pub room: String,
    pub id: u32,
    #[field(validate = len(..3))]
    pub username: String,
    #[field(validate = len(..8))]
    pub password: String,
}

impl User {
    pub fn new(id: u32, username: String, password: String) -> Self {
        User {
            id,
            username,
            password,
        }
    }
    pub fn from(id: usize) -> Self {
        let username = String::from("test");
        let password = String::from("test");
        User::new(id as u32, username, password)
    }
}

// TOKEN + GUARD

#[derive(Debug)]
struct Token<'r>(&'r str);

impl Token<'_> {
    fn create_from_header<'a>(header: &'a str) -> Token<'a> {
        let token = header.trim_start_matches("Bearer ");
        Token(token)
    }
}

#[derive(Debug)]
enum ApiAuthError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = ApiAuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if header contains valid auth token
        fn is_valid(header: &str) -> bool {
            // cut bearer from the header
            let token = header.trim_start_matches("Bearer ");
            token == "test"
        }

        match req.headers().get_one("Authorization") {
            // if not found
            None => Outcome::Failure((Status::BadRequest, ApiAuthError::Missing)),
            // if found check if valid
            Some(val) if is_valid(val) => Outcome::Success(Token::create_from_header(val)),
            // if not valid
            Some(_) => Outcome::Failure((Status::BadRequest, ApiAuthError::Invalid)),
        }
    }
}

// ROUTES

// home route with guard (Authoization header)
#[get("/")]
fn home(token: Token<'_>) -> &'static str {
    println!("token: {:?}", token);
    "Hello, user!"
}

// Using format = json forces “application/json” to be set
#[post("/signin", format = "json", data = "<user>")]
fn sign_in(user: Json<User>) -> Json<User> {
    println!("{:#?}", user);

    user
}

#[get("/signout")]
fn sign_out() -> &'static str {
    "Sign out"
}

#[get("/refresh")]
fn refresh() -> &'static str {
    "Refresh"
}

#[get("/ping")]
fn debug_ping() -> &'static str {
    "pong"
}

#[get("/json")]
fn debug_json() -> Value {
    json!({
        "success": true,
        "payload": {
            "ping": "pong",
        },
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home])
        .mount("/auth", routes![sign_in, sign_out])
        .mount("/token", routes![refresh])
        .mount("/debug", routes![debug_ping, debug_json])
}
