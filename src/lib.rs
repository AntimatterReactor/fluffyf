// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A wrapper to E621 and [E926]'s API
//! 
//! fluffyf allows for very simple and painless abstracted API requests
//! to the mature furry imageboard E621 and the safer [E926]
//! 
//! Note: most of the documentation will use [E926] instead of E621
//! for obvious reason
//! 
//! Another note: this documentation will only ever discuss fluffyf the
//! library, for how to use the binaries, refer to [the README]
//! 
//! # Usage
//! 
//! 
//! 
//! [E926]: https://e926.net
//! [the README]: https://github.com/AntimatterReactor/fluffyf/blob/main/README.md

extern crate reqwest;
extern crate bytes;
extern crate serde;
extern crate log;
extern crate time;
extern crate futures;

pub mod api;
pub mod connect;
pub mod client;

pub(crate) type RqResult<T> = Result<T, reqwest::Error>;

// pub mod utils;
