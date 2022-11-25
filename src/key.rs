use jwt_simple::prelude::*;
use std::fs;

// extern crate base64;
pub struct Key {

}

impl Key {
    pub fn read() -> Option<HS256Key> {
        let path = std::path::Path::new("key/HS256Key.key");
        let bytes = fs::read(path);
        match bytes {
            Ok(bytes) => {
                match bytes.len() {
                    0 => None,
                    _ => Some(HS256Key::from_bytes(&bytes))
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
                None
            }
        }
    }
    pub fn write(bytes:Vec<u8>) {
        let path = std::path::Path::new("key/HS256Key.key");
        fs::write(path, bytes).unwrap();
    }
    pub fn to_base64(bytes:Vec<u8>) -> String {
        base64::encode(bytes.clone())
    }
    pub fn from_base64(val:String) -> Option<HS256Key> {
        let bytes = base64::decode(val);
        match bytes {
            Ok(bytes) => Some(HS256Key::from_bytes(&bytes)),
            Err(error) => {
                println!("Error: {:?}", error);
                None
            }
        }
    }
    pub fn gen() -> HS256Key {
        HS256Key::generate()
    }
}