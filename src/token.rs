// #[macro_use]
use rocket::form::FromForm;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};


#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct Token<'r>(&'r str);

// #[derive(Debug)]
// pub struct Token {
//     pub token: String,
//     pub refresh_token: String,
//     pub expires_in: u64,
// }

impl Token<'_> {
    fn create_from_header<'a>(header: &'a str) -> Token<'a> {
        let token = header.trim_start_matches("Bearer ");
        Token(token)
    }
}

#[derive(Debug)]
pub enum ApiAuthError {
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
