use serde::{Deserialize, de::Visitor};
use time::OffsetDateTime;

use super::{super::supplement::IdType, datetimeformat};

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
    id: IdType,
    #[serde(with = "datetimeformat")]
    created_at: OffsetDateTime,
    post_id: IdType,
    reason: String,
    creator_id: IdType,
    is_resolved: bool,
    #[serde(with = "datetimeformat")]
    updated_at: OffsetDateTime,
    is_deletion: bool,
    r#type: Type
}
