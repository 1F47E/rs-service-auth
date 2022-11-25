
use crate::user::User;

pub struct DB {

}

impl DB {
    // returns hashmap of demo users with username as a key
    pub fn demo_users() -> std::collections::HashMap<String, User> {
        let mut users = std::collections::HashMap::new();
        for n in 1..1000 {
            let uid = 555000000+n;
            let username = String::from(format!("{}", uid));
            let key = username.clone();
            let password = format!("qwerty{}", n);
            users.insert(key, User::new(uid, username, password));
        }
        users
    }
    pub fn get_user_by_username(username: &str) -> Option<User> {
        let users = DB::demo_users();
        match users.get(username) {
            Some(user) => Some(user.clone()),
            None => None
        }
    }
}