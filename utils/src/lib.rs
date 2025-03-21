// this is shit but who cares its only loaded once
macro_rules! load_env {
    ($env:expr, $($field:ident : $type:ty => $key:expr),*) => {{
        use std::str::FromStr;
        let mut env_struct = DotEnv::default();
        $(
            env_struct.$field = match <$type>::from_str(
                &$env.get($key).expect(concat!($key, " is not set in .env"))
            ) {
                Ok(value) => value,
                Err(_) => panic!(concat!("Invalid value for ", $key)),
            };
        )*

        if ![0,1,2].contains(&env_struct.log_level) {
            env_struct.log_level = 0;
        }

        env_struct
    }};
}

#[derive(Default)]
pub struct DotEnv {
    pub database_url: String,
    pub sdk_sv_host: String,
    pub sdk_sv_port: u16,
    pub game_sv_host: String,
    pub game_sv_port: u16,
    pub login_env_type: i32,
    pub log_level: i32,
    pub auto_hotfix: bool,
}

impl DotEnv {
    pub fn load() -> Self {
        let env = envie::Envie::load().unwrap();

        load_env! {
            env,
            database_url: String => "DATABASE_URL",
            sdk_sv_host: String => "SDK_SV_HOST",
            sdk_sv_port: u16 => "SDK_SV_PORT",
            game_sv_host: String => "GAME_SV_HOST",
            game_sv_port: u16 => "GAME_SV_PORT",
            login_env_type: i32 => "LOGIN_ENV_TYPE",
            log_level: i32 => "LOG_LEVEL",
            auto_hotfix: bool => "AUTO_HOTFIX"
        }
    }
}

pub fn init_tracing() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().expect("Failed to enable ansi support");

    tracing_subscriber::fmt().init();
}

pub fn cur_timestamp_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn cur_timestamp_for_seed() -> u32 {
    (chrono::Utc::now().timestamp() / 5)
        .try_into()
        .expect("Timestamp integer overflow")
}

pub fn cur_date_full_precision() -> String {
    chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.6fZ")
        .to_string()
}

pub fn hash_password(pass: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(pass, bcrypt::DEFAULT_COST)
}

pub fn verify_password(pass: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(pass, hash)
}

pub fn rand_u32_hex() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    format!("{:X}", rng.random::<u32>())
}
