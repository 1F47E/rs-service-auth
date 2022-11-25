use crate::user::User;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct DB {
    users: HashMap<String, User>,
}

impl DB {
    pub fn init() -> Self {
        let path = Path::new("demo_db.json");
        // create empty file if not found
        if !path.exists() {
            fs::write(path, "{}").unwrap();
        }
        // deserialize to a HashMap
        let mut users: HashMap<String, User> =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();

        println!("loaded users: {:?}", users.len());
        if users.is_empty() {
            // generate 1000 users with login like 555000001 and qwerty1 password alike
            for n in 1..1000 {
                let uid = 555000000 + n;
                let username = String::from(format!("{}", uid));
                let key = username.clone();
                let password = format!("qwerty{}", n);
                users.insert(key, User::new(uid, username, password));
            }
            DB::save(users.clone());
        }
        DB { users: users }
    }

    pub fn get_user_by_username(self, username: &str) -> Option<User> {
        match self.users.get(username) {
            Some(user) => Some(user.clone()),
            None => None,
        }
    }

    pub fn create_user(self, username: &str, password: &str) -> Option<User> {
        let mut new_users = self.users.clone();
        match new_users.get(username) {
            Some(_) => None,
            None => {
                let uid: u32 = 555000000 + self.users.len() as u32;
                let user = User::new(uid, String::from(username), String::from(password));
                new_users.insert(String::from(username), user.clone());
                DB::save(new_users);
                Some(user)
            }
        }
    }

    // private
    fn save(users: HashMap<String, User>) {
        let path = std::path::Path::new("demo_db.json");
        let json = serde_json::to_string(&users).unwrap();
        std::fs::write(path, json).unwrap();
    }
}
