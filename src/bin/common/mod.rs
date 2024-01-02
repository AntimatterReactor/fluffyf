use std::fmt::Display;

use base64::{engine::general_purpose, Engine};
use log::info;
use reqwest::header::{USER_AGENT, AUTHORIZATION, HeaderMap};

pub fn encode_login<T: Display>(username: T, apikey: T) -> String {
    let r = general_purpose::STANDARD.encode(format!("{username}:{apikey}").as_bytes());
    info!("Acquired login: {username}, {apikey} => {r}");
    r
}

pub fn build_header<T: Display>(username: T, apikey: T) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, format!("fluffyf/0.0.4 (by ostipyroxene@e621, AntimatterReactor@github)").parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Basic {}", encode_login(username, apikey)).parse().unwrap());
    info!("Built header {headers:#?}");
    headers
}
