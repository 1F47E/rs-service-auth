use jwt_simple::prelude::*;

use crate::config::Config;

// extern crate base64;
pub struct Key {

}

impl Key {
    pub fn read() -> HS256Key {
        let config = Config::get();
        Key::from_base64(config.key_base64).unwrap()
    }

    pub fn from_base64(val:String) -> Option<HS256Key> {
        let bytes = base64::decode(val);
        match bytes {
            Ok(bytes) => Some(HS256Key::from_bytes(&bytes)),
            Err(error) => {
                println!("key decoding Error: {:?}", error);
                None
            }
        }
    }
}
