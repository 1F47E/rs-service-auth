use rocket::form::FromForm;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct AuthData {
    // #[field(validate = len(..3))]
    pub username: String,
    // #[field(validate = len(..8))]
    pub password: String,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: u32,
    pub username: String,
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
    pub fn check_pwd(&self, password: &str) -> bool {
        self.password == password
    }

    pub fn from_auth(form: AuthData) -> Self {
        // let username = form.username;
        // let password = form.password;
        User::new(0, form.username, form.password)
    }
}
