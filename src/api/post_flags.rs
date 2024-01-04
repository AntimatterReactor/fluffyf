// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    serde::Deserialize,
    time::OffsetDateTime,

    super::{datetimeformat, supplement::IdType},
};

pub const FLAGS_URL: &'static str = "post_flags.json";

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Type {
    Delete,
    Flag
}

#[derive(Debug, Deserialize)]
pub struct FlagObject {
    pub id: IdType,
    #[serde(with = "datetimeformat")]
    pub created_at: OffsetDateTime,
    pub post_id: IdType,
    pub reason: String,
    pub creator_id: IdType,
    pub is_resolved: bool,
    #[serde(with = "datetimeformat")]
    pub updated_at: OffsetDateTime,
    pub is_deletion: bool,
    pub r#type: Type
}
