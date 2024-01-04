// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An implementation/analogue of e926's JSON replies
//! 
//! 

pub mod pools;
pub mod posts;
pub mod file;
pub mod tags;
pub mod notes;
pub mod favorites;
pub mod post_flags;
pub mod search;
pub mod traits;
pub mod supplement;

time::serde::format_description!(datetimeformat, OffsetDateTime, "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory]:[offset_minute]");

const BASE_URL621: &'static str = "https://e621.net";
const BASE_URL926: &'static str = "https://e926.net";
