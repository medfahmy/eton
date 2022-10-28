use std::fmt::Debug;
use std::str::FromStr;
use anyhow::Result;

fn get_env_var<T>(env_var_name: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let var = std::env::var(env_var_name).unwrap();
    var.parse().unwrap()
}

#[derive(Debug)]
pub struct Config {
    pub env: String,
    pub host: String,
    pub port: u16,
    pub database_uri: String,
    pub auth_secret: String,
}

pub fn load() -> Result<Config> {
    todo!()
}
