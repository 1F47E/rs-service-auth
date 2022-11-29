use jwt_simple::prelude::*;

use crate::config::Config;

pub struct Key;

impl Key {
    pub fn read() -> HS256Key {
        let config = Config::get();
        // decode key or panic
        let bytes = base64::decode(config.key_base64).unwrap();
        HS256Key::from_bytes(&bytes)
    }
}
