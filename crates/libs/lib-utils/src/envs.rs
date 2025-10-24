use crate::b64::b64u_decode;
use std::env;
use std::str::FromStr;

// region:    --- Error

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, strum_macros::Display)]
pub enum Error {
    MissingEnv(&'static str),
    WrongFormat(&'static str),
}

// endregion: --- Error

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::MissingEnv(name))
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| Error::WrongFormat(name))
}

pub fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    b64u_decode(&get_env(name)?).map_err(|_| Error::WrongFormat(name))
}
