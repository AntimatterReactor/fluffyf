use std::str::FromStr;

use time::Date;

use super::{tags::TagObject, file::FileObject};

#[derive(Debug, PartialEq, Eq)]
pub struct Score {
    up: u32,   // As of 2023, the most upvoted post [2848682] is at +21425 raw whereas
    down: u32, // the most downvoted post (don't view, seriously) [378180] is at -7033 raw
    total: u32 // Note: unlike in the json, this will store only absolute values
}

#[derive(Debug, PartialEq, Eq)]
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

impl FromStr for Rating {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(Self::SAFE),
            "q" => Ok(Self::QUESTIONABLE),
            "e" => Ok(Self::EXPLICIT),
            _ => Err("Unexpected str".to_string()) // TODO: Make an actual error struct
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Relation {
    parent_id: Option<u32>,
    has_children: bool,
    has_active_children: bool,
    children: Option<Vec<u32>>
}

#[derive(Debug, PartialEq)]
pub struct PostObject {
    id: u32,
    created_at: Date,
    updated_at: Date,
    file: FileObject,
    preview: FileObject,
    sample: Option<FileObject>,
    score: Score,
    tags: TagObject,
    change_seq: u32,
    flags: Flags,
    rating: Rating,
    fav_count: u32,
    sources: Vec<String>,
    pools: Option<Vec<u32>>,
    relationships: Relation,
    approver_id: Option<u32>,
    uploader_id: u32,
    description: String,
    comment_count: bool,
    is_favorited: bool,
    has_notes: bool,
    // duration: // Undocumented by e621.net, observed to always be null, as such not implemented
}

impl PostObject {
    
}
