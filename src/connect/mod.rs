pub mod method;

use std::fmt::Display;

use base64::{engine::general_purpose, Engine};
use log::info;
use reqwest::{header::{HeaderMap, USER_AGENT, AUTHORIZATION}, Client};

pub fn encode_login<T: Display>(username: T, apikey: T) -> String {
    let r = general_purpose::STANDARD.encode(format!("{username}:{apikey}").as_bytes());
    info!("Acquired login: {username}, {apikey} => {r}");
    r
}

pub fn build_header(auth: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "BulkDownProject/1.2 (by ostipyroxene on e621)".parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Basic {}", auth).parse().unwrap());
    info!("Built header {headers:#?}");
    headers
}

pub fn create_client(the_header: HeaderMap) -> Result<Client, reqwest::Error> {
    let client = Client::builder().default_headers(the_header).build()?;
    info!("Created client {client:#?}");
    Ok(client)
}


