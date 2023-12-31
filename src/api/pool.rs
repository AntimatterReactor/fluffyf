use std::str::FromStr;
use time::Date;

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    SERIES,
    COLLECTION
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "series" => Ok(Self::SERIES),
            "collection" => Ok(Self::COLLECTION),
            _ => Err("Unexpected str".to_string()) // TODO: Make an actual error struct
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PoolObject {
    id: u32,
    name: String,
    created_at: Date,
    updated_at: Date,
    creator_id: u32,
    description: String,
    is_active: bool,
    category: Category,
    post_ids: Vec<u32>,
    creator_name: String,
    post_count: u16
}

impl PoolObject {

}
