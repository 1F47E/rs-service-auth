use rocket::form::FromForm;
use rocket::serde::{Deserialize, Serialize};

use pwhash::bcrypt;
use names::Generator;

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
    pub id: u64,
    pub username: String,
    pub password: String,
    pub fullname: String,
}

impl User {
    pub fn new(id: u64, username: String, password: String) -> Self {
        let hashed_pwd = bcrypt::hash(password).unwrap();
        let fullname = User::generate_random_name();
        User {
            id,
            username,
            password:hashed_pwd,
            fullname
        }
    }

    fn uppercase_first_letter(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        }
    }

    pub fn generate_random_name() -> String {
        // generate random name (slow!)
        let mut generator = Generator::default();
        let gen_name = generator.next().unwrap();
        let names = gen_name.replace("-", " ");
        // split the name into first and last and capitalize the first letter
        let mut split = names.split_whitespace();
        let first = split.next().unwrap();
        let last = split.next().unwrap();
        let first_name = User::uppercase_first_letter(first);
        let last_name = User::uppercase_first_letter(last);
        format!("{} {}", first_name, last_name)
    }
    pub fn check_pwd(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password)
    }
}
