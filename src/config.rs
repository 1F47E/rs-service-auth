use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "JWT_KEY")]
    pub key_base64: String,
}

impl Config {
    pub fn get() -> Self {
        Config::init_from_env().unwrap()
    }
}
    