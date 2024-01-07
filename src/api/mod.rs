// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An implementation/analogue of e926's JSON replies
//! 
//! The `api` module and all it's submodule provides structs that implements
//! Serde's `Deserialize` trait.
//! 
//! What this means in less technical jargon, is that when you want to turn json
//! acquired from requesting to E926 into legible rust form, you use the descendants
//! of this module specifically.
//! 
//! It is not recommended to use `api` objects directly for most high-level purposes,
//! instead, refer to [`Client`](crate::client::Client)

mod pool;
pub use self::pool::*;

mod post;
pub use self::post::*;

mod file;
pub use self::file::*;

mod tags;
pub use self::tags::*;

mod note;
pub use self::note::*;

mod favorites;
pub use self::favorites::*;

mod post_flag;
pub use self::post_flag::*;

mod search;
pub use self::search::*;

mod traits;
pub use self::traits::*;

mod supplement;
pub use self::supplement::*;

time::serde::format_description!(datetimeformat, OffsetDateTime, "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory]:[offset_minute]");

const BASE_URL621: &'static str = "https://e621.net";
const BASE_URL926: &'static str = "https://e926.net";
