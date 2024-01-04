// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    log::debug,
    reqwest::{Url, Client, Error as rqError, Response},
};

pub async fn get(client: Client, url: Url) -> Result<Response, rqError> {
    debug!("Fetching {url} using get...");
    let res = client.get(url).send().await?.error_for_status()?;
    debug!("Got {res:#?}");
    Ok(res)
}

pub async fn post(client: Client, url: Url, body: reqwest::multipart::Form) -> Result<Response, rqError> {
    debug!("Fetching {url} using post...");
    let res = client.post(url).multipart(body).send().await?.error_for_status()?;
    debug!("Got {res:#?}");
    Ok(res)
}

pub mod blocking {
    use {
        reqwest::blocking,
        
        super::*,
    };

    pub fn get(client: blocking::Client, url: Url) -> Result<blocking::Response, rqError> {
        debug!("Fetching {url} using get_blocking...");
        let res = client.get(url).send()?.error_for_status()?;
        debug!("Got {res:#?}");
        Ok(res)
    }

    pub fn post(client: blocking::Client, url: Url, body: blocking::multipart::Form)
        -> Result<blocking::Response, rqError> {
        debug!("Fetching {url} using post...");
        let res = client.post(url).multipart(body).send()?.error_for_status()?;
        debug!("Got {res:#?}");
        Ok(res)
    }
}
