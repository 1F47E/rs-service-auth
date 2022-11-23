#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

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
        User { id, username, password }
    }
    pub fn from(id: usize) -> Self {
        let username = String::from("test");
        let password = String::from("test");
        User::new(id as u32, username, password)
    }
    
    // pub fn Deserialize(data: String) -> Self {
    //     let user: User = serde_json::from_str(&data).unwrap();
    //     user
    // }

    // pub fn Deserialize(&self) -> String {
    //     let json = serde_json::to_string(&self).unwrap();
    //     json
    // }
}

// ROUTES
#[get("/")]
fn home() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn ping() -> &'static str {
    "pong"
}

#[get("/<id>")]
fn user(id: usize) -> Json<User> {
    let user_from_id = User::from(id);
    // let user = User {
    //     id: id as u64,
    //     login: String::from("test"),
    //     password: String::from("test"),
    // };
    // let user_from_id = User
    /* ... */
    Json(user_from_id)
}


// Using format = json means that any request that doesn’t specify “application/json”
// as its Content-Type header value will not be routed to the handler.
#[post("/", format = "json", data = "<creds>")]
fn sign_in(creds: Json<User>) -> Json<User> {
    println!("{:#?}", creds);
    // let user = User {
    //     id: 1,
    //     username: creds.username,
    //     password: creds.password,
    // };
    // Json(creds)
    creds
}


// add post handler with json
// #[post("/", format = "json", data = "<user_input>")]
// fn sign_in(user_input: Json<User>) -> String {
//     // return out json data pretty
//     println!("{:#?}", user_input);
//     // Json(data)
//     user_input
// }

// #[get("/signin")]
// fn sign_in() -> &'static str {
//     "Sign in"
// }

#[get("/signout")]
fn sign_out() -> &'static str {
    "Sign out"
}

#[get("/refresh")]
fn refresh() -> &'static str {
    "Refresh"
}


#[launch]
fn rocket() -> _ {

    rocket::build()
        .mount("/", routes![home])
        .mount("/ping", routes![ping])
        .mount("/signin", routes![sign_in])
        .mount("/signout", routes![sign_out])
        .mount("/refresh", routes![refresh])
        .mount("/user", routes![user])
}