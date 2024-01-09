// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    std::fmt::Display,
    
    base64::{engine::general_purpose, Engine},
    log::info,
    reqwest::header::{USER_AGENT, AUTHORIZATION, HeaderMap},
};

pub fn encode_login<T: Display>(username: T, apikey: T) -> String {
    let r = general_purpose::STANDARD.encode(format!("{username}:{apikey}").as_bytes());
    info!("Acquired login: {username}, {apikey} => {r}");
    r
}

pub fn build_header<T: Display>(username: T, apikey: T) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, format!("fluffyget/{} (by ostipyroxene@e621, AntimatterReactor@github)", env!("CARGO_PKG_VERSION")).parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Basic {}", encode_login(username, apikey)).parse().unwrap());
    info!("Built header {headers:#?}");
    headers
}
