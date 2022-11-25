// #[macro_use]
use jwt_simple::prelude::*;
// use jwt_simple::prelude::Ed25519;
use rocket::form::FromForm;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

use crate::key::Key;
use crate::user::User;

// #[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
// #[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
// #[serde(crate = "rocket::serde")]
#[derive(Serialize, Deserialize)]
pub struct TokenData {
    pub sub: u32,
    pub name: String,
    pub token_type: String,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u32,
}

impl Token {
    pub fn create_for_user(user: &User) -> Self {
        let access_token_data = TokenData {
            sub: user.id,
            name: user.username.clone(),
            token_type: "access".to_string(),
        };
        let refresh_token_data = TokenData {
            sub: user.id,
            name: user.username.clone(),
            token_type: "refresh".to_string(),
        };
        let access_token = Token::create_token(&access_token_data);
        let refresh_token = Token::create_token(&refresh_token_data);
        Token {
            access_token,
            refresh_token,
            expires_in: 60,
        }
    }
    pub fn create_token(data: &TokenData) -> String {
        let key= Key::read().unwrap();
        let claims = Claims::create(Duration::from_hours(2));
        // let claims = Claims::with_custom_claims(data, Duration::from_secs(30));
        key.authenticate(claims).unwrap()
        // let token_res = key.authenticate(claims)
        // let token = match token_res {
        //     Ok(token) => token,
        //     Err(error) => panic!("Cant create token: {:?}", error),
        // };
    }

    pub fn new() -> Self {
        // TODO: get key from env
        let key= Key::read().unwrap();

        // Construct claims
        // simple claims
        let claims = Claims::create(Duration::from_hours(2));

        // advanced claims
        // let token_data = TokenData {
        //     sub: 123,
        //     name: "test".to_string(),
        //     token_type: "access".to_string(),
        // };

        // let claims = Claims::with_custom_claims(token_data, Duration::from_secs(30));

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
            let token = header.trim_start_matches("Bearer ");

            let key = Key::read().unwrap();

            // custom claims
            // let claims = key.verify_token::<TokenData>(&token, None).unwrap();

            // simple claims
            let claims = key.verify_token::<NoCustomClaims>(&token, None);
            match claims {
                Ok(claims) => {
                    println!("Token verified, claims: {:?}", claims);
                    true
                }
                Err(error) => {
                    println!("Token error: {:?}", error);
                    false
                }
            }
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
