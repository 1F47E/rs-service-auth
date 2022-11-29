use rocket::form::FromForm;
use rocket::serde::{Deserialize, Serialize};

use names::Generator;
use crate::db::PostgresPool;
use pwhash::bcrypt;
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct AuthData {
    #[field(validate = len(..3))]
    pub username: String,
    #[field(validate = len(..8))]
    pub password: String,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, sqlx::FromRow)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub fullname: String,
}

impl User {
    pub fn create_demo_user() -> Self {
        let id = 5550000001;
        let username = format!("{}", id);
        let password_string = format!("qwerty{}", id);
        let password = bcrypt::hash(password_string).unwrap();
        let fullname = User::generate_random_name();
        User {
            id,
            username,
            password,
            fullname,
        }
    }

    pub fn new(id: i64, username: String, password: String) -> Self {
        let password = bcrypt::hash(password).unwrap();
        let fullname = User::generate_random_name();
        User {
            id,
            username,
            password,
            fullname,
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
        // generator is slow!
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

    pub async fn get_by_username(mut pool: Connection<PostgresPool>, username: &str) -> Option<Self> {
        let q = "SELECT * FROM users.users WHERE username = $1";
        let user = sqlx::query_as::<_, User>(q)
            .bind(username)
            .fetch_one(&mut *pool)
            .await;
        match user {
            Ok(user) => Some(user),
            Err(e) => { 
                println!("User::get_by_username error: {}", e);
                None
            },
        }
    }
    
    pub async fn create_user(mut pool: Connection<PostgresPool>, username: &str, password: &str) -> Option<Self> {
        let password_hashed = bcrypt::hash(password).unwrap();
        let fullname = User::generate_random_name();
        // username is unique
        let q = "INSERT INTO users.users (username, password, fullname) VALUES ($1, $2, $3) RETURNING *";
        let user = sqlx::query_as::<_, User>(q)
            .bind(username)
            .bind(password_hashed)
            .bind(fullname)
            .fetch_one(&mut *pool)
            .await;
        match user {
            Ok(user) => Some(user),
            Err(e) => { 
                println!("User::create_user error: {}", e);
                None
            },
        }
    }


}
