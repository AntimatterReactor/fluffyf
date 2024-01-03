// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::{Deserialize, de::Visitor};
use time::OffsetDateTime;

use super::{datetimeformat, supplement::IdType};

pub const FLAGS_URL: &'static str = "post_flags.json";

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Delete,
    Flag
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        struct TypeVisitor;
        
        impl<'de> Visitor<'de> for TypeVisitor {
            type Value = Type;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string equal to either `delete` or `flag`")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                match v {
                    "delete" => Ok(Self::Value::Delete),
                    "flag" => Ok(Self::Value::Flag),
                    _ => Err(E::custom(format!("unexpected type {v}")))
                }
            }
        }
        deserializer.deserialize_str(TypeVisitor)
    }
}

// TODO: impl Serialize for Type

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
