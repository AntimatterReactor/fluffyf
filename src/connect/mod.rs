pub mod method;

use log::info;
use reqwest::{header::{HeaderMap, USER_AGENT, AUTHORIZATION}, Client};

pub fn create_client(the_header: HeaderMap) -> Result<Client, reqwest::Error> {
    let client = Client::builder().default_headers(the_header).build()?;
    info!("Created client {client:#?}");
    Ok(client)
}

pub mod blocking {
    use super::*;

    use reqwest::blocking;
    pub fn create_client(the_header: HeaderMap) -> Result<blocking::Client, reqwest::Error> {
        let client = blocking::Client::builder().default_headers(the_header).build()?;
        info!("Created client {client:#?}");
        Ok(client)
    }
}
