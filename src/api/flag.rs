use time::Date;

use super::supplement::IdType;

pub const FLAGS_URL: &'static str = "post_flags.json";

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    DELETE,
    FLAG
}

#[derive(Debug)]
pub struct FlagObject {
    id: IdType,
    created_at: Date,
    post_id: IdType,
    reason: String,
    creator_id: IdType,
    is_resolved: bool,
    updated_at: Date,
    is_deletion: bool,
    r#type: Type
}
