use serde::Deserialize;

pub const TAGS_URL: &'static str = "tags.json";

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    GENERAL = 0,
    ARTIST,
    COPYRIGHT,
    CHARACTER,
    SPECIES,
    INVALID,
    META,
    LORE = 8,
}

pub type TagType = Vec<String>;

#[derive(Debug, Deserialize)]
pub struct TagObject {
    general: TagType,
    artist: TagType,
    copyright: TagType,
    character: TagType,
    species: TagType,
    invalid: TagType,
    meta: TagType,
    lore: TagType,
}

impl TagObject {
    
}
