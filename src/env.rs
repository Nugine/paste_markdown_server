use lazy_static::lazy_static;
use short_crypt::ShortCrypt;
use std::env;

pub const ADDR_KEY: &'static str = "PASTE_MARKDOWN_ADDR";
pub const TIMEOUT_KEY: &'static str = "PASTE_MARKDOWN_TIMEOUT";
pub const SECRET_KEY: &'static str = "PASTE_MARKDOWN_SECRET";
pub const MAXSIZE_KEY: &'static str = "PASTE_MARKDOWN_MAXSIZE";

fn read(key: &str) -> String {
    env::var(key).expect(&format!("Env Error: Can not read {}", key))
}

fn parse<T>(src: String, key: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    src.parse()
        .expect(&format!("Env Error: Can not parse {}", key))
}

lazy_static! {
    pub static ref ADDR: String = read(ADDR_KEY);
    pub static ref TIMEOUT: i64 = parse(read(TIMEOUT_KEY), TIMEOUT_KEY);
    pub static ref SECRET: String = read(SECRET_KEY);
    pub static ref MAXSIZE: usize = parse(read(MAXSIZE_KEY), MAXSIZE_KEY);
    pub static ref short_cryptor: ShortCrypt = ShortCrypt::new(&SECRET.clone());
}
