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
    pub general: TagType,
    pub artist: TagType,
    pub copyright: TagType,
    pub character: TagType,
    pub species: TagType,
    pub invalid: TagType,
    pub meta: TagType,
    pub lore: TagType,
}

impl TagObject {
    
}
