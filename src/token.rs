// #[macro_use]
use rocket::form::FromForm;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};
use jwt_simple::prelude::*;

// #[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
// #[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
// #[serde(crate = "rocket::serde")]
// pub struct Token<'r>(&'r str);


#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

impl Token {
    pub fn new() -> Self {
        // TODO: returns HS256Key, get from env later
        // HS256Key::from_bytes()
        let key = HS256Key::generate();
        // key.to_bytes() // save
        let claims = Claims::create(Duration::from_hours(2));
        // returns result or error Result<String, Error>
        let token_res = key.authenticate(claims);
        let token = match token_res {
            Ok(token) => token,
            Err(error) => panic!("Cant create token: {:?}", error),
        };


        Token {
            access_token: token.clone(),
            refresh_token: token.clone(),
            expires_in: 7200,
        }
    }

    fn create_from_header<'a>(header: &'a str) -> Token {
        let token = header.trim_start_matches("Bearer ");
        Token {
            access_token: String::from(token),
            refresh_token: String::from(token),
            expires_in: 7200,
        }
    }

}

#[derive(Debug)]
pub enum ApiAuthError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiAuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if header contains valid auth token
        fn is_valid(header: &str) -> bool {
            // cut bearer from the header
            let token = header.trim_start_matches("Bearer ");
            // let claims = key.verify_token::<NoCustomClaims>(&self.access_token, None)?;
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
