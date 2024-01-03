// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An abstraction library to easily fetch/post/modify/delete e621.net content
//! 
//! fluffyf allows for very simple and painless abstracted API
//! requests to the mature furry imageboard e621.net and e926.net
//! 
//! 
//! 
//! 
//! 
//! 
//! 

extern crate reqwest;
extern crate bytes;
extern crate tokio;
extern crate serde;
extern crate log;
extern crate time;
extern crate async_trait;
extern crate futures;

pub mod api;
pub mod connect;
pub mod utils;
