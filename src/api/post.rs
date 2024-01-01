use std::{str::FromStr, cmp::Ordering};

use serde::{Deserialize, de::Visitor};
use time::OffsetDateTime;

use super::{tags::{TagObject, TagType}, file::{FileObject, PreviewObject, SampleObject}, supplement::IdType, traits::*, datetimeformat};

pub const POSTS_URL: &'static str = "posts.json";

#[derive(Debug, Eq, Deserialize)]
pub struct Score {
    up: i32,   // As of 2023, the most upvoted post [2848682] is at +21425 raw whereas
    down: i32, // the most downvoted post (don't view, seriously) [378180] is at -7033 raw
    total: i32
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total.cmp(&other.total)
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total
    }
}

impl Score {
    fn count_total(&mut self) -> i32 {
        self.total = self.up + self.down;
        self.total
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Flags {
    pending: bool,
    flagged: bool,
    note_locked: bool,
    status_locked: bool,
    rating_locked: bool,
    deleted: bool
}

#[derive(Debug, PartialEq, Eq)]
pub enum Rating {
    SAFE,
    QUESTIONABLE,
    EXPLICIT
}

impl<'de> Deserialize<'de> for Rating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        struct RatingVisitor;
        
        impl<'de> Visitor<'de> for RatingVisitor {
            type Value = Rating;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string equals to either `s`, `q`, or `e`")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                match v {
                    "s" => Ok(Self::Value::SAFE),
                    "q" => Ok(Self::Value::QUESTIONABLE),
                    "e" => Ok(Self::Value::EXPLICIT),
                    _ => Err(E::custom(format!("unexpected rating {v}")))
                }
            }
        }
        deserializer.deserialize_str(RatingVisitor)
    }
}

// TODO: impl Serialize for Rating

#[derive(Debug, Deserialize)]
pub struct Relation {
    parent_id: Option<IdType>,
    has_children: bool,
    has_active_children: bool,
    children: Vec<IdType>
}

#[derive(Debug, Deserialize)]
pub struct PostObject {
    id: IdType,
    #[serde(with = "datetimeformat")]
    created_at: OffsetDateTime,
    #[serde(with = "datetimeformat")]
    updated_at: OffsetDateTime,
    file: FileObject,
    preview: PreviewObject,
    sample: SampleObject,
    score: Score,
    tags: TagObject,
    locked_tags: TagType,
    change_seq: u32,
    flags: Flags,
    rating: Rating,
    fav_count: u32,
    sources: Vec<String>,
    pools: Vec<IdType>,
    relationships: Relation,
    approver_id: Option<IdType>,
    uploader_id: IdType,
    description: String,
    comment_count: u16,
    is_favorited: bool,
    has_notes: bool,
    duration: Option<f32>
}

#[derive(Debug, Deserialize)]
pub struct PostObjectWrapper {
    post: PostObject
}

impl Create for PostObject {
    
}

impl Update for PostObject {

}

impl List for PostObject {

}

impl PostObject {
    async fn vote(&self, score: i8, no_unvote: bool) {

    }
}
