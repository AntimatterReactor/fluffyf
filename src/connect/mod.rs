// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod methods;

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
