// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A form of API abstraction
//! 
//! 

use reqwest::{header::HeaderMap, Url};

pub mod rate_limit;

#[derive(Debug)]
pub struct Client<L: rate_limit::Limiter = rate_limit::DefaultRateLimit> {
    pub(crate) client: reqwest::Client,
    limit: L,
    url: Url,
}

impl<L> Client<L>
    where L: rate_limit::Limiter {
    
}
