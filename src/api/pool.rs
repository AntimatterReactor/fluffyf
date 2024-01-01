use std::str::FromStr;
use time::Date;

use super::supplement::IdType;

pub const POOLS_URL: &'static str = "pools.json";

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    Series,
    Collection
}

#[derive(Debug, PartialEq, Eq)]
pub enum CategoryError {
    Default
}

impl FromStr for Category {
    type Err = CategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "series" => Ok(Self::Series),
            "collection" => Ok(Self::Collection),
            _ => Err(Self::Err::Default)
        }
    }
}

#[derive(Debug)]
pub struct PoolObject {
    id: IdType,
    name: String,
    created_at: Date,
    updated_at: Date,
    creator_id: IdType,
    description: String,
    is_active: bool,
    category: Category,
    post_ids: Vec<IdType>,
    creator_name: String,
    post_count: u16
}

impl PoolObject {

}
